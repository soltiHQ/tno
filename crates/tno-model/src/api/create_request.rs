use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::{
    domain::{OnConflict, TaskId},
    spec::CreateSpec,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub on_conflict: Option<OnConflict>,
    pub task_id: Option<TaskId>,
    pub request_id: Uuid,
    pub spec: CreateSpec,
}
