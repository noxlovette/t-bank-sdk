use serde::{Deserialize, Serialize};
pub type HandlerResult<T> = Result<ErrorWrapper<T>, Error>;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ErrorCode(pub String);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Reqwest Error: {0}")]
    Server(#[from] reqwest::Error),

    #[error("API Error. {message:?}; {details:?}; {causes:?}")]
    Api {
        message: Option<String>,
        details: Option<String>,
        causes: Option<Vec<String>>,
    },

    #[error("Simple API Error: {0}")]
    SimpleApi(String),
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
            causes,
            details,
            ..
        } = self;

        if error_code.0 == "0" {
            inner.ok_or_else(|| Error::Api {
                message: Some("error code 0 but got no body".to_string()),
                causes,
                details,
            })
        } else {
            Err(Error::Api {
                message: Some(message.unwrap_or(error_code.0)),
                causes,
                details,
            })
        }
    }
}
