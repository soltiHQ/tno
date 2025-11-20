use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::error::{ModelError, ModelResult};

/// Represents the execution backend used to run a task.
///
/// Each variant corresponds to a different runtime environment.
/// The runner layer is responsible for providing concrete implementations.
///
/// Variants:
/// - `Exec`: Launches a local process using the host's OS.
/// - `Wasm`: Executes a WebAssembly module (e.g., via WASI-compatible runtime).
/// - `Container`: Runs a task inside an OCI-compatible container runtime.
///
/// This enum is configuration-driven and intentionally independent of which runners are compiled in.
/// If a `TaskKind` is provided that has no available runner at runtime, the caller will receive
/// an `UnsupportedKind`/`InvalidSpec` error from the runner registry.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskKind {
    /// Execute a native process using the system shell or a direct binary.
    Exec,
    /// Run a WebAssembly module in a WASI-compatible environment.
    Wasm,
    /// Start a task inside an OCI-compatible container runtime.
    Container,
}

impl Default for TaskKind {
    fn default() -> Self {
        TaskKind::Exec
    }
}

impl FromStr for TaskKind {
    type Err = ModelError;
    fn from_str(s: &str) -> ModelResult<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "exec" => Ok(TaskKind::Exec),
            "wasm" | "wasi" => Ok(TaskKind::Wasm),
            "container" => Ok(TaskKind::Container),
            other => Err(ModelError::UnknownTaskKind(other.to_string())),
        }
    }
}
