use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;

/// JSON-объект с данными чека. Параметр обязательный, если подключена онлайн-касса.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all_fields = "PascalCase", untagged)]
pub enum Receipt {
    FFD105 {
        /// Массив позиций чека с информацией о товарах. Количество товаров в чеке — не больше 100.
        items: Vec<ItemFFD105>,
        ffd_version: Option<FfdVersion>,
        /// Requirements: <= 64 characters
        ///
        /// Тег ФФД: 1008.
        ///
        /// Электронная почта клиента. Параметр обязательный, если не передан Phone.
        email: Option<String>,
        /// Requirements: <= 64 characters
        ///
        /// Тег ФФД: 1008.
        ///
        /// Телефон клиента в формате +{Ц}. Параметр обязательный, если не передан Email.
        phone: Option<String>,
        taxation: Taxation,
        payments: Option<Payments>,
    },
    FFD12 {
        /// Массив с информацией о товарах. Количество товаров в чеке — не больше 100.
        ///
        /// Параметры, которые предусмотрены в протоколе для отправки чеков по  маркируемым товарам, не являются обязательными для товаров без маркировки.
        ///
        /// Если используется ФФД 1.2, но реализуемый товар не подлежит маркировке, поля можно не отправлять или отправить со значением null.
        items: Vec<ItemFFD12>,
        ffd_vesrion: Option<FfdVersion>,
        client_info: Option<ClientInfo>,
        taxation: Taxation,
        /// Requirements: <= 64 characters
        ///
        /// Тег ФФД: 1008.
        ///
        /// Электронная почта клиента. Параметр обязательный, если не передан Phone.
        email: Option<String>,
        /// Requirements: <= 64 characters
        ///
        /// Тег ФФД: 1008.
        ///
        /// Телефон клиента в формате +{Ц}. Параметр обязательный, если не передан Email.
        phone: Option<String>,
        /// Тег ФФД: 1227
        ///
        /// Идентификатор/имя клиента.
        ///
        /// В параметре можно передавать только email или номер телефона.
        customer: Option<String>,
        /// Тег ФФД: 1228
        ///
        /// ИНН клиента.
        customer_inn: Option<String>,
        payments: Option<Payments>,
        operating_check_props: Option<OperatingCheckProps>,
        sectoral_check_props: Option<SectoralCheckProps>,
        add_user_prop: Option<AddUserProp>,
        additional_check_props: Option<AdditionalCheckProps>,
    },
}

impl Receipt {
    pub fn ffd105(items: Vec<ItemFFD105>, taxation: Taxation) -> Self {
        Self::FFD105 {
            items,
            ffd_version: None,
            email: None,
            phone: None,
            taxation,
            payments: None,
        }
    }

    pub fn ffd12(items: Vec<ItemFFD12>, taxation: Taxation) -> Self {
        Self::FFD12 {
            items,
            ffd_vesrion: None,
            client_info: None,
            taxation,
            email: None,
            phone: None,
            customer: None,
            customer_inn: None,
            payments: None,
            operating_check_props: None,
            sectoral_check_props: None,
            add_user_prop: None,
            additional_check_props: None,
        }
    }

    pub fn ffd_version(mut self, version: FfdVersion) -> Self {
        match &mut self {
            Self::FFD105 { ffd_version, .. } => *ffd_version = Some(version),
            Self::FFD12 { ffd_vesrion, .. } => *ffd_vesrion = Some(version),
        }
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        match &mut self {
            Self::FFD105 { email: value, .. } | Self::FFD12 { email: value, .. } => {
                *value = Some(email.to_string())
            }
        }
        self
    }

    pub fn phone(mut self, phone: &str) -> Self {
        match &mut self {
            Self::FFD105 { phone: value, .. } | Self::FFD12 { phone: value, .. } => {
                *value = Some(phone.to_string())
            }
        }
        self
    }

    pub fn payments(mut self, payments: Payments) -> Self {
        match &mut self {
            Self::FFD105 {
                payments: value, ..
            }
            | Self::FFD12 {
                payments: value, ..
            } => *value = Some(payments),
        }
        self
    }
}

/// Позиция чека с информацией о товарах.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ItemFFD105 {
    /// Тег ФФД: 1030
    ///
    /// Наименование товара.
    pub name: String,
    /// Тег ФФД: 1078
    ///
    /// Цена в копейках.
    pub price: u32,
    /// Тег ФФД: 1023
    ///
    /// Количество или вес товара.
    pub quantity: u16,
    /// Тег ФФД: 1043
    ///
    /// Стоимость товара в копейках. Произведение Quantity и Price.
    pub amount: u32,
    pub payment_method: PaymentMethod,
    pub payment_object: PaymentObjectFF105,
    pub tax: Tax,
    pub ean_13: Option<String>,
    pub agent_data: Option<AgentData>,
    pub supplier_info: Option<SupplierInfo>,
}

