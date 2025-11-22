#![cfg(feature = "timezone-sync")]
//! Timezone synchronization task spec for the logger.
//!
//! Periodically syncs the local timezone offset to detect DST transitions
//! without requiring process restart.
//!
//! Backoff policy (current implementation):
//! - On success: fixed delay of 1 hour before next sync.
//! - On failure: retries with fixed 60s delay (no growth yet).
//!
//! Requires:
//! - `init_local_offset()` called in `main()` before tokio runtime.
//! - `timezone-sync` feature flag.
// TODO: https://github.com/soltiHQ/taskvisor/issues/46: remove Backoff strategy after new feature.
use taskvisor::{TaskError, TaskFn, TaskRef};
use tokio_util::sync::CancellationToken;
use tracing::debug;

use crate::logger::object::timezone::sync_local_offset;
use tno_model::{
    AdmissionStrategy, BackoffStrategy, CreateSpec, JitterStrategy, RestartStrategy, TaskKind,
};

/// Timeout applied to each sync operation (in milliseconds).
pub const TZ_SYNC_TIMEOUT_MS: u64 = 60_000;

/// Base delay after a failure (in milliseconds).
pub const TZ_SYNC_RETRY_MS: u64 = 60_000;

/// Delay after a successful sync (in milliseconds).
pub const TZ_SYNC_SUCCESS_DELAY_MS: u64 = 3_600_000;

/// Name of the internal timezone-sync task.
pub const TZ_SYNC_TASK_NAME: &str = "tno-logger-tz-sync";

/// Build the timezone sync task and its corresponding `CreateSpec`.
///
/// Returns:
/// - `TaskRef`    — concrete task body.
/// - `CreateSpec` — model-level specification with restart/backoff/admission policies.
///
/// # Example
/// ```no_run
/// let (task, spec) = timezone_sync_spec();
/// api.submit_with_task(task, &spec).await?;
/// ```
pub fn timezone_sync_spec() -> (TaskRef, CreateSpec) {
    let task: TaskRef = TaskFn::arc(TZ_SYNC_TASK_NAME, |ctx: CancellationToken| async move {
        debug!("timezone sync started");

        if ctx.is_cancelled() {
            return Err(TaskError::Canceled);
        }
        match sync_local_offset() {
            Ok(()) => {
                debug!("timezone offset sync success");
                Ok(())
            }
            Err(e) => Err(TaskError::Fail {
                reason: format!("failed to sync timezone offset: {e}"),
            }),
        }
    });

    let backoff = BackoffStrategy {
        jitter: JitterStrategy::Equal,
        delay_ms: Some(TZ_SYNC_SUCCESS_DELAY_MS),
        first_ms: TZ_SYNC_RETRY_MS,
        max_ms: TZ_SYNC_RETRY_MS,
        factor: 1.0,
    };
    let spec = CreateSpec {
        slot: TZ_SYNC_TASK_NAME.to_string(),
        kind: TaskKind::Fn,
        timeout_ms: TZ_SYNC_TIMEOUT_MS,
        restart: RestartStrategy::Always,
        backoff,
        admission: AdmissionStrategy::Replace,
    };
    (task, spec)
}
