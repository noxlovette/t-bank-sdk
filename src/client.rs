use crate::{
    AddCardReq, AddCardRes, AddCustomerReq, AddCustomerRes, CancelPaymentReq, CancelPaymentRes,
    CardInfo, ChargePaymentReq, ChargePaymentRes, ConfirmPaymentReq, ConfirmPaymentRes,
    DeriveToken, Error, GetCardListReq, GetCustomerReq, GetCustomerRes, GetStateReq, GetStateRes,
    HandlerResult, InitPaymentReq, InitPaymentRes, RemoveCardReq, RemoveCardRes, RemoveCustomerReq,
    RemoveCustomerRes, ResendNotificationReq, ResendNotificationRes, SendClosingReceiptReq,
    SendClosingReceiptRes, TokenWrapper,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ops::Deref, time::Duration};
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
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
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

impl Deref for TerminalKey {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Password {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
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

    /// Получить пароль терминала.
    pub fn password(&self) -> &Password {
        &self
            .credentials
            .as_ref()
            .expect("password is not configured in external mode")
            .password
    }

    /// Получить идентификатор терминала.
    pub fn terminal_key(&self) -> &TerminalKey {
        &self
            .credentials
            .as_ref()
            .expect("terminal key is not configured in external mode")
            .terminal_key
    }

    /// Проверить, настроены ли учетные данные в клиенте.
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
                "Client has no stored credentials. Use `Client::new()` or call the `_with_credentials` variant in external mode."
                    .to_string(),
            )
        })
    }
}

