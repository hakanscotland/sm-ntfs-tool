//! Block device operations
//!
//! This module provides low-level I/O operations for accessing block devices.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::FileExt;
use std::path::Path;
use crate::utils::error::{Result, SMNtfsError};

/// Default sector size for NTFS (512 bytes)
pub const DEFAULT_SECTOR_SIZE: usize = 512;

/// Represents a block device for NTFS operations
pub struct BlockDevice {
    file: File,
    block_size: usize,
    device_size: u64,
    read_only: bool,
}

impl BlockDevice {
    /// Open a block device at the given path in read-only mode
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::open_with_options(path, true)
    }

    /// Open a block device with specified read/write mode
    pub fn open_with_options<P: AsRef<Path>>(path: P, read_only: bool) -> Result<Self> {
        let path = path.as_ref();

        tracing::debug!("Opening device: {:?} (read_only: {})", path, read_only);

        let file = OpenOptions::new()
            .read(true)
            .write(!read_only)
            .open(path)
            .map_err(|e| {
                tracing::error!("Failed to open device {:?}: {}", path, e);
                SMNtfsError::DeviceNotFound(format!("{}: {}", path.display(), e))
            })?;

        // Get device size
        let device_size = file.metadata()
            .map_err(|e| SMNtfsError::SystemError(format!("Failed to get device metadata: {}", e)))?
            .len();

        tracing::info!(
            "Device opened: {:?}, size: {} bytes ({} MB)",
            path,
            device_size,
            device_size / 1024 / 1024
        );

        Ok(Self {
            file,
            block_size: DEFAULT_SECTOR_SIZE,
            device_size,
            read_only,
        })
    }

    /// Read a block from the device at the specified block number
    pub fn read_block(&self, block: u64) -> Result<Vec<u8>> {
        let offset = block * self.block_size as u64;
        self.read_at(offset, self.block_size)
    }

    /// Read multiple blocks starting from the specified block number
    pub fn read_blocks(&self, start_block: u64, count: usize) -> Result<Vec<u8>> {
        let offset = start_block * self.block_size as u64;
        let size = count * self.block_size;
        self.read_at(offset, size)
    }

    /// Read data at a specific offset
    pub fn read_at(&self, offset: u64, size: usize) -> Result<Vec<u8>> {
        // Validate offset
        if offset >= self.device_size {
            return Err(SMNtfsError::ReadError(format!(
                "Offset {} exceeds device size {}",
                offset, self.device_size
            )));
        }

        let mut buffer = vec![0u8; size];

        #[cfg(unix)]
        {
            // Use pread for thread-safe reading without seeking
            self.file
                .read_exact_at(&mut buffer, offset)
                .map_err(|e| SMNtfsError::ReadError(format!("Failed to read at offset {}: {}", offset, e)))?;
        }

        #[cfg(not(unix))]
        {
            // Fallback for non-Unix systems
            let mut file = &self.file;
            file.seek(SeekFrom::Start(offset))
                .map_err(|e| SMNtfsError::ReadError(format!("Failed to seek to offset {}: {}", offset, e)))?;
            file.read_exact(&mut buffer)
                .map_err(|e| SMNtfsError::ReadError(format!("Failed to read at offset {}: {}", offset, e)))?;
        }

        tracing::trace!("Read {} bytes at offset {}", size, offset);

        Ok(buffer)
    }

    /// Write a block to the device at the specified block number
    pub fn write_block(&mut self, block: u64, data: &[u8]) -> Result<()> {
        if self.read_only {
            return Err(SMNtfsError::PermissionDenied(
                "Device opened in read-only mode".to_string()
            ));
        }

        if data.len() != self.block_size {
            return Err(SMNtfsError::WriteError(format!(
                "Data size {} does not match block size {}",
                data.len(),
                self.block_size
            )));
        }

        let offset = block * self.block_size as u64;
        self.write_at(offset, data)
    }

    /// Write data at a specific offset
    pub fn write_at(&mut self, offset: u64, data: &[u8]) -> Result<()> {
        if self.read_only {
            return Err(SMNtfsError::PermissionDenied(
                "Device opened in read-only mode".to_string()
            ));
        }

        // Validate offset
        if offset >= self.device_size {
            return Err(SMNtfsError::WriteError(format!(
                "Offset {} exceeds device size {}",
                offset, self.device_size
            )));
        }

        #[cfg(unix)]
        {
            // Use pwrite for thread-safe writing without seeking
            self.file
                .write_all_at(data, offset)
                .map_err(|e| SMNtfsError::WriteError(format!("Failed to write at offset {}: {}", offset, e)))?;
        }

        #[cfg(not(unix))]
        {
            // Fallback for non-Unix systems
            let mut file = &self.file;
            file.seek(SeekFrom::Start(offset))
                .map_err(|e| SMNtfsError::WriteError(format!("Failed to seek to offset {}: {}", offset, e)))?;
            file.write_all(data)
                .map_err(|e| SMNtfsError::WriteError(format!("Failed to write at offset {}: {}", offset, e)))?;
        }

        tracing::trace!("Wrote {} bytes at offset {}", data.len(), offset);

        Ok(())
    }

    /// Flush all pending writes to disk
    pub fn flush(&mut self) -> Result<()> {
        if self.read_only {
            return Ok(()); // Nothing to flush in read-only mode
        }

        self.file
            .sync_all()
            .map_err(|e| SMNtfsError::FlushFailed(format!("Failed to sync device: {}", e)))?;

        tracing::debug!("Device flushed successfully");

        Ok(())
    }

    /// Get the block size
    pub fn block_size(&self) -> usize {
        self.block_size
    }

    /// Get the total device size in bytes
    pub fn device_size(&self) -> u64 {
        self.device_size
    }

    /// Get the number of blocks on the device
    pub fn block_count(&self) -> u64 {
        self.device_size / self.block_size as u64
    }

    /// Check if the device is opened in read-only mode
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
}

