use raft_core::channel::payload::{Request, Response};

#[derive(Debug, Clone)]
pub struct MyRequest {
    pub correlation_id: usize,
    pub content: String,
}

impl Request for MyRequest {}

#[derive(Debug, Clone)]
pub struct MyResponse {
    pub correlation_id: usize,
    pub content: String,
}

impl Response for MyResponse {}