#[cfg(feature = "serde")]
impl Client {
    /// Generic signed POST → JSON response.
    async fn post_json<Req, Res>(
        &self,
        path: &str,
        payload: Req,
        password: &Password,
    ) -> HandlerResult<Res>
    where
        Req: DeriveToken + Serialize,
        Res: for<'de> Deserialize<'de>,
    {
        let req = TokenWrapper::from_payload(payload, password);
        debug!("POST /{path}");
        let res = self
            .client
            .post(self.url(path))
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    // ─── Init ────────────────────────────────────────────────────────────────

    pub async fn initiate_payment(&self, payload: InitPaymentReq) -> HandlerResult<InitPaymentRes> {
        let credentials = self.credentials_required()?;
        self.post_json("Init", payload, &credentials.password).await
    }

    pub async fn initiate_payment_with_credentials(
        &self,
        mut payload: InitPaymentReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<InitPaymentRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("Init", payload, password).await
    }

    // ─── Confirm ─────────────────────────────────────────────────────────────

    /// Подтвердить двухстадийный платеж (перевести из AUTHORIZED в CONFIRMED).
    pub async fn confirm_payment(
        &self,
        payload: ConfirmPaymentReq,
    ) -> HandlerResult<ConfirmPaymentRes> {
        let credentials = self.credentials_required()?;
        self.post_json("Confirm", payload, &credentials.password)
            .await
    }

    pub async fn confirm_payment_with_credentials(
        &self,
        mut payload: ConfirmPaymentReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<ConfirmPaymentRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("Confirm", payload, password).await
    }

    // ─── Cancel ──────────────────────────────────────────────────────────────

    /// Отменить или вернуть платеж. Для частичного возврата передайте `Amount`.
    pub async fn cancel_payment(
        &self,
        payload: CancelPaymentReq,
    ) -> HandlerResult<CancelPaymentRes> {
        let credentials = self.credentials_required()?;
        self.post_json("Cancel", payload, &credentials.password)
            .await
    }

    pub async fn cancel_payment_with_credentials(
        &self,
        mut payload: CancelPaymentReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<CancelPaymentRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("Cancel", payload, password).await
    }

    // ─── Charge ──────────────────────────────────────────────────────────────

    /// Провести рекуррентный платеж по сохраненным реквизитам.
    pub async fn charge_payment(
        &self,
        payload: ChargePaymentReq,
    ) -> HandlerResult<ChargePaymentRes> {
        let credentials = self.credentials_required()?;
        self.post_json("Charge", payload, &credentials.password)
            .await
    }

    pub async fn charge_payment_with_credentials(
        &self,
        mut payload: ChargePaymentReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<ChargePaymentRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("Charge", payload, password).await
    }

    // ─── GetState ────────────────────────────────────────────────────────────

    /// Получить текущий статус платежа.
    pub async fn get_payment_state(&self, payload: GetStateReq) -> HandlerResult<GetStateRes> {
        let credentials = self.credentials_required()?;
        self.post_json("GetState", payload, &credentials.password)
            .await
    }

    pub async fn get_payment_state_with_credentials(
        &self,
        mut payload: GetStateReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<GetStateRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("GetState", payload, password).await
    }

    // ─── SendClosingReceipt ───────────────────────────────────────────────────

    /// Отправить закрывающий чек.
    pub async fn send_closing_receipt(
        &self,
        payload: SendClosingReceiptReq,
    ) -> HandlerResult<SendClosingReceiptRes> {
        let credentials = self.credentials_required()?;
        self.post_json("SendClosingReceipt", payload, &credentials.password)
            .await
    }

    pub async fn send_closing_receipt_with_credentials(
        &self,
        mut payload: SendClosingReceiptReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<SendClosingReceiptRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("SendClosingReceipt", payload, password)
            .await
    }

    // ─── ResendNotification ───────────────────────────────────────────────────

    /// Повторно отправить все невыполненные уведомления.
    pub async fn resend_notification(
        &self,
        payload: ResendNotificationReq,
    ) -> HandlerResult<ResendNotificationRes> {
        let credentials = self.credentials_required()?;
        self.post_json("Resend", payload, &credentials.password)
            .await
    }

    // ─── AddCustomer ─────────────────────────────────────────────────────────

    /// Зарегистрировать покупателя.
    pub async fn add_customer(&self, payload: AddCustomerReq) -> HandlerResult<AddCustomerRes> {
        let credentials = self.credentials_required()?;
        self.post_json("AddCustomer", payload, &credentials.password)
            .await
    }

    pub async fn add_customer_with_credentials(
        &self,
        mut payload: AddCustomerReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<AddCustomerRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("AddCustomer", payload, password).await
    }

    // ─── GetCustomer ─────────────────────────────────────────────────────────

    /// Получить данные покупателя.
    pub async fn get_customer(&self, payload: GetCustomerReq) -> HandlerResult<GetCustomerRes> {
        let credentials = self.credentials_required()?;
        self.post_json("GetCustomer", payload, &credentials.password)
            .await
    }

    pub async fn get_customer_with_credentials(
        &self,
        mut payload: GetCustomerReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<GetCustomerRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("GetCustomer", payload, password).await
    }

    // ─── RemoveCustomer ───────────────────────────────────────────────────────

    /// Удалить покупателя и все его привязанные карты.
    pub async fn remove_customer(
        &self,
        payload: RemoveCustomerReq,
    ) -> HandlerResult<RemoveCustomerRes> {
        let credentials = self.credentials_required()?;
        self.post_json("RemoveCustomer", payload, &credentials.password)
            .await
    }

    pub async fn remove_customer_with_credentials(
        &self,
        mut payload: RemoveCustomerReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<RemoveCustomerRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("RemoveCustomer", payload, password).await
    }

    // ─── AddCard ─────────────────────────────────────────────────────────────

    /// Инициировать привязку карты к покупателю.
    pub async fn add_card(&self, payload: AddCardReq) -> HandlerResult<AddCardRes> {
        let credentials = self.credentials_required()?;
        self.post_json("AddCard", payload, &credentials.password)
            .await
    }

    pub async fn add_card_with_credentials(
        &self,
        mut payload: AddCardReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<AddCardRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("AddCard", payload, password).await
    }

    // ─── GetCardList ──────────────────────────────────────────────────────────

    /// Получить список привязанных карт покупателя.
    ///
    /// T-Bank returns a JSON array on success and a standard error object on
    /// failure, so this method uses its own deserialization path.
    pub async fn get_card_list(&self, payload: GetCardListReq) -> Result<Vec<CardInfo>, Error> {
        let credentials = self.credentials_required()?;
        self.get_card_list_inner(payload, &credentials.password)
            .await
    }

    pub async fn get_card_list_with_credentials(
        &self,
        mut payload: GetCardListReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> Result<Vec<CardInfo>, Error> {
        payload.terminal_key = terminal_key.clone();
        self.get_card_list_inner(payload, password).await
    }

    async fn get_card_list_inner(
        &self,
        payload: GetCardListReq,
        password: &Password,
    ) -> Result<Vec<CardInfo>, Error> {
        let req = TokenWrapper::from_payload(payload, password);
        debug!("POST /GetCardList");
        let bytes = self
            .client
            .post(self.url("GetCardList"))
            .json(&req)
            .send()
            .await?
            .bytes()
            .await?;

        // Success → JSON array; error → standard error object.
        let value: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|e| Error::SimpleApi(e.to_string()))?;

        if value.is_array() {
            serde_json::from_value(value).map_err(|e| Error::SimpleApi(e.to_string()))
        } else {
            let code = value["ErrorCode"].as_str().unwrap_or("unknown");
            let msg = value["Message"].as_str().unwrap_or("unknown error");
            Err(Error::SimpleApi(format!("{code}: {msg}")))
        }
    }

    // ─── RemoveCard ───────────────────────────────────────────────────────────

    /// Удалить привязанную карту покупателя.
    pub async fn remove_card(&self, payload: RemoveCardReq) -> HandlerResult<RemoveCardRes> {
        let credentials = self.credentials_required()?;
        self.post_json("RemoveCard", payload, &credentials.password)
            .await
    }

    pub async fn remove_card_with_credentials(
        &self,
        mut payload: RemoveCardReq,
        terminal_key: &TerminalKey,
        password: &Password,
    ) -> HandlerResult<RemoveCardRes> {
        payload.terminal_key = terminal_key.clone();
        self.post_json("RemoveCard", payload, password).await
    }
}
