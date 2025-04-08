use tokio::sync::oneshot;
use super::payload::{Request, Response};

#[derive(Debug)]
pub struct Message<REQ: Request, RES: Response> {
    pub request: REQ,
    pub response_channel: oneshot::Sender<RES>,
}

impl<REQ: Request, RES: Response> Message<REQ, RES> {
    pub fn new(request: REQ, response: oneshot::Sender<RES>) -> Self {
        Self { request, response_channel: response }
    }
}
