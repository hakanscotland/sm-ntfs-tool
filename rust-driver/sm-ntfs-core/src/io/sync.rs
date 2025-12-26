//! Sync and flush operations for data integrity

use std::time::Duration;
use tokio::time::sleep;
use crate::utils::error::Result;

/// Sync policy for write operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncPolicy {
    /// Sync immediately after every write
    Immediate,

    /// Sync periodically (every N seconds)
    Periodic(Duration),

    /// Sync only on explicit flush
    Manual,
}

impl Default for SyncPolicy {
    fn default() -> Self {
        Self::Periodic(Duration::from_secs(5))
    }
}

/// Sync manager for coordinating flush operations
pub struct SyncManager {
    policy: SyncPolicy,
    last_sync: Option<std::time::Instant>,
    pending_writes: usize,
}

impl SyncManager {
    /// Create a new sync manager with default policy
    pub fn new() -> Self {
        Self::with_policy(SyncPolicy::default())
    }

    /// Create a new sync manager with specific policy
    pub fn with_policy(policy: SyncPolicy) -> Self {
        Self {
            policy,
            last_sync: None,
            pending_writes: 0,
        }
    }

    /// Record a write operation
    pub fn record_write(&mut self) {
        self.pending_writes += 1;
    }

    /// Check if sync is needed based on policy
    pub fn needs_sync(&self) -> bool {
        match self.policy {
            SyncPolicy::Immediate => self.pending_writes > 0,
            SyncPolicy::Periodic(duration) => {
                if self.pending_writes == 0 {
                    return false;
                }

                match self.last_sync {
                    Some(last) => last.elapsed() >= duration,
                    None => true,
                }
            }
            SyncPolicy::Manual => false,
        }
    }

    /// Mark sync as completed
    pub fn mark_synced(&mut self) {
        self.last_sync = Some(std::time::Instant::now());
        self.pending_writes = 0;
    }

    /// Get pending write count
    pub fn pending_writes(&self) -> usize {
        self.pending_writes
    }

    /// Reset pending writes
    pub fn reset(&mut self) {
        self.pending_writes = 0;
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Async sync helper for periodic flushing
pub async fn periodic_sync_loop<F>(
    interval: Duration,
    mut sync_fn: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    loop {
        sleep(interval).await;

        if let Err(e) = sync_fn() {
            tracing::warn!("Periodic sync failed: {}", e);
            // Continue despite errors
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_sync() {
        let mut mgr = SyncManager::with_policy(SyncPolicy::Immediate);

        assert!(!mgr.needs_sync());

        mgr.record_write();
        assert!(mgr.needs_sync());

        mgr.mark_synced();
        assert!(!mgr.needs_sync());
    }

    #[test]
    fn test_periodic_sync() {
        let mut mgr = SyncManager::with_policy(
            SyncPolicy::Periodic(Duration::from_millis(10))
        );

        mgr.record_write();

        // Should need sync after duration
        std::thread::sleep(Duration::from_millis(20));
        assert!(mgr.needs_sync());
    }

    #[test]
    fn test_manual_sync() {
        let mut mgr = SyncManager::with_policy(SyncPolicy::Manual);

        mgr.record_write();
        mgr.record_write();

        // Manual policy never auto-syncs
        assert!(!mgr.needs_sync());
        assert_eq!(mgr.pending_writes(), 2);
    }
}
