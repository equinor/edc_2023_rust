use axum::{
    extract::{FromRef, State},
    routing::get,
    Router,
};

#[derive(Clone)]
pub struct AppState {
    pub secret: String,
}

impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.secret.clone()
    }
}

async fn handler(State(state): State<AppState>) -> String {
    String::from_ref(&state)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        secret: "secret".to_string(),
    };

    let app = Router::new().route("/", get(handler)).with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
