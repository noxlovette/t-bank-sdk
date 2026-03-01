#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Configuration error (e.g., env variables).
    #[error("configuration error: {0}")]
    Config(String),

    #[error("timeout")]
    Timeout,

    #[error("network error: {0}")]
    Network(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("too many requests")]
    TooManyRequests,

    #[error("server error: {0}")]
    Server(String),

    #[error("api error: {0}")]
    Api(String),

    #[error("deserialization error at {path}: {message}\nraw body: {raw}")]
    Deserialize {
        message: String,
        path: String,
        raw: String,
    },
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(err.to_string())
    }
}
