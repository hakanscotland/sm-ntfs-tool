# sm-ntfs-tool

# Project Role: Senior Systems Engineer & macOS Developer
# Project Name: SM-NTFS for MacOS (Modern Rewrite)
# Objective: Create a high-performance, read/write NTFS driver and management tool for modern macOS (Apple Silicon & Intel).

## Context & Constraints
We are building "SM-NTFS," a tool to enable full Read/Write access to NTFS drives on macOS. The goal is to replace legacy kernel extensions with a modern, safe, and high-performance stack suitable for macOS 12 (Monterey) through macOS 15 (Sequoia).

## Tech Stack Requirements
1.  **Core Driver Logic:** Rust (utilizing crates like `ntfs` or a custom implementation for parsing, and `fuser` for the FUSE interface).
2.  **File System Interface:** FUSE (Filesystem in Userspace) or FUSE-T (to avoid kext dependencies).
3.  **GUI/Management App:** Swift 6 + SwiftUI (macOS target).
4.  **Inter-Process Communication:** Swift/Rust bridge via C-FFI or XPC Services.
5.  **Build System:** Cargo (for Rust) and XcodeGen/SwiftPM (for Swift).

## Core Features Implementation Plan

### 1. The Driver (Rust Backend)
- **Read/Write Operations:** Implement a thread-safe NTFS parser. Must support creating, deleting, renaming, and modifying files.
- **Smart Caching:** Implement a "Write-Back Cache" mechanism in memory to buffer small writes and flush them sequentially. This addresses the "Wait less for file transfers" requirement.
- **Reliability:** Use Rust's `Result` and `Option` types strictly to handle disk errors gracefully without crashing the OS.
- **Extended Attributes:** Map macOS Extended Attributes (xattr) to NTFS Alternate Data Streams (ADS) to preserve metadata (Finder tags, resource forks).

### 2. The SM Disk Manager (Swift Frontend)
- **Dashboard:** A clean SwiftUI view listing all connected volumes.
- **Mount/Unmount Logic:** Buttons to mount NTFS drives in R/W mode using the Rust backend.
- **Format/Repair:** Wrappers around `ntfsfix` (or internal Rust repair logic) and formatting tools.
- **Dark Mode Support:** Native look and feel.

## Architecture Prompt
Please generate the initial project structure and key source code files for the following:

1.  **`Cargo.toml`**: Dependencies including `fuser`, `libc`, `log`, and `anyhow`.
2.  **`src/main.rs`**: The entry point for the filesystem daemon. Setup a basic FUSE mount point that intercepts file operations.
3.  **`SMDiskManager.swift`**: A SwiftUI prototype showing a list of detected USB drives with "Mount Writeable" toggle buttons.
4.  **`BridgingHeader.h`**: A C-header to allow Swift to call the Rust mounting functions.

## Specific Requirement: "Rock-Solid Reliability"
Ensure the Rust code includes a `flush` mechanism that guarantees data is written to the physical disk before unmounting to prevent data corruption.

## Specific Requirement: "Smart Caching"
Sketch a `CacheManager` struct in Rust that uses an LRU (Least Recently Used) cache for read operations to speed up accessing frequently used files.

Let's start by scaffolding the Rust driver and the Swift UI structure.