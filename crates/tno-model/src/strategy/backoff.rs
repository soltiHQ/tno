use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct BackoffStrategy {
    pub jitter: super::JitterStrategy,
    pub delay_ms: Option<u64>,
    pub first_ms: u64,
    pub max_ms: u64,
    pub factor: f64,
}
