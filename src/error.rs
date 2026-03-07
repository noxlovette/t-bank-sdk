use serde::{Deserialize, Serialize};
pub type HandlerResult<T> = Result<ErrorWrapper<T>, Error>;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ErrorCode(pub String);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("reqwest error: {0}")]
    Server(#[from] reqwest::Error),

    #[error("api error: {0}")]
    Api(String),
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(err.to_string())
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorWrapper<T> {
    #[serde(flatten)]
    inner: Option<T>,
    /// Requirements: <= 255 characters
    ///
    /// Краткое описание ошибки.
    message: Option<String>,
    /// Подробное описание ошибки.
    details: Option<String>,
    /// Не указанное в API поле
    causes: Option<Vec<String>>,
    /// Requirements: <= 20 characters
    ///
    /// Код ошибки.
    error_code: ErrorCode,
    /// Успешность прохождения запроса — true/false.
    success: bool,
}

impl<T> ErrorWrapper<T> {
    pub fn unwrap(self) -> Result<T, Error> {
        let Self {
            inner,
            message,
            error_code,
            ..
        } = self;

        if error_code.0 == "0" {
            Ok(inner.unwrap_or_else(|| unreachable!()))
        } else {
            Err(Error::Api(message.unwrap_or(error_code.0)))
        }
    }
}
