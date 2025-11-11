use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::error::{ModelError, ModelResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum JitterStrategy {
    None,
    Full,
    Equal,
    Decorrelated,
}

impl Default for JitterStrategy {
    fn default() -> Self {
        JitterStrategy::Full
    }
}

impl FromStr for JitterStrategy {
    type Err = ModelError;
    fn from_str(s: &str) -> ModelResult<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "equal" => Ok(JitterStrategy::Equal),
            "" | "none" => Ok(JitterStrategy::None),
            "full" | "default" => Ok(JitterStrategy::Full),
            "decorrelated" => Ok(JitterStrategy::Decorrelated),
            other => Err(ModelError::UnknownJitter(other.to_string())),
        }
    }
}
