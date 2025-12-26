//! MFT (Master File Table) operations

use ntfs::{Ntfs, NtfsFile};
use ntfs::structured_values::NtfsFileNamespace;
use std::io::{Read, Seek};
use crate::utils::error::{Result, SMNtfsError};

/// Information about a file/directory entry
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// File name
    pub name: String,

    /// File size in bytes
    pub size: u64,

    /// Is this a directory?
    pub is_directory: bool,

    /// File record number (inode equivalent)
    pub record_number: u64,

    /// Allocated size (may be larger than actual size)
    pub allocated_size: u64,
}

impl FileInfo {
    /// Create FileInfo from NtfsFile
    pub fn from_ntfs_file<T: Read + Seek>(
        file: &NtfsFile,
        fs: &mut T,
        parent_record_number: Option<u64>,
    ) -> Result<Self> {
        // Get file name - try Win32 first, fall back to any namespace
        let file_name = if let Some(name_result) = file.name(fs, Some(NtfsFileNamespace::Win32), parent_record_number) {
            name_result
                .map_err(|_| SMNtfsError::CorruptedMft {
                    offset: file.file_record_number(),
                })?
                .name()
                .to_string_lossy()
        } else if let Some(name_result) = file.name(fs, None, parent_record_number) {
            name_result
                .map_err(|_| SMNtfsError::CorruptedMft {
                    offset: file.file_record_number(),
                })?
                .name()
                .to_string_lossy()
        } else {
            return Err(SMNtfsError::CorruptedMft {
                offset: file.file_record_number(),
            });
        };

        // Get file information from file directly
        let size = file.data_size() as u64;
        let allocated_size = file.allocated_size() as u64;
        let is_directory = file.is_directory();

        Ok(Self {
            name: file_name,
            size,
            is_directory,
            record_number: file.file_record_number(),
            allocated_size,
        })
    }

    /// Check if this is a system file (starts with $)
    pub fn is_system_file(&self) -> bool {
        self.name.starts_with('$')
    }
}

/// List directory contents
pub fn list_directory<T: Read + Seek>(
    ntfs: &Ntfs,
    directory: &NtfsFile,
    fs: &mut T,
) -> Result<Vec<FileInfo>> {
    let index = directory
        .directory_index(fs)
        .map_err(|e| SMNtfsError::ReadError(format!("Failed to get directory index: {}", e)))?;

    let mut entries = Vec::new();
    let parent_record_number = directory.file_record_number();

    let mut iter = index.entries();
    while let Some(entry_result) = iter.next(fs) {
        let entry = entry_result
            .map_err(|e| SMNtfsError::ReadError(format!("Failed to read directory entry: {}", e)))?;

        // Get the file name from the entry key
        let file_name = match entry.key() {
            Some(Ok(name)) => name,
            _ => continue,
        };

        // Skip empty names
        if file_name.name().len() == 0 {
            continue;
        }

        // Get the file for this entry
        if let Ok(file) = entry.to_file(ntfs, fs) {
            if let Ok(info) = FileInfo::from_ntfs_file(&file, fs, Some(parent_record_number)) {
                entries.push(info);
            }
        }
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_info_system_file() {
        let info = FileInfo {
            name: "$MFT".to_string(),
            size: 1024,
            is_directory: false,
            record_number: 0,
            allocated_size: 4096,
        };

        assert!(info.is_system_file());
    }

    #[test]
    fn test_file_info_regular_file() {
        let info = FileInfo {
            name: "document.txt".to_string(),
            size: 2048,
            is_directory: false,
            record_number: 100,
            allocated_size: 4096,
        };

        assert!(!info.is_system_file());
        assert!(!info.is_directory);
    }
}
