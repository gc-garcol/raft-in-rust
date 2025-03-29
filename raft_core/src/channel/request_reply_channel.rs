use std::fmt::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::timeout;

use super::message::Message;
use super::payload::{Request, Response};
use super::signal::SharedSignal;
    
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
        let signal: SharedSignal<RES> = SharedSignal::new();
        let message = Message::new(request, signal.clone());

        if let Err(error) = self.sender.send(message).await {
            println!("Failed to send message: {error}");
            return Err(Error::default());
        }

        match signal.wait(timeout).await {
            Ok(()) => {
                if let Some(response) = signal.response.lock().unwrap().take() {
                    println!("Producer: Message processed successfully with response: {:?}", response);
                    return Ok(response);
                } else {
                    println!("Producer: Message processed successfully but no response received!");
                    return Err(Error::default());
                }
            },
            Err(_) => {
                println!("Message processing timed out!");
                return Err(Error::default());
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
            match result {
                Ok(Some(message)) => {
                    let response = strategy(&message);
                    message.signal.complete(response);
                }
                Ok(None) => {
                    println!("Consumer: Channel closed");
                    break;
                }
                Err(_elapsed) => {
                    continue;
                }
            }
        }
    }
}