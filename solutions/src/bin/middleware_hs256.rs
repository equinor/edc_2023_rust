use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router, TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

async fn my_middleware<B>(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let token_message = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    if let Err(_) = token_message {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware::from_fn(my_middleware));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
