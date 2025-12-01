use std::process::Stdio;

use taskvisor::{TaskError, TaskFn, TaskRef};
use tokio::process::Command;
use tokio_util::sync::CancellationToken;
use tracing::{debug, trace};

use crate::subprocess::backend::SubprocessBackendConfig;
use crate::subprocess::task::SubprocessTaskConfig;
use tno_core::{BuildContext, Runner, RunnerError};
use tno_model::{CreateSpec, TaskKind};

/// Runner that executes `TaskKind::Subprocess` as OS subprocesses.
pub struct SubprocessRunner {
    name: &'static str,
    /// Backend configuration applied to all tasks spawned by this runner.
    ///
    /// Set once during runner initialization/registration.
    backend: Option<SubprocessBackendConfig>,
}

impl SubprocessRunner {
    /// Create a new subprocess with name.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            backend: None,
        }
    }

    /// Create a subprocess runner with explicit backend configuration.
    ///
    /// Backend settings (rlimits, cgroups, security) will be applied to
    /// all tasks spawned by this runner instance.
    pub fn with_backend(name: &'static str, backend: SubprocessBackendConfig) -> Self {
        Self {
            name,
            backend: Some(backend),
        }
    }

    /// Build normalized subprocess configuration from `CreateSpec` + `BuildContext`.
    fn build_config(
        &self,
        spec: &CreateSpec,
        ctx: &BuildContext,
    ) -> Result<SubprocessTaskConfig, RunnerError> {
        let cfg = match &spec.kind {
            TaskKind::Subprocess {
                command,
                args,
                env,
                cwd,
                fail_on_non_zero,
            } => SubprocessTaskConfig {
                run_id: self.build_run_id(&spec.slot),
                command: command.clone(),
                args: args.clone(),
                env: ctx.env().merged(env),
                cwd: cwd.clone(),
                fail_on_non_zero: *fail_on_non_zero,
            },
            other => {
                return Err(RunnerError::UnsupportedKind {
                    runner: self.name,
                    kind: other.kind().to_string(),
                });
            }
        };

        cfg.validate()
            .map_err(|e| RunnerError::InvalidSpec(e.to_string()))?;
        cfg.trace_state(&spec.slot);
        Ok(cfg)
    }
}

impl Runner for SubprocessRunner {
    fn name(&self) -> &'static str {
        self.name
    }

    fn supports(&self, spec: &CreateSpec) -> bool {
        matches!(spec.kind, TaskKind::Subprocess { .. })
    }

    fn build_task(&self, spec: &CreateSpec, ctx: &BuildContext) -> Result<TaskRef, RunnerError> {
        let cfg = self.build_config(spec, ctx)?;
        let backend = self.backend.clone();

        trace!(
            slot = %spec.slot.clone(),
            task = %cfg.run_id,
            "building subprocess task",
        );

        let task: TaskRef = TaskFn::arc(cfg.run_id.clone(), move |cancel: CancellationToken| {
            let cfg = cfg.clone();
            let backend = backend.clone();

            async move {
                trace!(
                    task = %cfg.run_id,
                    command = %cfg.command,
                    args = ?cfg.args,
                    cwd = ?cfg.cwd,
                    "spawning subprocess",
                );

                let mut cmd = Command::new(&cfg.command);
                cmd.args(&cfg.args);

                if let Some(cwd) = &cfg.cwd {
                    cmd.current_dir(cwd);
                }
                for kv in cfg.env.iter() {
                    cmd.env(kv.key(), kv.value());
                }
                cmd.stdout(Stdio::piped());
                cmd.stderr(Stdio::inherit());

                if let Some(backend) = &backend {
                    backend
                        .apply_to_command(&mut cmd, &cfg.run_id)
                        .map_err(|e| TaskError::Fatal {
                            reason: format!("backend config failed: {e}"),
                        })?;
                }

                let mut child = cmd.spawn().map_err(|e| TaskError::Fatal {
                    reason: format!("spawn failed: {e}"),
                })?;
                let status_fut = child.wait();

                tokio::select! {
                    res = status_fut => {
                        let status = res.map_err(|e| TaskError::Fatal {
                            reason: format!("wait failed: {e}"),
                        })?;

                        if !status.success() && cfg.fail_on_non_zero.is_enabled() {
                            return if let Some(code) = status.code() {
                                Err(TaskError::Fail {
                                    reason: format!("process exited with non-zero code: {code}"),
                                })
                            } else {
                                Err(TaskError::Fail {
                                    reason: "process terminated by signal".into(),
                                })
                            }
                        }
                        debug!("subprocess exited successfully");
                        Ok(())
                    }
                    _ = cancel.cancelled() => {
                        debug!("cancellation requested; killing subprocess");

                        if let Err(e) = child.kill().await {
                            debug!("failed to kill subprocess: {e}");
                        }
                        Err(TaskError::Canceled)
                    }
                }
            }
        });
        Ok(task)
    }
}
