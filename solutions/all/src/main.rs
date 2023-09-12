use axum::{
    extract::{FromRef, Path, Query, State},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router, TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

async fn handler() -> &'static str {
    "Hello, World!"
}

async fn user(Path(user_id): Path<String>) -> String {
    user_id
}

async fn query(Query(params): Query<HashMap<String, String>>) -> Result<String, StatusCode> {
    Ok(params["t"].clone())
}

async fn json(Json(payload): Json<serde_json::Value>) -> Result<String, StatusCode> {
    Ok(serde_json::to_string(&payload).unwrap())
}

async fn bearer(TypedHeader(Authorization(token)): TypedHeader<Authorization<Bearer>>) -> String {
    token.token().into()
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
}

#[derive(Clone)]
pub struct AppState {
    pub secret: String,
}

impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.secret.clone()
    }
}

async fn my_middleware<B>(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let secret = String::from_ref(&state);
    let token_message = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    dbg!(&token_message);
    if let Err(_) = token_message {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let response = next.run(request).await;
    response
}

#[tokio::main]
async fn main() {
    let secret = env::var("SECRET").unwrap();
    let state = AppState { secret };

    let app = Router::new()
        .route("/", get(handler))
        .route("/one/:user_id", get(user))
        .route("/params", get(query))
        .route("/json", post(json))
        .route("/bearer", get(bearer))
        .layer(middleware::from_fn_with_state(state.clone(), my_middleware))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
