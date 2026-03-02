use axum::{Json, Router, extract::State, routing::post};
use t_bank_sdk::{Client, InitPaymentReq, InitPaymentRes};

#[tokio::main]
async fn main() {
    let state = AppState::new().await;
    let app = Router::<AppState>::new()
        .route("/init", post(test_payment))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

impl AppState {
    async fn new() -> Self {
        Self {
            client: Client::new().await.unwrap(),
        }
    }
}

#[axum::debug_handler]
async fn test_payment(State(state): State<AppState>) -> Json<InitPaymentRes> {
    let payload = InitPaymentReq::new(state.client.terminal_key(), 1000, "order-1");

    let res = state
        .client
        .initiate_payment(payload)
        .await
        .expect("unable to initiate payment");

    Json(res)
}
