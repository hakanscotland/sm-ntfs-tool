//
//  SettingsView.swift
//  SM-NTFS Disk Manager
//
//  Application settings view.
//

import SwiftUI

struct SettingsView: View {
    var body: some View {
        TabView {
            GeneralSettings()
                .tabItem {
                    Label("General", systemImage: "gear")
                }

            AdvancedSettings()
                .tabItem {
                    Label("Advanced", systemImage: "slider.horizontal.3")
                }
        }
        .frame(width: 500, height: 400)
    }
}

struct GeneralSettings: View {
    var body: some View {
        Form {
            Text("TODO: Add general settings")
                .foregroundStyle(.secondary)
        }
        .padding()
    }
}

struct AdvancedSettings: View {
    var body: some View {
        Form {
            Text("TODO: Add advanced settings")
                .foregroundStyle(.secondary)
        }
        .padding()
    }
}
