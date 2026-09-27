use crate::Password;
#[cfg(feature = "serde")]
use crate::{
    AddCardReq, AddCustomerReq, CancelPaymentReq, ChargePaymentReq, ConfirmPaymentReq,
    ErrorWrapper, GetCardListReq, GetCustomerReq, GetStateReq, InitPaymentReq,
    PaymentNotificationRes, RemoveCardReq, RemoveCustomerReq, ResendNotificationReq,
    SendClosingReceiptReq,
};
#[cfg(feature = "serde")]
use serde::Serialize;
#[cfg(feature = "serde")]
use sha2::{Digest, Sha256};
#[cfg(feature = "serde")]
use std::collections::BTreeMap;
use std::ops::Deref;

/// Подпись запроса. [Как сформировать.](https://developer.tbank.ru/eacq/intro/developer/token)
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Token(String);

#[cfg(feature = "serde")]
pub struct TokenBuilder(BTreeMap<String, String>);

#[cfg(feature = "serde")]
impl TokenBuilder {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// inserts an entry into the token builder
    fn insert<T: Serialize>(&mut self, key: &str, value: &T) {
        self.0.insert(key.to_string(), serialize_token_value(value));
    }

    /// inserts value if present
    fn insert_opt<T: Serialize>(&mut self, key: &str, value: &Option<T>) {
        if let Some(v) = value {
            self.0.insert(key.to_string(), serialize_token_value(v));
        }
    }
}

#[cfg(feature = "serde")]
impl From<TokenBuilder> for Token {
    fn from(value: TokenBuilder) -> Self {
        let joined = value.0.into_values().collect::<String>();
        hex::encode(Sha256::digest(joined.as_bytes())).into()
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl Token {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for Token {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// This wrapper will generate a token for given payload
///
/// It stores the payload + the token that that payload will generate
#[cfg(feature = "serde")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TokenWrapper<P>
where
    P: DeriveToken,
{
    token: Token,
    #[serde(flatten)]
    payload: P,
}

#[cfg(feature = "serde")]
impl<P> TokenWrapper<P>
where
    P: DeriveToken,
{
    pub fn from_payload(payload: P, password: &Password) -> Self {
        let token = payload.derive_token(password);
        Self { payload, token }
    }
}

/// Creates a request token according to T-Bank's signing rules.
pub trait DeriveToken {
    /// Builds a SHA-256 token from root-level request fields and the provided password.
    fn derive_token(&self, password: &Password) -> Token;
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
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("Amount", &self.amount);
        builder.insert("OrderId", &self.order_id);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.insert_opt("Description", &self.description);
        builder.insert_opt("CustomerKey", &self.customer_key);
        builder.insert_opt("Recurrent", &self.recurrent);
        builder.insert_opt("PayType", &self.pay_type);
        builder.insert_opt("Language", &self.language);
        builder.insert_opt("NotificationUrl", &self.notification_url);
        builder.insert_opt("SuccessUrl", &self.success_url);
        builder.insert_opt("FailUrl", &self.fail_url);
        builder.insert_opt("RedirectDueDate", &self.redirect_due_date);

        builder.into()
    }
}

// ─── Confirm ─────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for ConfirmPaymentReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("PaymentId", &self.payment_id);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.insert_opt("Amount", &self.amount);

        builder.into()
    }
}

// ─── Cancel ──────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for CancelPaymentReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("PaymentId", &self.payment_id);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.insert_opt("Amount", &self.amount);
        builder.insert_opt("ExternalRequestId", &self.external_request_id);

        builder.into()
    }
}

// ─── Charge ──────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for ChargePaymentReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("PaymentId", &self.payment_id);
        builder.insert("Password", password);
        builder.insert("RebillId", &self.rebill_id);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.into()
    }
}

// ─── Notification (incoming webhook) ──────────────────────────────────────────

