use serde::{Deserialize, Serialize};

pub type Slot = String;
pub type Level = String;

pub type TimeoutMs = u64;

pub type Env = Vec<crate::common::KeyValue>;

pub type TaskId = String;

pub type Reason = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OnConflict {
    Error,
    Replace,
}
