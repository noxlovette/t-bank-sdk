use crate::Amount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;

/// Позиция чека с информацией о товарах.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ItemFFD105 {
    name: ItemName,
    price: ItemPrice,
    quantity: ItemQuantity,
    amount: ItemAmount,
    payment_method: PaymentMethod,
    payment_object: PaymentObjectFF105,
    tax: Tax,
    ean_13: Ean13,
    agent_data: AgentData,
    supplier_info: SupplierInfo,
}

/// Позиция чека с информацией о товарах.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ItemFFD12 {
    name: ItemName,
    price: ItemPrice,
    quantity: ItemQuantity,
    amount: ItemAmount,
    payment_method: PaymentMethod,
    payment_object: PaymentObjectFF12,
    tax: Tax,
    agent_data: AgentData,
    supplier_info: SupplierInfo,
    user_data: UserData,
    excise: Excise,
    country_code: CountryCode,
    declaration_number: DeclarationNumber,
    measurement_unit: MeasurementUnit,
    mark_processing_mode: MarkProcessingMode,
    mark_code: Vec<MarkCode>,
    mark_quantity: MarkQuantity,
    sectoral_item_props: SectoralCheckProps,
}

/// Данные агента. Параметр обязательный, если используется агентская схема.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct AgentData {
    agent_sign: AgentSign,
    operation_name: OperationName,
    phones: AgentPhones,
    receiver_phones: ReceiverPhones,
    transfer_phones: TransferPhones,
    operator_name: OperatorName,
    operator_address: OperatorAddress,
    perator_inn: OperatorInn,
}

/// Requirements: <= 128 characters
///
/// Тег ФФД: 1030
///
/// Наименование товара.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ItemName(String);

/// Тег ФФД: 1078
/// Цена в копейках.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ItemPrice(Amount);

/// Requirements: <= 8 characters
///
/// Тег ФФД: 1023
///
/// Количество или вес товара. Максимальное количество символов — 8, где:
///
/// целая часть — не больше 5 знаков;
/// дробная — не больше 3 знаков для Атол и 2 знаков для CloudPayments.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ItemQuantity(Quantity);

/// Requirements: <= 10 characters
///
/// Тег ФФД: 1043
///
/// Стоимость товара в копейках. Произведение Quantity и Price.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ItemAmount(Amount);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ItemTax(Tax);

/// Тег ФФД: 1073
///
/// Телефоны платежного агента в формате +{Ц}.
///
/// Параметр обязательный, если AgentSign передан в значениях:
///
/// - bank_paying_agent;
/// - bank_paying_subagent;
/// - paying_agent;
/// - paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct AgentPhones(Vec<Phone>);

/// Тег ФФД: 1074
///
/// Телефоны оператора по приему платежей в формате +{Ц}.
///
/// Параметр обязательный, если AgentSign передан в значениях paying_agent или paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ReceiverPhones(Vec<Phone>);

/// Тег ФФД: 1075
///
/// Телефоны оператора по приему платежей в формате +{Ц}.
///
/// Параметр обязательный, если AgentSign передан в значениях paying_agent или paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct TransferPhones(Vec<Phone>);

/// Requirements: <= 12 characters
///
/// Тег ФФД: 1016
///
/// ИНН оператора перевода. Параметр обязательный, если AgentSign передан в /// значениях bank_paying_agent или bank_paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct OperatorInn(Inn);

/// Requirements: [bank_paying_agent, bank_paying_subagent, paying_agent, paying_subagent, attorney, commission_agent, another]
///
/// Тег ФФД: 1222
///
/// Признак агента:
///
/// - bank_paying_agent — банковский платежный агент;
/// - bank_paying_subagent — банковский платежный субагент;
/// - paying_agent — платежный агент;
/// - paying_subagent — платежный субагент;
/// - attorney — поверенный;
/// - commission_agent — комиссионер;
/// - another — другой тип агента.
#[derive(Serialize, Deserialize, Debug)]
enum AgentSign {
    BankPayingAgent,
    BankPayingSubagent,
    PayingAgent,
    PayingSubagent,
    Attorney,
    CommisionAgent,
    Another,
}

