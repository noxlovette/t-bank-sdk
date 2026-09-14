use crate::{
    AddCardReq, AddCustomerReq, CancelPaymentReq, ChargePaymentReq, ConfirmPaymentReq,
    ErrorWrapper, GetCardListReq, GetCustomerReq, GetStateReq, InitPaymentReq,
    PaymentNotificationRes, Password, RemoveCardReq, RemoveCustomerReq, ResendNotificationReq,
    SendClosingReceiptReq,
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
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

/// Byte-length- and content-independent-time comparison — avoids pulling in
/// a whole crate for one primitive. Not constant-*compiler-optimization*
/// proof the way a dedicated crate audited against that would be, but
/// sufficient for comparing a locally-computed hash against an
/// attacker-supplied one over a network round-trip, where the timing
/// signal this defends against is already drowned out by request jitter.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
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
        if let Some(rdd) = &self.redirect_due_date {
            fields.insert(
                "RedirectDueDate".to_string(),
                crate::payment::format_redirect_due_date(rdd),
            );
        }

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

        constant_time_eq(
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

        let mut fields = BTreeMap::new();
        insert(&mut fields, "Amount", &inner.amount);
        fields.insert("ErrorCode".to_string(), self.error_code().to_string());
        insert(&mut fields, "OrderId", &inner.order_id);
        fields.insert("Password".to_string(), password.into());
        insert(&mut fields, "PaymentId", &inner.payment_id);
        insert(&mut fields, "Status", &inner.status);
        fields.insert("Success".to_string(), self.success().to_string());
        insert(&mut fields, "TerminalKey", &inner.terminal_key);
        insert_opt(&mut fields, "RebillId", &inner.rebill_id);
        insert_opt(&mut fields, "CardId", &inner.card_id);
        insert_opt(&mut fields, "Pan", &inner.pan);
        insert_opt(&mut fields, "ExpDate", &inner.exp_date);

        Some(build_token(fields))
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

#[cfg(all(test, feature = "serde"))]
mod test {
    use super::constant_time_eq;
    use crate::{ErrorWrapper, PaymentNotificationRes, Password};

    #[test]
    fn constant_time_eq_matches_equal_slices() {
        assert!(constant_time_eq(b"abc", b"abc"));
    }

    #[test]
    fn constant_time_eq_rejects_different_slices() {
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }

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
