use axum::extract::Path;
use axum::{routing::get, Router};

async fn user(Path(user_id): Path<String>) -> String {
    user_id
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:user_id", get(user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
