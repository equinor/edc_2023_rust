use axum::{
    extract::{FromRef, State},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::Jwks;

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
