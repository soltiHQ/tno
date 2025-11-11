use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::{
    domain::{Slot, TimeoutMs},
    kind::TaskKind,
    strategy::{AdmissionStrategy, BackoffStrategy, RestartStrategy},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateSpec {
    pub slot: Slot,
    pub kind: TaskKind,
    pub timeout_ms: TimeoutMs,
    pub restart: RestartStrategy,
    pub backoff: BackoffStrategy,
    pub admission: AdmissionStrategy,
}
