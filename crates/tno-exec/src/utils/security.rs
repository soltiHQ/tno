//! Basic security hardening for subprocess-based runners.
//!
//! ## Overview
//!
//! This module provides a structured API for configuring process-level security.
//! - **Linux capabilities** (drop-all + allowlist model);
//! - **`no_new_privs`** flag (Linux);
//! - a placeholder hook for **seccomp** profiles (Linux-only, not enforced yet).
use tokio::process::Command;
use tracing::warn;

/// Declarative security policy for a child process.
///
/// All fields are optional / opt-in:
/// - `drop_all_caps = false` do not touch capabilities;
/// - `keep_caps` is only meaningful when `drop_all_caps` is `true`;
/// - `no_new_privs = false` do not set `no_new_privs`;
/// - `seccomp = None` no seccomp filtering.
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// Drop all capabilities before exec.
    ///
    /// On Linux, this is intended to clear the capability sets.
    pub drop_all_caps: bool,
    /// Optional allowlist of capabilities to keep or re-add after `drop_all_caps`.
    pub keep_caps: Vec<LinuxCapability>,
    /// Enable `no_new_privs` for the child process.
    pub no_new_privs: bool,
    /// Optional seccomp profile to apply.
    pub seccomp: Option<SeccompProfile>,
}

impl SecurityConfig {
    /// Returns `true` if no security knobs are configured.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.drop_all_caps
            && self.keep_caps.is_empty()
            && self.seccomp.is_none()
            && !self.no_new_privs
    }
}

/// Linux capability identifiers used for allowlisting.
#[derive(Debug, Clone, Copy)]
pub enum LinuxCapability {
    /// Network administration (e.g. configuring interfaces, routing tables).
    NetAdmin,
    /// System administration (mounts, hostname, etc.).
    SysAdmin,
    /// Raw socket access and packet capture.
    NetRaw,
    /// Arbitrary filesystem operations beyond DAC (chmod/chown on files you don't own).
    Fowner,
    /// Generic placeholder for a capability not explicitly modelled yet.
    Other(&'static str),
}

/// Placeholder for a seccomp profile.
#[derive(Debug, Clone)]
pub enum SeccompProfile {
    /// No seccomp filtering (explicitly requested).
    Unrestricted,
    /// Named profile that can be resolved by the runtime (e.g. "default", "strict").
    Named(&'static str),
}

/// Attach security policy to a `tokio::process::Command`.
pub fn attach_security(cmd: &mut Command, config: &SecurityConfig) {
    if config.is_empty() {
        return;
    }

    #[cfg(target_os = "linux")]
    {
        linux_impl::attach(cmd, config);
    }

    #[cfg(not(target_os = "linux"))]
    {
        warn!(
            ?config,
            "security configuration is only enforced on Linux; current OS={} – settings will be ignored",
            std::env::consts::OS,
        );
    }
}

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::{LinuxCapability, SeccompProfile, SecurityConfig};

    use std::{io, os::unix::prelude::CommandExt};

    use tokio::process::Command;
    use tracing::warn;
    use libc;

    // `PR_SET_NO_NEW_PRIVS` usually exists in libc, but we keep a fallback definition
    // to be robust across libc versions.
    #[allow(non_camel_case_types)]
    type prctl_option_t = libc::c_int;

    const PR_SET_NO_NEW_PRIVS: prctl_option_t = 38;

    pub fn attach(cmd: &mut Command, config: &SecurityConfig) {
        if config.is_empty() {
            return;
        }

        let cfg = config.clone();
        unsafe {
            cmd.pre_exec(move || {
                if cfg.no_new_privs {
                    apply_no_new_privs()?;
                }

                if cfg.drop_all_caps || !cfg.keep_caps.is_empty() {
                    warn!(
                        drop_all_caps = cfg.drop_all_caps,
                        keep_caps = ?caps_debug(&cfg.keep_caps),
                        "capability management is not implemented yet; requested settings will NOT be enforced"
                    );
                }
                if let Some(profile) = &cfg.seccomp {
                    warn!(
                        ?profile,
                        "seccomp profiles are not implemented yet; requested profile will NOT be enforced"
                    );
                }
                Ok(())
            });
        }
    }

    fn apply_no_new_privs() -> io::Result<()> {
        let rc = unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
        if rc != 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn caps_debug(caps: &[LinuxCapability]) -> Vec<String> {
        caps.iter()
            .map(|c| match c {
                LinuxCapability::NetAdmin => "NET_ADMIN".to_string(),
                LinuxCapability::SysAdmin => "SYS_ADMIN".to_string(),
                LinuxCapability::NetRaw => "NET_RAW".to_string(),
                LinuxCapability::Fowner => "FOWNER".to_string(),
                LinuxCapability::Other(name) => format!("OTHER({})", name),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config_is_noop() {
        let cfg = SecurityConfig::default();
        assert!(cfg.is_empty());

        let mut cmd = Command::new("sh");
        attach_security(&mut cmd, &cfg);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn non_empty_config_attaches_pre_exec_hook_on_linux() {
        let cfg = SecurityConfig {
            drop_all_caps: true,
            keep_caps: vec![LinuxCapability::NetAdmin],
            no_new_privs: true,
            seccomp: Some(SeccompProfile::Named("default")),
        };

        assert!(!cfg.is_empty());

        let mut cmd = Command::new("sh");
        attach_security(&mut cmd, &cfg);
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn non_empty_config_is_ignored_on_non_linux() {
        let cfg = SecurityConfig {
            drop_all_caps: true,
            keep_caps: vec![LinuxCapability::NetAdmin],
            no_new_privs: true,
            seccomp: Some(SeccompProfile::Named("default")),
        };

        assert!(!cfg.is_empty());

        let mut cmd = Command::new("sh");
        attach_security(&mut cmd, &cfg);
    }
}