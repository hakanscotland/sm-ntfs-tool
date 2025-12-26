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

    /// Integration test demonstrating NTFS parser workflow
    /// This test shows how to use the parser components together
    #[test]
    fn test_ntfs_parser_integration() {
        use crate::parser::{BlockDeviceAdapter, NtfsVolume, FileInfo, list_directory};
        use crate::parser::{list_streams, read_default_stream};

        // This is a demonstration of the integration pattern.
        // In a real scenario, you would:
        //
        // 1. Open a block device:
        //    let device = BlockDevice::open("/path/to/ntfs/volume")?;
        //
        // 2. Adapt it for NTFS reading:
        //    let mut fs = BlockDeviceAdapter::new(device);
        //
        // 3. Parse NTFS structures:
        //    let mut ntfs = Ntfs::new(&mut fs)?;
        //    ntfs.read_upcase_table(&mut fs)?;
        //
        // 4. Create volume wrapper:
        //    let volume = NtfsVolume::new(&ntfs, &mut fs)?;
        //    println!("Volume: {}", volume.volume_name());
        //    println!("Serial: 0x{:X}", volume.serial_number());
        //
        // 5. Access root directory:
        //    let root = volume.root_directory(&mut fs)?;
        //    let entries = list_directory(&ntfs, &root, &mut fs)?;
        //
        //    for entry in entries {
        //        println!("{}: {} bytes", entry.name, entry.size);
        //
        //        // List alternate data streams
        //        let file = /* get NtfsFile for entry */;
        //        let streams = list_streams(&file, &mut fs)?;
        //
        //        // Read default stream
        //        let data = read_default_stream(&file, &mut fs)?;
        //    }
        //
        // NOTE: This test would require a real NTFS volume to work.
        // For unit testing, we test each component separately.

        // Verify the types are available and documented
        let _ = std::marker::PhantomData::<BlockDeviceAdapter>;
        let _ = std::marker::PhantomData::<NtfsVolume>;
        let _ = std::marker::PhantomData::<FileInfo>;
    }
}
