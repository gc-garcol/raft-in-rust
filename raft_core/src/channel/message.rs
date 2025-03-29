use tokio::sync::oneshot;
use super::payload::Response;

#[derive(Debug)]
pub struct Message<REQ, RES: Response> {
    pub request: REQ,
    pub response: oneshot::Sender<RES>,
}

impl<REQ, RES: Response> Message<REQ, RES> {
    pub fn new(request: REQ, response: oneshot::Sender<RES>) -> Self {
        Self { request, response }
    }
}
