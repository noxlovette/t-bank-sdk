use std::ops::Deref;

use crate::TBankApiError;

pub type HandlerResult<T> = Result<ErrorWrapper<T>, Error>;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ErrorCode(String);

impl Deref for ErrorCode {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

    #[error("API Error {}. {message:?}; {details:?}; {causes:?}", kind.code())]
    Api {
        kind: TBankApiError,
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
        if self.error_code.0 == "0" {
            match self.inner {
                Some(inner) => Ok(inner),
                None => Err(Error::Api {
                    kind: TBankApiError::from(self.error_code.0.as_str()),
                    message: Some("error code 0 but got no body".to_string()),
                    causes: self.causes,
                    details: self.details,
                }),
            }
        } else {
            Err(self.into())
        }
    }

    /// `Success` — non-consuming, unlike [`Self::unwrap`].
    pub fn success(&self) -> bool {
        self.success
    }

    /// `ErrorCode` — non-consuming, unlike [`Self::unwrap`].
    pub fn error_code(&self) -> &str {
        &self.error_code
    }

    /// `Message`, if T-Bank sent one.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// The typed payload, present whenever T-Bank included one —
    /// regardless of `error_code`/`success`.
    ///
    /// Unlike [`Self::unwrap`] (meant for *outbound* request responses,
    /// where a non-`"0"` error code means the call genuinely failed and
    /// carries no usable payload), an inbound webhook notification
    /// legitimately carries a non-`"0"` `ErrorCode`/`Success: false`
    /// alongside a fully-populated body — e.g. a declined payment still
    /// reports its `Status`/`PaymentId`/etc. Callers processing
    /// notifications should read the payload through this accessor (or
    /// [`Self::into_inner`]), not `unwrap()`, which would incorrectly
    /// discard it.
    pub fn inner(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    /// Owned version of [`Self::inner`], for a caller that no longer needs
    /// the wrapper (e.g. after already reading [`Self::success`]/
    /// [`Self::error_code`]/[`Self::verify_token`]).
    pub fn into_inner(self) -> Option<T> {
        self.inner
    }
}

impl<T> From<ErrorWrapper<T>> for Error {
    fn from(wrapper: ErrorWrapper<T>) -> Self {
        Error::Api {
            kind: TBankApiError::from(wrapper.error_code.0),
            message: wrapper.message,
            details: wrapper.details,
            causes: wrapper.causes,
        }
    }
}
