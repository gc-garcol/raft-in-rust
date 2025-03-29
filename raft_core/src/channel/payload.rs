use std::fmt::Debug;

pub trait Request: Debug + Send + Sync + Clone {}
pub trait Response: Debug + Send + Sync + Clone {}
