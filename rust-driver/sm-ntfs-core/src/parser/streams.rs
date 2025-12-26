//! NTFS data streams handling

use ntfs::{NtfsAttributeType, NtfsFile, NtfsReadSeek};
use std::io::{Read, Seek};
use crate::utils::error::{Result, SMNtfsError};

/// Information about a data stream
#[derive(Debug, Clone)]
pub struct StreamInfo {
    /// Stream name (empty for default stream)
    pub name: String,

    /// Stream size
    pub size: u64,

    /// Allocated size
    pub allocated_size: u64,
}

/// Get all data streams for a file
pub fn list_streams<T: Read + Seek>(
    file: &NtfsFile,
    fs: &mut T,
) -> Result<Vec<StreamInfo>> {
    let mut streams = Vec::new();

    // Iterate through all attributes
    let mut attributes = file.attributes();

    while let Some(attr_item_result) = attributes.next(fs) {
        let attr_item = attr_item_result
            .map_err(|e| SMNtfsError::ReadError(format!("Failed to read attribute: {}", e)))?;

        let attr = attr_item
            .to_attribute()
            .map_err(|e| SMNtfsError::ReadError(format!("Failed to get attribute: {}", e)))?;

        // We're only interested in DATA attributes
        if !matches!(attr.ty(), Ok(NtfsAttributeType::Data)) {
            continue;
        }

        let name = match attr.name() {
            Ok(name) => name.to_string_lossy(),
            Err(_) => String::new(), // Default stream has no name
        };

        let size = attr.value_length();
        let allocated_size = attr.value_length();

        streams.push(StreamInfo {
            name,
            size,
            allocated_size,
        });
    }

    Ok(streams)
}

/// Read data from the default stream
pub fn read_default_stream<T: Read + Seek>(
    file: &NtfsFile,
    fs: &mut T,
) -> Result<Vec<u8>> {
    read_stream(file, fs, "")
}

/// Read data from a named stream
pub fn read_named_stream<T: Read + Seek>(
    file: &NtfsFile,
    fs: &mut T,
    stream_name: &str,
) -> Result<Vec<u8>> {
    read_stream(file, fs, stream_name)
}

/// Internal helper to read a stream
fn read_stream<T: Read + Seek>(
    file: &NtfsFile,
    fs: &mut T,
    stream_name: &str,
) -> Result<Vec<u8>> {
    // Get the DATA attribute
    let data_item = match file.data(fs, stream_name) {
        Some(item) => item,
        None => {
            let msg = if stream_name.is_empty() {
                "No default data stream found".to_string()
            } else {
                format!("Stream '{}' not found", stream_name)
            };
            return Err(SMNtfsError::ReadError(msg));
        }
    };

    let data_item = data_item
        .map_err(|e| SMNtfsError::ReadError(format!("Failed to get data attribute: {}", e)))?;

    let data_attr = data_item
        .to_attribute()
        .map_err(|e| SMNtfsError::ReadError(format!("Failed to get attribute: {}", e)))?;

    let mut data_value = data_attr
        .value(fs)
        .map_err(|e| SMNtfsError::ReadError(format!("Failed to get data value: {}", e)))?;

    let size = data_value.len();
    let mut data = vec![0u8; size as usize];
    let mut buf = [0u8; 4096];
    let mut pos = 0;

    loop {
        let bytes_read = data_value
            .read(fs, &mut buf)
            .map_err(|e| SMNtfsError::ReadError(format!("Failed to read data: {}", e)))?;

        if bytes_read == 0 {
            break;
        }

        data[pos..pos + bytes_read].copy_from_slice(&buf[..bytes_read]);
        pos += bytes_read;
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_info_creation() {
        let stream = StreamInfo {
            name: String::new(), // Default stream
            size: 1024,
            allocated_size: 4096,
        };

        assert!(stream.name.is_empty());
        assert_eq!(stream.size, 1024);
    }

    #[test]
    fn test_named_stream_info() {
        let stream = StreamInfo {
            name: "Zone.Identifier".to_string(),
            size: 26,
            allocated_size: 512,
        };

        assert!(!stream.name.is_empty());
        assert_eq!(stream.name, "Zone.Identifier");
    }
}
