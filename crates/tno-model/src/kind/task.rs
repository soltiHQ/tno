use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Env, Flag};

/// Execution configuration for a task.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskKind {
    /// Execute a function registered inside the runtime.
    Fn,
    /// Execute a native process.
    Exec {
        /// Command to execute (e.g., "ls", "/usr/bin/python").
        command: String,

        /// Command-line arguments.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,

        /// Environment variables for the process.
        #[serde(default, skip_serializing_if = "Env::is_empty")]
        env: Env,

        /// Working directory. If `None`, inherits from parent process.
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<PathBuf>,

        /// Whether to treat non-zero exit codes as task failure.
        #[serde(default)]
        fail_on_non_zero: Flag,
    },
    /// Execute a WebAssembly module via WASI runtime.
    Wasm {
        /// Path to the .wasm module.
        module: PathBuf,

        /// Arguments passed to the WASI main function.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,

        /// Environment variables exposed to the WASI module.
        #[serde(default, skip_serializing_if = "Env::is_empty")]
        env: Env,
    },
    /// Run a task inside an OCI-compatible container.
    Container {
        /// Container image (e.g., "nginx:latest", "docker.io/library/redis:7").
        image: String,

        /// Override container entrypoint.
        #[serde(skip_serializing_if = "Option::is_none")]
        command: Option<Vec<String>>,

        /// Arguments passed to the container entrypoint.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,

        /// Environment variables for the container.
        #[serde(default, skip_serializing_if = "Env::is_empty")]
        env: Env,
    },
}

impl TaskKind {
    /// Returns the kind as a static string.
    pub fn kind(&self) -> &'static str {
        match self {
            TaskKind::Fn => "fn",
            TaskKind::Exec { .. } => "exec",
            TaskKind::Wasm { .. } => "wasm",
            TaskKind::Container { .. } => "container",
        }
    }
}

impl Default for TaskKind {
    fn default() -> Self {
        TaskKind::Fn
    }
}
