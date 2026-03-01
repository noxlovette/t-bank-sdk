use axum::{Json, Router, extract::State, routing::post};
use t_bank_sdk::{Client, CreateToken, Error, InitPaymentReq, InitPaymentRes};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/init", post(test_payment));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

#[axum::debug_handler]
async fn test_payment(State(state): State<AppState>) -> Result<Json<InitPaymentRes>, Error> {
    let req = InitPaymentReq::new(&state.client.terminal_key(), 1000, "order-1", "token")
        .unwrap()
        .create_token(&state.client.password());

    let res = state.client.initiate_payment(req).await?;

    Ok(Json(res))
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::Api(ref err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::Config(ref err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            Self::Server(ref err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        (status, message).into_response()
    }
}
