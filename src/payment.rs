use crate::{Receipt, TerminalKey};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use url::Url;
use utoipa::ToSchema;

/// Запрос для инициации платежа
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    /// Сумма в копейках.
    ///
    /// Например, 3 руб. 12коп. — это число 312.
    ///
    /// Параметр должен быть равен сумме всех параметров Amount, переданных в объекте Items.
    /// Минимальная сумма операции с помощью СБП составляет 10 руб.
    pub amount: u32,
    /// Идентификатор заказа в системе мерчанта. Должен быть уникальным для каждой операции.
    pub order_id: String,
    /// Описание заказа. Значение параметра будет отображено на платежной форме.
    ///
    // Параметр обязательный при привязке и одновременной оплате через СБП. При оплате через СБП текст из этого параметра отобразится в мобильном банке клиента.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Идентификатор покупателя в системе мерчанта. Нужен для сохранения карт на платежной форме — платежи в один клик.
    ///
    /// Параметр обязательный, если передан параметр Recurrent=Y и автоплатеж проводится по карте.
    ///
    /// Если передан, в уведомлении будут указаны [CustomerKey] и его CardId. Подробнее — в методе [Получить список карт клиента](https://developer.tbank.ru/eacq/api/get-card-list).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_key: Option<String>,
    /// Признак родительского CC-платежа. Обязателен для проведения операции с сохранением реквизитов карты покупателя.
    ///
    /// Если передается и установлен в Y, при платеже будут сохранены реквизиты карты покупателя. В этом случае после оплаты в уведомлении на AUTHORIZED будет передан параметр RebillId для использования в методе [Провести платеж по сохраненным реквизитам](https://developer.tbank.ru/eacq/api/charge). Для привязки и одновременной оплаты по CБП передавайте Y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<PayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// URL на веб-сайте мерчанта, куда будет отправлен POST-запрос о статусе выполнения вызываемых методов — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    ///
    /// [Подробнее](https://developer.tbank.ru/eacq/intro/developer/notification)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NotificationURL")]
    #[schema(value_type = String)]
    pub notification_url: Option<Url>,
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае успешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SuccessURL")]
    #[schema(value_type = String)]
    pub success_url: Option<Url>,
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае неуспешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "FailURL")]
    #[schema(value_type = String)]
    pub fail_url: Option<Url>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_due_date: Option<DateTime<Utc>>,
    #[serde(rename = "DATA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Receipt>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shops: Vec<Shop>,
}

impl InitPaymentReq {
    pub fn new(terminal_key: &TerminalKey, amount: u32, order_id: &str) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            amount,
            order_id: order_id.to_ascii_lowercase(),
            ..Default::default()
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn customer_key(mut self, ck: &str) -> Self {
        self.customer_key = Some(ck.to_string());
        self
    }

    pub fn recurrent(mut self) -> Self {
        self.recurrent = Some("Y".to_string());
        self
    }

    pub fn pay_type(mut self, pt: PayType) -> Self {
        self.pay_type = Some(pt);
        self
    }

    pub fn language(mut self, l: Language) -> Self {
        self.language = Some(l);
        self
    }

    pub fn notification_url(mut self, url: &str) -> Self {
        self.notification_url = Url::parse(url).ok();
        self
    }

    pub fn success_url(mut self, url: &str) -> Self {
        self.success_url = Url::parse(url).ok();
        self
    }

    pub fn fail_url(mut self, url: &str) -> Self {
        self.fail_url = Url::parse(url).ok();
        self
    }

    pub fn redirect_due_date(mut self, rdd: DateTime<Utc>) -> Self {
        let valid = |d| d > Duration::minutes(1) && d > Duration::days(90);

        self.redirect_due_date = valid(rdd - Utc::now()).then_some(rdd);
        self
    }

    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn receipt(mut self, receipt: Receipt) -> Self {
        self.receipt = Some(receipt);
        self
    }

