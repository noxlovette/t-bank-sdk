use crate::{TerminalKey, impl_string_conversions_default};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use url::Url;
use utoipa::ToSchema;

// ─── AddCustomer ─────────────────────────────────────────────────────────────

/// Зарегистрировать покупателя.
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddCustomerReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    /// Идентификатор покупателя в системе мерчанта (до 36 символов).
    pub customer_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddCustomerRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
}

// ─── GetCustomer ─────────────────────────────────────────────────────────────

/// Получить данные покупателя.
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct GetCustomerReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct GetCustomerRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

// ─── RemoveCustomer ───────────────────────────────────────────────────────────

/// Удалить покупателя и все привязанные к нему карты.
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveCustomerReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveCustomerRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
}

// ─── Shared card types ────────────────────────────────────────────────────────

/// Тип проверки карты при привязке.
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Display, AsRefStr, EnumString)]
pub enum CheckType {
    /// Без проверки (только сохранение реквизитов).
    #[default]
    #[serde(rename = "NO")]
    #[strum(serialize = "NO")]
    No,
    /// Холдирование 1 руб. без 3DS.
    #[serde(rename = "HOLD")]
    #[strum(serialize = "HOLD")]
    Hold,
    /// Проверка с 3DS без холдирования.
    #[serde(rename = "3DS")]
    #[strum(serialize = "3DS")]
    ThreeDs,
    /// Проверка с 3DS и холдированием 1 руб.
    #[serde(rename = "3DSHOLD")]
    #[strum(serialize = "3DSHOLD")]
    ThreeDsHold,
}

impl_string_conversions_default!(CheckType);

/// Статус карты в системе Т‑Бизнес.
#[derive(Debug, Serialize, Deserialize, Default, ToSchema, Display, AsRefStr, EnumString)]
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
#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
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
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddCardReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<CheckType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Признак резидентства карты: true — РФ, false — не РФ, null — универсальный.
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn check_type(mut self, check_type: CheckType) -> Self {
        self.check_type = Some(check_type);
        self
    }

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    pub fn resident_state(mut self, resident: bool) -> Self {
        self.resident_state = Some(resident);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AddCardRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    /// Уникальный идентификатор запроса на привязку карты.
    pub request_key: String,
    /// Ссылка на форму привязки карты.
    #[serde(rename = "PaymentURL")]
    #[schema(value_type = String)]
    pub payment_url: Option<Url>,
}

// ─── GetCardList ──────────────────────────────────────────────────────────────

/// Получить список привязанных карт покупателя.
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct GetCardListReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    /// Фильтровать по признаку сохранения для оплаты в один клик.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_card: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn saved_card(mut self, saved: bool) -> Self {
        self.saved_card = Some(saved);
        self
    }

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

// ─── RemoveCard ───────────────────────────────────────────────────────────────

/// Удалить привязанную карту покупателя.
#[derive(Debug, Serialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveCardReq {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    /// Идентификатор карты в системе Т‑Бизнес (до 40 символов).
    pub card_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveCardRes {
    #[schema(value_type = String)]
    pub terminal_key: TerminalKey,
    pub customer_key: String,
    pub card_id: String,
    pub status: CardStatus,
}
