use thiserror::Error;

#[derive(Error, Debug)]
/// Error type used by this crate
pub enum RsysError {
    #[error("Failed to parse command output - `{0}`")]
    CommandParseError(String),
    #[error("Failed to read a file at `{0}` - `{1}`")]
    FileReadError(String, String),
    #[error("Failed to acquire local time - `{0}`")]
    TimeError(String),
    #[error("Failed to parse value from input `{0}` - `{1}`")]
    InvalidInputError(String, String),
    #[error("Failed to serialize `{0}` - `{1}`")]
    SerializeError(String, String),
    #[cfg(unix)]
    #[error("Syscall failed - `{0}`")]
    NixSyscallError(#[from] nix::Error),
    #[cfg(unix)]
    #[error("Sysctl failed - `{0}`")]
    SysctlError(#[from] sysctl::SysctlError),
    #[error("Unexpected sysctl value `{0:?}`")]
    UnexpectedSysctlValue(sysctl::CtlValue),
    #[cfg(target_os = "macos")]
    #[error("Invalid sysctl property `{0}`")]
    InvalidSysctlProp(String),

    // Windows
    #[cfg(target_os = "windows")]
    #[error("Failed to communicate with Win32 api. Error code: `{0}`, Reason: `{1}`")]
    WinApiError(u32, String),
}

/// Helper result definition for less repetition in type signatures
/// used throughout this crate
pub type RsysResult<T> = std::result::Result<T, RsysError>;
