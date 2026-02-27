use crate::{Receipt, TerminalKey};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use url::Url;

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
#[derive(Serialize, Deserialize, Debug)]
pub struct Amount(NonZeroU32);

/// Requirements: <= 36 characters
///
/// Идентификатор заказа в системе мерчанта. Должен быть уникальным для каждой операции.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderId(String);

/// Подпись запроса
#[derive(Serialize, Deserialize, Debug)]
pub struct Token(String);

/// Requirements: <= 140 characters
///
/// Описание заказа. Значение параметра будет отображено на платежной форме.
///
// Параметр обязательный при привязке и одновременной оплате через СБП. При оплате через СБП текст из этого параметра отобразится в мобильном банке клиента.
#[derive(Serialize, Deserialize, Debug)]
pub struct Description(String);

/// Requirements: <= 36 characters
///
/// Идентификатор покупателя в системе мерчанта. Нужен для сохранения карт на платежной форме — платежи в один клик.
///
/// Параметр обязательный, если передан параметр Recurrent=Y и автоплатеж проводится по карте.
///
/// Если передан, в уведомлении будут указаны [CustomerKey] и его [CardId]. Подробнее — в методе [Получить список карт клиента](https://developer.tbank.ru/eacq/api/get-card-list).
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerKey(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct CardId(String);

/// Requirements: <= 1 characters, [Y]
///
/// Признак родительского CC-платежа. Обязателен для проведения операции с сохранением реквизитов карты покупателя.
///
/// Если передается и установлен в Y, при платеже будут сохранены реквизиты карты покупателя. В этом случае после оплаты в уведомлении на AUTHORIZED будет передан параметр RebillId для использования в методе [Провести платеж по сохраненным реквизитам](https://developer.tbank.ru/eacq/api/charge). Для привязки и одновременной оплаты по CБП передавайте Y.
#[derive(Serialize, Deserialize, Debug)]
pub struct Recurrent(String);

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
pub enum Language {
    #[default]
    Ru,
    En,
}

/// URL на веб-сайте мерчанта, куда будет отправлен POST-запрос о статусе выполнения вызываемых методов — настраивается в личном кабинете.
///
/// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
///
/// [Подробнее](https://developer.tbank.ru/eacq/intro/developer/notification)
#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationUrl(Url);

/// URL на веб-сайте мерчанта, куда будет переведен клиент в случае успешной оплаты — настраивается в личном кабинете.
///
/// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessUrl(Url);

/// URL на веб-сайте мерчанта, куда будет переведен клиент в случае неуспешной оплаты — настраивается в личном кабинете.
///
/// Если параметр передан, используется его значение, если нет — значение из настроек терминала.
#[derive(Serialize, Deserialize, Debug)]
pub struct FailUrl(Url);

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
#[derive(Serialize, Deserialize, Debug)]
pub struct RedirectDueDate(DateTime<Utc>);

/// JSON-объект с дополнительными параметрами по операции и настройками в формате ключ:значение.
///
/// Максимальная длина ключа — 20 знаков, значения — 100 знаков.
///
/// Максимальное количество пар ключ:значение — не больше 20.
///
/// Если ключи или значения содержат в себе специальные символы, получившееся значение должно быть закодировано функцией urlencode.
/// ВНИМАНИЕ: SDK не имплементирует LongPay
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    additional_properties: String,
    operation_initiator_type: OperationInitiatorType,
    device: Device,
    device_os: DeviceOs,
    device_web_view: DeviceWebView,
    device_browser: DeviceBrowser,
    tinkoff_pay_web: TinkoffPayWeb,
}

/// Requirements: [SDK, Desktop, Mobile]
///
/// Тип устройства:
///
/// SDK — вызов из мобильного приложения,
/// Desktop — вызов из браузера с десктопа,
/// Mobile — вызов из браузера с мобильного устройства.
#[derive(Serialize, Deserialize, Debug)]
pub struct Device(String);

/// ОС устройства.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceOs(String);

/// Признак открытия в WebView.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceWebView(bool);

/// Браузер.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceBrowser(String);

/// Признак проведения операции через T‑Pay по API.
#[derive(Serialize, Deserialize, Debug)]
pub struct TinkoffPayWeb(bool);

/// JSON-объект с данными маркетплейса. Параметр обязательный для маркетплейсов.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shop {
    shop_code: ShopCode,
    amount: ShopAmount,
    name: ShopName,
    fee: Fee,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ShopCode(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ShopAmount(Amount);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ShopName(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Fee(String);

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

/// Запрос для инициации платежа
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentReq {
    terminal_key: TerminalKey,
    amount: Amount,
    order_id: OrderId,
    token: Token,
    description: Option<Description>,
    customer_key: Option<CustomerKey>,
    recurrent: Option<Recurrent>,
    pay_type: Option<PayType>,
    language: Option<Language>,
    notification_url: Option<NotificationUrl>,
    success_url: Option<SuccessUrl>,
    fail_url: Option<FailUrl>,
    redirect_due_date: Option<DateTime<Utc>>,
    #[serde(rename = "DATA")]
    data: Option<Data>,
    receipt: Option<Receipt>,
    shops: Vec<Shop>,
}

/// Ответ инициатора платежа
#[derive(Deserialize)]
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
    message: String,
    details: String,
}

/// Requirements: <= 20 characters
///
/// Статус транзакции.
#[derive(Deserialize)]
struct Status(String);

/// Requirements: <= 20 characters
///
/// Идентификатор платежа в системе Т‑Бизнес.
#[derive(Deserialize)]
struct PaymentId(String);

mod test {

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
}
