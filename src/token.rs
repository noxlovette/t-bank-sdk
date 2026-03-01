use crate::{InitPaymentReq, Token};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Creates a request token according to T-Bank's signing rules.
pub trait CreateToken {
    /// Builds a SHA-256 token from root-level request fields and the provided password.
    fn create_token(&self, password: &str) -> Token;
}

impl CreateToken for InitPaymentReq {
    fn create_token(&self, password: &str) -> Token {
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

        Token::new(format!("{hash:x}"))
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

#[cfg(test)]
mod tests {
    use super::CreateToken;
    use crate::InitPaymentReq;

    #[test]
    fn create_token_uses_only_root_level_fields() {
        let req = InitPaymentReq::new("MerchantTerminalKey", 19200, "00000", "")
            .unwrap()
            .description("Подарочная карта на 1000 рублей")
            .unwrap();

        assert_eq!(
            req.create_token("11111111111111").to_string(),
            "72dd466f8ace0a37a1f740ce5fb78101712bc0665d91a8108c7c8a0ccd426db2"
        );
    }

    #[test]
    fn create_token_ignores_existing_token_value() {
        let req = InitPaymentReq::new("terminal", 100, "order-1", "already-present").unwrap();
        let same_req_with_other_token =
            InitPaymentReq::new("terminal", 100, "order-1", "different-token").unwrap();

        assert_eq!(
            req.create_token("password").to_string(),
            same_req_with_other_token
                .create_token("password")
                .to_string()
        );
    }
}
