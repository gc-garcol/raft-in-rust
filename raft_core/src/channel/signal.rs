use std::sync::{
    atomic::{AtomicBool, Ordering}, Arc, Mutex
};
use tokio::{sync::Notify, time::error::Elapsed};
use tokio::time::Duration;

use super::payload::Response;

#[derive(Debug, Clone)]
pub struct SharedSignal<R: Response> {
    processed: Arc<AtomicBool>,
    notify: Arc<Notify>,
    pub response: Arc<Mutex<Option<R>>>,
}

impl<R: Response> SharedSignal<R> {
    pub fn new() -> Self {
        Self {
            processed: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
            response: Arc::new(Mutex::new(None)),
        }
    }

    /// Producer waits until the message is marked as processed or times out after the specified duration
    pub async fn wait(&self, timeout: Duration) -> Result<(), Elapsed> {
        tokio::time::timeout(timeout, async {
            while !self.processed.load(Ordering::Acquire) {
                self.notify.notified().await;
            }
        })
        .await
    }

    /// Consumer marks the message as processed and wakes up the producer
    pub fn complete(&self, response: R) {
        *self.response.lock().unwrap() = Some(response);
        self.processed.store(true, Ordering::Release);
        self.notify.notify_waiters();
    }
}
