use async_trait::async_trait;
use std::error::Error;

use crate::rpc::{
    AppendEntryRequest, AppendEntryResponse, RequestVoteRequest, RequestVoteResponse,
};

/// Represents different types of RPC messages in the Raft protocol
#[derive(Debug)]
pub enum RpcMessage {
    RequestVote(RequestVoteRequest),
    RequestVoteResponse(RequestVoteResponse),
    AppendEntry(AppendEntryRequest),
    AppendEntryResponse(AppendEntryResponse),
}

/// Network communication trait for Raft nodes
#[async_trait]
pub trait ClusterOutboundNetwork: Send + Sync {
    /// Sends an RPC message to another node in the cluster
    ///
    /// # Arguments
    /// * `message` - The RPC message to send
    /// * `node_id` - The ID of the target node
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(NetworkError)` if the send operation failed
    async fn send(&self, message: RpcMessage, node_id: u64) -> Result<(), Box<dyn Error + Send + Sync>>;
}

/// Network communication trait for Raft nodes
#[async_trait]
pub trait ClusterInboundNetwork: Send + Sync {
    /// Receives an RPC message from another node in the cluster
    ///
    /// # Arguments
    /// * `message` - The RPC message to receive
    /// * `node_id` - The ID of the source node
    ///
    /// # Returns
    /// * `Ok(())` if the message was received successfully
    /// * `Err(NetworkError)` if the receive operation failed
    async fn receive(&self, message: RpcMessage, node_id: u64) -> Result<(), Box<dyn Error + Send + Sync>>;
}
