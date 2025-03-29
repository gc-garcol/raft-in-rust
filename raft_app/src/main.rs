pub mod domain;
pub mod infrastructure;
pub mod transport;

use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use raft_core::channel::{message::Message, request_reply_channel::{Consumer, RequestReplyChannel}};
use transport::helloworld::payload::{MyRequest, MyResponse};

#[tokio::main]
async fn main() {
    // Initialize the logger with detailed configuration
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
            .default_write_style_or("always")
    )
    .format_timestamp_millis()
    .format_level(true)
    .format_target(true)
    .init();
    
    log::info!("Starting Raft application...");
    // Create the channel and get the receiver
    let (channel, receiver) = RequestReplyChannel::new(32);

    // Create a producer
    let producer = channel.new_producer();

    // Create a consumer with the receiver
    let mut consumer = Consumer::new(receiver);
    let running_signal = Arc::new(AtomicBool::new(true));

    let strategy = |message: &Message<MyRequest, MyResponse>| {
        let response = MyResponse {
            correlation_id: message.request.correlation_id,
            content: "Hello, world!".to_string(),
        };
        response
    };

    let running_signal_producer = running_signal.clone();
    let running_signal_consumer = running_signal.clone();

    let producer_task = tokio::spawn(async move {
        let response = producer
            .send(
                MyRequest {
                    correlation_id: 1,
                    content: "Hello, world!".to_string(),
                },
                Duration::from_secs(1),
            )
            .await;
        log::info!("Producer response: {:?}", response);
        running_signal_producer.store(false, Ordering::Release);
    });

    let consumer_task = tokio::spawn(async move {
        consumer.consume(strategy, running_signal_consumer).await;
    });

    let _ = tokio::try_join!(producer_task);
    log::info!("Producer done!");

    let _ = tokio::try_join!(consumer_task);
    log::info!("Consumer done!");
}
