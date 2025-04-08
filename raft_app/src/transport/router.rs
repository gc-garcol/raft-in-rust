use async_trait::async_trait;
use axum::{Router, http::HeaderValue};
use hyper::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::CorsLayer;

use super::app_router::helloworld_router;
use shaku::{Component, Interface};

#[async_trait]
pub trait AppRouter: Interface {
    async fn start_router(&self);
}

#[derive(Component)]
#[shaku(interface = AppRouter)]
pub struct AppRouterImpl {
    domain: String,
}

impl AppRouterImpl {
    fn create_router(&self) -> Router {
        let cors = CorsLayer::new()
            .allow_origin("*".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let router = Router::new()
            .layer(cors)
            .merge(helloworld_router::create_router());

        router
    }
}

#[async_trait]
impl AppRouter for AppRouterImpl {
    async fn start_router(&self) {
        let router = self.create_router();
        let listener = tokio::net::TcpListener::bind(&self.domain).await.unwrap();
        log::info!("Starting router on {}", &self.domain);
        axum::serve(listener, router).await.unwrap();
    }
}
