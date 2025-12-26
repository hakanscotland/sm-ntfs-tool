//
//  DiskRowView.swift
//  SM-NTFS Disk Manager
//
//  Individual disk row in the list.
//

import SwiftUI

struct DiskRowView: View {
    let disk: Disk

    var body: some View {
        HStack(spacing: 12) {
            // Disk icon
            Image(systemName: "externaldrive.fill")
                .font(.title2)
                .foregroundStyle(disk.isMounted ? .blue : .secondary)

            VStack(alignment: .leading, spacing: 4) {
                // Volume name
                Text(disk.volumeLabel.isEmpty ? "Untitled" : disk.volumeLabel)
                    .font(.headline)

                // Device path
                Text(disk.devicePath)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }

            Spacer()

            // Status
            if disk.isMounted {
                Text("Mounted")
                    .font(.caption)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(.green.opacity(0.2))
                    .foregroundStyle(.green)
                    .cornerRadius(4)
            }
        }
        .padding(.vertical, 8)
    }
}
