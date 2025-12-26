//! Configuration management

use serde::{Deserialize, Serialize};

/// Configuration for SM-NTFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Cache size in megabytes
    pub cache_size_mb: usize,

    /// Write-back buffer size in megabytes
    pub write_buffer_size_mb: usize,

    /// Enable read-ahead
    pub enable_read_ahead: bool,

    /// Enable write coalescing
    pub enable_write_coalescing: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_size_mb: 64,
            write_buffer_size_mb: 32,
            enable_read_ahead: true,
            enable_write_coalescing: true,
        }
    }
}
