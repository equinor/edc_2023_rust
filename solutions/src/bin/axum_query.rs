use std::collections::HashMap;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::{routing::get, Router};

async fn query(Query(params): Query<HashMap<String, String>>) -> Result<String, StatusCode> {
    match params.get("key") {
        Some(v) => Ok(v.to_string()),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(query));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