/// Requirements: <= 24 characters
///
/// Тег ФФД: 1044
///
/// Наименование операции.
///
/// Параметр обязательный, если AgentSign передан в значениях bank_paying_agent или bank_paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct OperationName(String);

/// Телефон в формате +{Ц}.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Phone(String);

/// Requirements: <= 64 characters
///
/// Тег ФФД: 1026
///
/// Наименование оператора перевода.
///
/// Параметр обязательный, если AgentSign передан в значениях bank_paying_agent или bank_paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct OperatorName(String);

/// Requirements: <= 243 characters
///
/// Тег ФФД: 1005
///
/// Адрес оператора перевода.
///
/// Параметр обязательный, если AgentSign передан в значениях bank_paying_agent или bank_paying_subagent.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct OperatorAddress(String);

/// Тег ФФД: 1191
///
/// Дополнительный реквизит предмета расчета.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct UserData(String);

/// Requirements: <= 3 characters
///
/// Тег ФФД: 1230
///
/// Цифровой код страны происхождения товара в соответствии с Общероссийским классификатором стран мира — 3 цифры.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct CountryCode(String);

/// Тег ФФД: 2108
///
/// Единицы измерения:
///
/// шт — применяется для предметов расчета, которые могут быть реализованы поштучно или единицами;
/// г — грамм;
/// кг — килограмм;
/// т — тонна;
/// см — сантиметр;
/// дм — дециметр;
/// м — метр;
/// см2 — квадратный сантиметр;
/// дм2 — квадратный дециметр;
/// м2 — квадратный метр;
/// мл — миллиметр;
/// л — китр;
/// м3 — кубический метр;
/// кВт*ч — киловатт/час;
/// Гкал — гигакалория;
/// сут или дн — сутки или день;
/// ч — час;
/// мин — минута;
/// с — секунда;
/// Кбайт — килобайт;
/// Мбайт — мегабайт;
/// Гбайт — гигабайт;
/// Тбайт — терабайт;
/// — — применяется при использовании иных едениц измерения
/// Также возможна передача произвольных значений.
///
/// Параметр обязательный, если версия ФФД онлайн-кассы — 1.2.
#[derive(Debug, Serialize, Deserialize)]
pub enum MeasurementUnit {
    #[serde(rename = "шт")]
    Piece,

    #[serde(rename = "г")]
    Gram,

    #[serde(rename = "кг")]
    Kilogram,

    #[serde(rename = "т")]
    Ton,

    #[serde(rename = "см")]
    Centimeter,

    #[serde(rename = "дм")]
    Decimeter,

    #[serde(rename = "м")]
    Meter,

    #[serde(rename = "см2")]
    SquareCentimeter,

    #[serde(rename = "дм2")]
    SquareDecimeter,

    #[serde(rename = "м2")]
    SquareMeter,

    #[serde(rename = "мл")]
    Milliliter,

    #[serde(rename = "л")]
    Liter,

    #[serde(rename = "м3")]
    CubicMeter,

    #[serde(rename = "кВт*ч")]
    KilowattHour,

    #[serde(rename = "Гкал")]
    Gigacalorie,

    #[serde(rename = "сут")]
    Day,

    #[serde(rename = "дн")]
    DayAlt,

    #[serde(rename = "ч")]
    Hour,

    #[serde(rename = "мин")]
    Minute,

    #[serde(rename = "с")]
    Second,

    #[serde(rename = "Кбайт")]
    Kilobyte,

    #[serde(rename = "Мбайт")]
    Megabyte,

    #[serde(rename = "Гбайт")]
    Gigabyte,

    #[serde(rename = "Тбайт")]
    Terabyte,

    #[serde(rename = "-")]
    Other,
}

/// Тег ФФД: 2102
///
/// Режим обработки кода маркировки. Должен принимать значение, равное 0.
///
/// Включается в чек, если предметом расчета является товар, который подлежит обязательной маркировке сканером — соответствующий код в поле paymentObject.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct MarkProcessingMode(String);

/// Тег ФФД: 1163
///
/// Код маркировки. Предназначен для нанесения на потребительскую упаковку, товары или товарный ярлык.
///
/// Включается в чек, если предметом расчета является товар, который подлежит обязательной маркировке сканером — соответствующий код в поле paymentObject.
///
/// С 01.09.2025 для чеков с маркированными товарами обязательно передается часовая зона места расчета (тег 1011). По умолчанию — Москва. Для изменения напишите на acq_help@tbank.ru.
#[derive(Serialize, Deserialize, Debug)]
struct MarkCode {
    mark_code_type: MarkCodeType,
    value: Value,
}

