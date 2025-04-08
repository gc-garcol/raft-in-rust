use std::fmt::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot};
use tokio::time::Duration;
use tokio::time::timeout;

use super::message::Message;
use super::payload::{Request, Response};
    
pub struct RequestReplyChannel<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    sender: mpsc::Sender<Message<REQ, RES>>,
}

impl<REQ, RES> RequestReplyChannel<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    pub fn new(size: usize) -> (Self, mpsc::Receiver<Message<REQ, RES>>) {
        let (sender, receiver) = mpsc::channel(size);
        (Self { sender }, receiver)
    }
}

impl<REQ, RES> RequestReplyChannel<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    pub fn new_producer(&self) -> Producer<REQ, RES> {
        Producer::new(self.sender.clone())
    }
}

pub struct Producer<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    sender: mpsc::Sender<Message<REQ, RES>>,
}

impl<REQ, RES> Producer<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    pub fn new(sender: mpsc::Sender<Message<REQ, RES>>) -> Self {
        Self {
            sender,
        }
    }

    pub async fn send(&self, request: REQ, timeout: Duration) -> Result<RES, Error> {
        let (tx, rx) = oneshot::channel();
        let message = Message::new(request, tx);

        if let Err(error) = self.sender.send(message).await {
            log::error!("Failed to send message: {error}");
            return Err(Error::default());
        }

        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(response)) => {
                log::debug!("Producer: Message processed successfully with response: {:?}", response);
                Ok(response)
            },
            Ok(Err(_)) => {
                log::error!("Producer: Message processed successfully but no response received!");
                Err(Error::default())
            },
            Err(_) => {
                log::error!("Message processing timed out!");
                Err(Error::default())
            }
        }
    }
}

pub struct Consumer<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    receiver: mpsc::Receiver<Message<REQ, RES>>,
}

impl<REQ, RES> Consumer<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    pub fn new(receiver: mpsc::Receiver<Message<REQ, RES>>) -> Self {
        Self {
            receiver,
        }
    }
}

impl<REQ, RES> Consumer<REQ, RES>
where
    REQ: Request,
    RES: Response,
{
    pub async fn consume(&mut self, strategy: impl Fn(&Message<REQ, RES>) -> RES, running_signal: Arc<AtomicBool>) {
        while running_signal.load(Ordering::Relaxed) {
            let result = timeout(Duration::from_millis(500), self.receiver.recv()).await;
            log::debug!("Consumer: Received message: {:?}", result);
            match result {
                Ok(Some(message)) => {
                    let response = strategy(&message);
                    if let Err(e) = message.response_channel.send(response) {
                        log::error!("Failed to send response: {:?}", e);
                    }
                }
                Ok(None) => {
                    log::info!("Consumer: Channel closed");
                    break;
                }
                Err(_elapsed) => {
                    continue;
                }
            }
        }
    }
}
