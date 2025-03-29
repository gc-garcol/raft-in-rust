use std::error::Error;

use raft_core::network::{RpcMessage, ClusterInboundNetwork};

pub struct RaftRpcInboundNetwork {}

#[async_trait::async_trait]
impl ClusterInboundNetwork for RaftRpcInboundNetwork {
    async fn receive(&self, message: RpcMessage, node_id: u64) -> Result<(), Box<dyn Error + Send + Sync>> {
        log::info!("Received message from node {} with message {:?}", node_id, message);
        todo!()
    }
}
