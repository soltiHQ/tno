use serde::{Deserialize, Serialize};

/// Logical identifier for a controller slot.
///
/// A slot groups tasks that must not run concurrently.
/// The controller enforces admission policies per slot.
pub type Slot = String;

/// Timeout value in milliseconds.
///
/// Used in task specifications and controller rules where
/// an explicit time limit is required.
pub type TimeoutMs = u64;

/// List of environment variables passed to the task.
///
/// Each entry is a simple key–value pair.
pub type Env = Vec<KeyValue>;

/// Behavior when a new task targets a slot that already has a running task.
///
/// Defines how the controller handles conflicts when multiple tasks
/// are submitted for the same slot:
///
/// - `Error`: reject the new task and return an error.
/// - `Replace`: cancel the currently running task and start the new one.
/// - `Skip`: ignore the new task and report success to the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OnConflict {
    /// Reject the incoming task and return an error.
    Error,
    /// Cancel the currently running task and replace it with the new one.
    Replace,
    /// Silently ignore the new task if a task is already running, returning success without scheduling anything.
    Skip,
}

/// Key–value pair used for environment variables or generic metadata.
///
/// Both fields are plain UTF-8 strings with no validation applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValue {
    /// Name of the variable or key.
    pub key: String,
    /// Value associated with the key.
    pub value: String,
}
