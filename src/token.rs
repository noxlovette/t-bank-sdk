use crate::InitPaymentReq;

impl InitPaymentReq {
    pub fn new_token(&self) {
        let mut fields: Vec<(String, String)> = vec![
            ("TerminalKey".into(), self.terminal_key.to_string()),
            ("Amount".into(), self.amount.to_string()),
            ("OrderId".into(), self.order_id.to_string()),
        ];

        if let Some(v) = &self.description {
            fields.push(("Description".into(), v.to_string()));
        }
        if let Some(v) = &self.customer_key {
            fields.push(("CustomerKey".into(), v.to_string()));
        }
        if let Some(v) = &self.recurrent {
            fields.push(("Recurrent".into(), v.to_string()));
        }
        if let Some(v) = &self.pay_type {
            fields.push(("PayType".into(), v.to_string()));
        }
        if let Some(v) = &self.language {
            fields.push(("Language".into(), v.to_string()));
        }
        if let Some(v) = &self.notification_url {
            fields.push(("NotificationUrl".into(), v.to_string()));
        }
        if let Some(v) = &self.success_url {
            fields.push(("SuccessUrl".into(), v.to_string()));
        }
        if let Some(v) = &self.fail_url {
            fields.push(("FailUrl".into(), v.to_string()));
        }
        if let Some(v) = &self.redirect_due_date {
            fields.push(("RedirectDueDate".into(), v.to_string()));
        }

        fields.sort_by(|a, b| a.0.cmp(&b.0));

        let data_check_string = fields
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");

        let secret_key = Sha256::digest(self.bot_token.as_bytes());
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret_key)
            .map_err(|_| NotificationError::InvalidPayload)?;
        mac.update(data_check_string.as_bytes());

        let hash_bytes = hex::decode(&self.hash).map_err(|_| NotificationError::InvalidPayload)?;

        mac.verify_slice(&hash_bytes)
            .map_err(|_| NotificationError::InvalidPayload)
    }
}
