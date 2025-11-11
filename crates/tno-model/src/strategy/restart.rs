use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::error::{ModelError, ModelResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RestartStrategy {
    Never,
    Always,
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