    pub fn shops(mut self, shops: Vec<Shop>) -> Self {
        self.shops = shops;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    #[default]
    New,
    Authorized,
    Confirmed,
    PartialReversed,
    Reversed,
    Canceled,
    PartialRefunded,
    Refunded,
    Rejected,
    DeadlineExpired,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentNotificationRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub status: PaymentStatus,
    pub amount: u32,
    pub order_id: String,
    pub payment_id: u64,
    pub rebill_id: Option<u64>,
    pub card_id: Option<u64>,
    pub pan: Option<String>,
    pub exp_date: Option<String>,
    pub token: Option<String>,
    pub data: Option<DataNotification>,
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct DataNotification {
    pub route: String,
    pub source: String,
    pub credit_amount: u32,
}

/// Ответ инициатора платежа
#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    /// Requirements: <= 20 characters
    ///
    /// Статус транзакции.
    pub status: PaymentStatus,
    /// Requirements: <= 20 chars
    ///
    /// Сумма в копейках.
    pub amount: u32,
    /// Requirements: <= 36 characters
    ///
    /// Идентификатор заказа в системе мерчанта. Должен быть уникальным для каждой операции.
    pub order_id: String,
    /// Requirements: <= 20 characters
    ///
    /// Идентификатор платежа в системе Т‑Бизнес.
    pub payment_id: String,
    /// Requirements: <= 100 characters
    ///
    /// Ссылка на платежную форму. Параметр возвращается только для мерчантов, которые используют платежную форму Т-Банка.
    #[serde(rename = "PaymentURL")]
    #[schema(value_type = String)]
    pub payment_url: Option<Url>,
}

/// JSON-объект с дополнительными параметрами по операции и настройками в формате ключ:значение.
///
/// Максимальная длина ключа — 20 знаков, значения — 100 знаков.
///
/// Максимальное количество пар ключ:значение — не больше 20.
///
/// Если ключи или значения содержат в себе специальные символы, получившееся значение должно быть закодировано функцией urlencode.
/// ВНИМАНИЕ: SDK не имплементирует LongPay
#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    additional_properties: String,
    operation_initiator_type: OperationInitiatorType,
    /// Тип устройства:
    ///
    /// SDK — вызов из мобильного приложения,
    /// Desktop — вызов из браузера с десктопа,
    /// Mobile — вызов из браузера с мобильного устройства.
    device: String,
    /// ОС устройства.
    device_os: String,
    /// Признак открытия в WebView.
    device_web_view: bool,
    /// Браузер.
    device_browser: String,
    /// Признак проведения операции через T-Pay по API.
    tinkoff_pay_web: bool,
}

/// Requirements: [O, T]
///
/// Определяет тип проведения платежа:
///
/// O — одностадийная оплата;
/// T — двухстадийная оплата.
/// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum PayType {
    O,
    T,
}

impl Display for PayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::O => write!(f, "o"),
            Self::T => write!(f, "t"),
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
#[derive(Default, Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Ru,
    En,
}

impl Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ru => write!(f, "ru"),
            Self::En => write!(f, "en"),
        }
    }
}

/// JSON-объект с данными маркетплейса. Параметр обязательный для маркетплейсов.
#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Shop {
    /// Код магазина. Для параметра ShopСode нужно использовать значение параметра Submerchant_ID, который возвращается в ответе при регистрации магазинов через XML. Если XML не используется, передавать поле не нужно.
    shop_code: String,
    /// Сумма в копейках.
    ///
    /// Например, 3 руб. 12коп. — это число 312.
    ///
    /// Параметр должен быть равен сумме всех параметров Amount, переданных в объекте Items.
    /// Минимальная сумма операции с помощью СБП составляет 10 руб.
    ///
    /// P.S. I'm not sure anyone will pay more than 42 949 672,96 RUB with this
    amount: u32,
    name: Option<String>,
    fee: Option<String>,
}