impl ItemFFD105 {
    pub fn new(name: &str, price: u32, quantity: u16, amount: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
            quantity,
            amount,
            ..Default::default()
        }
    }

    pub fn payment_method(mut self, payment_method: PaymentMethod) -> Self {
        self.payment_method = payment_method;
        self
    }

    pub fn payment_object(mut self, payment_object: PaymentObjectFF105) -> Self {
        self.payment_object = payment_object;
        self
    }

    pub fn tax(mut self, tax: Tax) -> Self {
        self.tax = tax;
        self
    }

    pub fn ean_13(mut self, ean_13: &str) -> Self {
        self.ean_13 = Some(ean_13.to_string());
        self
    }

    pub fn agent_data(mut self, agent_data: AgentData) -> Self {
        self.agent_data = Some(agent_data);
        self
    }

    pub fn supplier_info(mut self, supplier_info: SupplierInfo) -> Self {
        self.supplier_info = Some(supplier_info);
        self
    }
}

/// Позиция чека с информацией о товарах.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ItemFFD12 {
    agent_data: AgentData,
    supplier_info: SupplierInfo,
    /// Requirements: <= 128 characters
    ///
    /// Тег ФФД: 1030
    ///
    /// Наименование товара.
    name: String,
    /// Тег ФФД: 1079
    ///
    /// Цена в копейках.
    price: u32,
    /// Requirements: <= 8 characters
    /// Тег ФФД: 1023
    ///
    /// Количество или вес товара. Максимальное количество символов — 8, где:
    ///
    /// целая часть — не больше 5 знаков,
    /// дробная — не больше 3 знаков для Атол и 2 знаков для CloudPayments.
    quantity: u16,
    /// Requirements: <= 10 characters
    ///
    /// Тег ФФД: 1043
    ///
    /// Стоимость товара в копейках. Произведение Quantity и Price.
    amount: u32,
    payment_method: PaymentMethod,
    payment_object: PaymentObjectFF12,
    tax: Tax,
    /// Тег ФФД: 1191
    ///
    /// Дополнительный реквизит предмета расчета.
    user_data: String,
    /// Тег ФФД: 1229
    ///
    /// Сумма акциза в рублях с учетом копеек, которая включена в стоимость предмета расчета:
    ///
    /// целая часть — не больше 8 знаков;
    /// дробная часть — не больше 2 знаков;
    /// значение не может быть отрицательным.
    excise: String,
    /// Requirements: <= 3 characters
    ///
    /// Тег ФФД: 1230
    ///
    /// Цифровой код страны происхождения товара в соответствии с Общероссийским  классификатором стран мира — 3 цифры.
    country_code: String,
    /// Requirements: <= 32 characters
    ///
    /// Тег ФФД: 1231
    ///
    /// Номер таможенной декларации.
    declaration_number: String,
    measurement_unit: MeasurementUnit,
    /// Тег ФФД: 2102
    ///
    /// Режим обработки кода маркировки. Должен принимать значение, равное 0.
    ///
    /// обязательной маркировке сканером — соответствующий код в поле paymentObject.
    mark_processing_mode: String,
    mark_code: Vec<MarkCode>,
    mark_quantity: MarkQuantity,
    sectoral_item_props: SectoralItemProps,
}

impl ItemFFD12 {
    pub fn new(name: &str, price: u32, quantity: u16, amount: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
            quantity,
            amount,
            mark_processing_mode: "0".to_string(),
            ..Default::default()
        }
    }

    pub fn payment_method(mut self, payment_method: PaymentMethod) -> Self {
        self.payment_method = payment_method;
        self
    }

    pub fn tax(mut self, tax: Tax) -> Self {
        self.tax = tax;
        self
    }

    pub fn user_data(mut self, user_data: &str) -> Self {
        self.user_data = user_data.to_string();
        self
    }

    pub fn excise(mut self, excise: &str) -> Self {
        self.excise = excise.to_string();
        self
    }

    pub fn country_code(mut self, country_code: &str) -> Self {
        self.country_code = country_code.to_string();
        self
    }

    pub fn declaration_number(mut self, declaration_number: &str) -> Self {
        self.declaration_number = declaration_number.to_string();
        self
    }

    pub fn measurement_unit(mut self, measurement_unit: MeasurementUnit) -> Self {
        self.measurement_unit = measurement_unit;
        self
    }

    pub fn mark_processing_mode(mut self, mark_processing_mode: &str) -> Self {
        self.mark_processing_mode = mark_processing_mode.to_string();
        self
    }

    pub fn agent_data(mut self, agent_data: AgentData) -> Self {
        self.agent_data = agent_data;
        self
    }

    pub fn supplier_info(mut self, supplier_info: SupplierInfo) -> Self {
        self.supplier_info = supplier_info;
        self
    }
}

