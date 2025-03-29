use crate::log::LogEntry;

/// Request message sent by candidates during leader election
/// 
/// When a candidate starts an election, it sends RequestVote RPCs to all other
/// servers in parallel. The candidate continues in this state until either:
/// - It wins the election
/// - Another server establishes itself as leader
/// - A period of time goes by with no winner
#[derive(Debug, Clone)]
pub struct RequestVoteRequest {
    pub current_term: u64,      // Candidate's term number, used for term comparison
    pub candidate_id: u64,      // ID of the node requesting votes
    pub last_log_index: u64,    // Index of candidate's last log entry for log completeness check
    pub last_log_term: u64,     // Term of candidate's last log entry for log completeness check
}

/// Response message for a RequestVote RPC
/// 
/// Followers respond to vote requests based on term numbers and log completeness.
/// A candidate must receive votes from a majority of servers to become leader.
#[derive(Debug, Clone)]
pub struct RequestVoteResponse {
    pub term: u64,              // Responding server's current term, for candidate to update itself
    pub vote_granted: bool,     // True means candidate received vote from this follower
}

/// Request message sent by leader to replicate log entries and maintain heartbeat
/// 
/// Leaders send AppendEntries RPCs to all followers to:
/// - Replicate new log entries
/// - Maintain heartbeat signals
/// - Update commit index across the cluster
#[derive(Debug, Clone)]
pub struct AppendEntryRequest {
    pub current_term: u64,      // Leader's term, used by followers to detect stale leaders
    pub leader_id: u64,         // Leader's ID, so followers can redirect clients

    pub prev_log_index: u64,    // Used for log consistency check
    pub prev_log_term: u64,     // Used for log consistency check

    pub append_index: u64,      // Starting index for appending new entries
    pub entries: Vec<LogEntry>, // Log entries to store (empty for heartbeat)

    pub leader_commit: u64,     // Leader's commit index to advance followers' commit index
}

/// Response message for an AppendEntries RPC
/// 
/// Followers respond to append entries requests to indicate success or failure
/// of log replication attempts. Failed attempts may trigger log backtracking.
#[derive(Debug, Clone)]
pub struct AppendEntryResponse {
    pub term: u64,              // Follower's current term, for leader to update itself
    pub success: bool,          // True if follower contained entry matching prev_log_index/term
}