impl Shop {
    pub fn new(shop_code: &str, amount: u32) -> Self {
        Self {
            shop_code: shop_code.to_string(),
            amount,
            name: None,
            fee: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn fee(mut self, fee: &str) -> Self {
        self.fee = Some(fee.to_string());
        self
    }
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
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct OperationInitiatorType;

#[cfg(test)]
mod test {
    use crate::{
        ErrorWrapper, InitPaymentReq, ItemFFD105, PaymentNotificationRes, PaymentStatus, Receipt,
        Shop, Tax, Taxation, TerminalKey,
    };
    use serde_json::json;

    #[test]
    fn parse_request() {
        let json = r#"
        {"TerminalKey":"TBankTest","Amount":140000,"OrderId":"21090","Description":"Подарочная карта на 1000 рублей","Token":"68711168852240a2f34b6a8b19d2cfbd296c7d2a6dff8b23eda6278985959346","DATA":{"Phone":"+71234567890","Email":"a@test.com"},"Receipt":{"Email":"a@test.ru","Phone":"+79031234567","Taxation":"osn","Items":[{"Name":"Наименование товара 1","Price":10000,"Quantity":1,"Amount":10000,"Tax":"vat10","Ean13":"303130323930303030630333435"},{"Name":"Наименование товара 2","Price":20000,"Quantity":2,"Amount":40000,"Tax":"vat20"},{"Name":"Наименование товара 3","Price":30000,"Quantity":3,"Amount":90000,"Tax":"vat10"}]}}
        "#;
        let _ = json;
    }

    #[test]
    fn receipt_serializes_without_ffd_wrapper() {
        let payload = InitPaymentReq::new(&TerminalKey::default(), 1000, "32451")
            .receipt(
                Receipt::ffd105(
                    vec![ItemFFD105::new("Item1", 1000, 1, 1000).tax(Tax::Vat20)],
                    Taxation::UsnIncome,
                )
                .email("a@test.com"),
            )
            .data(json!({
                "Phone": "%2B71234567890",
                "Email": "a%40test.com",
            }));

        let value = serde_json::to_value(payload).unwrap();

        assert!(value["Receipt"].get("FFD105").is_none());
        assert_eq!(value["Receipt"]["Items"][0]["Name"], "Item1");
    }

    #[test]
    fn shops_serialize_from_builder() {
        let payload = InitPaymentReq::new(&TerminalKey::default(), 1000, "32451")
            .shops(vec![Shop::new("submerchant-1", 1000).name("Shop 1")]);
        let value = serde_json::to_value(payload).unwrap();

        assert_eq!(value["Shops"][0]["ShopCode"], "submerchant-1");
        assert_eq!(value["Shops"][0]["Amount"], 1000);
        assert_eq!(value["Shops"][0]["Name"], "Shop 1");
    }

    #[test]
    fn init_payment_omits_absent_root_fields() {
        let payload = InitPaymentReq::new(&TerminalKey::default(), 1000, "32451");
        let value = serde_json::to_value(payload).unwrap();

        assert!(value.get("Description").is_none());
        assert!(value.get("CustomerKey").is_none());
        assert!(value.get("Recurrent").is_none());
        assert!(value.get("PayType").is_none());
        assert!(value.get("Language").is_none());
        assert!(value.get("NotificationUrl").is_none());
        assert!(value.get("SuccessUrl").is_none());
        assert!(value.get("FailUrl").is_none());
        assert!(value.get("RedirectDueDate").is_none());
        assert!(value.get("DATA").is_none());
        assert!(value.get("Receipt").is_none());
        assert!(value.get("Shops").is_none());
    }

    #[test]
    fn parse_response() {
        let json = r#"{"Success":true,"ErrorCode":"0","TerminalKey":"TBankTest","Status":"NEW","PaymentId":"3093639567","OrderId":"21090","Amount":140000,"PaymentURL":"https://pay.tbank.ru/new/fU1ppgqa"}"#;
        let _ = json;
    }

    #[test]
    fn payment_notification_deserializes_into_error_wrapper() {
        let json = r#"
        {
          "TerminalKey": "TBankTest",
          "Amount": 1000,
          "OrderId": "yLRDLX4nGyyiQQEdq5kB3",
          "Success": true,
          "Status": "CONFIRMED",
          "PaymentId": 8104656673,
          "ErrorCode": "0",
          "Message": "string",
          "Details": "string",
          "RebillId": 3207469334,
          "CardId": 10452089,
          "Pan": "string",
          "ExpDate": "0229",
          "Token": "7241ac8307f349afb7bb9dda760717721bbb45950b97c67289f23d8c69cc7b96",
          "Data": {
            "Route": "TCB",
            "Source": "Installment",
            "CreditAmount": 10000
          }
        }
        "#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();
        let response = wrapper.unwrap().unwrap();

        assert!(matches!(response.status, PaymentStatus::Confirmed));
        assert_eq!(response.order_id, "yLRDLX4nGyyiQQEdq5kB3");
        assert_eq!(response.payment_id, 8104656673);
        assert_eq!(response.data.unwrap().credit_amount, 10000);
    }

    #[test]
    fn payment_notification_deserializes_real_webhook_payload() {
        let json = r#"{"TerminalKey":"1772186527637DEMO","OrderId":"lde1q7xbsoisn1vjj2k6s","Success":true,"Status":"AUTHORIZED","PaymentId":8111987112,"ErrorCode":"0","Amount":1000,"CardId":659561464,"Pan":"430000******0777","ExpDate":"1230","Token":"17bf87f32797d961db48f4500cc9110be5cc2f480e4e555e2440eec09108a760"}"#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();
        let response = wrapper.unwrap().unwrap();

        assert_eq!(response.order_id, "lde1q7xbsoisn1vjj2k6s");
        assert_eq!(response.amount, 1000);
        assert_eq!(response.payment_id, 8111987112);
        assert_eq!(response.rebill_id, None);
        assert_eq!(response.card_id, Some(659561464));
        assert_eq!(response.exp_date.as_deref(), Some("1230"));
        assert!(response.data.is_none());
    }
}
