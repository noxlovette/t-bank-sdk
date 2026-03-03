use crate::{Receipt, TerminalKey};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use url::Url;

/// Запрос для инициации платежа
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentReq {
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
    pub notification_url: Option<Url>,
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае успешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<Url>,
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае неуспешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn receipt(mut self, receipt: Receipt) -> Self {
        self.receipt = Some(receipt);
        self
    }

    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Ответ инициатора платежа
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentRes {
    pub terminal_key: TerminalKey,
    /// Requirements: <= 20 characters
    ///
    /// Статус транзакции.
    pub status: String,
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
            Self::Ru => write!(f, "ru"),
            Self::En => write!(f, "en"),
        }
    }
}

/// JSON-объект с данными маркетплейса. Параметр обязательный для маркетплейсов.
#[derive(Serialize, Deserialize, Debug)]
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

#[cfg(test)]
mod test {
    use crate::{InitPaymentReq, ItemFFD105, Receipt, Taxation, TerminalKey};
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
            .receipt(Receipt::FFD105 {
                items: vec![ItemFFD105 {
                    name: "Item1".to_string(),
                    price: 1000,
                    quantity: 1,
                    amount: 1000,
                    ..Default::default()
                }],
                ffd_version: None,
                email: Some("a@test.com".to_string()),
                phone: None,
                taxation: Taxation::UsnIncome,
                payments: None,
            })
            .data(json!({
                "Phone": "%2B71234567890",
                "Email": "a%40test.com",
            }));

        let value = serde_json::to_value(payload).unwrap();

        assert!(value["Receipt"].get("FFD105").is_none());
        assert_eq!(value["Receipt"]["Items"][0]["Name"], "Item1");
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
}
