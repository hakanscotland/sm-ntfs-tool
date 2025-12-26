# SM-NTFS Tool - Mimari Tasarım Dokümantasyonu

**Versiyon:** 1.0
**Tarih:** 2024
**Proje:** SM-NTFS for macOS - Modern Read/Write NTFS Driver

---

## İçindekiler

1. [Genel Bakış](#1-genel-bakış)
2. [Sistem Gereksinimleri](#2-sistem-gereksinimleri)
3. [Teknoloji Stack'i](#3-teknoloji-stacki)
4. [Mimari Tasarım](#4-mimari-tasarım)
5. [Rust Driver Detayları](#5-rust-driver-detayları)
6. [Swift GUI Detayları](#6-swift-gui-detayları)
7. [Swift-Rust Bridge](#7-swift-rust-bridge)
8. [Cache ve Performans](#8-cache-ve-performans)
9. [Güvenlik ve İzinler](#9-güvenlik-ve-izinler)
10. [Build ve Deployment](#10-build-ve-deployment)
11. [Geliştirme Yol Haritası](#11-geliştirme-yol-haritası)

---

## 1. Genel Bakış

### 1.1 Proje Hedefi

SM-NTFS, macOS sistemlerinde NTFS sürücülerine tam okuma/yazma erişimi sağlayan modern bir araçtır. Eski kernel extension (kext) tabanlı çözümlerin yerine, userspace FUSE teknolojisi kullanarak güvenli ve performanslı bir alternatif sunar.

### 1.2 Temel Özellikler

- ✅ **Full Read/Write Access:** NTFS sürücülerine tam erişim
- ✅ **Native Performance:** Optimize edilmiş cache ve buffering
- ✅ **Modern GUI:** SwiftUI ile native macOS deneyimi
- ✅ **Rock-Solid Reliability:** Rust ile güvenli, crash-free driver
- ✅ **Smart Caching:** LRU cache ve write-back buffering
- ✅ **No Kernel Extensions:** Userspace FUSE, SIP uyumlu
- ✅ **Universal Binary:** Apple Silicon ve Intel desteği

### 1.3 Hedef Kullanıcılar

- macOS kullanıcıları (Monterey 12+)
- Windows/Mac dual-boot kullanıcıları
- Fotoğrafçılar ve video editörleri
- IT profesyonelleri

---

## 2. Sistem Gereksinimleri

### 2.1 Platform Gereksinimleri

| Gereksinim | Minimum | Önerilen |
|------------|---------|----------|
| macOS Version | 12.0 (Monterey) | 13.0+ (Ventura) |
| Processor | Apple M1 / Intel Core i5 | Apple M2+ / Intel i7+ |
| RAM | 4 GB | 8 GB+ |
| Disk Space | 100 MB | 200 MB |

### 2.2 Sistem İzinleri

- **Full Disk Access** (Zorunlu)
- **Removable Volumes Access** (Zorunlu)
- **File System Access** (Otomatik)

### 2.3 Teknik Kısıtlamalar

#### ⚠️ Zorunlu Kısıtlamalar
- Kernel Extension (kext) kullanılamaz
- System Integrity Protection (SIP) aktif kalmalı
- Notarization gerekli (App Store dışı dağıtım)

#### ✅ Çözümler
- FUSE-T kullanımı (userspace)
- C-FFI bridge (Swift ↔ Rust)
- SMJobBless (privileged operations)

---

## 3. Teknoloji Stack'i

### 3.1 Backend (Rust)

```toml
[dependencies]
# NTFS & Filesystem
ntfs = "0.4"              # NTFS parser
fuser = "0.14"            # FUSE bindings

# Async Runtime
tokio = "1.35"            # Async runtime
async-trait = "0.1"       # Async traits

# Caching
lru = "0.12"              # LRU cache

# Error Handling
thiserror = "1.0"         # Error derive
anyhow = "1.0"            # Error handling

# Logging
tracing = "0.1"           # Structured logging
```

### 3.2 Frontend (Swift)

```swift
// Platform
- Swift 6
- SwiftUI (macOS 13+)
- Combine (Reactive)

// Dependencies
- swift-log (Logging)
- DiskArbitration (Disk monitoring)
```

### 3.3 FUSE Stack

**Seçim: FUSE-T (Primary) + macFUSE (Optional)**

| Özellik | macFUSE | FUSE-T |
|---------|---------|--------|
| Kernel Extension | ✅ Gerekli | ❌ Gereksiz |
| Performance | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Future-Proof | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| Stability | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Karar** | Opsiyonel | **Primary** |

---

## 4. Mimari Tasarım

### 4.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   User Interface                         │
│              (SwiftUI macOS Application)                 │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  Communication Layer                     │
│         C-FFI (sync) + XPC Services (async)             │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                    Rust Driver Core                      │
│         (NTFS Operations + FUSE Interface)              │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  FUSE-T / macFUSE                       │
│             (Userspace Filesystem Layer)                 │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                   macOS VFS Kernel                       │
│              (Virtual File System Layer)                 │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                   Physical NTFS Disk                     │
└─────────────────────────────────────────────────────────┘
```

### 4.2 Component Diagram

```
┌────────────────────────────────────────────────────────┐
│                   Swift Application                     │
├────────────────────────────────────────────────────────┤
│  • DiskListView (UI)                                   │
│  • DiskViewModel (Logic)                               │
│  • DiskManager (Service)                               │
│  • XPCClient (IPC)                                     │
└────────────────┬───────────────────────────────────────┘
                 │
                 ▼ [C-FFI / XPC]
┌────────────────────────────────────────────────────────┐
│                   Rust Driver                          │
├────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────┐ │
│  │         FUSE Layer (fuser crate)                 │ │
│  │  • lookup, read, write, readdir, ...            │ │
│  └──────────────────────────────────────────────────┘ │
│                         │                              │
│  ┌──────────────────────────────────────────────────┐ │
│  │      NTFS Operations Coordinator                 │ │
│  │  • Read Engine  • Write Engine  • Metadata      │ │
│  └──────────────────────────────────────────────────┘ │
│                         │                              │
│  ┌──────────────────────────────────────────────────┐ │
│  │          Cache Manager                           │ │
│  │  • LRU Read Cache  • Write-Back Buffer          │ │
│  └──────────────────────────────────────────────────┘ │
│                         │                              │
│  ┌──────────────────────────────────────────────────┐ │
│  │         NTFS Parser (ntfs crate)                 │ │
│  │  • Volume  • MFT  • Data Streams                │ │
│  └──────────────────────────────────────────────────┘ │
│                         │                              │
│  ┌──────────────────────────────────────────────────┐ │
│  │           Block Device I/O                       │ │
│  │  • Direct disk access  • Buffering              │ │
│  └──────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────┘
```

### 4.3 Data Flow

**Mount Operation Flow:**

```
1. User clicks "Mount" button
   ↓
2. Swift: DiskViewModel.mount()
   ↓
3. Swift: XPCClient.mountVolume() [Async]
   ↓
4. XPC Helper: Authenticate if needed
   ↓
5. XPC → C-FFI: sm_ntfs_mount()
   ↓
6. Rust: NtfsCoordinator.mount()
   ↓
7. Rust: Initialize FUSE filesystem
   ↓
8. Rust: Start FUSE daemon loop
   ↓
9. Rust → Swift: Progress callback
   ↓
10. Swift: Update UI
    ↓
11. User: Disk appears in Finder
```

**Read Operation Flow:**

```
1. App reads file via Finder
   ↓
2. macOS VFS → FUSE
   ↓
3. FUSE → Rust: read(inode, offset, size)
   ↓
4. Cache Manager: Check LRU cache
   ├─ HIT → Return cached data
   └─ MISS ↓
5. NTFS Parser: Locate file data
   ↓
6. Block I/O: Read from disk
   ↓
7. Cache Manager: Store in cache
   ↓
8. Return data to FUSE
   ↓
9. FUSE → macOS VFS
   ↓
10. App receives data
```

**Write Operation Flow:**

```
1. App writes file via Finder
   ↓
2. macOS VFS → FUSE
   ↓
3. FUSE → Rust: write(inode, offset, data)
   ↓
4. Write Engine: Add to write-back buffer
   ↓
5. Check flush conditions:
   ├─ Buffer full? → Flush
   ├─ Time expired? → Flush
   └─ Otherwise → Return immediately
   ↓
6. [On Flush]:
   ├─ Coalesce sequential writes
   ├─ Update NTFS journal
   ├─ Write to disk
   └─ Clear buffer
   ↓
7. Return success to FUSE
```

---

## 5. Rust Driver Detayları

### 5.1 Proje Yapısı

```
rust-driver/
├── Cargo.toml
├── src/
│   ├── main.rs              // Entry point
│   ├── lib.rs               // Library exports
│   │
│   ├── ffi/                 // C-FFI Bridge
│   │   ├── mod.rs
│   │   ├── exports.rs       // C function exports
│   │   └── types.rs         // FFI-safe types
│   │
│   ├── fuse/                // FUSE Interface
│   │   ├── mod.rs
│   │   ├── filesystem.rs    // FUSE callbacks
│   │   ├── macfuse.rs       // macFUSE backend
│   │   └── fuset.rs         // FUSE-T backend
│   │
│   ├── ntfs/                // NTFS Operations
│   │   ├── mod.rs
│   │   ├── coordinator.rs   // Operation coordinator
│   │   ├── read.rs          // Read operations
│   │   ├── write.rs         // Write operations
│   │   ├── metadata.rs      // Metadata management
│   │   ├── journal.rs       // Journal handling
│   │   └── attributes.rs    // Extended attributes
│   │
│   ├── cache/               // Cache System
│   │   ├── mod.rs
│   │   ├── lru.rs           // LRU read cache
│   │   ├── writeback.rs     // Write-back buffer
│   │   └── policy.rs        // Cache policies
│   │
│   ├── parser/              // NTFS Parser
│   │   ├── mod.rs
│   │   ├── volume.rs        // Volume operations
│   │   ├── mft.rs           // MFT handling
│   │   └── streams.rs       // Data streams
│   │
│   ├── io/                  // Block I/O
│   │   ├── mod.rs
│   │   ├── device.rs        // Device access
│   │   ├── buffer.rs        // I/O buffering
│   │   └── sync.rs          // Sync/flush
│   │
│   └── utils/               // Utilities
│       ├── mod.rs
│       ├── error.rs         // Error types
│       ├── logging.rs       // Logging
│       └── config.rs        // Configuration
```

### 5.2 Core Modules

#### A) FUSE Filesystem Implementation

```rust
pub struct SMNtfsFilesystem {
    coordinator: Arc<NtfsCoordinator>,
    cache: Arc<CacheManager>,
    config: Config,
}

impl fuser::Filesystem for SMNtfsFilesystem {
    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry);
    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr);
    fn read(&mut self, req: &Request, ino: u64, fh: u64, offset: i64, size: u32, flags: i32, lock_owner: Option<u64>, reply: ReplyData);
    fn write(&mut self, req: &Request, ino: u64, fh: u64, offset: i64, data: &[u8], write_flags: u32, flags: i32, lock_owner: Option<u64>, reply: ReplyWrite);
    // ... other callbacks
}
```

#### B) NTFS Coordinator

```rust
pub struct NtfsCoordinator {
    volume: Arc<RwLock<NtfsVolume>>,
    read_engine: ReadEngine,
    write_engine: WriteEngine,
    metadata_mgr: MetadataManager,
}

impl NtfsCoordinator {
    pub async fn read_file(&self, inode: u64, offset: u64, size: usize) -> Result<Vec<u8>>;
    pub async fn write_file(&self, inode: u64, offset: u64, data: &[u8]) -> Result<usize>;
    pub async fn create_file(&self, parent: u64, name: &str) -> Result<u64>;
    pub async fn delete_file(&self, inode: u64) -> Result<()>;
}
```

### 5.3 Error Handling

```rust
#[derive(Error, Debug)]
pub enum SMNtfsError {
    #[error("Failed to read from device: {0}")]
    ReadError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid NTFS volume: {0}")]
    InvalidNtfs(String),

    // ... more error types
}

pub type Result<T> = std::result::Result<T, SMNtfsError>;
```

---

## 6. Swift GUI Detayları

### 6.1 MVVM Architecture

```
View ──► ViewModel ──► Model/Service
 │          │              │
 │          │              │
 └──────────┴──────────────┘
      SwiftUI Binding
```

### 6.2 Proje Yapısı

```swift
SMDiskManager/
├── Sources/
│   ├── App/
│   │   └── SMDiskManagerApp.swift
│   │
│   ├── Views/
│   │   ├── MainWindow.swift
│   │   ├── DiskListView.swift
│   │   ├── DiskRowView.swift
│   │   ├── DiskDetailView.swift
│   │   ├── MountButton.swift
│   │   └── SettingsView.swift
│   │
│   ├── ViewModels/
│   │   ├── DiskViewModel.swift
│   │   └── SettingsViewModel.swift
│   │
│   ├── Models/
│   │   ├── Disk.swift
│   │   └── MountStatus.swift
│   │
│   ├── Services/
│   │   ├── DiskManager.swift
│   │   ├── DiskMonitor.swift
│   │   ├── XPCClient.swift
│   │   └── FFIBridge.swift
│   │
│   └── Resources/
│       └── Assets.xcassets
```

### 6.3 Key Components

#### DiskViewModel (Observable)

```swift
@MainActor
class DiskViewModel: ObservableObject {
    @Published var disks: [Disk] = []
    @Published var isLoading = false
    @Published var error: Error?

    func mount(_ disk: Disk, readWrite: Bool) async throws
    func unmount(_ disk: Disk) async throws
    func refresh() async
}
```

#### DiskManager (Service)

```swift
class DiskManager {
    static let shared = DiskManager()

    func scanDisks() async throws -> [Disk]
    func mount(devicePath: String, readWrite: Bool) async throws
    func unmount(mountPoint: String) async throws
}
```

---

## 7. Swift-Rust Bridge

### 7.1 Communication Strategy

**Hybrid Approach:**

```
Swift App
    │
    ├─── [C-FFI] ────────► Rust (Sync, low-latency)
    │                      • Disk list queries
    │                      • Status checks
    │
    └─── [XPC Service] ──► Rust (Async, privileged)
                           • Mount operations
                           • Unmount operations
```

### 7.2 C-FFI Interface

**Header (SMNTFSBridge.h):**

```c
typedef struct SMNtfsContext SMNtfsContext;

typedef enum {
    SM_SUCCESS = 0,
    SM_ERROR_INVALID_DEVICE = -1,
    SM_ERROR_MOUNT_FAILED = -2,
    SM_ERROR_PERMISSION_DENIED = -3,
} SMErrorCode;

SMNtfsContext* sm_ntfs_init(void);
void sm_ntfs_destroy(SMNtfsContext* ctx);

SMErrorCode sm_ntfs_mount(
    SMNtfsContext* ctx,
    const char* device_path,
    const char* mount_point,
    bool read_write
);

SMErrorCode sm_ntfs_unmount(
    SMNtfsContext* ctx,
    const char* mount_point
);
```

**Rust Implementation:**

```rust
#[no_mangle]
pub extern "C" fn sm_ntfs_init() -> *mut SMNtfsContext {
    let ctx = Box::new(SMNtfsContext::new());
    Box::into_raw(ctx)
}

#[no_mangle]
pub extern "C" fn sm_ntfs_mount(
    ctx: *mut SMNtfsContext,
    device_path: *const c_char,
    mount_point: *const c_char,
    read_write: bool,
) -> SMErrorCode {
    // Safe wrapper
    // ...
}
```

**Swift Wrapper:**

```swift
class FFIBridge {
    private var context: OpaquePointer?

    init() {
        context = sm_ntfs_init()
    }

    func mount(device: String, mountPoint: String, readWrite: Bool) throws {
        let result = sm_ntfs_mount(context, device, mountPoint, readWrite)
        guard result == SM_SUCCESS else {
            throw MountError.failed(code: result)
        }
    }
}
```

### 7.3 XPC Service

```swift
@objc protocol SMNTFSHelperProtocol {
    func mountVolume(
        devicePath: String,
        mountPoint: String,
        readWrite: Bool,
        reply: @escaping (Error?) -> Void
    )
}
```

---

## 8. Cache ve Performans

### 8.1 Multi-Tier Caching

```
L1: Read Cache (LRU)
    • Size: 64-256 MB
    • MFT entries, metadata
    • Frequently accessed files

L2: Write-Back Buffer
    • Size: 32-128 MB
    • Delayed flush (5s timeout)
    • Coalescing sequential writes

L3: Disk I/O
    • Batched operations
    • Optimized seek patterns
```

### 8.2 Performance Targets

| Operation | Target | Strategy |
|-----------|--------|----------|
| Sequential Read | 400+ MB/s | Read-ahead, buffering |
| Sequential Write | 350+ MB/s | Write coalescing |
| Random Read (4K) | 10K+ IOPS | LRU cache |
| Random Write (4K) | 8K+ IOPS | Write-back buffer |
| Metadata ops | <1ms | MFT cache |
| Mount time | <2s | Lazy loading |

### 8.3 Optimization Techniques

#### A) Read-Ahead

```rust
pub struct ReadAheadManager {
    access_pattern: HashMap<u64, AccessPattern>,
    prefetch_size: usize, // 128 KB
}

impl ReadAheadManager {
    pub fn predict_and_prefetch(&mut self, inode: u64, offset: u64) {
        if self.is_sequential(inode, offset) {
            self.schedule_prefetch(inode, offset + BLOCK_SIZE);
        }
    }
}
```

#### B) Write Coalescing

```rust
fn coalesce_chunks(&self, chunks: Vec<WriteChunk>) -> Vec<WriteChunk> {
    // Merge sequential writes
    // Reduces disk seeks
}
```

#### C) Zero-Copy Transfer

```rust
#[cfg(target_os = "macos")]
unsafe fn sendfile_wrapper(src: RawFd, dst: RawFd, size: u64) -> Result<u64> {
    libc::sendfile(dst, src, 0, &mut size, std::ptr::null_mut(), 0)
}
```

---

## 9. Güvenlik ve İzinler

### 9.1 Security Layers

```
Layer 1: Code Signing & Notarization
    • Developer ID certificate
    • Hardened runtime
    • Apple notarization

Layer 2: Privilege Separation
    • GUI (user privileges)
    • XPC Helper (elevated when needed)
    • FUSE Daemon (minimal permissions)

Layer 3: System Permissions
    • Full Disk Access (TCC)
    • Removable Volumes
    • File System Access

Layer 4: Data Protection
    • Input validation
    • Buffer overflow protection
    • Secure IPC (XPC)
```

### 9.2 Permission Flow

```
1. App Launch
   ↓
2. Check Full Disk Access
   ├─ Granted → Continue
   └─ Denied → Show alert → Open System Settings
   ↓
3. User Operation (Mount)
   ↓
4. Check if admin needed
   ├─ Yes → Request via XPC Helper
   └─ No → Direct operation
```

### 9.3 Input Validation

```rust
pub fn validate_path(path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path);

    // No parent directory references
    if path.components().any(|c| c == Component::ParentDir) {
        return Err(SMNtfsError::InvalidPath("Path traversal not allowed".into()));
    }

    // Must be absolute
    if !path.is_absolute() {
        return Err(SMNtfsError::InvalidPath("Path must be absolute".into()));
    }

    Ok(path.canonicalize()?)
}
```

---

## 10. Build ve Deployment

### 10.1 Workspace Yapısı

```
sm-ntfs-tool/
├── Cargo.toml              # Rust workspace
├── Package.swift           # Swift package
├── Makefile               # Build orchestration
│
├── rust-driver/           # Rust crates
├── macos-app/             # Swift sources
├── bridge/                # C headers
├── scripts/               # Build scripts
│   ├── build.sh
│   ├── test.sh
│   ├── package.sh
│   └── sign.sh
└── dist/                  # Build output
```

### 10.2 Build Commands

```bash
# Build everything
make all

# Build Rust (universal binary)
make build-rust

# Build Swift app
make build-swift

# Run tests
make test

# Create distributable package
make package

# Install locally
make install
```

### 10.3 CI/CD Pipeline

```yaml
# GitHub Actions
on: [push, pull_request]

jobs:
  test:
    - Checkout code
    - Install Rust toolchain
    - Run Rust tests
    - Run Swift tests
    - Build debug

  build-release:
    - Build universal binary
    - Sign with Developer ID
    - Notarize with Apple
    - Create DMG
    - Upload artifact
```

### 10.4 Release Checklist

- [ ] All tests passing
- [ ] Code review complete
- [ ] Security audit done
- [ ] Performance benchmarks met
- [ ] Signed and notarized
- [ ] Documentation updated
- [ ] Release notes written

---

## 11. Geliştirme Yol Haritası

### Phase 1: Foundation (Weeks 1-4)

**Week 1-2: Rust Driver Core**
- [ ] Project structure setup
- [ ] NTFS parser integration (ntfs crate)
- [ ] Basic FUSE implementation
- [ ] Device I/O layer
- [ ] Unit tests

**Week 3-4: Swift GUI Core**
- [ ] SwiftUI app structure
- [ ] Disk list view
- [ ] C-FFI bridge
- [ ] DiskArbitration integration
- [ ] Basic mount/unmount

### Phase 2: Features (Weeks 5-8)

**Week 5-6: Cache System**
- [ ] LRU read cache
- [ ] Write-back buffer
- [ ] Performance testing
- [ ] Cache statistics

**Week 7-8: Advanced Features**
- [ ] Extended attributes (xattr → ADS)
- [ ] Journal handling
- [ ] Error recovery
- [ ] Crash recovery

### Phase 3: Polish (Weeks 9-12)

**Week 9-10: UI/UX**
- [ ] Settings panel
- [ ] Progress indicators
- [ ] Notifications
- [ ] Accessibility

**Week 11-12: Release Prep**
- [ ] Code signing setup
- [ ] Notarization
- [ ] DMG packaging
- [ ] Documentation
- [ ] Beta testing

### Phase 4: Post-Launch

- [ ] Performance optimizations
- [ ] Bug fixes from user feedback
- [ ] Feature requests
- [ ] Continuous improvement

---

## 12. Sonuç

Bu dokümantasyon, SM-NTFS Tool projesinin kapsamlı mimari tasarımını sunmaktadır. Proje, modern teknolojiler (Rust + Swift), güvenlik odaklı yaklaşım ve kullanıcı deneyimi önceliğiyle tasarlanmıştır.

### Temel Kararlar

✅ **FUSE-T Primary** - Geleceğe hazır, kext-free
✅ **Rust Backend** - Güvenli, performanslı
✅ **SwiftUI Frontend** - Native macOS deneyimi
✅ **Hybrid Bridge** - C-FFI + XPC
✅ **Multi-Tier Cache** - Optimal performans
✅ **Security First** - Notarized, sandboxed

### Başarı Kriterleri

- 🎯 Native NTFS hızının %70+'ı
- 🎯 <2s mount time
- 🎯 Zero kernel panics
- 🎯 Full macOS 12+ compatibility
- 🎯 Apple Silicon + Intel universal binary

---

**Hazırlayan:** Claude
**Tarih:** 2024
**Versiyon:** 1.0
