//
//  DiskViewModel.swift
//  SM-NTFS Disk Manager
//
//  ViewModel for disk operations.
//

import SwiftUI

@MainActor
class DiskViewModel: ObservableObject {
    @Published var disks: [Disk] = []
    @Published var isLoading = false
    @Published var error: Error?

    private let diskManager = DiskManager.shared

    func refresh() async {
        isLoading = true
        defer { isLoading = false }

        do {
            // TODO: Implement actual disk scanning via FFI
            // For now, use mock data
            disks = [
                Disk(
                    id: UUID(),
                    devicePath: "/dev/disk2s1",
                    volumeLabel: "NTFS Volume",
                    isMounted: false
                )
            ]
        } catch {
            self.error = error
        }
    }

    func mount(_ disk: Disk, readWrite: Bool) async throws {
        // TODO: Implement mount via FFI/XPC
        print("TODO: Mount \(disk.devicePath)")
    }

    func unmount(_ disk: Disk) async throws {
        // TODO: Implement unmount via FFI/XPC
        print("TODO: Unmount \(disk.devicePath)")
    }

    func unmountAll() async {
        for disk in disks where disk.isMounted {
            try? await unmount(disk)
        }
    }
}
