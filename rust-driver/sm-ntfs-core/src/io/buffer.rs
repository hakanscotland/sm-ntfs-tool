//! I/O buffering for optimized read/write operations

use std::collections::VecDeque;
use crate::utils::error::{Result, SMNtfsError};

/// Default buffer size (128 KB)
const DEFAULT_BUFFER_SIZE: usize = 128 * 1024;

/// Buffer for optimizing I/O operations
pub struct IOBuffer {
    /// Internal buffer storage
    buffer: VecDeque<u8>,

    /// Maximum buffer size
    max_size: usize,

    /// Current offset in the device
    offset: u64,
}

impl IOBuffer {
    /// Create a new I/O buffer with default size
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_SIZE)
    }

    /// Create a new I/O buffer with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            max_size: capacity,
            offset: 0,
        }
    }

    /// Write data to the buffer
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let available = self.max_size - self.buffer.len();

        if data.len() > available {
            return Err(SMNtfsError::SystemError(
                "Buffer overflow".to_string()
            ));
        }

        self.buffer.extend(data.iter());
        Ok(data.len())
    }

    /// Read data from the buffer
    pub fn read(&mut self, size: usize) -> Vec<u8> {
        let actual_size = size.min(self.buffer.len());
        self.buffer.drain(..actual_size).collect()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.max_size
    }

    /// Get current buffer size
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Get available space in buffer
    pub fn available(&self) -> usize {
        self.max_size - self.buffer.len()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get the current offset
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Set the offset
    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }
}

impl Default for IOBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_write_read() {
        let mut buffer = IOBuffer::new();
        let data = vec![1, 2, 3, 4, 5];

        buffer.write(&data).unwrap();
        assert_eq!(buffer.len(), 5);

        let read_data = buffer.read(3);
        assert_eq!(read_data, vec![1, 2, 3]);
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buffer = IOBuffer::with_capacity(10);
        let large_data = vec![0u8; 20];

        let result = buffer.write(&large_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_buffer_clear() {
        let mut buffer = IOBuffer::new();
        buffer.write(&[1, 2, 3]).unwrap();

        assert!(!buffer.is_empty());
        buffer.clear();
        assert!(buffer.is_empty());
    }
}
