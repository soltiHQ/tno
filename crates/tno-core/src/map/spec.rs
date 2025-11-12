use std::time::Duration;

use taskvisor::{ControllerSpec, TaskRef, TaskSpec};
use tno_model::CreateSpec;

use super::{to_admission_policy, to_backoff_policy, to_restart_policy};

pub fn to_task_spec(task: TaskRef, s: &CreateSpec) -> TaskSpec {
    TaskSpec::new(
        task,
        to_restart_policy(s.restart),
        to_backoff_policy(&s.backoff),
        Some(Duration::from_millis(s.timeout_ms)),
    )
}

pub fn to_controller_spec(task: TaskRef, s: &CreateSpec) -> ControllerSpec {
    ControllerSpec {
        admission: to_admission_policy(s.admission),
        task_spec: to_task_spec(task, s),
    }
}
