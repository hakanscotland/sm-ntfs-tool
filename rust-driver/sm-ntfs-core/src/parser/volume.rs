//! NTFS Volume operations and wrapper

use ntfs::{Ntfs, NtfsFile};
use std::io::{Read, Seek, SeekFrom};
use crate::io::BlockDevice;
use crate::utils::error::{Result, SMNtfsError};

/// Wrapper around ntfs::Ntfs for easier volume operations
pub struct NtfsVolume<'n> {
    ntfs: &'n Ntfs,
    volume_name: String,
    serial_number: u64,
}

impl<'n> NtfsVolume<'n> {
    /// Create a new NTFS volume from parsed ntfs data
    pub fn new<T: Read + Seek>(ntfs: &'n Ntfs, fs: &mut T) -> Result<Self> {
        // Get volume name
        let volume_name = if let Some(Ok(name_attr)) = ntfs.volume_name(fs) {
            name_attr.name().to_string_lossy()
        } else {
            String::new()
        };

        // Serial number is directly available
        let serial_number = ntfs.serial_number();

        tracing::info!(
            "NTFS Volume initialized - Name: '{}', Serial: 0x{:X}",
            volume_name,
            serial_number
        );

        Ok(Self {
            ntfs,
            volume_name,
            serial_number,
        })
    }

    /// Get the root directory
    pub fn root_directory<T: Read + Seek>(&self, fs: &mut T) -> Result<NtfsFile<'n>> {
        self.ntfs
            .root_directory(fs)
            .map_err(|e| SMNtfsError::InvalidNtfs(format!("Failed to get root directory: {}", e)))
    }

    /// Get volume serial number
    pub fn serial_number(&self) -> u64 {
        self.serial_number
    }

    /// Get volume name
    pub fn volume_name(&self) -> &str {
        &self.volume_name
    }

    /// Get cluster size
    pub fn cluster_size(&self) -> u32 {
        self.ntfs.cluster_size()
    }

    /// Get sector size
    pub fn sector_size(&self) -> u16 {
        self.ntfs.sector_size()
    }

    /// Get total size of the volume
    pub fn size(&self) -> u64 {
        self.ntfs.size()
    }
}

/// Helper struct to adapt BlockDevice to NtfsReadSeek
pub struct BlockDeviceAdapter {
    device: BlockDevice,
    position: u64,
}

impl BlockDeviceAdapter {
    /// Create a new adapter from a block device
    pub fn new(device: BlockDevice) -> Self {
        Self {
            device,
            position: 0,
        }
    }

    /// Get a reference to the underlying device
    pub fn device(&self) -> &BlockDevice {
        &self.device
    }

    /// Get a mutable reference to the underlying device
    pub fn device_mut(&mut self) -> &mut BlockDevice {
        &mut self.device
    }
}

impl Read for BlockDeviceAdapter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let data = self.device
            .read_at(self.position, buf.len())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let bytes_read = data.len().min(buf.len());
        buf[..bytes_read].copy_from_slice(&data[..bytes_read]);
        self.position += bytes_read as u64;

        Ok(bytes_read)
    }
}

impl Seek for BlockDeviceAdapter {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_position = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(offset) => {
                if offset >= 0 {
                    self.position + offset as u64
                } else {
                    self.position.saturating_sub(offset.unsigned_abs())
                }
            }
            SeekFrom::End(offset) => {
                let device_size = self.device.device_size();
                if offset >= 0 {
                    device_size + offset as u64
                } else {
                    device_size.saturating_sub(offset.unsigned_abs())
                }
            }
        };

        self.position = new_position;
        Ok(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_block_device_adapter_read() {
        let mut temp = NamedTempFile::new().unwrap();
        let test_data = b"Hello, NTFS!";
        temp.write_all(test_data).unwrap();
        temp.flush().unwrap();

        let device = BlockDevice::open(temp.path()).unwrap();
        let mut adapter = BlockDeviceAdapter::new(device);

        let mut buf = vec![0u8; 12];
        let bytes_read = adapter.read(&mut buf).unwrap();

        assert_eq!(bytes_read, 12);
        assert_eq!(&buf, test_data);
    }

    #[test]
    fn test_block_device_adapter_seek() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(b"0123456789").unwrap();
        temp.flush().unwrap();

        let device = BlockDevice::open(temp.path()).unwrap();
        let mut adapter = BlockDeviceAdapter::new(device);

        // Seek to position 5
        adapter.seek(SeekFrom::Start(5)).unwrap();
        assert_eq!(adapter.position, 5);

        // Seek forward by 2
        adapter.seek(SeekFrom::Current(2)).unwrap();
        assert_eq!(adapter.position, 7);

        // Seek backward by 3
        adapter.seek(SeekFrom::Current(-3)).unwrap();
        assert_eq!(adapter.position, 4);
    }
}
