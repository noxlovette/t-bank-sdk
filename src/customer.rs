use crate::{TerminalKey, impl_string_conversions_default};
use strum::{AsRefStr, Display, EnumString};
use url::Url;

// ─── AddCustomer ─────────────────────────────────────────────────────────────

/// Зарегистрировать покупателя.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddCustomerReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    /// Идентификатор покупателя в системе мерчанта (до 36 символов).
    pub customer_key: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub email: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub phone: Option<String>,
}

impl AddCustomerReq {
    pub fn new(terminal_key: &TerminalKey, customer_key: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            customer_key: customer_key.into(),
            ..Default::default()
        }
    }

    /// Установить электронную почту покупателя.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Установить номер телефона покупателя.
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    /// Установить IP-адрес покупателя.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddCustomerRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
}

// ─── GetCustomer ─────────────────────────────────────────────────────────────

/// Получить данные покупателя.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetCustomerReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
}

impl GetCustomerReq {
    pub fn new(terminal_key: &TerminalKey, customer_key: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            customer_key: customer_key.into(),
            ip: None,
        }
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetCustomerRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

// ─── RemoveCustomer ───────────────────────────────────────────────────────────

/// Удалить покупателя и все привязанные к нему карты.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemoveCustomerReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
}

impl RemoveCustomerReq {
    pub fn new(terminal_key: &TerminalKey, customer_key: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            customer_key: customer_key.into(),
            ip: None,
        }
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemoveCustomerRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
}

// ─── Shared card types ────────────────────────────────────────────────────────

/// Тип проверки карты при привязке.
#[derive(Debug, Default, Display, AsRefStr, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CheckType {
    /// Без проверки (только сохранение реквизитов).
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "NO"))]
    #[strum(serialize = "NO")]
    No,
    /// Холдирование 1 руб. без 3DS.
    #[cfg_attr(feature = "serde", serde(rename = "HOLD"))]
    #[strum(serialize = "HOLD")]
    Hold,
    /// Проверка с 3DS без холдирования.
    #[cfg_attr(feature = "serde", serde(rename = "3DS"))]
    #[strum(serialize = "3DS")]
    ThreeDs,
    /// Проверка с 3DS и холдированием 1 руб.
    #[cfg_attr(feature = "serde", serde(rename = "3DSHOLD"))]
    #[strum(serialize = "3DSHOLD")]
    ThreeDsHold,
}

impl_string_conversions_default!(CheckType);

/// Статус карты в системе Т‑Бизнес.
#[derive(Debug, Default, Display, AsRefStr, EnumString)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "UPPERCASE")]
pub enum CardStatus {
    /// Активна.
    #[default]
    A,
    /// Неактивна.
    I,
    /// Удалена.
    D,
}

impl_string_conversions_default!(CardStatus);

/// Информация о привязанной карте.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CardInfo {
    pub card_id: String,
    /// Маскированный номер карты, например 430000****0777.
    pub pan: String,
    /// Срок действия в формате MMYY.
    pub exp_date: String,
    /// Тип платёжной системы: 0=Visa, 1=MC, 2=MIR, 3=МС/Maestro, 4=неизвестно.
    pub card_type: Option<u8>,
    pub status: CardStatus,
    /// Идентификатор для рекуррентных платежей.
    pub rebill_id: Option<String>,
    pub is_default: Option<bool>,
}

// ─── AddCard ─────────────────────────────────────────────────────────────────

/// Инициировать привязку карты к покупателю.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddCardReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub check_type: Option<CheckType>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
    /// Признак резидентства карты: true — РФ, false — не РФ, null — универсальный.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resident_state: Option<bool>,
}

impl AddCardReq {
    pub fn new(terminal_key: &TerminalKey, customer_key: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            customer_key: customer_key.into(),
            ..Default::default()
        }
    }

    /// Установить тип проверки карты при привязке.
    pub fn check_type(mut self, check_type: CheckType) -> Self {
        self.check_type = Some(check_type);
        self
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    /// Установить признак резидентства карты.
    pub fn resident_state(mut self, resident: bool) -> Self {
        self.resident_state = Some(resident);
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddCardRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    /// Уникальный идентификатор запроса на привязку карты.
    pub request_key: String,
    /// Ссылка на форму привязки карты.
    #[cfg_attr(feature = "serde", serde(rename = "PaymentURL"))]
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub payment_url: Option<Url>,
}

// ─── GetCardList ──────────────────────────────────────────────────────────────

/// Получить список привязанных карт покупателя.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetCardListReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    /// Фильтровать по признаку сохранения для оплаты в один клик.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub saved_card: Option<bool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
}

impl GetCardListReq {
    pub fn new(terminal_key: &TerminalKey, customer_key: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            customer_key: customer_key.into(),
            ..Default::default()
        }
    }

    /// Фильтровать карты по признаку сохранения для оплаты в один клик.
    pub fn saved_card(mut self, saved: bool) -> Self {
        self.saved_card = Some(saved);
        self
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

// ─── RemoveCard ───────────────────────────────────────────────────────────────

/// Удалить привязанную карту покупателя.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemoveCardReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    /// Идентификатор карты в системе Т‑Бизнес (до 40 символов).
    pub card_id: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
}

impl RemoveCardReq {
    pub fn new(
        terminal_key: &TerminalKey,
        customer_key: impl Into<String>,
        card_id: impl Into<String>,
    ) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            customer_key: customer_key.into(),
            card_id: card_id.into(),
            ip: None,
        }
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemoveCardRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    pub card_id: String,
    pub status: CardStatus,
}
