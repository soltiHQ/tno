use std::path::PathBuf;

/// Resource limits for the subprocess backend.
///
/// These limits are shared by all tasks handled by a particular`SubprocessRunner` instance.
/// They do *not* depend on a specific `CreateSpec`.
#[derive(Debug, Clone, Default)]
pub struct SubprocessLimits {
    /// Optional CPU quota for the subprocess.
    ///
    /// Interpretation is runtime/OS dependent. One common convention:
    /// - `None`  → no explicit CPU limit;
    /// - `Some(1.0)` → full core;
    /// - `Some(0.5)` → half a core, etc.
    pub cpu_quota: Option<f32>,
    /// Optional memory limit in bytes.
    ///
    /// `None` means no explicit memory limit is enforced by this runner.
    pub memory_limit_bytes: Option<u64>,
}

/// Process isolation options for the subprocess backend.
///
/// These settings describe how strongly the subprocess should be
/// isolated from the host (namespaces, chroot, etc.).
#[derive(Debug, Clone, Default)]
pub struct SubprocessIsolation {
    /// Whether to run the subprocess in a dedicated PID namespace (if supported).
    pub use_pid_namespace: bool,
    /// Whether to run the subprocess in a dedicated network namespace (if supported).
    pub use_net_namespace: bool,
    /// Optional chroot directory for the subprocess.
    ///
    /// When set, the subprocess will be executed inside this root (subject to platform and permission constraints).
    pub chroot: Option<PathBuf>,
}

/// Backend-level configuration for the subprocess runner.
///
/// This configuration is owned by a `SubprocessRunner` and shared across all tasks it handles.
/// Task-specific settings (command, args, env, etc.) live in `SubprocessConfig`.
#[derive(Debug, Clone, Default)]
pub struct SubprocessBackendConfig {
    /// Resource limits (CPU / memory) applied to all subprocesses.
    pub limits: SubprocessLimits,

    /// Isolation knobs (namespaces, chroot, etc.) for all subprocesses.
    pub isolation: SubprocessIsolation,
}

impl SubprocessBackendConfig {
    /// Create a new backend config with default settings.
    ///
    /// By default:
    /// - no explicit CPU limit;
    /// - no explicit memory limit;
    /// - no additional isolation (no extra namespaces, no chroot).
    pub fn new() -> Self {
        Self::default()
    }
}