/// Код маркировки
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Value(String);

/// Тип штрихкода:
///
/// - UNKNOWN — код товара, формат которого не идентифицирован, как один из реквизитов;
/// - EAN8 — код товара в формате EAN-8;
/// - EAN13 — код товара в формате EAN-13;
/// - ITF14 — код товара в формате ITF-14;
/// - GS10 — код товара в формате GS1, который нанесен на товар, не подлежащий маркировке;
/// - GS1M — код товара в формате GS1, который нанесен на товар, подлежащий маркировке;
/// - SHORT — код товара в формате короткого кода маркировки, который нанесен на товар;
/// - FUR — контрольно-идентификационный знак мехового изделия;
/// - EGAIS20 — код товара в формате ЕГАИС-2.0;
/// - EGAIS30 — код товара в формате ЕГАИС-3.0;
/// - RAWCODE — код маркировки, как он был прочитан сканером.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum MarkCodeType {
    Unknown,
    Ean8,
    Ean13,
    Itf14,
    Gs10,
    Gs1m,
    Short,
    Fur,
    Egais20,
    Egais30,
    Rawcode,
}

/// Реквизит «Дробное количество маркированного товара». Передается, только если расчет осуществляется
/// за маркированный товар — соответствующий код в поле paymentObject и значение в поле measurementUnit равно 0.
///
/// Пример:
///
/// { "numenator": "1" "denominator" "2" }
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MarkQuantity {
    numerator: Numerator,
    denominator: Denominator,
}

///
/// Тег ФФД: 1293
///
/// Числитель дробной части предмета расчета. Значение должно быть строго меньше значения реквизита «знаменатель».
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Numerator(u32);

/// Тег ФФД: 1294
///
/// Знаменатель дробной части предмета расчета. Значение равно количеству товара в партии (упаковке),
/// которая имет общий код маркировки товара.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Denominator(u32);

/// Отраслевой реквизит предмета расчета. Указывается только для товаров, которые подлежат
/// обязательной маркировке сканером. Включение этого реквизита предусмотрено НПА отраслевого
/// регулирования для соответствующей товарной группы.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct SectoralItemProps(Vec<SectoralItemProp>);

/// Отраслевой реквизит предмета расчета.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SectoralItemProp {
    federal_id: FederalId,
    date: SectoralDate,
    number: SectoralNumber,
    value: SectoralValue,
}

/// Тег ФФД: 1262
///
/// Идентификатор ФОИВ — федеральный орган исполнительной власти.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct FederalId(String);

/// Тег ФФД: 1263
///
/// Дата нормативного акта ФОИВ.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct SectoralDate(DateTime<Utc>);

/// Тег ФФД: 1264
///
/// Номер нормативного акта ФОИВ.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct SectoralNumber(String);

/// Тег ФФД: 1265
///
/// Состав значений, котрые определены нормативным актом ФОИВ.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct SectoralValue(String);

/// Тег ФФД: 1270
///
/// Операционный реквизит чека.
#[derive(Serialize, Deserialize, Debug)]
struct OperatingCheckProps;

/// Тег ФФД: 1261
///
/// Отраслевой реквизит чека.
#[derive(Serialize, Deserialize, Debug)]
struct SectoralCheckProps;

/// Тег ФФД: 1084
///
/// Дополнительный реквизит пользователя.
#[derive(Serialize, Deserialize, Debug)]
struct AddUserProp;

/// Тег ФФД: 1192
///
/// Дополнительный реквизит чека (БСО).
#[derive(Serialize, Deserialize, Debug)]
struct AdditionalCheckProps;

/// Requirements: <= 32 characters
///
/// Тег ФФД: 1231
///
/// Номер таможенной декларации.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct DeclarationNumber(String);

/// Тег ФФД: 1229
///
/// Сумма акциза в рублях с учетом копеек, которая включена в стоимость предмета расчета:
///
/// целая часть — не больше 8 знаков;
/// дробная часть — не больше 2 знаков;
/// значение не может быть отрицательным.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Excise(String);

