use crate::Error;
use serde::{Deserialize, Serialize};
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
        match s {
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
}

#[derive(Debug)]
pub struct Client {
    pub(crate) client: reqwest::Client,
    env: Environment,
}

/// Requirements: <= 20 characters
///
/// Идентификатор терминала. Выдается мерчанту в Т‑Бизнес при заведении терминала.
#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalKey(String);

impl TerminalKey {
    /// Gets and validates the terminal key from the environment
    fn from_env() -> Result<Self, Error> {
        let tk = std::env::var("TERMINAL_KEY")
            .ok()
            .filter(|t| t.len() <= 20)
            .ok_or_else(|| Error::Config("TERMINAL_KEY variable is missing".to_string()))?;

        Ok(Self(tk))
    }
}
impl Client {
    /// Создать клиента для указанного окружения.  
    pub async fn new() -> Result<Self, Error> {
        let version = env!("CARGO_PKG_VERSION");

        tracing_subscriber::fmt::init();

        debug!("Initializing T-Bank SDK client v{version}");

        let env: Environment = std::env::var("TOCHKA_ENV")
            .unwrap_or(String::from("Test"))
            .into();

        debug!(
            "Environment resolved as {:?}, base URL {}",
            env,
            env.base_url()
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(format!("tbank-rust-sdk/{version}"))
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        debug!("Reqwest client constructed with standard timeouts");

        Ok(Self { client, env })
    }
}

impl Client {
    /// RU: Собрать полный URL для сервиса/версии/пути.  
    /// EN: Build a fully-qualified URL for the given service, version and path.
    pub fn url(&self, service: Service, version: ApiVersion, path: &str) -> String {
        format!(
            "{}{}/{}/{}",
            self.env.base_url(),
            service.path(),
            version.as_str(),
            path.trim_start_matches('/')
        )
    }
}

impl Client {
    /// RU: Отправить запрос: добавить авторизацию, проверить HTTP-статусы и десериализовать тело.  
    /// EN: Send a request with auth, map HTTP errors, and deserialize the body.
    pub async fn send<T>(&self, req: reqwest::RequestBuilder) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let request_snapshot = req.try_clone().and_then(|builder| builder.build().ok());
        if let Some(snapshot) = request_snapshot.as_ref() {
            debug!(
                "Sending {} request to {}",
                snapshot.method(),
                snapshot.url()
            );
        } else {
            debug!("Sending request (unable to snapshot builder)");
        }
        let resp = req.bearer_auth(&self.token).send().await.map_err(|e| {
            if e.is_timeout() {
                debug!("Request timed out: {e}");
                Error::Timeout
            } else {
                debug!("Network error: {e}");
                Error::Network(e.without_url().to_string())
            }
        })?;

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default(); // always capture raw JSON
        if let Some(snapshot) = request_snapshot {
            debug!(
                "Response for {} {} returned status {}",
                snapshot.method(),
                snapshot.url(),
                status
            );
        } else {
            debug!("Response received with status {}", status);
        }
        debug!("Raw response body: {body}");

        match status {
            reqwest::StatusCode::UNAUTHORIZED => {
                debug!("API responded with Unauthorized");
                return Err(Error::Unauthorized);
            }
            reqwest::StatusCode::FORBIDDEN => {
                debug!("API responded with Forbidden");
                return Err(Error::Forbidden);
            }
            reqwest::StatusCode::NOT_FOUND => {
                debug!("API responded with NotFound");
                return Err(Error::NotFound);
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                debug!("API responded with TooManyRequests");
                return Err(Error::TooManyRequests);
            }
            code if code.is_server_error() => {
                debug!("API responded with server error");
                return Err(Error::Server(body));
            }
            _ => {}
        }

        if !status.is_success() {
            debug!("API responded with non-success status {}", status);
            return Err(Error::Api(body));
        }

        // ------- Enhanced Deserialization --------
        let mut deserializer = serde_json::Deserializer::from_str(&body);

        match serde_path_to_error::deserialize::<_, T>(&mut deserializer) {
            Ok(result) => {
                debug!("Deserialization succeeded for {}", type_name::<T>());
                Ok(result)
            }
            Err(err) => {
                let path = err.path().to_string();
                let inner = err.into_inner();
                debug!(
                    "Deserialization error for {} at {path}: {inner}",
                    type_name::<T>()
                );

                Err(Error::Deserialize {
                    message: inner.to_string(),
                    path,
                    raw: body,
                })
            }
        }
    }
}
