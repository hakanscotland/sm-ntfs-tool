//
//  MainWindow.swift
//  SM-NTFS Disk Manager
//
//  Main window with disk list.
//

import SwiftUI

struct MainWindow: View {
    @EnvironmentObject var viewModel: DiskViewModel
    @State private var selectedDisk: Disk?

    var body: some View {
        NavigationSplitView {
            // Left: Sidebar
            DiskListView(selection: $selectedDisk)
                .frame(minWidth: 400)
        } detail: {
            // Right: Detail Panel
            if let disk = selectedDisk {
                DiskDetailView(disk: disk)
                    .frame(minWidth: 300)
            } else {
                ContentUnavailableView(
                    "No Disk Selected",
                    systemImage: "externaldrive",
                    description: Text("Select a disk to view details")
                )
            }
        }
        .toolbar {
            ToolbarItem(placement: .navigation) {
                Button(action: {
                    Task {
                        await viewModel.refresh()
                    }
                }) {
                    Image(systemName: "arrow.clockwise")
                }
                .help("Refresh disk list")
            }
        }
        .task {
            await viewModel.refresh()
        }
    }
}
