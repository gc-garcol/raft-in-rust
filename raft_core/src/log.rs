//! Core log management module for Raft consensus implementation.
//! Handles storage and manipulation of replicated log entries.

/// Represents a single log entry in the Raft log
/// Each entry contains arbitrary bytes that represent commands to be replicated
/// across the cluster. The actual interpretation of these bytes is left to the
/// state machine implementation.
pub type LogEntry = Vec<u8>;

/// LogPosition represents the unique identifier for a log entry in the Raft log.
/// Each log entry is identified by both its term and index, which together
/// provide a unique position in the log history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogPosition {
    /// The term number when this entry was created
    /// Terms are monotonically increasing numbers that represent different
    /// leader epochs in the Raft cluster
    pub term: u64,

    /// The index position in the log (1-based indexing)
    /// Indexes start at 1 and increase monotonically for each new log entry
    pub index: u64,
}

impl LogPosition {
    /// Creates a new LogPosition with the specified term and index
    ///
    /// # Arguments
    /// * `term` - The term number for this log entry
    /// * `index` - The position in the log (1-based)
    ///
    /// # Examples
    /// ```
    /// let pos = LogPosition::new(1, 1); // First entry in term 1
    /// ```
    pub fn new(term: u64, index: u64) -> Self {
        Self { term, index }
    }
}

/// SegmentLog represents a contiguous section of the Raft log from a single term.
/// This structure is used to manage and replicate log entries efficiently, keeping
/// entries from the same term grouped together.
#[derive(Debug, Clone)]
pub struct SegmentLog {
    /// The term number when these entries were received by the leader
    /// All entries in this segment share the same term, which is important
    /// for maintaining log consistency and handling leader elections
    pub term: u64,

    /// Vector of commands to be applied to the state machine
    /// Index 0 corresponds to log index 1 (1-based indexing)
    /// Each command is an opaque byte vector that will be interpreted
    /// by the state machine
    pub commands: Vec<LogEntry>,
}

impl SegmentLog {
    /// Creates a new SegmentLog with the specified term and commands
    ///
    /// # Arguments
    /// * `term` - The term number for all entries in this segment
    /// * `commands` - Vector of commands to be stored in this segment
    ///
    /// # Examples
    /// ```
    /// let commands = vec![vec![1, 2, 3]]; // Single command
    /// let segment = SegmentLog::new(1, commands);
    /// ```
    pub fn new(term: u64, commands: Vec<LogEntry>) -> Self {
        Self { term, commands }
    }

    /// Returns the index of the last entry in this segment
    /// Since we use 1-based indexing, this is equal to the length of the commands vector
    ///
    /// # Returns
    /// The index of the last entry, or 0 if the segment is empty
    pub fn last_log_index(&self) -> u64 {
        self.commands.len() as u64
    }

    /// Retrieves the log entry at the specified index
    ///
    /// # Arguments
    /// * `index` - The 1-based index of the desired log entry
    ///
    /// # Returns
    /// * `Some(&LogEntry)` if the index exists
    /// * `None` if the index is out of bounds
    ///
    /// Note: Index is 1-based, so we subtract 1 to access the internal vector
    pub fn log_at(&self, index: u64) -> Option<&LogEntry> {
        self.commands.get(index as usize - 1)
    }
}
