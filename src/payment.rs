use crate::{Receipt, TerminalKey, newtype};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    num::NonZeroU32,
};
use url::Url;

const TERMINAL_KEY_MAX_LEN: usize = 20;
const ORDER_ID_MAX_LEN: usize = 36;
const DESCRIPTION_MAX_LEN: usize = 140;
const CUSTOMER_KEY_MAX_LEN: usize = 36;
const REDIRECT_DUE_DATE_MINUTES: i64 = 1;
const REDIRECT_DUE_DATE_MAX_DAYS: i64 = 90;

/// Запрос для инициации платежа
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentReq {
    pub terminal_key: TerminalKey,
    pub amount: Amount,
    pub order_id: OrderId,
    pub token: Token,
    pub description: Option<Description>,
    pub customer_key: Option<CustomerKey>,
    pub recurrent: Option<Recurrent>,
    pub pay_type: Option<PayType>,
    pub language: Option<Language>,
    pub notification_url: Option<NotificationUrl>,
    pub success_url: Option<SuccessUrl>,
    pub fail_url: Option<FailUrl>,
    pub redirect_due_date: Option<DateTime<Utc>>,
    #[serde(rename = "DATA")]
    pub data: Option<Data>,
    pub receipt: Option<Receipt>,
    pub shops: Vec<Shop>,
}

/// Ответ инициатора платежа
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentRes {
    terminal_key: TerminalKey,
    amount: Amount,
    order_id: OrderId,
    success: bool,
    status: Status,
    payment_id: PaymentId,
    error_code: String,
    payment_url: Option<Url>,
    message: Option<String>,
    details: Option<String>,
}

