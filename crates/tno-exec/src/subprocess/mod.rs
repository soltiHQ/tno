//! Subprocess runner for `tno_model::TaskKind::Subprocess`.

mod config;
mod runner;
mod backend;

pub use runner::SubprocessRunner;

use crate::ExecError;
use std::sync::Arc;
use tno_core::RunnerRouter;
use tno_model::{LABEL_RUNNER_TAG, Labels};
use crate::subprocess::backend::SubprocessBackendConfig;

/// Register a subprocess runner with default settings (no backend limits).
pub fn register_subprocess_runner(
    router: &mut RunnerRouter,
    name: &'static str,
) -> Result<(), ExecError> {
    if router.contains_runner_tag(name) {
        return Err(ExecError::DuplicateRunnerTag {
            tag: name.to_string(),
        });
    }

    let mut labels = Labels::new();
    labels.insert(LABEL_RUNNER_TAG, name);

    router.register_with_labels(Arc::new(SubprocessRunner::new(name)), labels);
    Ok(())
}

/// Register a subprocess runner with explicit backend configuration.
///
/// Backend settings (rlimits, cgroups, security) are applied to all tasks
/// spawned by this runner instance.
pub fn register_subprocess_runner_with_backend(
    router: &mut RunnerRouter,
    name: &'static str,
    backend: SubprocessBackendConfig,
) -> Result<(), ExecError> {
    if router.contains_runner_tag(name) {
        return Err(ExecError::DuplicateRunnerTag {
            tag: name.to_string(),
        });
    }

    // Validate backend config at registration time
    backend.validate().map_err(|e| ExecError::InvalidSpec(e.to_string()))?;

    let mut labels = Labels::new();
    labels.insert(LABEL_RUNNER_TAG, name);

    router.register_with_labels(
        Arc::new(SubprocessRunner::with_backend(name, backend)),
        labels,
    );
    Ok(())
}