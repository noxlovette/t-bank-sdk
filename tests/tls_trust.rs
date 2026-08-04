//! Verifies the TLS handshake against the real T-Bank production host succeeds
//! now that the Russian Trusted Root/Sub CA anchors are embedded. This talks
//! to the network on purpose — a mocked transport can't prove a cert chain
//! validates against a real, non-Mozilla-trusted CA.

use t_bank_sdk::{Client, Environment, Error, GetStateReq, Password, TerminalKey};

/// Pinned to `Environment::Production` deliberately — this must talk to
/// `securepay.tinkoff.ru` regardless of the ambient `TBANK_ENV`, since the
/// whole point is proving the Russian Trusted Root/Sub CA anchors validate
/// against production's real cert chain. Going through `Client::external()`
/// (which reads `TBANK_ENV`) previously let this test silently exercise the
/// test host instead whenever the var was unset.
#[tokio::test]
async fn external_client_completes_tls_handshake_against_production() {
    let client = Client::external_with_environment(Environment::Production)
        .await
        .expect("client construction (including trust anchor loading) must succeed");

    // No real terminal/payment id, so a well-formed, authenticated T-Bank
    // API error is the expected outcome here — that's proof the TLS
    // handshake completed and the request round-tripped as real JSON.
    // A `reqwest::Error` wrapping a TLS failure, or a JSON-decode error
    // (e.g. from an HTML error page on the wrong host), both mean the trust
    // anchors — or the host — are wrong.
    let result = client
        .get_payment_state_with_credentials(
            GetStateReq::new(&TerminalKey::new("nonexistent").unwrap(), "0"),
            &TerminalKey::new("nonexistent").unwrap(),
            &Password::new("nonexistent").unwrap(),
        )
        .await;

    match result {
        Ok(_) => {}
        Err(Error::Api { .. }) => {}
        Err(other) => panic!("expected a well-formed T-Bank API error, got: {other:?}"),
    }
}