/// Ошибка валидации параметров запроса на инициацию платежа.
#[derive(Debug, thiserror::Error)]
pub enum InitPaymentReqError {
    #[error("{field} must not be empty")]
    Empty { field: &'static str },

    #[error("{field} must be at most {max} characters, got {actual}")]
    TooLong {
        field: &'static str,
        max: usize,
        actual: usize,
    },

    #[error("Amount must be greater than 0")]
    AmountMustBePositive,

    #[error("Recurrent must be \"Y\"")]
    InvalidRecurrent,

    #[error("RedirectDueDate must be between {min_minutes} minute and {max_days} days from now")]
    InvalidRedirectDueDate { min_minutes: i64, max_days: i64 },

    #[error("{field} is not a valid URL: {source}")]
    InvalidUrl {
        field: &'static str,
        #[source]
        source: url::ParseError,
    },
}

impl InitPaymentReq {
    /// Создает запрос с обязательными полями.
    ///
    /// `data`, `receipt` и `shops` пока не настраиваются через builder API.
    pub fn new(
        terminal_key: impl Into<String>,
        amount: u32,
        order_id: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, InitPaymentReqError> {
        Ok(Self {
            terminal_key: validate_string(
                "TerminalKey",
                terminal_key.into(),
                TERMINAL_KEY_MAX_LEN,
                TerminalKey::new,
            )?,
            amount: validate_amount(amount)?,
            order_id: validate_string("OrderId", order_id.into(), ORDER_ID_MAX_LEN, OrderId)?,
            token: Token(token.into()),
            description: None,
            customer_key: None,
            recurrent: None,
            pay_type: None,
            language: None,
            notification_url: None,
            success_url: None,
            fail_url: None,
            redirect_due_date: None,
            data: None,
            receipt: None,
            shops: Vec::new(),
        })
    }

    /// Устанавливает описание заказа.
    pub fn description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, InitPaymentReqError> {
        self.description = Some(validate_string(
            "Description",
            description.into(),
            DESCRIPTION_MAX_LEN,
            Description,
        )?);
        Ok(self)
    }

    /// Устанавливает идентификатор покупателя.
    pub fn customer_key(
        mut self,
        customer_key: impl Into<String>,
    ) -> Result<Self, InitPaymentReqError> {
        self.customer_key = Some(validate_string(
            "CustomerKey",
            customer_key.into(),
            CUSTOMER_KEY_MAX_LEN,
            CustomerKey,
        )?);
        Ok(self)
    }

    /// Включает сохранение реквизитов карты покупателя.
    pub fn recurrent(mut self) -> Self {
        self.recurrent = Some(Recurrent(String::from("Y")));
        self
    }

    /// Устанавливает явное значение признака рекуррентного платежа.
    pub fn recurrent_value(
        mut self,
        recurrent: impl Into<String>,
    ) -> Result<Self, InitPaymentReqError> {
        let recurrent = recurrent.into();
        if recurrent != "Y" {
            return Err(InitPaymentReqError::InvalidRecurrent);
        }

        self.recurrent = Some(Recurrent(recurrent));
        Ok(self)
    }

    /// Устанавливает тип проведения платежа.
    pub fn pay_type(mut self, pay_type: PayType) -> Self {
        self.pay_type = Some(pay_type);
        self
    }

    /// Устанавливает язык платежной формы.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Устанавливает URL для уведомлений.
    pub fn notification_url(
        mut self,
        notification_url: impl AsRef<str>,
    ) -> Result<Self, InitPaymentReqError> {
        self.notification_url = Some(NotificationUrl(parse_url(
            "NotificationUrl",
            notification_url.as_ref(),
        )?));
        Ok(self)
    }

    /// Устанавливает URL для возврата после успешной оплаты.
    pub fn success_url(
        mut self,
        success_url: impl AsRef<str>,
    ) -> Result<Self, InitPaymentReqError> {
        self.success_url = Some(SuccessUrl(parse_url("SuccessUrl", success_url.as_ref())?));
        Ok(self)
    }

    /// Устанавливает URL для возврата после неуспешной оплаты.
    pub fn fail_url(mut self, fail_url: impl AsRef<str>) -> Result<Self, InitPaymentReqError> {
        self.fail_url = Some(FailUrl(parse_url("FailUrl", fail_url.as_ref())?));
        Ok(self)
    }

    /// Устанавливает срок жизни ссылки или QR-кода СБП.
    pub fn redirect_due_date(
        mut self,
        redirect_due_date: DateTime<Utc>,
    ) -> Result<Self, InitPaymentReqError> {
        validate_redirect_due_date(redirect_due_date)?;
        self.redirect_due_date = Some(redirect_due_date);
        Ok(self)
    }
}

fn validate_string<T>(
    field: &'static str,
    value: String,
    max_len: usize,
    make: impl FnOnce(String) -> T,
) -> Result<T, InitPaymentReqError> {
    if value.is_empty() {
        return Err(InitPaymentReqError::Empty { field });
    }

    let actual = value.chars().count();
    if actual > max_len {
        return Err(InitPaymentReqError::TooLong {
            field,
            max: max_len,
            actual,
        });
    }

    Ok(make(value))
}

fn validate_amount(amount: u32) -> Result<Amount, InitPaymentReqError> {
    NonZeroU32::new(amount)
        .map(Amount)
        .ok_or(InitPaymentReqError::AmountMustBePositive)
}

fn parse_url(field: &'static str, value: &str) -> Result<Url, InitPaymentReqError> {
    Url::parse(value).map_err(|source| InitPaymentReqError::InvalidUrl { field, source })
}

fn validate_redirect_due_date(redirect_due_date: DateTime<Utc>) -> Result<(), InitPaymentReqError> {
    let now = Utc::now();
    let min = now + chrono::Duration::minutes(REDIRECT_DUE_DATE_MINUTES);
    let max = now + chrono::Duration::days(REDIRECT_DUE_DATE_MAX_DAYS);

    if redirect_due_date < min || redirect_due_date > max {
        return Err(InitPaymentReqError::InvalidRedirectDueDate {
            min_minutes: REDIRECT_DUE_DATE_MINUTES,
            max_days: REDIRECT_DUE_DATE_MAX_DAYS,
        });
    }

    Ok(())
}

/// JSON-объект с дополнительными параметрами по операции и настройками в формате ключ:значение.
///
/// Максимальная длина ключа — 20 знаков, значения — 100 знаков.
///
/// Максимальное количество пар ключ:значение — не больше 20.
///
/// Если ключи или значения содержат в себе специальные символы, получившееся значение должно быть закодировано функцией urlencode.
/// ВНИМАНИЕ: SDK не имплементирует LongPay
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    additional_properties: String,
    operation_initiator_type: OperationInitiatorType,
    device: Device,
    device_os: DeviceOs,
    device_web_view: DeviceWebView,
    device_browser: DeviceBrowser,
    tinkoff_pay_web: TinkoffPayWeb,
}

/// Requirements: [O, T]
///
/// Определяет тип проведения платежа:
///
/// O — одностадийная оплата;
/// T — двухстадийная оплата.
/// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
#[derive(Serialize, Deserialize, Debug)]
pub enum PayType {
    O,
    T,
}

impl Display for PayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::O => {
                write!(f, "o")
            }
            Self::T => {
                write!(f, "t")
            }
        }
    }
}

/// Requirements: <= 2 characters
///
/// Default: ru
///
/// Язык платежной формы:
///
/// ru — русский;
/// en — английский.
/// Если параметр не передан, форма откроется на русском языке.
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Ru,
    En,
}

