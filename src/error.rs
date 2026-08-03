pub type HandlerResult<T> = Result<ErrorWrapper<T>, Error>;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ErrorCode(pub String);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration Error: {0}")]
    Config(String),

    // `reqwest::Error`'s `Display` never includes the underlying cause (DNS
    // failure, TLS handshake failure, connection reset, timeout, ...) —
    // only its `Debug` impl does (kind/url/source fields). `{0:?}` is
    // deliberate here, not a mistake.
    #[error("Reqwest Error: {0:?}")]
    Server(#[from] reqwest::Error),

    #[error("API Error {error_code}. {message:?}; {details:?}; {causes:?}")]
    Api {
        error_code: String,
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

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ErrorWrapper<T> {
    #[cfg_attr(feature = "serde", serde(flatten))]
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
                error_code: error_code.0,
                message: Some("error code 0 but got no body".to_string()),
                causes,
                details,
            })
        } else {
            Err(Error::Api {
                error_code: error_code.0,
                message,
                causes,
                details,
            })
        }
    }
}
