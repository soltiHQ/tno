//! Define capabilities.

/// Linux capability.
///
/// This enum covers the most commonly used capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LinuxCapability {
    /// `CAP_CHOWN`: Make arbitrary changes to file UIDs and GIDs
    Chown,
    /// `CAP_DAC_OVERRIDE`: Bypass file read, write, and execute permission checks
    DacOverride,
    /// `CAP_DAC_READ_SEARCH`: Bypass file read permission checks and directory read/execute checks
    DacReadSearch,
    /// `CAP_FOWNER`: Bypass permission checks on operations that normally require the filesystem UID
    Fowner,
    /// `CAP_FSETID`: Don't clear set-user-ID and set-group-ID mode bits
    Fsetid,
    /// `CAP_KILL`: Bypass permission checks for sending signals
    Kill,
    /// `CAP_SETGID`: Make arbitrary manipulations of process GIDs and supplementary GID list
    Setgid,
    /// `CAP_SETUID`: Make arbitrary manipulations of process UIDs
    Setuid,
    /// `CAP_SETPCAP`: Modify process capabilities
    Setpcap,
    /// `CAP_NET_BIND_SERVICE`: Bind a socket to privileged ports (port numbers less than 1024)
    NetBindService,
    /// `CAP_NET_RAW`: Use RAW and PACKET sockets; bind to any address for transparent proxying
    NetRaw,
    /// `CAP_NET_ADMIN`: Perform various network-related operations
    NetAdmin,
    /// `CAP_SYS_CHROOT`: Use chroot()
    SysChroot,
    /// `CAP_SYS_PTRACE`: Trace arbitrary processes using ptrace()
    SysPtrace,
    /// `CAP_SYS_ADMIN`: Perform a range of system administration operations
    SysAdmin,
    /// `CAP_SYS_BOOT`: Use reboot() and kexec_load()
    SysBoot,
    /// `CAP_SYS_NICE`: Raise process nice value and change the nice value for arbitrary processes
    SysNice,
    /// `CAP_SYS_RESOURCE`: Override resource limits
    SysResource,
    /// `CAP_SYS_TIME`: Set system clock; set real-time (hardware) clock
    SysTime,
    /// `CAP_MKNOD`: Create special files using mknod()
    Mknod,
    /// `CAP_AUDIT_WRITE`: Write records to kernel auditing log
    AuditWrite,
    /// `CAP_AUDIT_CONTROL`: Enable and disable kernel auditing
    AuditControl,
    /// `CAP_SETFCAP`: Set file capabilities
    Setfcap,
}

impl LinuxCapability {
    /// return capability name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Chown => "CHOWN",
            Self::DacOverride => "DAC_OVERRIDE",
            Self::DacReadSearch => "DAC_READ_SEARCH",
            Self::Fowner => "FOWNER",
            Self::Fsetid => "FSETID",
            Self::Kill => "KILL",
            Self::Setgid => "SETGID",
            Self::Setuid => "SETUID",
            Self::Setpcap => "SETPCAP",
            Self::NetBindService => "NET_BIND_SERVICE",
            Self::NetRaw => "NET_RAW",
            Self::NetAdmin => "NET_ADMIN",
            Self::SysChroot => "SYS_CHROOT",
            Self::SysPtrace => "SYS_PTRACE",
            Self::SysAdmin => "SYS_ADMIN",
            Self::SysBoot => "SYS_BOOT",
            Self::SysNice => "SYS_NICE",
            Self::SysResource => "SYS_RESOURCE",
            Self::SysTime => "SYS_TIME",
            Self::Mknod => "MKNOD",
            Self::AuditWrite => "AUDIT_WRITE",
            Self::AuditControl => "AUDIT_CONTROL",
            Self::Setfcap => "SETFCAP",
        }
    }

    /// Numeric value as in `<linux/capability.h>`.
    pub(crate) fn to_cap_value(self) -> u32 {
        match self {
            Self::Chown => 0,           // CAP_CHOWN
            Self::DacOverride => 1,     // CAP_DAC_OVERRIDE
            Self::DacReadSearch => 2,   // CAP_DAC_READ_SEARCH
            Self::Fowner => 3,          // CAP_FOWNER
            Self::Fsetid => 4,          // CAP_FSETID
            Self::Kill => 5,            // CAP_KILL
            Self::Setgid => 6,          // CAP_SETGID
            Self::Setuid => 7,          // CAP_SETUID
            Self::Setpcap => 8,         // CAP_SETPCAP
            Self::NetBindService => 10, // CAP_NET_BIND_SERVICE
            Self::NetRaw => 13,         // CAP_NET_RAW
            Self::NetAdmin => 12,       // CAP_NET_ADMIN
            Self::SysChroot => 18,      // CAP_SYS_CHROOT
            Self::SysPtrace => 19,      // CAP_SYS_PTRACE
            Self::SysAdmin => 21,       // CAP_SYS_ADMIN
            Self::SysBoot => 22,        // CAP_SYS_BOOT
            Self::SysNice => 23,        // CAP_SYS_NICE
            Self::SysResource => 24,    // CAP_SYS_RESOURCE
            Self::SysTime => 25,        // CAP_SYS_TIME
            Self::Mknod => 27,          // CAP_MKNOD
            Self::AuditWrite => 29,     // CAP_AUDIT_WRITE
            Self::AuditControl => 30,   // CAP_AUDIT_CONTROL
            Self::Setfcap => 31,        // CAP_SETFCAP
        }
    }
}
