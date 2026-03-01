use crate::InitPaymentReq;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Creates a request token according to T-Bank's signing rules.
pub trait CreateToken {
    /// Builds a SHA-256 token from root-level request fields and the provided password.
    fn create_token(self, password: &str) -> Self;
}

impl CreateToken for InitPaymentReq {
    fn create_token(mut self, password: &str) -> Self {
        let mut fields = BTreeMap::from([
            (String::from("Amount"), serialize_token_value(&self.amount)),
            (
                String::from("OrderId"),
                serialize_token_value(&self.order_id),
            ),
            (String::from("Password"), String::from(password)),
            (
                String::from("TerminalKey"),
                serialize_token_value(&self.terminal_key),
            ),
        ]);

        if let Some(value) = &self.description {
            fields.insert(String::from("Description"), serialize_token_value(value));
        }
        if let Some(value) = &self.customer_key {
            fields.insert(String::from("CustomerKey"), serialize_token_value(value));
        }
        if let Some(value) = &self.recurrent {
            fields.insert(String::from("Recurrent"), serialize_token_value(value));
        }
        if let Some(value) = &self.pay_type {
            fields.insert(String::from("PayType"), serialize_token_value(value));
        }
        if let Some(value) = &self.language {
            fields.insert(String::from("Language"), serialize_token_value(value));
        }
        if let Some(value) = &self.notification_url {
            fields.insert(
                String::from("NotificationUrl"),
                serialize_token_value(value),
            );
        }
        if let Some(value) = &self.success_url {
            fields.insert(String::from("SuccessUrl"), serialize_token_value(value));
        }
        if let Some(value) = &self.fail_url {
            fields.insert(String::from("FailUrl"), serialize_token_value(value));
        }
        if let Some(value) = &self.redirect_due_date {
            fields.insert(
                String::from("RedirectDueDate"),
                serialize_token_value(value),
            );
        }

        let joined_values = fields.into_values().collect::<String>();
        let hash = Sha256::digest(joined_values.as_bytes());

        self.token = format!("{hash:x}");
        self
    }
}

fn serialize_token_value<T>(value: &T) -> String
where
    T: Serialize,
{
    match serde_json::to_value(value).expect("token fields must serialize") {
        Value::String(value) => value,
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Null => String::new(),
        Value::Array(_) | Value::Object(_) => {
            unreachable!("token generation only supports scalar root fields")
        }
    }
}
