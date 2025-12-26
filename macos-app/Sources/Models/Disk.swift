//
//  Disk.swift
//  SM-NTFS Disk Manager
//
//  Disk model representing an NTFS volume.
//

import Foundation

struct Disk: Identifiable, Hashable {
    let id: UUID
    let devicePath: String
    let volumeLabel: String
    let isMounted: Bool

    // TODO: Add more properties
    // - totalSize: UInt64
    // - freeSize: UInt64
    // - mountPoint: String?
    // - isWritable: Bool
}
