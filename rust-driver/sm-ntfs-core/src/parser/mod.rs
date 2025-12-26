//! NTFS parser wrapper module
//!
//! This module provides high-level wrappers around the ntfs crate
//! for parsing and accessing NTFS filesystem structures.

pub mod volume;
pub mod mft;
pub mod streams;

pub use volume::{NtfsVolume, BlockDeviceAdapter};
pub use mft::{FileInfo, list_directory};
pub use streams::{StreamInfo, list_streams, read_default_stream, read_named_stream};
