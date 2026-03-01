use crate::{Error, InitPaymentReq, InitPaymentRes, newtype};
use std::time::Duration;
use tracing::debug;

pub const PRODUCTION_BASE: &str = "https://securepay.tinkoff/v2/Init";
pub const TEST_BASE: &str = "https://rest-api-test.tinkoff/v2/Init";

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

newtype! {
    /// Requirements: <= 20 characters
    ///
    /// Идентификатор терминала. Выдается мерчанту в Т‑Бизнес при заведении терминала.
    #[derive(Clone)]
    pub struct TerminalKey(String);
}

impl TerminalKey {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

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
impl Client {
    /// Создать клиента для указанного окружения.  
    pub async fn new() -> Result<Self, Error> {
        tracing_subscriber::fmt::init();

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

    pub fn password(&self) -> String {
        self.password.0.clone()
    }

    pub fn terminal_key(&self) -> String {
        self.terminal_key.0.clone()
    }
}

impl Client {
    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", self.env.base_url(), path.trim_start_matches('/'))
    }

    pub async fn initiate_payment(&self, req: InitPaymentReq) -> Result<InitPaymentRes, Error> {
        let res = self
            .client
            .post(self.url("Init"))
            .json(&req)
            .send()
            .await?
            .json::<InitPaymentRes>()
            .await?;

        Ok(res)
    }
}
