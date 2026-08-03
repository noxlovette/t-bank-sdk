//! Verifies the TLS handshake against the real T-Bank production host succeeds
//! now that the Russian Trusted Root/Sub CA anchors are embedded. This talks
//! to the network on purpose — a mocked transport can't prove a cert chain
//! validates against a real, non-Mozilla-trusted CA.

use t_bank_sdk::Client;

#[tokio::test]
async fn external_client_completes_tls_handshake_against_production() {
    let client = Client::external()
        .await
        .expect("client construction (including trust anchor loading) must succeed");

    // No real terminal/payment id — we only care that the TLS handshake
    // completes and we get back a T-Bank API response (any shape, even an
    // error body) instead of a `reqwest::Error` wrapping a TLS failure.
    let result = client
        .get_payment_state_with_credentials(
            t_bank_sdk::GetStateReq::new(
                &t_bank_sdk::TerminalKey::new("nonexistent").unwrap(),
                "0",
            ),
            &t_bank_sdk::TerminalKey::new("nonexistent").unwrap(),
            &t_bank_sdk::Password::new("nonexistent").unwrap(),
        )
        .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            let msg = e.to_string();
            assert!(
                !msg.to_lowercase().contains("certificate")
                    && !msg.to_lowercase().contains("invaliddata")
                    && !msg.to_lowercase().contains("not trusted"),
                "expected an API-level error, got what looks like a TLS trust failure: {msg}"
            );
        }
    }
}
