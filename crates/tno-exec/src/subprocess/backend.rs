use tokio::process::Command;

use crate::utils::{CgroupLimits, RlimitConfig, SecurityConfig};

/// Low-level OS/kernel configuration for subprocess execution.
///
/// Controls resource limits, security policies, and isolation mechanisms.
/// All fields are optional - if not specified, the subprocess inherits parent process settings.
#[derive(Debug, Clone, Default)]
pub struct SubprocessBackendConfig {
    /// POSIX rlimit-based resource limits.
    pub rlimits: Option<RlimitConfig>,

    /// Linux cgroup v2 resource limits.
    pub cgroups: Option<CgroupLimits>,

    /// Security hardening.
    pub security: Option<SecurityConfig>,
}

impl SubprocessBackendConfig {
    /// Create an empty backend config (no limits).
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any backend features are configured.
    pub fn is_empty(&self) -> bool {
        self.rlimits.is_none() && self.cgroups.is_none() && self.security.is_none()
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), crate::ExecError> {
        if let Some(cgroups) = &self.cgroups {
            if let Some(mem) = cgroups.memory {
                if mem == 0 {
                    return Err(crate::ExecError::InvalidRunnerConfig(
                        "cgroups.memory cannot be zero".into(),
                    ));
                }
            }
            if let Some(pids) = cgroups.pids {
                if pids == 0 {
                    return Err(crate::ExecError::InvalidRunnerConfig(
                        "cgroups.pids cannot be zero".into(),
                    ));
                }
            }
        }
        if let Some(rlimits) = &self.rlimits {
            if let Some(fsize) = rlimits.max_file_size_bytes {
                if fsize == 0 {
                    return Err(crate::ExecError::InvalidRunnerConfig(
                        "rlimits.max_file_size_bytes cannot be zero".into(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Apply all configured backend features to a `tokio::process::Command`.
    ///
    /// This method mutates the command by attaching pre_exec hooks for:
    /// - rlimits
    /// - cgroups
    /// - security policies
    ///
    /// Call this immediately before spawning the subprocess.
    pub fn apply_to_command(&self, cmd: &mut Command, cgroup_name: &str) -> Result<(), crate::ExecError> {
        if self.is_empty() {
            return Ok(());
        }

        if let Some(rlimits) = &self.rlimits {
            crate::utils::attach_rlimits(cmd, rlimits);
        }
        if let Some(cgroups) = &self.cgroups {
            crate::utils::attach_cgroup_limits(cmd, cgroup_name, cgroups)?;
        }
        if let Some(security) = &self.security {
            crate::utils::attach_security(cmd, security);
        }
        Ok(())
    }
}
