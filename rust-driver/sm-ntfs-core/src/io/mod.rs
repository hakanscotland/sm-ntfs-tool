//! Block I/O operations for NTFS devices
//!
//! This module provides low-level device access and I/O operations.

pub mod device;
pub mod buffer;
pub mod sync;

pub use device::BlockDevice;
