//! SM-NTFS Core Library
//!
//! This library provides the core functionality for NTFS filesystem operations,
//! including parsing, caching, and low-level I/O.

pub mod io;
pub mod ntfs;
pub mod cache;
pub mod parser;
pub mod utils;
pub mod ffi;

// Re-export commonly used types
pub use utils::{SMNtfsError, Result};
pub use utils::config::Config;
pub use io::BlockDevice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_types() {
        let err = SMNtfsError::DeviceNotFound("test".to_string());
        assert!(err.user_message().contains("test"));
    }
}