#[cfg(feature = "serde")]
impl ErrorWrapper<PaymentNotificationRes> {
    /// Verifies a payment notification's `Token` field against `password`,
    /// in (best-effort) constant time. `false` — not an error — for a
    /// notification with no `Token` at all, or no payload.
    ///
    /// Notifications sign a different (and, unlike every other endpoint
    /// here, a *superset*) field list from any outbound request: on top of
    /// `TerminalKey`/`PaymentId`/`Amount`, they also cover the
    /// [`ErrorWrapper`]-level `Success`/`ErrorCode`, plus `OrderId`/
    /// `Status` and whichever of `RebillId`/`CardId`/`Pan`/`ExpDate`
    /// T-Bank included. [Docs.](https://developer.tbank.ru/eacq/intro/webhook)
    ///
    /// Verified against a real T-Bank payload + token in this module's
    /// tests (`token::test::verify_token_matches_a_real_tbank_notification`),
    /// not just a self-consistency round-trip.
    pub fn verify_token(&self, password: &Password) -> bool {
        let Some(received) = self.inner().and_then(|i| i.token.as_deref()) else {
            return false;
        };
        let Some(computed) = self.compute_token(password) else {
            return false;
        };

        constant_time_eq::constant_time_eq(
            computed.as_str().as_bytes(),
            received.to_lowercase().as_bytes(),
        )
    }

    /// The token T-Bank should have sent for this notification, given
    /// `password` — the same computation [`Self::verify_token`] checks
    /// against. `None` if there's no payload to compute one from. Exposed
    /// so a caller building a fixture notification (e.g. a test simulating
    /// a webhook delivery) doesn't have to duplicate this field list.
    pub fn compute_token(&self, password: &Password) -> Option<Token> {
        let inner = self.inner()?;

        let mut builder = TokenBuilder::new();
        builder.insert("Amount", &inner.amount);
        builder.insert("ErrorCode", &self.error_code());
        builder.insert("OrderId", &inner.order_id);
        builder.insert("Password", password);
        builder.insert("PaymentId", &inner.payment_id);
        builder.insert("Status", &inner.status);
        builder.insert("Success", &self.success());
        builder.insert("TerminalKey", &inner.terminal_key);
        builder.insert_opt("RebillId", &inner.rebill_id);
        builder.insert_opt("CardId", &inner.card_id);
        builder.insert_opt("Pan", &inner.pan);
        builder.insert_opt("ExpDate", &inner.exp_date);

        Some(builder.into())
    }
}

// ─── GetState ────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for GetStateReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("PaymentId", &self.payment_id);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);

        builder.into()
    }
}

// ─── SendClosingReceipt ───────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for SendClosingReceiptReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("PaymentId", &self.payment_id);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);

        builder.into()
    }
}

// ─── ResendNotification ───────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for ResendNotificationReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);

        builder.into()
    }
}

// ─── AddCustomer ─────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for AddCustomerReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("CustomerKey", &self.customer_key);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.insert_opt("Email", &self.email);
        builder.insert_opt("Phone", &self.phone);
        builder.into()
    }
}

// ─── GetCustomer ─────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for GetCustomerReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("CustomerKey", &self.customer_key);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.into()
    }
}

// ─── RemoveCustomer ───────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for RemoveCustomerReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("CustomerKey", &self.customer_key);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.into()
    }
}

// ─── AddCard ─────────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for AddCardReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("CustomerKey", &self.customer_key);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.insert_opt("CheckType", &self.check_type);
        builder.into()
    }
}

// ─── GetCardList ──────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for GetCardListReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("CustomerKey", &self.customer_key);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.into()
    }
}

// ─── RemoveCard ───────────────────────────────────────────────────────────────

#[cfg(feature = "serde")]
impl DeriveToken for RemoveCardReq {
    fn derive_token(&self, password: &Password) -> Token {
        let mut builder = TokenBuilder::new();
        builder.insert("CardId", &self.card_id);
        builder.insert("CustomerKey", &self.customer_key);
        builder.insert("Password", password);
        builder.insert("TerminalKey", &self.terminal_key);
        builder.into()
    }
}

#[cfg(all(test, feature = "serde"))]
mod test {
    use crate::{ErrorWrapper, Password, PaymentNotificationRes};

    /// Real payload + real `Token`, straight from
    /// `payment.rs`'s `payment_notification_deserializes_real_webhook_payload`
    /// — this is T-Bank's own computed signature for this exact body, not a
    /// value this test derived itself, so a pass here proves the algorithm
    /// matches T-Bank's, not just that `verify_token` agrees with itself.
    /// Password is the same public demo-terminal credential used elsewhere
    /// against this terminal (`1772186527637DEMO`) — not a real secret.
    #[test]
    fn verify_token_matches_a_real_tbank_notification() {
        let json = r#"{"TerminalKey":"1772186527637DEMO","OrderId":"lde1q7xbsoisn1vjj2k6s","Success":true,"Status":"AUTHORIZED","PaymentId":8111987112,"ErrorCode":"0","Amount":1000,"CardId":659561464,"Pan":"430000******0777","ExpDate":"1230","Token":"17bf87f32797d961db48f4500cc9110be5cc2f480e4e555e2440eec09108a760"}"#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();
        let password = Password::new("rpnstdt1ekalr00f").unwrap();

        assert!(wrapper.verify_token(&password));
    }