impl Drop for BlockDevice {
    fn drop(&mut self) {
        if !self.read_only {
            // Attempt to flush on drop, but don't panic if it fails
            if let Err(e) = self.flush() {
                tracing::warn!("Failed to flush device on drop: {}", e);
            }
        }
        tracing::debug!("Block device closed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_open_device() {
        // Create a temporary file to act as a device
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&vec![0u8; 4096]).unwrap();
        temp.flush().unwrap();

        let device = BlockDevice::open(temp.path()).unwrap();
        assert_eq!(device.block_size(), DEFAULT_SECTOR_SIZE);
        assert_eq!(device.device_size(), 4096);
        assert!(device.is_read_only());
    }

    #[test]
    fn test_read_block() {
        let mut temp = NamedTempFile::new().unwrap();
        // Write test data
        let test_data = vec![0xAB; 512];
        temp.write_all(&test_data).unwrap();
        temp.flush().unwrap();

        let device = BlockDevice::open(temp.path()).unwrap();
        let block_data = device.read_block(0).unwrap();

        assert_eq!(block_data.len(), 512);
        assert_eq!(block_data[0], 0xAB);
    }

    #[test]
    fn test_read_write() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&vec![0u8; 4096]).unwrap();
        temp.flush().unwrap();

        let mut device = BlockDevice::open_with_options(temp.path(), false).unwrap();

        // Write a block
        let write_data = vec![0xCD; 512];
        device.write_block(0, &write_data).unwrap();
        device.flush().unwrap();

        // Read it back
        let read_data = device.read_block(0).unwrap();
        assert_eq!(read_data, write_data);
    }

    #[test]
    fn test_read_only_write_fails() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&vec![0u8; 512]).unwrap();
        temp.flush().unwrap();

        let mut device = BlockDevice::open(temp.path()).unwrap();
        let result = device.write_block(0, &vec![0xFF; 512]);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SMNtfsError::PermissionDenied(_)));
    }
}
