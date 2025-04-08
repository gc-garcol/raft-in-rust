use axum::{routing::post, response::IntoResponse, Router, Json};

use super::balance_payload::CreateBalanceResponse;

pub fn create_router() -> Router {
    Router::new()
        .route("/balance", post(create_balance))
}

async fn create_balance() -> impl IntoResponse {
    Json(CreateBalanceResponse {
        uuid_most_significant: 0,
        uuid_least_significant: 0,
        balance_id: 0,
    })
}