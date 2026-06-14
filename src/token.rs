use crate::{
    AddCardReq, AddCustomerReq, CancelPaymentReq, ChargePaymentReq, ConfirmPaymentReq,
    GetCardListReq, GetCustomerReq, GetStateReq, InitPaymentReq, Password, RemoveCardReq,
    RemoveCustomerReq, ResendNotificationReq, SendClosingReceiptReq,
};
#[cfg(feature = "serde")]
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Подпись запроса. [Как сформировать.](https://developer.tbank.ru/eacq/intro/developer/token)
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Token(String);

impl Token {
    fn from_string(s: String) -> Self {
        Self(s)
    }
}

/// This wrapper will generate a token for given payload
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct TokenWrapper<P>
where
    P: DeriveToken,
{
    token: Token,
    #[cfg_attr(feature = "serde", serde(flatten))]
    payload: P,
}

impl<P> TokenWrapper<P>
where
    P: DeriveToken,
{
    pub fn from_payload(payload: P, password: &Password) -> Self {
        let token = payload.create_token(password);
        Self { payload, token }
    }
}

/// Creates a request token according to T-Bank's signing rules.
pub trait DeriveToken {
    /// Builds a SHA-256 token from root-level request fields and the provided password.
    fn create_token(&self, password: &Password) -> Token;
}

// ─── helpers ─────────────────────────────────────────────────────────────────

fn build_token(fields: BTreeMap<String, String>) -> Token {
    let joined = fields.into_values().collect::<String>();
    let hash = Sha256::digest(joined.as_bytes());
    Token::from_string(format!("{hash:x}"))
}

#[cfg(feature = "serde")]
fn insert<T: Serialize>(map: &mut BTreeMap<String, String>, key: &str, value: &T) {
    map.insert(key.to_string(), serialize_token_value(value));
}

#[cfg(feature = "serde")]
fn insert_opt<T: Serialize>(map: &mut BTreeMap<String, String>, key: &str, value: &Option<T>) {
    if let Some(v) = value {
        map.insert(key.to_string(), serialize_token_value(v));
    }
}

#[cfg(feature = "serde")]
pub fn serialize_token_value<T>(value: &T) -> String
where
    T: Serialize,
{
    match serde_json::to_value(value).expect("token fields must serialize") {
        serde_json::Value::String(value) => value,
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            unreachable!("token generation only supports scalar root fields")
        }
    }
}

// ─── Init ─────────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for InitPaymentReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "Amount", &self.amount);
        insert(&mut fields, "OrderId", &self.order_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);

        insert_opt(&mut fields, "Description", &self.description);
        insert_opt(&mut fields, "CustomerKey", &self.customer_key);
        insert_opt(&mut fields, "Recurrent", &self.recurrent);
        insert_opt(&mut fields, "PayType", &self.pay_type);
        insert_opt(&mut fields, "Language", &self.language);
        insert_opt(&mut fields, "NotificationUrl", &self.notification_url);
        insert_opt(&mut fields, "SuccessUrl", &self.success_url);
        insert_opt(&mut fields, "FailUrl", &self.fail_url);
        insert_opt(&mut fields, "RedirectDueDate", &self.redirect_due_date);

        build_token(fields)
    }
}

// ─── Confirm ─────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for ConfirmPaymentReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "PaymentId", &self.payment_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        insert_opt(&mut fields, "Amount", &self.amount);
        build_token(fields)
    }
}

// ─── Cancel ──────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for CancelPaymentReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "PaymentId", &self.payment_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        insert_opt(&mut fields, "Amount", &self.amount);
        insert_opt(&mut fields, "ExternalRequestId", &self.external_request_id);
        build_token(fields)
    }
}

// ─── Charge ──────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for ChargePaymentReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "PaymentId", &self.payment_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "RebillId", &self.rebill_id);
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── GetState ────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for GetStateReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "PaymentId", &self.payment_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── SendClosingReceipt ───────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for SendClosingReceiptReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "PaymentId", &self.payment_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── ResendNotification ───────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for ResendNotificationReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── AddCustomer ─────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for AddCustomerReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "CustomerKey", &self.customer_key);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        insert_opt(&mut fields, "Email", &self.email);
        insert_opt(&mut fields, "Phone", &self.phone);
        build_token(fields)
    }
}

// ─── GetCustomer ─────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for GetCustomerReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "CustomerKey", &self.customer_key);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── RemoveCustomer ───────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for RemoveCustomerReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "CustomerKey", &self.customer_key);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── AddCard ─────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for AddCardReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "CustomerKey", &self.customer_key);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        insert_opt(&mut fields, "CheckType", &self.check_type);
        build_token(fields)
    }
}

// ─── GetCardList ──────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for GetCardListReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "CustomerKey", &self.customer_key);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}

// ─── RemoveCard ───────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for RemoveCardReq {
    fn create_token(&self, password: &Password) -> Token {
        let mut fields = BTreeMap::new();
        insert(&mut fields, "CardId", &self.card_id);
        insert(&mut fields, "CustomerKey", &self.customer_key);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "TerminalKey", &self.terminal_key);
        build_token(fields)
    }
}