impl Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ru => {
                write!(f, "ru")
            }
            Self::En => {
                write!(f, "en")
            }
        }
    }
}

/// JSON-объект с данными маркетплейса. Параметр обязательный для маркетплейсов.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Shop {
    shop_code: ShopCode,
    amount: ShopAmount,
    name: Option<ShopName>,
    fee: Option<Fee>,
}

newtype! {
    /// Requirements: <= 10 chars
    ///
    /// Сумма в копейках.
    ///
    /// Например, 3 руб. 12коп. — это число 312.
    ///
    /// Параметр должен быть равен сумме всех параметров Amount, переданных в объекте Items.
    /// Минимальная сумма операции с помощью СБП составляет 10 руб.
    ///
    ///
    /// P.S. I'm not sure anyone will pay more than 42 949 672,96 RUB with this
    pub struct Amount(NonZeroU32);
}

newtype! {
    /// Подпись запроса. [Как сформировать.](https://developer.tbank.ru/eacq/intro/developer/token)
    pub struct Token(String);
}

impl Token {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }
}

newtype! {
    /// Requirements: <= 36 characters
    ///
    /// Идентификатор заказа в системе мерчанта. Должен быть уникальным для каждой операции.
    pub struct OrderId(String);
}

newtype! {
    /// Requirements: <= 140 characters
    ///
    /// Описание заказа. Значение параметра будет отображено на платежной форме.
    ///
    // Параметр обязательный при привязке и одновременной оплате через СБП. При оплате через СБП текст из этого параметра отобразится в мобильном банке клиента.
    pub struct Description(String);
}

newtype! {
    /// Requirements: <= 36 characters
    ///
    /// Идентификатор покупателя в системе мерчанта. Нужен для сохранения карт на платежной форме — платежи в один клик.
    ///
    /// Параметр обязательный, если передан параметр Recurrent=Y и автоплатеж проводится по карте.
    ///
    /// Если передан, в уведомлении будут указаны [CustomerKey] и его [CardId]. Подробнее — в методе [Получить список карт клиента](https://developer.tbank.ru/eacq/api/get-card-list).
    pub struct CustomerKey(String);
}

newtype! {
    pub struct CardId(String);
}

newtype! {
    /// Requirements: <= 1 characters, [Y]
    ///
    /// Признак родительского CC-платежа. Обязателен для проведения операции с сохранением реквизитов карты покупателя.
    ///
    /// Если передается и установлен в Y, при платеже будут сохранены реквизиты карты покупателя. В этом случае после оплаты в уведомлении на AUTHORIZED будет передан параметр RebillId для использования в методе [Провести платеж по сохраненным реквизитам](https://developer.tbank.ru/eacq/api/charge). Для привязки и одновременной оплаты по CБП передавайте Y.
    pub struct Recurrent(String);
}

newtype! {
    /// URL на веб-сайте мерчанта, куда будет отправлен POST-запрос о статусе выполнения вызываемых методов — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    ///
    /// [Подробнее](https://developer.tbank.ru/eacq/intro/developer/notification)
    pub struct NotificationUrl(Url);
}

newtype! {
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае успешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    pub struct SuccessUrl(Url);
}

newtype! {
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае неуспешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    pub struct FailUrl(Url);
}

newtype! {
    /// Cрок жизни ссылки или динамического QR-кода СБП, если выбран этот способ оплаты.
    ///
    /// Если дата в параметре меньше текущей, оплата по ссылке и QR будет  недоступна.
    ///
    /// - Минимальное значение — 1 минута от текущей даты.
    /// - Максимальное значение — 90 дней от текущей даты.
    /// - Формат даты — YYYY-MM-DDTHH24:MI:SS+GMT.
    /// - Пример даты — 2016-08-31T12:28:00+03:00.
    ///
    /// Если параметр не был передан, проверяется настроечный параметр терминала REDIRECT_TIMEOUT, который содержит значение срока жизни ссылки в часах. Если его значение:
    ///
    /// больше нуля — оно будет установлено в качестве срока жизни ссылки или динамического QR-кода;
    /// меньше нуля — устанавливается значение по умолчанию: 1440 мин. (1 сутки).
    pub struct RedirectDueDate(DateTime<Utc>);
}

newtype! {
    /// Requirements: [SDK, Desktop, Mobile]
    ///
    /// Тип устройства:
    ///
    /// SDK — вызов из мобильного приложения,
    /// Desktop — вызов из браузера с десктопа,
    /// Mobile — вызов из браузера с мобильного устройства.
    pub struct Device(String);
}

newtype! {
    /// ОС устройства.
    pub struct DeviceOs(String);
}

newtype! {
    /// Признак открытия в WebView.
    pub struct DeviceWebView(bool);
}

