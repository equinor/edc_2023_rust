use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

async fn handler() -> &'static str {
    "Hello, World!"
}

async fn my_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    dbg!(request.headers().keys());
    next.run(request).await
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(my_middleware));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