/// Данные агента. Параметр обязательный, если используется агентская схема.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AgentData {
    agent_sign: AgentSign,
    operation_name: String,
    phones: Vec<String>,
    receiver_phones: Vec<String>,
    transfer_phones: Vec<String>,
    operator_name: String,
    operator_address: String,
    perator_inn: String,
}

impl Default for AgentData {
    fn default() -> Self {
        Self {
            agent_sign: AgentSign::Another,
            operation_name: String::new(),
            phones: Vec::new(),
            receiver_phones: Vec::new(),
            transfer_phones: Vec::new(),
            operator_name: String::new(),
            operator_address: String::new(),
            perator_inn: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SupplierInfo {
    phones: Vec<String>,
    name: String,
    inn: String,
}

/// Тег ФФД: 1163
///
/// Код маркировки. Предназначен для нанесения на потребительскую упаковку, товары или товарный ярлык.
///
/// Включается в чек, если предметом расчета является товар, который подлежит обязательной маркировке сканером — соответствующий код в поле paymentObject.
///
/// С 01.09.2025 для чеков с маркированными товарами обязательно передается часовая зона места расчета (тег 1011). По умолчанию — Москва. Для изменения напишите на acq_help@tbank.ru.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MarkCode {
    mark_code_type: MarkCodeType,
    value: String,
}

/// Реквизит «Дробное количество маркированного товара». Передается, только если расчет осуществляется
/// за маркированный товар — соответствующий код в поле paymentObject и значение в поле measurementUnit равно 0.
///
/// Пример:
///
/// { "numenator": "1" "denominator" "2" }
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MarkQuantity {
    numerator: u32,
    denominator: u32,
}

/// Отраслевой реквизит предмета расчета.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SectoralItemProps {
    /// Тег ФФД: 1262
    ///
    /// Идентификатор ФОИВ — федеральный орган исполнительной власти.
    federal_id: String,
    /// Тег ФФД: 1263
    ///
    /// Дата нормативного акта ФОИВ.
    date: DateTime<Utc>,
    /// Тег ФФД: 1264
    ///
    /// Номер нормативного акта ФОИВ.
    number: String,
    /// Тег ФФД: 1265
    ///
    /// Состав значений, котрые определены нормативным актом ФОИВ.
    value: String,
}

/// Детали платежа.
///
/// Если объект не передан, автоматически указывается итоговая сумма чека с видом оплаты «Безналичный».
///
/// Если передан объект receipt.Payments, значение в Electronic должно быть равно итоговому значению Amount в методе Инициировать платеж. При этом сумма введенных значений по всем видам оплат, включая Electronic, должна быть равна сумме (Amount) всех товаров, которые были переданы в объекте receipt.Items.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Payments {
    electronic: u32,
    cash: Option<u32>,
    advance_payment: Option<u32>,
    credit: Option<u32>,
    provision: Option<u32>,
}

impl Payments {
    pub fn new(electronic: u32) -> Self {
        Self {
            electronic,
            cash: None,
            advance_payment: None,
            credit: None,
            provision: None,
        }
    }

    pub fn cash(mut self, cash: u32) -> Self {
        self.cash = Some(cash);
        self
    }

    pub fn advance_payment(mut self, advance_payment: u32) -> Self {
        self.advance_payment = Some(advance_payment);
        self
    }

    pub fn credit(mut self, credit: u32) -> Self {
        self.credit = Some(credit);
        self
    }

    pub fn provision(mut self, provision: u32) -> Self {
        self.provision = Some(provision);
        self
    }
}

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

#[derive(Debug, Serialize, Deserialize, Default)]
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
    #[default]
    Other,
}

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
pub enum PaymentMethod {
    FullPrepayment,
    Prepayment,
    Advance,
    #[default]
    FullPayment,
    PartialPayment,
    Credit,
    CreditPayment,
}

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
pub enum PaymentObjectFF105 {
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
#[derive(Serialize, Deserialize, Debug, Default, Display)]
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
#[derive(Serialize, Deserialize, Debug, Display, Default)]
#[serde(rename_all = "lowercase")]
pub enum Tax {
    #[default]
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
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum Taxation {
    UsnIncome,
    #[default]
    Osn,
    UsnIncomeOutcome,
    Esn,
    Patent,
}

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

/// Requirements: [1.2, 1.05]
///
/// Default: 1.05
///
/// Версия ФФД.
#[derive(Serialize, Deserialize, Debug, Default)]
pub enum FfdVersion {
    #[serde(rename = "1.2")]
    V12,
    #[default]
    #[serde(rename = "1.05")]
    V105,
}

/// Информация по клиенту.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    birthdate: String,
    citizenship: String,
    document_code: DocumentCode,
    document_data: String,
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperatingCheckProps;

#[derive(Serialize, Deserialize, Debug)]
pub struct SectoralCheckProps;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddUserProp;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalCheckProps;
