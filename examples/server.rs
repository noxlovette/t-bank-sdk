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
    let items = vec![ItemFFD105 {
        name: "Item1".to_string(),
        price: 1000,
        quantity: 1,
        amount: 1000,
        ..Default::default()
    }];
    let payload = InitPaymentReq::new(state.client.terminal_key(), 1000, "32451")
        .receipt(Receipt::FFD105 {
            items,
            ffd_version: None,
            email: Some("danila.volkov@noxlovette.com".to_string()),
            phone: None,
            taxation: t_bank_sdk::Taxation::UsnIncome,
            payments: None,
        })
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
