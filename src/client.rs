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
    password: Password,
    terminal_key: TerminalKey,
}

/// Requirements: <= 20 characters
///
/// Идентификатор терминала. Выдается мерчанту в Т‑Бизнес при заведении терминала.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct TerminalKey(String);

impl TerminalKey {
    /// Gets and validates the terminal key from the environment
    fn from_env() -> Result<Self, Error> {
        let tk = std::env::var("TERMINAL_ID")
            .ok()
            .filter(|t| t.len() <= 20)
            .ok_or_else(|| Error::Config("TERMINAL_ID variable is missing".to_string()))?;

        Ok(Self(tk))
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    fn from_env() -> Result<Self, Error> {
        let p = std::env::var("TBANK_PASSWORD")?;

        Ok(Self(p))
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
            terminal_key,
            password,
            env,
        })
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn terminal_key(&self) -> &TerminalKey {
        &self.terminal_key
    }
}

impl Client {
    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", self.env.base_url(), path.trim_start_matches('/'))
    }

    pub async fn initiate_payment(&self, payload: InitPaymentReq) -> HandlerResult<InitPaymentRes> {
        let req = TokenWrapper::from_payload(payload, &self.password());

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
}
