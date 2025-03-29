use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use raft_core::channel::request_reply_channel::{Consumer, RequestReplyChannel};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
struct BenchRequest {
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct BenchResponse {
    data: Vec<u8>,
}

impl raft_core::channel::payload::Request for BenchRequest {}
impl raft_core::channel::payload::Response for BenchResponse {}

fn pubsub_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("pubsub");
    group.sample_size(50);  // Increased sample size for better statistics
    group.measurement_time(Duration::from_secs(30));  // Longer measurement time
    group.warm_up_time(Duration::from_secs(5));  // Longer warm-up
    group.confidence_level(0.99);  // 99% confidence level

    // Benchmark different message sizes
    for size in [32, 128, 512, 1024, 4096] {
        let id = BenchmarkId::new("message_size", size);
        group.throughput(Throughput::Bytes(size as u64));
        
        group.bench_with_input(id, &size, |b, &size| {
            b.iter(|| {
                rt.block_on(async {
                    let (channel, receiver) = RequestReplyChannel::<BenchRequest, BenchResponse>::new(1024);
                    let producer = channel.new_producer();
                    let mut consumer = Consumer::<BenchRequest, BenchResponse>::new(receiver);
                    let running_signal = Arc::new(AtomicBool::new(true));

                    // Consumer task
                    let running_signal_consumer = running_signal.clone();
                    let consumer_task = tokio::spawn(async move {
                        consumer.consume(
                            |message| BenchResponse {
                                data: message.request.data.clone(),
                            },
                            running_signal_consumer,
                        )
                        .await;
                    });

                    // Producer task
                    let request = BenchRequest {
                        data: vec![0; size],
                    };
                    let response = producer
                        .send(request, Duration::from_secs(1))
                        .await
                        .unwrap();
                    
                    // Verify response data
                    assert_eq!(response.data.len(), size);

                    running_signal.store(false, Ordering::Release);
                    let _ = consumer_task.await;
                });
            });
        });
    }

    group.finish();
}

criterion_group!(benches, pubsub_benchmark);
criterion_main!(benches); 