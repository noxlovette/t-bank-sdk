use crate::{Receipt, TerminalKey};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use url::Url;

/// Запрос для инициации платежа
#[derive(Debug, Serialize)]
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
    /// Подпись запроса. [Как сформировать.](https://developer.tbank.ru/eacq/intro/developer/token)
    pub token: String,
    /// Описание заказа. Значение параметра будет отображено на платежной форме.
    ///
    // Параметр обязательный при привязке и одновременной оплате через СБП. При оплате через СБП текст из этого параметра отобразится в мобильном банке клиента.
    pub description: Option<String>,
    /// Идентификатор покупателя в системе мерчанта. Нужен для сохранения карт на платежной форме — платежи в один клик.
    ///
    /// Параметр обязательный, если передан параметр Recurrent=Y и автоплатеж проводится по карте.
    ///
    /// Если передан, в уведомлении будут указаны [CustomerKey] и его CardId. Подробнее — в методе [Получить список карт клиента](https://developer.tbank.ru/eacq/api/get-card-list).
    pub customer_key: Option<String>,
    /// Признак родительского CC-платежа. Обязателен для проведения операции с сохранением реквизитов карты покупателя.
    ///
    /// Если передается и установлен в Y, при платеже будут сохранены реквизиты карты покупателя. В этом случае после оплаты в уведомлении на AUTHORIZED будет передан параметр RebillId для использования в методе [Провести платеж по сохраненным реквизитам](https://developer.tbank.ru/eacq/api/charge). Для привязки и одновременной оплаты по CБП передавайте Y.
    pub recurrent: Option<String>,
    pub pay_type: Option<PayType>,
    pub language: Option<Language>,
    /// URL на веб-сайте мерчанта, куда будет отправлен POST-запрос о статусе выполнения вызываемых методов — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    ///
    /// [Подробнее](https://developer.tbank.ru/eacq/intro/developer/notification)
    pub notification_url: Option<Url>,
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае успешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
    pub success_url: Option<Url>,
    /// URL на веб-сайте мерчанта, куда будет переведен клиент в случае неуспешной оплаты — настраивается в личном кабинете.
    ///
    /// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
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
    pub redirect_due_date: Option<DateTime<Utc>>,
    #[serde(rename = "DATA")]
    pub data: Option<Data>,
    pub receipt: Option<Receipt>,
    pub shops: Vec<Shop>,
}

/// Ответ инициатора платежа
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentRes {
    pub terminal_key: TerminalKey,
    /// Requirements: <= 20 chars
    ///
    /// Сумма в копейках.
    pub amount: u32,
    /// Requirements: <= 36 characters
    ///
    /// Идентификатор заказа в системе мерчанта. Должен быть уникальным для каждой операции.
    pub order_id: String,
    /// Успешность прохождения запроса — true/false.
    pub success: bool,
    ///  Requirements: <= 20 characters
    ///
    /// Статус транзакции.
    pub status: String,
    /// Requirements: <= 20 characters
    ///
    /// Идентификатор платежа в системе Т‑Бизнес.
    pub payment_id: String,
    /// Requirements: <= 20 characters
    ///
    /// Код ошибки.
    pub error_code: String,
    ///Requirements: <= 100 characters
    ///
    /// Ссылка на платежную форму. Параметр возвращается только для мерчантов, которые используют платежную форму Т-Банка.
    pub payment_url: Option<Url>,
    /// Requirements: <= 255 characters
    ///
    /// Краткое описание ошибки.
    pub message: Option<String>,
    /// Подробное описание ошибки.
    pub details: Option<String>,
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

    #[test]
    fn parse_request() {
        let json = r#"
        {"TerminalKey":"TBankTest","Amount":140000,"OrderId":"21090","Description":"Подарочная карта на 1000 рублей","Token":"68711168852240a2f34b6a8b19d2cfbd296c7d2a6dff8b23eda6278985959346","DATA":{"Phone":"+71234567890","Email":"a@test.com"},"Receipt":{"Email":"a@test.ru","Phone":"+79031234567","Taxation":"osn","Items":[{"Name":"Наименование товара 1","Price":10000,"Quantity":1,"Amount":10000,"Tax":"vat10","Ean13":"303130323930303030630333435"},{"Name":"Наименование товара 2","Price":20000,"Quantity":2,"Amount":40000,"Tax":"vat20"},{"Name":"Наименование товара 3","Price":30000,"Quantity":3,"Amount":90000,"Tax":"vat10"}]}}
        "#;
        let _ = json;
    }

    #[test]
    fn parse_response() {
        let json = r#"{"Success":true,"ErrorCode":"0","TerminalKey":"TBankTest","Status":"NEW","PaymentId":"3093639567","OrderId":"21090","Amount":140000,"PaymentURL":"https://pay.tbank.ru/new/fU1ppgqa"}"#;
        let _ = json;
    }
}
