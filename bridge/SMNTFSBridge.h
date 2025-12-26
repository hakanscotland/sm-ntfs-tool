/*
 * SMNTFSBridge.h
 * SM-NTFS Tool
 *
 * C-FFI Bridge between Swift and Rust
 */

#ifndef SMNTFS_BRIDGE_H
#define SMNTFS_BRIDGE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque context type
typedef struct SMNtfsContext SMNtfsContext;

// Disk information structure
typedef struct {
    char device_path[256];
    char volume_label[128];
    uint64_t total_size;
    uint64_t free_size;
    bool is_mounted;
    bool is_writable;
} SMDiskInfo;

// Error codes
typedef enum {
    SM_SUCCESS = 0,
    SM_ERROR_INVALID_DEVICE = -1,
    SM_ERROR_MOUNT_FAILED = -2,
    SM_ERROR_PERMISSION_DENIED = -3,
    SM_ERROR_ALREADY_MOUNTED = -4,
    SM_ERROR_NOT_NTFS = -5,
    SM_ERROR_DEVICE_NOT_FOUND = -6,
} SMErrorCode;

// ============================================================================
// MARK: - Initialization
// ============================================================================

/**
 * Initialize SM-NTFS context
 * @return Opaque context pointer, or NULL on failure
 */
SMNtfsContext* sm_ntfs_init(void);

/**
 * Destroy SM-NTFS context and free resources
 * @param ctx Context to destroy
 */
void sm_ntfs_destroy(SMNtfsContext* ctx);

// ============================================================================
// MARK: - Disk Discovery
// ============================================================================

/**
 * Scan for NTFS disks
 * @param ctx Context
 * @param disks Output array of disk info (caller must free with sm_ntfs_free_disk_list)
 * @param count Output count of disks found
 * @return Error code
 */
SMErrorCode sm_ntfs_scan_disks(
    SMNtfsContext* ctx,
    SMDiskInfo** disks,
    int* count
);

/**
 * Free disk list returned by sm_ntfs_scan_disks
 * @param disks Disk array to free
 */
void sm_ntfs_free_disk_list(SMDiskInfo* disks);

// ============================================================================
// MARK: - Mount Operations
// ============================================================================

/**
 * Mount an NTFS volume
 * @param ctx Context
 * @param device_path Device path (e.g., "/dev/disk2s1")
 * @param mount_point Mount point directory
 * @param read_write Enable read-write mode
 * @return Error code
 */
SMErrorCode sm_ntfs_mount(
    SMNtfsContext* ctx,
    const char* device_path,
    const char* mount_point,
    bool read_write
);

/**
 * Unmount an NTFS volume
 * @param ctx Context
 * @param mount_point Mount point directory
 * @return Error code
 */
SMErrorCode sm_ntfs_unmount(
    SMNtfsContext* ctx,
    const char* mount_point
);

// ============================================================================
// MARK: - Status Queries
// ============================================================================

/**
 * Check if a device is mounted
 * @param ctx Context
 * @param device_path Device path
 * @return true if mounted, false otherwise
 */
bool sm_ntfs_is_mounted(
    SMNtfsContext* ctx,
    const char* device_path
);

/**
 * Get disk status information
 * @param ctx Context
 * @param device_path Device path
 * @param info Output disk info
 * @return Error code
 */
SMErrorCode sm_ntfs_get_status(
    SMNtfsContext* ctx,
    const char* device_path,
    SMDiskInfo* info
);

/**
 * Get last error message
 * @param ctx Context
 * @return Error message string (valid until next call)
 */
const char* sm_ntfs_last_error(SMNtfsContext* ctx);

#ifdef __cplusplus
}
#endif

#endif // SMNTFS_BRIDGE_H
