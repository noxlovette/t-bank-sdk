use crate::{Receipt, TerminalKey};

use crate::payment::{PaymentStatus, Shop};

// ─── Confirm ─────────────────────────────────────────────────────────────────

/// Подтверждение двухстадийного платежа (после статуса AUTHORIZED).
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ConfirmPaymentReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    /// Идентификатор платежа в системе Т‑Бизнес.
    pub payment_id: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
    /// Сумма в копейках. Если не передана — подтверждается полная сумма.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub receipt: Option<Receipt>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub shops: Vec<Shop>,
}

impl ConfirmPaymentReq {
    pub fn new(terminal_key: &TerminalKey, payment_id: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            payment_id: payment_id.into(),
            ..Default::default()
        }
    }

    /// Установить сумму подтверждения в копейках.
    pub fn amount(mut self, amount: u32) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    /// Установить данные чека.
    pub fn receipt(mut self, receipt: Receipt) -> Self {
        self.receipt = Some(receipt);
        self
    }

    /// Установить список магазинов (для маркетплейсов).
    pub fn shops(mut self, shops: Vec<Shop>) -> Self {
        self.shops = shops;
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ConfirmPaymentRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub status: PaymentStatus,
    pub payment_id: String,
    pub order_id: String,
    pub amount: u32,
}

// ─── Cancel ──────────────────────────────────────────────────────────────────

/// Отмена или возврат платежа.
///
/// Если Amount не передан, отменяется полная сумма. Для частичного возврата
/// передайте Amount < исходной суммы.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CancelPaymentReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub payment_id: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
    /// Сумма возврата в копейках. По умолчанию — полная сумма платежа.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount: Option<u32>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub receipt: Option<Receipt>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub shops: Vec<Shop>,
    /// Идентификатор операции на стороне мерчанта (до 256 символов).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub external_request_id: Option<String>,
}

impl CancelPaymentReq {
    pub fn new(terminal_key: &TerminalKey, payment_id: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            payment_id: payment_id.into(),
            ..Default::default()
        }
    }

    /// Установить сумму возврата в копейках.
    pub fn amount(mut self, amount: u32) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    /// Установить данные чека.
    pub fn receipt(mut self, receipt: Receipt) -> Self {
        self.receipt = Some(receipt);
        self
    }

    /// Установить список магазинов (для маркетплейсов).
    pub fn shops(mut self, shops: Vec<Shop>) -> Self {
        self.shops = shops;
        self
    }

    /// Установить идентификатор операции мерчанта.
    pub fn external_request_id(mut self, id: impl Into<String>) -> Self {
        self.external_request_id = Some(id.into());
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CancelPaymentRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub status: PaymentStatus,
    pub payment_id: String,
    pub order_id: String,
    pub amount: u32,
    /// Исходная сумма платежа в копейках.
    pub original_amount: Option<u32>,
    /// Новая сумма платежа после частичного возврата.
    pub new_amount: Option<u32>,
}

// ─── Charge ──────────────────────────────────────────────────────────────────

/// Провести платеж по сохраненным реквизитам (рекуррентный / автоплатеж).
///
/// Требует, чтобы ранее был проведен родительский платеж с Recurrent=Y и
/// получен RebillId.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChargePaymentReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    /// Идентификатор платежа, созданного через Init.
    pub payment_id: String,
    /// Уникальный идентификатор сохраненных реквизитов карты.
    pub rebill_id: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
    /// Отправить уведомление покупателю на email.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub send_email: Option<bool>,
    /// Email покупателя (обязателен если SendEmail=true).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub info_email: Option<String>,
}

impl ChargePaymentReq {
    pub fn new(
        terminal_key: &TerminalKey,
        payment_id: impl Into<String>,
        rebill_id: impl Into<String>,
    ) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            payment_id: payment_id.into(),
            rebill_id: rebill_id.into(),
            ..Default::default()
        }
    }

    /// Установить IP-адрес запроса.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    /// Установить email для отправки уведомления покупателю.
    pub fn send_email(mut self, email: impl Into<String>) -> Self {
        self.send_email = Some(true);
        self.info_email = Some(email.into());
        self
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChargePaymentRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub status: PaymentStatus,
    pub payment_id: String,
    pub order_id: String,
    pub amount: u32,
}

// ─── GetState ────────────────────────────────────────────────────────────────

/// Получить статус платежа.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetStateReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub payment_id: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ip: Option<String>,
}

impl GetStateReq {
    pub fn new(terminal_key: &TerminalKey, payment_id: impl Into<String>) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            payment_id: payment_id.into(),
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
pub struct GetStateRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub status: PaymentStatus,
    pub payment_id: String,
    pub order_id: String,
    pub amount: u32,
}

// ─── SendClosingReceipt ───────────────────────────────────────────────────────

/// Отправить закрывающий чек (для двухстадийных платежей, постоплаты и т.п.).
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SendClosingReceiptReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub payment_id: String,
    pub receipt: Receipt,
}

impl SendClosingReceiptReq {
    pub fn new(terminal_key: &TerminalKey, payment_id: impl Into<String>, receipt: Receipt) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
            payment_id: payment_id.into(),
            receipt,
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SendClosingReceiptRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    pub payment_id: String,
}

// ─── ResendNotification ───────────────────────────────────────────────────────

/// Повторная отправка всех неотправленных уведомлений.
///
/// Возвращает количество повторно отправленных уведомлений.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ResendNotificationReq {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
}

impl ResendNotificationReq {
    pub fn new(terminal_key: &TerminalKey) -> Self {
        Self {
            terminal_key: terminal_key.clone(),
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ResendNotificationRes {
    #[cfg_attr(feature = "utoipa", schema(value_type = String))]
    pub terminal_key: TerminalKey,
    /// Количество повторно отправленных уведомлений.
    pub count: u32,
}
