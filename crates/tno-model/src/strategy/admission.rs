use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::error::{ModelError, ModelResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum AdmissionStrategy {
    DropIfRunning,
    Replace,
    Queue,
}

impl Default for AdmissionStrategy {
    fn default() -> Self {
        AdmissionStrategy::Queue
    }
}

impl FromStr for AdmissionStrategy {
    type Err = ModelError;
    fn from_str(s: &str) -> ModelResult<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "drop-if-running" | "drop" => Ok(AdmissionStrategy::DropIfRunning),
            "queue" | "add" | "new" | "" => Ok(AdmissionStrategy::Queue),
            "replace" => Ok(AdmissionStrategy::Replace),
            other => Err(ModelError::UnknownAdmission(other.to_string())),
        }
    }
}