newtype! {
    /// Браузер.
    pub struct DeviceBrowser(String);
}

newtype! {
    /// Признак проведения операции через T‑Pay по API.
    pub struct TinkoffPayWeb(bool);
}

newtype! {
    struct ShopCode(String);
}

newtype! {
    struct ShopAmount(Amount);
}

newtype! {
    struct ShopName(String);
}

newtype! {
    struct Fee(String);
}

/// Requirements: [0, 1, 2, R, I, D, N]
///
/// Признак инициатора операции для платежа. Параметр обязательный при создании родительского CC-платежа при оплате картой.
///
/// Подробнее о признаке инициатора операции.
///
/// 0 — обычный платеж;
/// 1 — CIT CC;
/// 2 — CIT COF;
/// R — MIT COF Recurring;
/// I — MIT COF Installment;
/// D — MIT COF Delayed-Charge;
/// N — MIT COF No-Show.
/// Если передавать значения параметров, которые не соответствуют таблице, MAPI вернет ошибку 1126 — несопоставимые значения [rebillId] или [Recurrent] с переданным значением [OperationInitiatorType].
#[derive(Serialize, Deserialize, Debug)]
pub struct OperationInitiatorType;

newtype! {
    /// Requirements: <= 20 characters
    ///
    /// Статус транзакции.
    struct Status(String);
}

newtype! {
    /// Requirements: <= 20 characters
    ///
    /// Идентификатор платежа в системе Т‑Бизнес.
    struct PaymentId(String);
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Duration;

    #[test]
    fn parse_request() {
        let json = r#"
        {"TerminalKey":"TBankTest","Amount":140000,"OrderId":"21090","Description":"Подарочная карта на 1000 рублей","Token":"68711168852240a2f34b6a8b19d2cfbd296c7d2a6dff8b23eda6278985959346","DATA":{"Phone":"+71234567890","Email":"a@test.com"},"Receipt":{"Email":"a@test.ru","Phone":"+79031234567","Taxation":"osn","Items":[{"Name":"Наименование товара 1","Price":10000,"Quantity":1,"Amount":10000,"Tax":"vat10","Ean13":"303130323930303030630333435"},{"Name":"Наименование товара 2","Price":20000,"Quantity":2,"Amount":40000,"Tax":"vat20"},{"Name":"Наименование товара 3","Price":30000,"Quantity":3,"Amount":90000,"Tax":"vat10"}]}}
        "#;
    }

    #[test]
    fn parse_response() {
        let json = r#"{"Success":true,"ErrorCode":"0","TerminalKey":"TBankTest","Status":"NEW","PaymentId":"3093639567","OrderId":"21090","Amount":140000,"PaymentURL":"https://pay.tbank.ru/new/fU1ppgqa"}"#;
    }

    #[test]
    fn builder_accepts_valid_values() {
        let redirect_due_date = Utc::now() + Duration::minutes(5);

        let req = InitPaymentReq::new("terminal", 100, "order-1", "token")
            .unwrap()
            .description("test description")
            .unwrap()
            .customer_key("customer-1")
            .unwrap()
            .recurrent()
            .pay_type(PayType::O)
            .language(Language::Ru)
            .notification_url("https://example.com/notification")
            .unwrap()
            .success_url("https://example.com/success")
            .unwrap()
            .fail_url("https://example.com/fail")
            .unwrap()
            .redirect_due_date(redirect_due_date)
            .unwrap();

        assert_eq!(req.terminal_key.to_string(), "terminal");
        assert_eq!(req.amount.to_string(), "100");
        assert_eq!(req.order_id.to_string(), "order-1");
        assert_eq!(req.token.to_string(), "token");
        assert_eq!(req.description.unwrap().to_string(), "test description");
        assert_eq!(req.customer_key.unwrap().to_string(), "customer-1");
        assert_eq!(req.recurrent.unwrap().to_string(), "Y");
        assert!(req.data.is_none());
        assert!(req.receipt.is_none());
        assert!(req.shops.is_empty());
    }

    #[test]
    fn builder_rejects_invalid_values() {
        assert!(matches!(
            InitPaymentReq::new("", 100, "order-1", "token"),
            Err(InitPaymentReqError::Empty {
                field: "TerminalKey"
            })
        ));

        assert!(matches!(
            InitPaymentReq::new("terminal", 0, "order-1", "token"),
            Err(InitPaymentReqError::AmountMustBePositive)
        ));

        assert!(matches!(
            InitPaymentReq::new("terminal", 100, "order-1", "token")
                .unwrap()
                .recurrent_value("N"),
            Err(InitPaymentReqError::InvalidRecurrent)
        ));
    }
}