    #[test]
    fn verify_token_rejects_wrong_password() {
        let json = r#"{"TerminalKey":"1772186527637DEMO","OrderId":"lde1q7xbsoisn1vjj2k6s","Success":true,"Status":"AUTHORIZED","PaymentId":8111987112,"ErrorCode":"0","Amount":1000,"CardId":659561464,"Pan":"430000******0777","ExpDate":"1230","Token":"17bf87f32797d961db48f4500cc9110be5cc2f480e4e555e2440eec09108a760"}"#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();
        let password = Password::new("definitely-the-wrong-password").unwrap();

        assert!(!wrapper.verify_token(&password));
    }

    #[test]
    fn verify_token_rejects_a_tampered_field() {
        // Same payload as the real one above, `Amount` changed from 1000 to
        // 2000 — the `Token` no longer covers what's actually being
        // reported, exactly what this guards against.
        let json = r#"{"TerminalKey":"1772186527637DEMO","OrderId":"lde1q7xbsoisn1vjj2k6s","Success":true,"Status":"AUTHORIZED","PaymentId":8111987112,"ErrorCode":"0","Amount":2000,"CardId":659561464,"Pan":"430000******0777","ExpDate":"1230","Token":"17bf87f32797d961db48f4500cc9110be5cc2f480e4e555e2440eec09108a760"}"#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();
        let password = Password::new("rpnstdt1ekalr00f").unwrap();

        assert!(!wrapper.verify_token(&password));
    }

    #[test]
    fn verify_token_rejects_missing_token() {
        let json = r#"{"TerminalKey":"1772186527637DEMO","OrderId":"lde1q7xbsoisn1vjj2k6s","Success":false,"Status":"REJECTED","PaymentId":8111987112,"ErrorCode":"1051","Amount":1000}"#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();
        let password = Password::new("rpnstdt1ekalr00f").unwrap();

        assert!(!wrapper.verify_token(&password));
    }

    #[test]
    fn verify_token_reads_declined_payment_without_unwrap() {
        // OGO-292 in Ogonek's own history: `.unwrap()` treats any non-"0"
        // `ErrorCode` as an API failure and discards the payload — wrong
        // for a legitimate decline notification, which still needs its
        // `Status`/`PaymentId`/etc. processed as ordinary business data.
        // `inner()` must expose the payload regardless.
        let json = r#"{"TerminalKey":"1772186527637DEMO","OrderId":"lde1q7xbsoisn1vjj2k6s","Success":false,"Status":"REJECTED","PaymentId":8111987112,"ErrorCode":"1051","Amount":1000}"#;

        let wrapper: ErrorWrapper<PaymentNotificationRes> = serde_json::from_str(json).unwrap();

        assert!(!wrapper.success());
        assert_eq!(wrapper.error_code(), "1051");
        assert_eq!(wrapper.inner().unwrap().payment_id, 8111987112);
    }

    /// `compute_token` is what a fixture-building caller (e.g. a test
    /// simulating a webhook delivery, in this crate or downstream) uses to
    /// sign a payload it constructs itself — must round-trip through
    /// `verify_token`.
    #[test]
    fn compute_token_round_trips_through_verify_token() {
        let json = r#"{"TerminalKey":"TBankTest","OrderId":"order-1","Success":true,"Status":"CONFIRMED","PaymentId":42,"ErrorCode":"0","Amount":1000}"#;
        let mut value: serde_json::Value = serde_json::from_str(json).unwrap();
        let password = Password::new("fixture-password").unwrap();

        let without_token: ErrorWrapper<PaymentNotificationRes> =
            serde_json::from_value(value.clone()).unwrap();
        let token = without_token.compute_token(&password).unwrap();
        value["Token"] = serde_json::Value::String(token.to_string());

        let signed: ErrorWrapper<PaymentNotificationRes> = serde_json::from_value(value).unwrap();
        assert!(signed.verify_token(&password));
    }
}
