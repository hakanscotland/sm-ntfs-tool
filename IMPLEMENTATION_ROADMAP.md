# SM-NTFS Tool - İmplementasyon Yol Haritası

**Proje:** SM-NTFS for macOS
**Hedef:** 12 haftalık geliştirme süreci
**Metodoloji:** Agile, iterative development

---

## İçindekiler

1. [Genel Plan](#1-genel-plan)
2. [Phase 1: Foundation (Hafta 1-4)](#phase-1-foundation-hafta-1-4)
3. [Phase 2: Features (Hafta 5-8)](#phase-2-features-hafta-5-8)
4. [Phase 3: Polish (Hafta 9-12)](#phase-3-polish-hafta-9-12)
5. [Günlük Geliştirme Workflow](#günlük-geliştirme-workflow)
6. [Test Stratejisi](#test-stratejisi)
7. [Risk Yönetimi](#risk-yönetimi)

---

## 1. Genel Plan

### 1.1 Milestone'lar

```
Week 1-4:  Foundation      → MVP (Minimal Viable Product)
Week 5-8:  Features        → Beta Release
Week 9-12: Polish & Launch → Production Release v1.0
```

### 1.2 Başlangıç Checklist

- [ ] Development environment setup
  - [ ] Rust toolchain (stable + nightly)
  - [ ] Xcode 15+
  - [ ] FUSE-T installed
  - [ ] Git repository initialized

- [ ] Project structure creation
  - [ ] Rust workspace
  - [ ] Swift package
  - [ ] Build scripts
  - [ ] CI/CD pipeline

- [ ] Documentation
  - [ ] README.md
  - [ ] ARCHITECTURE.md ✅
  - [ ] CONTRIBUTING.md
  - [ ] LICENSE

---

## Phase 1: Foundation (Hafta 1-4)

### 🎯 Hedef: Çalışan MVP

**Deliverable:** Basic mount/unmount functionality

---

## Week 1: Rust Driver Temel

### Gün 1-2: Proje İskeleti

**Tasks:**
- [ ] Rust workspace oluştur
  ```bash
  cargo new --lib rust-driver/sm-ntfs-core
  cargo new --lib rust-driver/sm-ntfs-fuse
  cargo new --bin rust-driver/sm-ntfs-cli
  ```

- [ ] Dependencies ekle (Cargo.toml)
  ```toml
  [dependencies]
  ntfs = "0.4"
  fuser = "0.14"
  tokio = { version = "1.35", features = ["full"] }
  tracing = "0.1"
  thiserror = "1.0"
  ```

- [ ] Modül yapısını oluştur
  ```
  src/
  ├── main.rs
  ├── lib.rs
  ├── ffi/mod.rs
  ├── fuse/mod.rs
  ├── ntfs/mod.rs
  └── utils/mod.rs
  ```

**Output:**
- Derlenebilen boş proje
- Temel modül iskeletleri

### Gün 3-4: Device I/O Layer

**Tasks:**
- [ ] Device açma/kapatma
  ```rust
  // src/io/device.rs
  pub struct BlockDevice {
      file: File,
      block_size: usize,
  }

  impl BlockDevice {
      pub fn open(path: &str) -> Result<Self>;
      pub fn read_block(&self, block: u64) -> Result<Vec<u8>>;
  }
  ```

- [ ] Block I/O implementasyonu
- [ ] Error handling yapısı
  ```rust
  // src/utils/error.rs
  #[derive(Error, Debug)]
  pub enum SMNtfsError {
      #[error("Device not found: {0}")]
      DeviceNotFound(String),
  }
  ```

**Output:**
- `/dev/disk2` gibi device'lar açılabiliyor
- Block okuma çalışıyor

**Test:**
```bash
cargo test io::tests::test_device_open
```

### Gün 5: NTFS Parser Integration

**Tasks:**
- [ ] `ntfs` crate entegrasyonu
  ```rust
  // src/parser/volume.rs
  pub struct NtfsVolumeWrapper {
      ntfs: Ntfs,
  }

  impl NtfsVolumeWrapper {
      pub fn mount(device: BlockDevice) -> Result<Self>;
      pub fn root_directory(&self) -> Result<NtfsDirectory>;
  }
  ```

- [ ] Volume mount
- [ ] Root directory okuma

**Test:**
```bash
# USB drive takılı olmalı
cargo run -- --device /dev/disk2s1 --mount /tmp/test
```

**Output:**
- NTFS volume tanınıyor
- Root directory listelebiliyor

---

## Week 2: FUSE Implementation

### Gün 6-7: FUSE Temel Callbacks

**Tasks:**
- [ ] FUSE filesystem struct
  ```rust
  // src/fuse/filesystem.rs
  pub struct SMNtfsFilesystem {
      volume: Arc<RwLock<NtfsVolumeWrapper>>,
  }

  impl fuser::Filesystem for SMNtfsFilesystem {
      fn lookup(&mut self, ...) { /* TODO */ }
      fn getattr(&mut self, ...) { /* TODO */ }
      fn readdir(&mut self, ...) { /* TODO */ }
  }
  ```

- [ ] Implement:
  - [ ] `lookup` (dosya/klasör arama)
  - [ ] `getattr` (dosya özellikleri)
  - [ ] `readdir` (klasör içeriği)

**Output:**
- FUSE mount point oluşturuluyor
- Klasörler görüntülenebiliyor (read-only)

**Test:**
```bash
# Terminal 1
cargo run -- --device /dev/disk2s1 --mount /tmp/ntfs

# Terminal 2
ls /tmp/ntfs
```

### Gün 8-9: Read Operations

**Tasks:**
- [ ] `open` callback
- [ ] `read` callback
  ```rust
  fn read(
      &mut self,
      _req: &Request,
      ino: u64,
      fh: u64,
      offset: i64,
      size: u32,
      _flags: i32,
      _lock: Option<u64>,
      reply: ReplyData,
  ) {
      // NTFS'ten veri oku
  }
  ```

- [ ] Data stream handling

**Output:**
- Dosyalar okunabiliyor
- `cat`, `less` çalışıyor

**Test:**
```bash
cat /tmp/ntfs/test.txt
md5sum /tmp/ntfs/largefile.bin
```

### Gün 10: Debugging & Stabilization

**Tasks:**
- [ ] Logging infrastructure
  ```rust
  tracing::info!("Mounting volume at {}", mount_point);
  tracing::debug!("Reading inode {} at offset {}", ino, offset);
  ```

- [ ] Error handling improvements
- [ ] Memory leak check (valgrind)
- [ ] Crash recovery

**Output:**
- Detaylı log dosyaları
- Stabil read-only mount

---

## Week 3: Swift GUI Başlangıç

### Gün 11-12: Swift Project Setup

**Tasks:**
- [ ] Swift package oluştur
  ```bash
  cd macos-app
  swift package init --type executable
  ```

- [ ] SwiftUI app template
  ```swift
  @main
  struct SMDiskManagerApp: App {
      var body: some Scene {
          WindowGroup {
              ContentView()
          }
      }
  }
  ```

- [ ] MVVM structure
  ```
  Sources/
  ├── App/
  ├── Views/
  ├── ViewModels/
  ├── Models/
  └── Services/
  ```

**Output:**
- Çalışan boş SwiftUI app

### Gün 13-14: C-FFI Bridge

**Tasks:**
- [ ] C header oluştur
  ```c
  // bridge/SMNTFSBridge.h
  typedef struct SMNtfsContext SMNtfsContext;

  SMNtfsContext* sm_ntfs_init(void);
  int sm_ntfs_scan_disks(SMNtfsContext* ctx, char** devices, int* count);
  ```

- [ ] Rust FFI exports
  ```rust
  // src/ffi/exports.rs
  #[no_mangle]
  pub extern "C" fn sm_ntfs_init() -> *mut SMNtfsContext {
      Box::into_raw(Box::new(SMNtfsContext::new()))
  }
  ```

- [ ] Swift wrapper
  ```swift
  // Services/FFIBridge.swift
  class FFIBridge {
      private var context: OpaquePointer?

      init() {
          context = sm_ntfs_init()
      }
  }
  ```

**Output:**
- Swift, Rust fonksiyonlarını çağırabiliyor

**Test:**
```swift
let bridge = FFIBridge()
let disks = bridge.scanDisks()
print("Found \(disks.count) NTFS disks")
```

### Gün 15: DiskArbitration Integration

**Tasks:**
- [ ] Disk monitoring service
  ```swift
  // Services/DiskMonitor.swift
  import DiskArbitration

  class DiskMonitor {
      private var session: DASession?

      func start() {
          session = DASessionCreate(kCFAllocatorDefault)
          DARegisterDiskAppearedCallback(...)
      }
  }
  ```

- [ ] USB disk detection
- [ ] NTFS volume filtering

**Output:**
- USB disk takıldığında app bildirim alıyor

---

## Week 4: MVP Integration

### Gün 16-17: UI Implementation

**Tasks:**
- [ ] Disk list view
  ```swift
  struct DiskListView: View {
      @EnvironmentObject var viewModel: DiskViewModel

      var body: some View {
          List(viewModel.disks) { disk in
              DiskRowView(disk: disk)
          }
      }
  }
  ```

- [ ] Mount button
- [ ] Status indicators

**Output:**
- GUI'de disk listesi görünüyor

### Gün 18-19: Mount/Unmount Integration

**Tasks:**
- [ ] Mount functionality
  ```swift
  func mount(_ disk: Disk, readWrite: Bool) async throws {
      try await diskManager.mount(
          devicePath: disk.devicePath,
          readWrite: readWrite
      )
  }
  ```

- [ ] XPC helper (basic)
- [ ] Permission handling

**Output:**
- GUI'den mount/unmount çalışıyor

### Gün 20: MVP Testing & Demo

**Tasks:**
- [ ] End-to-end test
- [ ] Performance baseline
- [ ] Bug fixes
- [ ] Internal demo

**Checklist:**
- [ ] USB disk takılıyor → GUI'de görünüyor
- [ ] "Mount" tıklanıyor → Finder'da açılıyor
- [ ] Dosyalar okunabiliyor
- [ ] "Unmount" çalışıyor

**🎉 Milestone: MVP Complete!**

---

## Phase 2: Features (Hafta 5-8)

### 🎯 Hedef: Beta Release

**Deliverable:** Read/Write support + Performance optimization

---

## Week 5: Write Operations

### Gün 21-22: Basic Write Support

**Tasks:**
- [ ] FUSE write callback
  ```rust
  fn write(&mut self, ..., data: &[u8], reply: ReplyWrite) {
      // NTFS'e yaz
  }
  ```

- [ ] File modification
- [ ] NTFS journal updates

**Output:**
- Dosyalara yazma çalışıyor

### Gün 23-24: Create/Delete Operations

**Tasks:**
- [ ] `create` callback (yeni dosya)
- [ ] `unlink` callback (dosya silme)
- [ ] `mkdir` callback (klasör oluşturma)
- [ ] `rmdir` callback (klasör silme)

**Output:**
- CRUD operations tam çalışıyor

### Gün 25: Write Testing

**Tasks:**
- [ ] Stress test (büyük dosyalar)
- [ ] Reliability test (çökme kontrolü)
- [ ] Data integrity check (checksum)

**Test:**
```bash
dd if=/dev/urandom of=/tmp/ntfs/test.bin bs=1M count=100
md5sum /tmp/ntfs/test.bin
```

---

## Week 6: Cache System

### Gün 26-27: LRU Read Cache

**Tasks:**
- [ ] Cache manager
  ```rust
  pub struct ReadCache {
      file_cache: LruCache<FileKey, Vec<u8>>,
      mft_cache: LruCache<u64, MftEntry>,
  }
  ```

- [ ] Cache hit/miss tracking
- [ ] Adaptive eviction

**Output:**
- %30+ read performance improvement

### Gün 28-29: Write-Back Buffer

**Tasks:**
- [ ] Write buffering
  ```rust
  pub struct WriteBackBuffer {
      buffers: HashMap<u64, FileBuffer>,
      max_dirty_time: Duration,
  }
  ```

- [ ] Delayed flush (5s)
- [ ] Coalescing sequential writes

**Output:**
- Small write performance gelişti

### Gün 30: Performance Tuning

**Tasks:**
- [ ] Benchmark suite
- [ ] Profile (Instruments)
- [ ] Optimize hot paths

**Metrics:**
- Sequential read: 400+ MB/s ✅
- Sequential write: 350+ MB/s ✅

---

## Week 7: Advanced Features

### Gün 31-32: Extended Attributes

**Tasks:**
- [ ] xattr → NTFS ADS mapping
  ```rust
  fn setxattr(&mut self, ..., name: &OsStr, value: &[u8], ...) {
      // NTFS Alternate Data Stream'e yaz
  }
  ```

- [ ] Finder tags support
- [ ] Resource forks

**Output:**
- macOS metadata korunuyor

### Gün 33-34: Journal & Recovery

**Tasks:**
- [ ] NTFS journal handling
- [ ] Transaction logging
- [ ] Crash recovery
  ```rust
  pub struct CrashRecovery {
      state_file: PathBuf,
  }

  impl CrashRecovery {
      pub fn save_state(&self, state: &RecoveryState) -> Result<()>;
      pub fn recover() -> Result<()>;
  }
  ```

**Output:**
- Çökme sonrası veri kaybı yok

### Gün 35: Error Handling

**Tasks:**
- [ ] User-friendly error messages
- [ ] Retry mechanisms
- [ ] Fallback strategies

---

## Week 8: Beta Preparation

### Gün 36-37: UI Polish

**Tasks:**
- [ ] Progress indicators
- [ ] Notifications
  ```swift
  NotificationService.shared.show(
      title: "Disk Mounted",
      message: "NTFS drive is ready to use"
  )
  ```

- [ ] Settings panel

### Gün 38-39: Code Signing & Packaging

**Tasks:**
- [ ] Developer ID certificate setup
- [ ] Code signing all binaries
- [ ] Notarization
- [ ] DMG creation

### Gün 40: Beta Release

**Tasks:**
- [ ] Internal testing
- [ ] Beta tester recruitment
- [ ] Crash reporting setup
- [ ] Release notes

**🎉 Milestone: Beta Release!**

---

## Phase 3: Polish (Hafta 9-12)

### 🎯 Hedef: Production Release v1.0

---

## Week 9: Testing & Bug Fixes

### Gün 41-45: Beta Feedback

**Tasks:**
- [ ] Bug triage
- [ ] Critical bug fixes
- [ ] Performance issues
- [ ] UX improvements

**Daily:**
- Morning: Review crash reports
- Afternoon: Fix bugs
- Evening: Deploy beta updates

---

## Week 10: Documentation

### Gün 46-48: User Documentation

**Tasks:**
- [ ] User guide
  - Installation
  - Basic usage
  - Troubleshooting
- [ ] Video tutorials
- [ ] FAQ

### Gün 49-50: Developer Documentation

**Tasks:**
- [ ] API documentation (rustdoc)
- [ ] Architecture diagrams
- [ ] Contributing guide

---

## Week 11: Final Polish

### Gün 51-53: UI/UX Refinement

**Tasks:**
- [ ] Accessibility audit
- [ ] VoiceOver support
- [ ] Keyboard navigation
- [ ] Visual polish

### Gün 54-55: Performance Optimization

**Tasks:**
- [ ] Final benchmarks
- [ ] Memory optimization
- [ ] CPU usage reduction

---

## Week 12: Launch

### Gün 56-58: Pre-Launch

**Tasks:**
- [ ] Final QA testing
- [ ] Launch checklist
- [ ] Marketing materials
- [ ] Press release

### Gün 59: Launch Day

**Tasks:**
- [ ] Release on website
- [ ] GitHub release
- [ ] Social media announcement
- [ ] Monitor for issues

### Gün 60: Post-Launch

**Tasks:**
- [ ] Monitor crash reports
- [ ] Quick bug fixes
- [ ] User support
- [ ] Plan v1.1

**🎉 Milestone: v1.0 LAUNCHED!**

---

## Günlük Geliştirme Workflow

### Daily Routine

```
09:00 - 09:30   Standup / Planning
09:30 - 12:00   Deep work (coding)
12:00 - 13:00   Lunch
13:00 - 15:00   Coding / Testing
15:00 - 15:30   Code review
15:30 - 17:00   Documentation / Bug fixes
17:00 - 17:30   Tomorrow planning
```

### Git Workflow

```bash
# Feature branch
git checkout -b feature/write-operations

# Commit often
git commit -m "feat: implement FUSE write callback"

# Push daily
git push origin feature/write-operations

# PR review
# Merge to main
```

### Testing Cadence

- **Unit tests:** After each function
- **Integration tests:** End of day
- **E2E tests:** End of week
- **Performance tests:** Weekly

---

## Test Stratejisi

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_read_cache_hit() {
        let cache = ReadCache::new(64);
        cache.put(1, vec![1,2,3]);
        assert_eq!(cache.get(1), Some(vec![1,2,3]));
    }
}
```

### Integration Tests

```bash
# Mount test NTFS image
tests/integration_test.sh
```

### Performance Tests

```rust
#[bench]
fn bench_sequential_read(b: &mut Bencher) {
    b.iter(|| {
        // Read 100MB
    });
}
```

### Manual Test Scenarios

1. **Basic Operations**
   - [ ] Mount USB drive
   - [ ] Read files
   - [ ] Write files
   - [ ] Create/delete files
   - [ ] Unmount

2. **Edge Cases**
   - [ ] Large files (>4GB)
   - [ ] Many small files
   - [ ] Deep directory structures
   - [ ] Special characters in names
   - [ ] Corrupted NTFS volume

3. **Performance**
   - [ ] Copy 10GB to NTFS
   - [ ] Copy 10GB from NTFS
   - [ ] Random access patterns
   - [ ] Concurrent operations

4. **Reliability**
   - [ ] Force quit during write
   - [ ] Disconnect during operation
   - [ ] Low disk space
   - [ ] Low memory

---

## Risk Yönetimi

### Yüksek Riskli Alanlar

#### 1. NTFS Corruption Risk
**Risk:** Write operations sırasında veri kaybı
**Mitigation:**
- Aggressive journaling
- Flush before unmount
- Crash recovery mechanism
- Extensive testing

#### 2. Performance Issues
**Risk:** Native NTFS'ten çok yavaş
**Mitigation:**
- Early benchmarking
- Profiling tools (Instruments)
- Cache optimization
- Consider macFUSE if needed

#### 3. macOS Compatibility
**Risk:** Farklı macOS versiyonlarında sorun
**Mitigation:**
- Test on macOS 12, 13, 14, 15
- VM farm setup
- Beta tester diversity

#### 4. Permissions & Security
**Risk:** Full Disk Access red edilirse
**Mitigation:**
- Clear user messaging
- Helper app for privileges
- Fallback modes
- Documentation

### Contingency Plans

**Plan A:** FUSE-T primary
**Plan B:** If performance bad → Add macFUSE support
**Plan C:** If stability issues → Read-only release first

---

## Başarı Metrikleri

### Technical Metrics

- [ ] Test coverage >80%
- [ ] Zero crashes in 1 week testing
- [ ] Performance within 30% of native
- [ ] Mount time <2 seconds

### User Metrics

- [ ] 100 beta testers
- [ ] <5% crash rate
- [ ] >90% successful mounts
- [ ] Positive feedback

### Timeline Metrics

- [ ] MVP on time (Week 4)
- [ ] Beta on time (Week 8)
- [ ] Launch on time (Week 12)

---

## Kaynaklar

### Development

- **Rust:** https://doc.rust-lang.org/
- **FUSE:** https://github.com/cberner/fuser
- **NTFS:** https://docs.rs/ntfs/
- **SwiftUI:** https://developer.apple.com/swiftui/

### Testing

- **Test NTFS images:** Prepare various test cases
- **USB drives:** Different sizes, brands
- **macOS VMs:** Parallels/VMware

### Tools

- **Xcode:** GUI development
- **Instruments:** Profiling
- **Cargo:** Rust build
- **Git:** Version control
- **GitHub Actions:** CI/CD

---

**Prepared by:** Claude
**Date:** 2024
**Version:** 1.0

**Next Steps:**
1. Review and approve this roadmap
2. Set up development environment
3. Create initial project structure
4. Start Week 1, Day 1 tasks!

**Let's build something amazing! 🚀**
