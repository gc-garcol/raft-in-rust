use std::error::Error;

use raft_core::network::{RpcMessage, ClusterOutboundNetwork};

pub struct RaftRpcOutboundNetwork {}

#[async_trait::async_trait]
impl ClusterOutboundNetwork for RaftRpcOutboundNetwork {
    async fn send(&self, message: RpcMessage, node_id: u64) -> Result<(), Box<dyn Error + Send + Sync>> {
        log::info!("Sending message to node {} with message {:?}", node_id, message);
        todo!()
    }
}