/// Requirements: [full_prepayment, prepayment, advance, full_payment, partial_payment, credit, credit_payment]
///
/// Default: full_payment
///
/// Тег ФФД: 1214
///
/// Признак способа расчета:
///
/// - full_prepayment — предоплата 100%;
/// - prepayment — предоплата;
/// - advance — аванс;
/// - full_payment — полный расчет;
/// - partial_payment — частичный расчет и кредит;
/// - credit — передача в кредит;
/// - credit_payment — оплата кредита.
///
/// Если значение не передано, по умолчанию в онлайн-кассу отправляется признак предмета расчета full_payment.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
enum PaymentMethod {
    FullPrepayment,
    Prepayment,
    Advance,
    #[default]
    FullPayment,
    PartialPayment,
    Credit,
    CreditPayment,
}

/// ИНН
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Inn(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SupplierInfo {
    phones: Vec<Phone>,
    name: SupplierName,
    inn: Inn,
}

/// Requirements: <= 239 characters
///
/// Тег ФФД: 1225
///
/// Наименование поставщика. Параметр обязательный, если передается значение AgentSign в объекте AgentData. Состоит из 239 символов, в которые включаются телефоны поставщика — + 4 символа на каждый телефон.
///
/// Например, если передано два телефона поставщика длиной 12 и 14 символов, максимальная длина наименования поставщика будет 239 – (12 + 4) – (14 + 4) = 205 символов.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct SupplierName(String);

/// Requirements: [commodity, excise, job, service, gambling_bet, gambling_prize, lottery, lottery_prize, intellectual_activity, payment, agent_commission, composite, another]
///
/// Default: commodity
///
/// Тег ФФД: 1212
///
/// Признак предмета расчета:
///
/// commodity — товар;
/// excise — подакцизный товар;
/// job — работа;
/// service — услуга;
/// gambling_bet — ставка азартной игры;
/// gambling_prize — выигрыш азартной игры;
/// lottery — лотерейный билет;
/// lottery_prize — выигрыш лотереи;
/// intellectual_activity — предоставление результатов интеллектуальной деятельности;
/// payment — платеж;
/// agent_commission — агентское вознаграждение;
/// composite — составной предмет расчета;
/// another — иной предмет расчета.
/// Если значение не передано, по умолчанию в онлайн-кассу отправляется признак предмета расчета commodity.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
enum PaymentObjectFF105 {
    #[default]
    Commodity,
    Excise,
    Job,
    Service,
    GamblingBet,
    GamblingPrize,
    Lottery,
    LotteryPrize,
    IntellectualActivity,
    Payment,
    AgentCommission,
    Composite,
    Another,
}

/// Requirements: [commodity, excise, job, service, gambling_bet, gambling_prize, lottery, lottery_prize, intellectual_activity, payment, agent_commission, contribution, property_rights, unrealization, tax_reduction, trade_fee, resort_tax, pledge, income_decrease, ie_pension_insurance_without_payments, ie_pension_insurance_with_payments, ie_medical_insurance_without_payments, ie_medical_insurance_with_payments, social_insurance, casino_chips, agent_payment, excisable_goods_without_marking_code, excisable_goods_with_marking_code, goods_without_marking_code, goods_with_marking_code, another]
///
/// Тег ФФД: 1212
///
/// Значения реквизита «Признак предмета расчета» — тег 1212, таблица 101:
///
/// - commodity — товар;
/// - excise — подакцизный товар;
/// - job — работа;
/// - service — услуга;
/// - gambling_bet — ставка азартной игры;
/// - gambling_prize — выигрыш азартной игры;
/// - lottery — лотерейный билет;
/// - lottery_prize — выигрыш лотереи;
/// - intellectual_activity — предоставление, результатов интеллектуальной деятельности;
/// - payment — платеж;
/// - agent_commission — агентское вознаграждение;
/// - contribution — выплата;
/// - property_rights — имущественное право;
/// - unrealization — внереализационный доход;
/// - tax_reduction — иные платежи и взносы;
/// - trade_fee — торговый сбор;
/// - resort_tax — курортный сбор;
/// - pledge — залог;
/// - income_decrease — расход;
/// - ie_pension_insurance_without_payments — взносы на ОПС ИП;
/// - ie_pension_insurance_with_payments — взносы на ОПС;
/// - ie_medical_insurance_without_payments — взносы на ОМС ИП;
/// - ie_medical_insurance_with_payments — взносы на ОМС;
/// - social_insurance — взносы на ОСС;
/// - casino_chips — платеж казино;
/// - agent_payment — выдача ДС;
/// - excisable_goods_without_marking_code — АТНМ;
/// - excisable_goods_with_marking_code — АТМ;
/// - goods_without_marking_code — ТНМ;
/// - goods_with_marking_code — ТМ;
/// - another — иной предмет расчета.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
enum PaymentObjectFF12 {
    #[default]
    Commodity,
    Excise,
    Job,
    Service,
    GamblingBet,
    GamblingPrize,
    Lottery,
    LotteryPrize,
    IntellectualActivity,
    Payment,
    AgentCommission,
    Contribution,
    PropertyRights,
    Unrealization,
    TaxReduction,
    TradeFee,
    ResortTax,
    Pledge,
    IncomeDecrease,
    IePensionInsuranceWithoutPayments,
    IePensionInsuranceWithPayments,
    IeMedicalInsuranceWithoutPayments,
    IeMedicalInsuranceWithPayments,
    SocialInsurance,
    CasinoChips,
    AgentPayment,
    ExcisableGoodsWithoutMarkingCode,
    ExcisableGoodsWithMarkingCode,
    GoodsWithoutMarkingCode,
    GoodsWithMarkingCode,
    Another,
}

/// Requirements: [none, vat0, vat5, vat7, vat10, vat20, vat22, vat105, vat107, vat110, vat120, vat122]
///
/// Тег ФФД: 1199
///
/// Ставка НДС:
///
/// - none — без НДС,
/// - vat0 — НДС по ставке 0%;
/// - vat5 — НДС по ставке 5%;
/// - vat7 — НДС по ставке 7%;
/// - vat10 — НДС по ставке 10%;
/// - vat20 — НДС по ставке 20%;
/// - vat22 — НДС по ставке 22% (c 01.01.2026);
/// - vat105 — НДС чека по расчетной ставке 5/105;
/// - vat107 — НДС чека по расчетной ставке 7/107;
/// - vat110 — НДС чека по расчетной ставке 10/110;
/// - vat120 — НДС чека по расчетной ставке 20/120;
/// - vat122 — НДС чека по расчетной ставке 22/122 (с 01.01.2026).
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Tax {
    None,
    Vat0,
    Vat5,
    Vat7,
    Vat10,
    Vat20,
    Vat22,
    Vat105,
    Vat107,
    Vat110,
    Vat120,
    Vat122,
}

/// Requirements: <= 300 characters
///
/// Тег ФФД: 1162
///
/// Штрихкод. В зависимости от типа кассы требования к штрихкоду могут отличаться:
///
/// АТОЛ Онлайн — шестнадцатеричное представление с пробелами. Максимальная длина — 32 байта (^[a-fA-F0-9]{2}$)|(^([afA-F0-9]{2}\s){1,31}[a-fA-F0-9]{2}$).
/// Пример: 00 00 00 01 00 21 FA 41 00 23 05 41 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 12 00 AB 00
///
/// CloudKassir — длина строки: четная, от 8 до 150 байт. То есть от 16 до 300 ASCII символов ['0' - '9' , 'A' - 'F' ] шестнадцатеричного представления кода маркировки товара.
/// Пример: 303130323930303030630333435
///
/// OrangeData — строка, содержащая Base64- кодированный массив от 8 до 32 байт.
/// Пример: igQVAAADMTIzNDU2Nzg5MDEyMwAAAAAAAQ==
///
/// Если в запросе передается параметр Ean13, который не прошел валидацию, то вернется неуспешный ответ с текстом ошибки в параметре message = Неверный параметр Ean13.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Ean13(String);

/// Requirements: <= 8 characters
///
/// Количество или вес товара. Максимальное количество символов — 8, где:
///
/// целая часть — не больше 5 знаков;
/// дробная — не больше 3 знаков для Атол и 2 знаков для CloudPayments.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Quantity(NonZeroU16);

/// Код магазина. Для параметра ShopСode нужно использовать значение параметра Submerchant_ID, который возвращается в ответе при регистрации магазинов через XML. Если XML не используется, передавать поле не нужно.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ShopCode(String);

/// JSON-объект с данными чека. Параметр обязательный, если подключена онлайн-касса.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Receipt {
    FFD105 {
        items: ReceiptItemsFFD105,
        ffd_version: FfdVersion,
        email: ReceiptEmail,
        phone: ReceiptPhone,
        taxation: Taxation,
        payments: Payments,
    },
    FFD12 {
        items: ReceiptItemsFFD12,
        ffd_vesrion: FfdVersion,
        client_info: ClientInfo,
        taxation: Taxation,
        email: ReceiptEmail,
        phone: ReceiptPhone,
        customer: Customer,
        customer_inn: CustomerInn,
        payments: Payments,
        operating_check_props: OperatingCheckProps,
        sectoral_check_props: SectoralCheckProps,
        add_user_prop: AddUserProp,
        additional_check_props: AdditionalCheckProps,
    },
}

