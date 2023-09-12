use std::env;

use axum::{
    extract::{FromRef, State},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Router, TypedHeader,
};

use axum_jwks::Jwks;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EmptyClaims {}

#[derive(Clone)]
pub struct AppState {
    pub jwks: Jwks,
}

impl FromRef<AppState> for Jwks {
    fn from_ref(state: &AppState) -> Self {
        state.jwks.clone()
    }
}

pub async fn my_middleware<B>(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let jwks = Jwks::from_ref(&state);

    if let Err(_) = jwks.validate_claims::<EmptyClaims>(bearer.token()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(request).await
}

#[tokio::main]
async fn main() {
    let authserver = env::var("AUTHSERVER").expect("AUTHSERVER");
    let audience = env::var("AUDIENCE").expect("AUDIENCE").to_owned();

    let jwks = Jwks::from_oidc_url(&authserver, audience).await.unwrap();
    let state = AppState { jwks };
    let app = Router::new()
        .route("/", get(|| async { "hello" }))
        .route_layer(middleware::from_fn_with_state(state.clone(), my_middleware))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
