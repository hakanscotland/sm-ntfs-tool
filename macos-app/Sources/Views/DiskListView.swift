//
//  DiskListView.swift
//  SM-NTFS Disk Manager
//
//  List view showing all NTFS disks.
//

import SwiftUI

struct DiskListView: View {
    @EnvironmentObject var viewModel: DiskViewModel
    @Binding var selection: Disk?

    var body: some View {
        List(selection: $selection) {
            ForEach(viewModel.disks) { disk in
                DiskRowView(disk: disk)
                    .tag(disk)
            }
        }
        .listStyle(.inset)
        .overlay {
            if viewModel.isLoading {
                ProgressView("Scanning disks...")
            } else if viewModel.disks.isEmpty {
                ContentUnavailableView(
                    "No NTFS Disks Found",
                    systemImage: "externaldrive.badge.xmark",
                    description: Text("Connect an NTFS-formatted drive")
                )
            }
        }
    }
}
