# Cafe raft

## Documentation

- [The Raft Consensus Algorithm ](https://raft.github.io/)
- [paper](https://raft.github.io/raft.pdf)

## Structure
- `raft_core`: lib
- `raft_app`: example

## Development

```bash
cp .env.example .env
```

```bash
SERVER_DOMAIN=127.0.0.1:8080 RUST_LOG=raft_app=debug,raft_core=info cargo run
```

### Benchmark

# Run all benchmarks
```bash
cargo bench -p raft_core --bench pubsub_benchmark
```

# Run specific message size (ex: [32, 128, 512, 1024, 4096])
```bash
cargo bench -p raft_core --bench pubsub_benchmark -- message_size/32
```

# Save baseline for comparison
```bash
cargo bench -p raft_core --bench pubsub_benchmark -- --save-baseline main
```