use axum::{Json, Router, extract::State, routing::post};
use serde_json::json;
use t_bank_sdk::{Client, ErrorWrapper, InitPaymentReq, InitPaymentRes, ItemFFD105, Receipt};
use url::form_urlencoded::byte_serialize;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::init();
    let state = AppState::new().await;
    let app = Router::<AppState>::new()
        .route("/init", post(test_payment))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server listening at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

fn urlencode(value: &str) -> String {
    byte_serialize(value.as_bytes()).collect()
}

impl AppState {
    async fn new() -> Self {
        Self {
            client: Client::new().await.unwrap(),
        }
    }
}

#[axum::debug_handler]
async fn test_payment(State(state): State<AppState>) -> Json<ErrorWrapper<InitPaymentRes>> {
    let payload = InitPaymentReq::new(state.client.terminal_key(), 1000, "32453")
        .receipt(
            Receipt::ffd105(
                vec![ItemFFD105::new("Item1", 1000, 1, 1000)],
                t_bank_sdk::Taxation::UsnIncome,
            )
            .email("danila.o.volkov@noxlovette.com"),
        )
        .data(json!({
            "Phone": urlencode("+71234567890"),
            "Email": urlencode("a@test.com"),
        }));

    let res = state
        .client
        .initiate_payment(payload)
        .await
        .expect("unable to initiate payment");

    Json(res)
}
