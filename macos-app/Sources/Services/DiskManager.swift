//
//  DiskManager.swift
//  SM-NTFS Disk Manager
//
//  Core disk management service.
//

import Foundation

class DiskManager {
    static let shared = DiskManager()

    private init() {}

    func scanDisks() async throws -> [Disk] {
        // TODO: Implement via FFI bridge
        // Call Rust sm_ntfs_scan_disks() function
        return []
    }

    func mount(devicePath: String, readWrite: Bool) async throws {
        // TODO: Implement via XPC/FFI
        print("Mounting \(devicePath) with readWrite: \(readWrite)")
    }

    func unmount(mountPoint: String) async throws {
        // TODO: Implement via XPC/FFI
        print("Unmounting \(mountPoint)")
    }
}
