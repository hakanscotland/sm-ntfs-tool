//
//  SMDiskManagerApp.swift
//  SM-NTFS Disk Manager
//
//  Main application entry point.
//

import SwiftUI

@main
struct SMDiskManagerApp: App {
    @StateObject private var diskViewModel = DiskViewModel()

    var body: some Scene {
        WindowGroup {
            MainWindow()
                .environmentObject(diskViewModel)
                .frame(minWidth: 800, minHeight: 600)
        }
        .commands {
            CommandGroup(replacing: .newItem) {}

            CommandMenu("Disk") {
                Button("Refresh") {
                    Task {
                        await diskViewModel.refresh()
                    }
                }
                .keyboardShortcut("r", modifiers: .command)

                Divider()

                Button("Unmount All") {
                    Task {
                        await diskViewModel.unmountAll()
                    }
                }
            }
        }

        Settings {
            SettingsView()
        }
    }
}
