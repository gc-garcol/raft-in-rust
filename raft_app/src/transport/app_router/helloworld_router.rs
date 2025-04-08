use axum::{routing::get, Router};

pub fn create_router() -> Router {
    log::info!("Creating hello world router");
    Router::new()
        .route("/hello", get(hello_world))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
