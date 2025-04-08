use axum::{routing::post, response::IntoResponse, Router, Json};
use shaku::{Component, Interface};

use super::balance_payload::CreateBalanceResponse;

pub trait BalanceRouter: Interface {
    fn create_router(&self) -> Router;
}

#[derive(Component)]
#[shaku(interface = BalanceRouter)]
pub struct BalanceRouterImpl {
}

impl BalanceRouter for BalanceRouterImpl {
    fn create_router(&self) -> Router {
        Router::new()
            .route("/balance", post(create_balance))
    }
}

async fn create_balance() -> impl IntoResponse {
    Json(CreateBalanceResponse {
        uuid_most_significant: 0,
        uuid_least_significant: 0,
        balance_id: 0,
    })
}