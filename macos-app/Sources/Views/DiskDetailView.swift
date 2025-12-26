//
//  DiskDetailView.swift
//  SM-NTFS Disk Manager
//
//  Detailed view of a selected disk.
//

import SwiftUI

struct DiskDetailView: View {
    let disk: Disk

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                Text(disk.volumeLabel.isEmpty ? "Untitled" : disk.volumeLabel)
                    .font(.title)

                Divider()

                VStack(alignment: .leading, spacing: 8) {
                    InfoRow(label: "Device", value: disk.devicePath)
                    InfoRow(label: "File System", value: "NTFS")
                    InfoRow(label: "Status", value: disk.isMounted ? "Mounted" : "Not Mounted")
                }

                Divider()

                // TODO: Add mount/unmount button
                Text("TODO: Add mount controls")
                    .foregroundStyle(.secondary)
            }
            .padding()
        }
    }
}

struct InfoRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .foregroundStyle(.secondary)
            Spacer()
            Text(value)
        }
    }
}
