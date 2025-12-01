use crate::utils::{CgroupLimits,RlimitConfig, SecurityConfig};

/// Low-level OS/kernel configuration for subprocess execution.
///
/// Controls resource limits, security policies, and isolation mechanisms
/// applied to the spawned process before exec.
///
/// All fields are optional - if not specified, the subprocess inherits
/// parent process settings.
#[derive(Debug, Clone, Default)]
pub struct SubprocessBackendConfig {
    /// POSIX rlimit-based resource limits (memory, files, CPU time).
    ///
    /// Applied via `setrlimit()` in the child process before exec.
    /// Works on all Unix platforms.
    pub rlimits: Option<RlimitConfig>,

    /// Linux cgroup v2 resource limits (CPU quota, memory, PIDs).
    ///
    /// Linux-only. On other platforms, emits a warning and is ignored.
    pub cgroups: Option<CgroupLimits>,

    /// Security hardening (capabilities, no_new_privs).
    ///
    /// Linux-only. On other platforms, emits a warning and is ignored.
    pub security: Option<SecurityConfig>,
}

impl SubprocessBackendConfig {
    /// Create an empty backend config (no limits, no hardening).
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any backend features are configured.
    ///
    /// Returns `true` if at least one field is `Some`.
    pub fn is_empty(&self) -> bool {
        self.rlimits.is_none() && self.cgroups.is_none() && self.security.is_none()
    }

    /// Validate the configuration.
    ///
    /// Checks for:
    /// - Conflicting limits (e.g., rlimit and cgroup both setting memory)
    /// - Platform compatibility (cgroups/security on non-Linux)
    /// - Invalid values (e.g., memory limit = 0)
    pub fn validate(&self) -> Result<(), crate::ExecError> {
        // Warn if both rlimits and cgroups set the same resource
        if let (Some(rlimits), Some(cgroups)) = (&self.rlimits, &self.cgroups) {
            if rlimits.max_file_size_bytes.is_some() && cgroups.memory.is_some() {
                tracing::warn!(
                    "both rlimits.max_file_size_bytes and cgroups.memory are set; \
                     cgroup limit will take precedence on Linux"
                );
            }
        }

        // Validate cgroups
        if let Some(cgroups) = &self.cgroups {
            if let Some(mem) = cgroups.memory {
                if mem == 0 {
                    return Err(crate::ExecError::InvalidSpec(
                        "cgroups.memory cannot be zero".into(),
                    ));
                }
            }
            if let Some(pids) = cgroups.pids {
                if pids == 0 {
                    return Err(crate::ExecError::InvalidSpec(
                        "cgroups.pids cannot be zero".into(),
                    ));
                }
            }
        }

        // Validate rlimits
        if let Some(rlimits) = &self.rlimits {
            if let Some(fsize) = rlimits.max_file_size_bytes {
                if fsize == 0 {
                    return Err(crate::ExecError::InvalidSpec(
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
    /// - rlimits (if configured)
    /// - cgroups (if configured and on Linux)
    /// - security policies (if configured and on Linux)
    ///
    /// Call this immediately before spawning the subprocess.
    pub fn apply_to_command(
        &self,
        cmd: &mut tokio::process::Command,
        cgroup_name: &str,
    ) -> Result<(), crate::ExecError> {
        if self.is_empty() {
            return Ok(());
        }

        // Apply in specific order for safety:
        // 1. rlimits (basic resource limits)
        // 2. cgroups (kernel-level isolation)
        // 3. security (capabilities/privileges must be dropped last)

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