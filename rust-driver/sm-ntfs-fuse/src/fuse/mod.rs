//! FUSE filesystem implementation

use sm_ntfs_core::Config;

/// SM-NTFS FUSE Filesystem
pub struct SMNtfsFilesystem {
    config: Config,
}

impl SMNtfsFilesystem {
    /// Create a new FUSE filesystem instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// TODO: Implement fuser::Filesystem trait
// This will be implemented in Week 2
