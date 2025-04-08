use raft_core::channel::payload::{Request, Response};

#[derive(Debug, Clone)]
pub struct CreateBalance {
    pub uuid_most_significant: u64,
    pub uuid_least_significant: u64,

    pub user_id: u64, // todo remove after implementing auth
}

impl Request for CreateBalance {}

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateBalanceResponse {
    pub uuid_most_significant: u64,
    pub uuid_least_significant: u64,
    pub balance_id: u64,
}

impl Response for CreateBalanceResponse {}
