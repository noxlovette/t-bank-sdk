use crate::{Error, HandlerResult, InitPaymentReq, InitPaymentRes, TokenWrapper};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::debug;

pub const PRODUCTION_BASE: &str = "https://securepay.tinkoff.ru/v2";
pub const TEST_BASE: &str = "https://rest-api-test.tinkoff.ru/v2";

#[derive(Clone, Debug, Default)]
pub enum Environment {
    Test,
    #[default]
    Production,
}

impl From<&str> for Environment {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "PRODUCTION" => Self::Production,
            "TEST" => Self::Test,
            _ => Self::Test,
        }
    }
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        Environment::from(s.as_str())
    }
}
impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => PRODUCTION_BASE,
            Environment::Test => TEST_BASE,
        }
    }

    fn from_env() -> Self {
        std::env::var("TBANK_ENV")
            .unwrap_or(String::from("Test"))
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) client: reqwest::Client,
    env: Environment,
    credentials: Option<Credentials>,
}

#[derive(Debug, Clone)]
struct Credentials {
    terminal_key: TerminalKey,
    password: Password,
}

/// Requirements: <= 20 characters
///
/// Идентификатор терминала. Выдается мерчанту в Т‑Бизнес при заведении терминала.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct TerminalKey(String);

impl TerminalKey {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.is_empty() || value.len() > 20 {
            return Err(Error::Config(
                "TerminalKey must be between 1 and 20 characters".to_string(),
            ));
        }

        Ok(Self(value))
    }

    /// Gets and validates the terminal key from the environment
    fn from_env() -> Result<Self, Error> {
        let tk = std::env::var("TERMINAL_ID")
            .map_err(|_| Error::Config("TERMINAL_ID variable is missing".to_string()))?;
        Self::new(tk)
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.is_empty() {
            return Err(Error::Config("Password must not be empty".to_string()));
        }

        Ok(Self(value))
    }

    fn from_env() -> Result<Self, Error> {
        let p = std::env::var("TBANK_PASSWORD")
            .map_err(|_| Error::Config("TBANK_PASSWORD variable is missing".to_string()))?;

        Self::new(p)
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl From<&Password> for String {
    fn from(value: &Password) -> Self {
        value.0.clone()
    }
}

impl Client {
    /// Создать клиента для указанного окружения.  
    pub async fn new() -> Result<Self, Error> {
        let version = env!("CARGO_PKG_VERSION");

        debug!("Initializing T-Bank SDK client v{version}");

        let env = Environment::from_env();
        let terminal_key = TerminalKey::from_env()?;
        let password = Password::from_env()?;
        let credentials = Credentials {
            terminal_key,
            password,
        };

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(format!("tbank-rust-sdk/{version}"))
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        debug!("Reqwest client constructed with standard timeouts");

        Ok(Self {
            client,
            env,
            credentials: Some(credentials),
        })
    }

    /// Создать клиента для "external" режима.
    ///
    /// В этом режиме пароль/терминал не хранятся в клиенте и должны
    /// передаваться в методы вида `*_with_credentials`.
    pub async fn external() -> Result<Self, Error> {
        let version = env!("CARGO_PKG_VERSION");

        debug!("Initializing T-Bank SDK external client v{version}");

        let env = Environment::from_env();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(format!("tbank-rust-sdk/{version}"))
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        Ok(Self {
            client,
            env,
            credentials: None,
        })
    }

    pub fn password(&self) -> &Password {
        &self
            .credentials
            .as_ref()
            .expect("password is not configured in external mode")
            .password
    }

    pub fn terminal_key(&self) -> &TerminalKey {
        &self
            .credentials
            .as_ref()
            .expect("terminal key is not configured in external mode")
            .terminal_key
    }

    pub fn has_stored_credentials(&self) -> bool {
        self.credentials.is_some()
    }
}

impl Client {
    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", self.env.base_url(), path.trim_start_matches('/'))
    }

    fn credentials_required(&self) -> Result<&Credentials, Error> {
        self.credentials.as_ref().ok_or_else(|| {
            Error::Config(
                "Client has no stored credentials. Use `Client::new()` or call `initiate_payment_with_credentials` in external mode."
                    .to_string(),
            )
        })
    }

    async fn initiate_payment_inner(
        &self,
        payload: InitPaymentReq,
        password: &Password,
    ) -> HandlerResult<InitPaymentRes> {
        let req = TokenWrapper::from_payload(payload, password);

        println!("{:?}", req);
        let res = self
            .client
            .post(self.url("Init"))
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        println!("{:?}", res);

        Ok(res)
    }

    pub async fn initiate_payment(&self, payload: InitPaymentReq) -> HandlerResult<InitPaymentRes> {
        let credentials = self.credentials_required()?;
        self.initiate_payment_inner(payload, &credentials.password).await
    }

    pub async fn initiate_payment_with_credentials(
        &self,
        mut payload: InitPaymentReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<InitPaymentRes> {
        payload.terminal_key = terminal_key.clone();
        self.initiate_payment_inner(payload, password).await
    }
}
