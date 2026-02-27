/// RU: Возможные ошибки SDK.  
/// EN: All possible errors produced by the SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// RU: Ошибка конфигурации (например, переменные окружения). EN: Configuration error (e.g., env variables).
    #[error("configuration error: {0}")]
    Config(String),

    /// RU: Превышено время ожидания запроса. EN: Request timed out.
    #[error("timeout")]
    Timeout,

    /// RU: Ошибка сети (подробности внутри). EN: Network error details.
    #[error("network error: {0}")]
    Network(String),

    /// RU: Токен недействителен или отсутствует. EN: Unauthorized (missing/invalid token).
    #[error("unauthorized")]
    Unauthorized,

    /// RU: Недостаточно прав или запрет. EN: Forbidden.
    #[error("forbidden")]
    Forbidden,

    /// RU: Ресурс не найден. EN: Resource not found.
    #[error("not found")]
    NotFound,

    /// RU: Превышен лимит запросов. EN: Too many requests.
    #[error("too many requests")]
    TooManyRequests,

    /// RU: Ошибка на стороне сервера Tochka. EN: Server-side error.
    #[error("server error: {0}")]
    Server(String),

    /// RU: Ошибка прикладного уровня API (код не 2xx). EN: API returned non-success response.
    #[error("api error: {0}")]
    Api(String),

    /// RU: Ошибка десериализации ответа API. EN: Failed to deserialize API response.
    #[error("deserialization error at {path}: {message}\nraw body: {raw}")]
    Deserialize {
        /// RU: Текст ошибки. EN: Error message.
        message: String,
        /// RU: Путь в JSON, где произошла ошибка. EN: JSON path of the error.
        path: String,
        /// RU: Исходное тело ответа. EN: Raw response body.
        raw: String,
    },
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(err.to_string())
    }
}