/// Тег ФФД: 1228
///
/// ИНН клиента.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct CustomerInn(Inn);

/// Тег ФФД: 1227
///
/// Идентификатор/имя клиента.
///
/// В параметре можно передавать только email или номер телефона.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Customer(String);

/// Информация по клиенту.
#[derive(Serialize, Deserialize, Debug)]
struct ClientInfo {
    birthdate: Birthdate,
    citizenship: Citizenship,
    document_code: DocumentCode,
    document_data: DocumentData,
    address: Address,
}

/// Тег ФФД: 1243
///
/// Дата рождения клиента в формате ДД.ММ.ГГГГ.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Birthdate(String);

/// Тег ФФД: 1244
///
/// Числовой код страны, гражданином которой является клиент. Код страны указывается в соответствии с Общероссийским классификатором стран мира [ОКСМ](https://classifikators.ru/oksm).
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Citizenship(String);

/// Тег ФФД: 1245
///
/// Числовой код вида документа, удостоверяющего личность.
///
/// Может принимать только следующие значения:
///
/// - 21 — паспорт гражданина Российской Федерации.
/// - 22 — паспорт гражданина Российской Федерации, дипломатический паспорт, служебный паспорт, удостоверяющие личность гражданина Российской Федерации за пределами Российской Федерации.
/// - 26 — временное удостоверение личности гражданина Российской Федерации, выдаваемое на период оформления паспорта гражданина Российской Федерации.
/// - 27 — свидетельство о рождении гражданина Российской Федерации. Для граждан Российской Федерации в возрасте до 14 лотереи.
/// - 28 — иные документы, признаваемые документами, удостоверяющими личность гражданина Российской Федерации в соответствии с законодательством Российской Федерации.
/// - 31 — паспорт иностранного гражданина.
/// - 32 — иные документы, признаваемые документами, удостоверяющими личность иностранного гражданина в соответствии с законодательством Российской Федерации и международным договором Российской Федерации.
/// - 33 — документ, выданный иностранным государством и признаваемый в соответствии с международным договором Российской Федерации в качестве документа, удостоверяющего личность лица безгражданства.
/// - 34 — вид на жительство, для лиц без гражданства.
/// - 35 - разрешение на временное проживание, для лиц без гражданства.
/// - 36 — свидетельство о рассмотрении ходатайства о признании лица без гражданства беженцем на территории Российской Федерации по существу.
/// - 37 — удостоверение беженца.
/// - 38 — иные документы, признаваемые документами, удостоверяющими личность лиц без гражданства в соответствии с  законодательством Российской Федерации и международным договором Российской Федерации.
/// - 40 — документ, удостоверяющий личность лица, не имеющего действительного документа, удостоверяющего личность, на период рассмотрения заявления о признании гражданином Российской Федерации или о приеме в гражданство Российской  Федерации.
#[derive(Serialize, Deserialize, Debug)]
enum DocumentCode {
    #[serde(rename = "21")]
    C21,
    #[serde(rename = "22")]
    C22,
    #[serde(rename = "26")]
    C26,
    #[serde(rename = "27")]
    C27,
    #[serde(rename = "28")]
    C28,
    #[serde(rename = "31")]
    C31,
    #[serde(rename = "32")]
    C32,
    #[serde(rename = "33")]
    C33,
    #[serde(rename = "34")]
    C34,
    #[serde(rename = "35")]
    C35,
    #[serde(rename = "36")]
    C36,
    #[serde(rename = "37")]
    C37,
    #[serde(rename = "38")]
    C38,
    #[serde(rename = "40")]
    C40,
}

