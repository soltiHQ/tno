use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::error::{ModelError, ModelResult};

/// Determines whether a task should be automatically restarted after it completes or fails.
///
/// The restart strategy operates at the supervisor layer and is applied to each task instance independently.
/// If combined with a backoff policy, restarts will be delayed according to the configured backoff + jitter.
///
/// Strategies:
/// - `Never`: Do not restart the task under any circumstances.
/// - `Always`: Restart the task every time it completes, regardless of exit status.
/// - `OnFailure`: Restart only when the task ends with an error.
///
/// Restart behavior is evaluated after each task execution cycle.
/// If a task is canceled (via controller or shutdown), it is **not** considered a failure
/// and will not be restarted unless explicitly treated as such by the runner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RestartStrategy {
    /// Never restart the task.
    Never,
    /// Always restart the task after it finishes, regardless of outcome.
    Always,
    /// Restart the task only if it failed (non-zero exit, error, panic, etc.).
    OnFailure,
}

impl Default for RestartStrategy {
    fn default() -> Self {
        RestartStrategy::OnFailure
    }
}

impl FromStr for RestartStrategy {
    type Err = ModelError;
    fn from_str(s: &str) -> ModelResult<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "always" => Ok(RestartStrategy::Always),
            "never" | "" => Ok(RestartStrategy::Never),
            "on-failure" | "failure" => Ok(RestartStrategy::OnFailure),
            other => Err(ModelError::UnknownRestart(other.to_string())),
        }
    }
}
