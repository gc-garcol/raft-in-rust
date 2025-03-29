use super::{payload::Response, signal::SharedSignal};
#[derive(Debug, Clone)]
pub struct Message<REQ, RES: Response> {
    pub request: REQ,
    pub signal: SharedSignal<RES>,
}

impl<REQ, RES: Response> Message<REQ, RES> {
    pub fn new(request: REQ, signal: SharedSignal<RES>) -> Self {
        Self {request, signal }
    }
}
