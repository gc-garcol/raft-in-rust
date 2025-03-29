use crate::log::{LogPosition, SegmentLog};

/// Represents the possible states/roles a Raft server can be in
/// 
/// In the Raft consensus algorithm, a server can be in one of three states:
/// - Leader: Handles all client requests and log replication
/// - Follower: Passive state that responds to requests from leaders and candidates
/// - Candidate: Used during leader election when a follower becomes a candidate
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Leader,     // Coordinates all system changes, handles client requests
    Follower,   // Responds to RPCs from leaders and candidates
    Candidate,  // Initiates leader elections, requests votes from other servers
}

/// Represents the complete state of a Raft server as described in the Raft paper.
/// 
/// This structure maintains all state required for a Raft consensus server to function,
/// including both persistent state that survives crashes and volatile state that
/// is rebuilt after crashes.
#[derive(Debug, Clone)]
pub struct RaftState {
    // Persistent state on all servers (must be saved to stable storage before responding to RPCs)
    pub current_term: u64,        // Monotonically increasing term number
    pub voted_for: Option<u64>,   // Tracks which candidate received vote in current term
    pub logs: Vec<SegmentLog>,    // Log entries containing state machine commands and terms

    // Volatile state on all servers (rebuilt after crashes)
    pub commit_position: LogPosition,        // Highest log entry known to be committed (safe to apply)
    pub last_applied: LogPosition,           // Highest log entry applied to state machine

    // Volatile state on leaders (reinitialized after election)
    pub next_position: Vec<LogPosition>,     // Tracks next entry to send to each follower
    pub match_position: Vec<LogPosition>,    // Tracks highest replicated entry for each follower

    // Server identification and role
    pub state: State,             // Current server role (Leader/Follower/Candidate)
    pub server_id: u64,           // Unique identifier for this server in the cluster
}

impl RaftState {
    /// Creates a new RaftState instance with initial values
    /// 
    /// All servers start in the Follower state with:
    /// - Term 0
    /// - No votes cast
    /// - Empty log
    /// - Initial positions at 0
    /// 
    /// # Arguments
    /// * `server_id` - Unique identifier for this server in the cluster
    pub fn new(server_id: u64) -> Self {
        Self {
            current_term: 0,
            voted_for: None,
            logs: Vec::new(),
            commit_position: LogPosition::new(0, 0),
            last_applied: LogPosition::new(0, 0),
            next_position: Vec::new(),
            match_position: Vec::new(),
            state: State::Follower,
            server_id,
        }
    }

    /// Returns the index of the last log entry
    /// 
    /// Returns 0 if the log is empty, otherwise returns
    /// the index of the most recent entry in the log
    pub fn last_log_index(&self) -> u64 {
        if self.logs.is_empty() {
            0
        } else {
            self.logs.last().unwrap().last_log_index()
        }
    }

    /// Returns the term of the last log entry
    /// 
    /// Returns 0 if the log is empty, otherwise returns
    /// the term of the most recent entry in the log
    pub fn last_log_term(&self) -> u64 {
        if self.logs.is_empty() {
            0
        } else {
            self.logs.last().unwrap().term
        }
    }
}
