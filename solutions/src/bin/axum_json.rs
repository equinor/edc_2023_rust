use axum::extract::Json;
use axum::http::StatusCode;
use axum::{routing::post, Router};

async fn json(Json(payload): Json<serde_json::Value>) -> Result<String, StatusCode> {
    match serde_json::to_string(&payload) {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(v) => Ok(v),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(json));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
