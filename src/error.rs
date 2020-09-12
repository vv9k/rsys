use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RsysError {
    #[error("Failed to execute command - `{0}`")]
    CommandRunError(String),
    #[error("Failed to parse command output - `{0}`")]
    CommandParseError(String),
    #[error("Failed to read a file at `{0}` - `{1}`")]
    FileReadError(String, String),
    #[error("Failed to acquire local time - `{0}`")]
    TimeError(String),
    #[error("Failed to parse value from input `{0}` - `{1}`")]
    InvalidInputError(String, String),

    // Windows
    #[cfg(target_os = "windows")]
    #[error("Failed to communicate with Win32 api. Error code: `{0}`, Reason: `{1}`")]
    WinApiError(u32, String),
}