/// Тег ФФД: 1246
///
/// Реквизиты документа, удостоверяющего личность. Например, серия и номер паспорта.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct DocumentData(String);

/// Requirements: <= 256 characters
///
/// Тег ФФД: 1254
///
/// Адрес клиента-грузополучателя.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Address(String);

/// Requirements: [1.2, 1.05]
///
/// Default: 1.05
///
/// Версия ФФД.
#[derive(Serialize, Deserialize, Debug, Default)]
enum FfdVersion {
    #[serde(rename = "1.2")]
    V12,
    #[default]
    #[serde(rename = "1.05")]
    V105,
}

/// Requirements: <= 64 characters
///
/// Тег ФФД: 1008.
///
/// Электронная почта клиента. Параметр обязательный, если не передан Phone.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ReceiptEmail(Email);

/// Requirements: <= 64 characters
///
/// Тег ФФД: 1008.
///
/// Телефон клиента в формате +{Ц}. Параметр обязательный, если не передан Email.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ReceiptPhone(Phone);

/// Requirements: [osn, usn_income, usn_income_outcome, esn, patent]
///
/// Тег ФФД: 1055.
///
/// Система налогообложения:
///
/// - osn — общая СН;
/// - usn_income — упрощенная СН (доходы). Налоговая автоматически определит АУСН по ИНН и пробьет чеки с нужной СНО;
/// - usn_income_outcome — упрощенная СН (доходы минус расходы). Налоговая автоматически определит АУСН по ИНН и пробьет чеки с нужной СНО;
/// - esn — единый сельскохозяйственный налог;
/// - patent — патентная СН.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Taxation {
    Osn,
    UsnIncome,
    UsnIncomeOutcome,
    Esn,
    Patent,
}

