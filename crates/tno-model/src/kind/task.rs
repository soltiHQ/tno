use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::error::{ModelError, ModelResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TaskKind {
    Exec,
    Wasm,
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
