//! Error types for SM-NTFS

use thiserror::Error;

/// Main error type for SM-NTFS operations
#[derive(Error, Debug)]
pub enum SMNtfsError {
    // I/O Errors
    #[error("Failed to read from device: {0}")]
    ReadError(String),

    #[error("Failed to write to device: {0}")]
    WriteError(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    // NTFS Errors
    #[error("Invalid NTFS volume: {0}")]
    InvalidNtfs(String),

    #[error("Corrupted MFT entry at offset {offset}")]
    CorruptedMft { offset: u64 },

    #[error("Journal replay failed: {0}")]
    JournalError(String),

    // Mount Errors
    #[error("Mount failed: {0}")]
    MountFailed(String),

    #[error("Already mounted at {0}")]
    AlreadyMounted(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    // Cache Errors
    #[error("Cache full")]
    CacheFull,

    #[error("Failed to flush write buffer: {0}")]
    FlushFailed(String),

    // System Errors
    #[error("System error: {0}")]
    SystemError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Result type alias for SM-NTFS operations
pub type Result<T> = std::result::Result<T, SMNtfsError>;

impl SMNtfsError {
    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Self::PermissionDenied(_) => {
                "Permission denied. Please grant Full Disk Access in System Settings.".to_string()
            }
            Self::DeviceNotFound(device) => {
                format!("Device '{}' not found. Is it connected?", device)
            }
            Self::InvalidNtfs(_) => {
                "This is not a valid NTFS volume.".to_string()
            }
            Self::AlreadyMounted(path) => {
                format!("Already mounted at '{}'", path)
            }
            _ => {
                format!("An error occurred: {}", self)
            }
        }
    }
}