/// Инвариант электронной почты
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Email(String);

/// Массив позиций чека с информацией о товарах. Количество товаров в чеке — не больше 100.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ReceiptItemsFFD105(Vec<ItemFFD105>);

/// Массив с информацией о товарах. Количество товаров в чеке — не больше 100.
///
/// Параметры, которые предусмотрены в протоколе для отправки чеков по маркируемым товарам, не являются обязательными для товаров без маркировки.
///
/// Если используется ФФД 1.2, но реализуемый товар не подлежит маркировке, поля можно не отправлять или отправить со значением null.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ReceiptItemsFFD12(Vec<ItemFFD12>);

/// Детали платежа.
///
/// Если объект не передан, автоматически указывается итоговая сумма чека с видом оплаты «Безналичный».
///
/// Если передан объект receipt.Payments, значение в Electronic должно быть равно итоговому значению Amount в методе Инициировать платеж. При этом сумма введенных значений по всем видам оплат, включая Electronic, должна быть равна сумме (Amount) всех товаров, которые были переданы в объекте receipt.Items.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Payments {
    electronic: Electronic,
    cash: Option<Cash>,
    advance_payment: Option<AdvancePayment>,
    credit: Option<Credit>,
    provision: Option<Provision>,
}

/// Requirements: <= 14 characters
///
/// Тег ФФД: 1031.
///
/// Вид оплаты «Наличные». Сумма к оплате в копейках.
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Cash(Amount);

/// Requirements: <= 14 characters
///
/// Тег ФФД: 1081.
///
/// Вид оплаты «Безналичный».
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Electronic(Amount);

/// Requirements: <= 14 characters
///
/// Тег ФФД: 1215.
///
/// Вид оплаты «Предварительная оплата (Аванс)».
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct AdvancePayment(Amount);

/// Requirements: <= 14 characters
///
/// Тег ФФД: 1216.
///
/// Вид оплаты «Постоплата (Кредит)».
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Credit(Amount);

/// Requirements: <= 14 characters
///
/// Тег ФФД: 1217.
///
/// Вид оплаты «Иная форма оплаты».
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Provision(Amount);
