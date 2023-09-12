use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::TypedHeader;
use axum::{routing::get, Router};

async fn bearer(TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>) -> String {
    bearer.token().to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(bearer));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
