use axum::{routing::get, Router};
use shaku::{Component, Interface};

    
pub trait HelloWorldRouter: Interface {
    fn create_router(&self) -> Router;
}

#[derive(Component)]
#[shaku(interface = HelloWorldRouter)]
pub struct HelloWorldRouterImpl {
}

impl HelloWorldRouter for HelloWorldRouterImpl {
    fn create_router(&self) -> Router {
        Router::new()
            .route("/hello", get(hello_world))
    }
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
