# Bootstrap: Linux Kernel Primitives for Ordo

**Purpose:** Quick-start guide for using the extracted Linux kernel primitives to inform Ordo's architecture design.

**Date:** 2026-06-20  
**Source:** 221 UAPI headers from `F:\OPENCLAW-PROJECTS\linux-master`  
**Output:** 1.34 MB of structured analysis in `output/`

---

## Quick Start

### 1. Read This First

Start with these summary documents (in order):

1. **`99-FINAL-EXTRACTION-SUMMARY.md`** - Grand total, key patterns, Ordo mappings
2. **`00-EXTRACTION-SUMMARY.md`** - Core uapi primitives (ioctl, capabilities, event FDs)
3. **`02-GPU-DRIVERS-SUMMARY.md`** - GPU command submission, buffer objects, fences
4. **`04-INPUT-SECURITY-FS-STORAGE-SUMMARY.md`** - Security sandboxing (KVM, VFIO, seccomp, landlock)

### 2. Browse by Category

Navigate to `output/` subdirectories:

```
output/
├── ioctl_analysis.md          # Command encoding (START HERE)
├── capability_analysis.md     # Permission model
├── futex_analysis.md          # Sync primitives
├── eventfd_analysis.md        # Event signaling
├── netlink_analysis.md        # Messaging protocol
├── kvm_analysis.md            # Virtualization (606 defines!)
├── vfio_analysis.md           # Device passthrough
├── gpu-drivers/               # 15 GPU vendors
├── drivers/                   # Generic drivers
├── net-audio-graphics/        # Network + ALSA audio
├── remaining/                 # Input, security, filesystems
├── batch5/                    # Misc (DVB, ELF, FUSE, etc.)
└── batch6/                    # Networking (if_*, IPv4/v6, netfilter)
```

### 3. Use the JSON Data Files

Each `*_analysis.md` has a companion `*_data.json` for programmatic access:

```python
import json

with open('output/ioctl_data.json') as f:
    data = json.load(f)
    
print(f"Defines: {len(data['defines'])}")
print(f"Structs: {len(data['structs'])}")
print(f"Ioctl commands: {len(data['ioctls'])}")

# Example: List all ioctl commands
for cmd in data['ioctls']:
    print(f"{cmd['macro']}: {cmd['direction']} type={cmd['type_char']} nr={cmd['nr']}")
```

### 4. Search for Specific Patterns

Use `grep` or PowerShell to find patterns across all extracted files:

```powershell
# Find all capability-related defines
Select-String -Path "output\*\*.md" -Pattern "CAP_" | Select-Object -First 20

# Find all struct definitions with "fence" in the name
Select-String -Path "output\*\*.md" -Pattern "struct.*fence" -Context 3,3

# Find all error codes
Select-String -Path "output\errno_analysis.md" -Pattern "E[A-Z]+"
```

---

## Key Patterns to Steal for Ordo

### A. Message Type Encoding (from ioctl)

**Linux:**
```c
#define _IOWR(type, nr, argtype) \
    (((type) << 8) | (nr) | ((_IOC_READ|_IOC_WRITE) << 30) | (sizeof(argtype) << 16))
```

**Ordo Application:**
```rust
#[repr(transparent)]
pub struct MessageType(u32);

impl MessageType {
    pub const fn new(direction: Direction, subsystem: u8, op: u8, flags: u12) -> Self {
        MessageType(
            ((direction as u32) << 30) |
            ((subsystem as u32) << 22) |
            ((op as u32) << 14) |
            (flags as u32)
        )
    }
}

pub enum Direction {
    Notification = 0b00,  // No payload
    Request = 0b01,       // Client → Server
    Response = 0b10,      // Server → Client
    Stream = 0b11,        // Bidirectional
}
```

**File to read:** `output/ioctl_analysis.md`, `output/asm-generic/ioctl_analysis.md`

---

### B. Capability-Based Permissions (from Linux capabilities + Landlock)

**Linux:**
```c
// 40+ discrete capabilities (not root/non-root)
#define CAP_NET_ADMIN      12
#define CAP_SYS_ADMIN      21
#define CAP_DAC_OVERRIDE   1
// ...

// Landlock FS access rights (15 bits)
#define LANDLOCK_ACCESS_FS_READ_FILE   (1ULL << 2)
#define LANDLOCK_ACCESS_FS_WRITE_FILE  (1ULL << 1)
#define LANDLOCK_ACCESS_FS_REFER       (1ULL << 13)  // rename/link
```

**Ordo Application:**
```rust
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Permissions: u64 {
        const NET_ADMIN       = 1 << 0;
        const FS_READ         = 1 << 1;
        const FS_WRITE        = 1 << 2;
        const FS_EXECUTE      = 1 << 3;
        const PROCESS_TRACE   = 1 << 4;
        const DEVICE_ACCESS   = 1 << 5;
        const MOUNT           = 1 << 6;
        const SIGNAL          = 1 << 7;
        const SETUID          = 1 << 8;
        const SETGID          = 1 << 9;
        const CHROOT          = 1 << 10;
        const MODULE_LOAD     = 1 << 11;
        const RAW_IO          = 1 << 12;
        const TIME_SET        = 1 << 13;
        const AUDIT_WRITE     = 1 << 14;
        const BPF             = 1 << 15;
        const CHECKPOINT_RESTORE = 1 << 16;
        // ... extend as needed
    }
}

pub struct CapabilitySet {
    pub effective: Permissions,
    pub permitted: Permissions,
    pub inheritable: Permissions,
}
```

**Files to read:** `output/capability_analysis.md`, `output/landlock_analysis.md`, `output/prctl_analysis.md`

---

### C. Event-Driven Async Completion (from eventfd, signalfd, timerfd, fences)

**Linux:**
```c
// Everything is a pollable FD
int efd = eventfd(0, EFD_NONBLOCK);
write(efd, &count, sizeof(count));  // Signal

struct signalfd_siginfo info;
read(sfd, &info, sizeof(info));     // Wait for signal

struct drm_amdgpu_fence fence = { .seq_no = 42 };
// Wait until GPU completes sequence number 42
```

**Ordo Application:**
```rust
pub trait Completable {
    type Event;
    
    /// Check if completion event is ready (non-blocking)
    fn poll(&self) -> Option<Self::Event>;
    
    /// Block until completion (blocking)
    fn wait(&self) -> Self::Event;
    
    /// Wait with timeout
    fn wait_timeout(&self, dur: Duration) -> Option<Self::Event>;
}

pub struct CompletionHandle {
    event_fd: RawFd,
    sequence: u64,
}

impl Completable for CompletionHandle {
    type Event = CompletionEvent;
    
    fn poll(&self) -> Option<Self::Event> {
        let mut buf = [0u8; 8];
        match libc::read(self.event_fd, buf.as_mut_ptr() as *mut _, 8) {
            n if n > 0 => Some(CompletionEvent { sequence: self.sequence }),
            _ => None,
        }
    }
    
    fn wait(&self) -> Self::Event {
        loop {
            if let Some(event) = self.poll() {
                return event;
            }
            // Use epoll/kqueue to wait efficiently
            wait_for_readable(self.event_fd);
        }
    }
}
```

**Files to read:** `output/eventfd_analysis.md`, `output/signalfd_analysis.md`, `output/timerfd_analysis.md`, `output/futex_analysis.md`

---

### D. Handle-Based Resource Lifecycle (from FD model)

**Linux:**
```c
int fd = open("/dev/dri/card0", O_RDWR);
// Configure via ioctl
drmModeAtomicReqPtr req = drmModeAtomicAlloc();
drmModeAtomicAddProperty(req, crtc_id, "ACTIVE", 1);
drmModeAtomicCommit(fd, req, DRM_MODE_ATOMIC_NONBLOCK, NULL);
// Use
// ...
close(fd);  // Explicit lifecycle end
```

**Ordo Application:**
```rust
pub trait Resource: Sized {
    type Config;
    type Handle;
    
    /// Create resource in unconfigured state
    fn create() -> Result<Self, ResourceError>;
    
    /// Configure resource (transition to configured state)
    fn configure(&mut self, config: Self::Config) -> Result<(), ResourceError>;
    
    /// Activate resource (transition to active state)
    fn activate(&mut self) -> Result<(), ResourceError>;
    
    /// Deactivate and destroy
    fn destroy(self) -> Result<(), ResourceError>;
}

pub enum ResourceState<T: Resource> {
    Created(T),
    Configured(T),
    Active(T),
    Draining(T),
    Closed,
}

impl<T: Resource> ResourceState<T> {
    pub fn configure(&mut self, config: T::Config) -> Result<(), ResourceError> {
        match self {
            ResourceState::Created(resource) => {
                resource.configure(config)?;
                *self = ResourceState::Configured(resource.take());
                Ok(())
            }
            _ => Err(ResourceError::InvalidState),
        }
    }
}
```

**Files to read:** `output/fcntl_analysis.md`, `output/fd_analysis.md`, `output/drm_analysis.md`

---

### E. Multi-Engine Scheduling (from GPU command rings, VirtIO)

**Linux:**
```c
// GPU has multiple independent engines
enum IpType { GFX, COMPUTE, DMA, UVD, VCE };

// Submit to specific engine ring
struct drm_amdgpu_cs_ib {
    uint64_t va_start;
    uint32_t ib_bytes;
    uint32_t ip_type;  // Which engine
    uint32_t ring;     // Which ring within engine
};

// VirtIO virtqueue rings
struct vring {
    uint32_t num;           // Number of descriptors
    void *desc;             // Descriptor table
    void *avail;            // Available ring (driver → device)
    void *used;             // Used ring (device → driver)
};
```

**Ordo Application:**
```rust
pub struct Scheduler {
    lanes: Vec<ExecutionLane>,
}

pub struct ExecutionLane {
    id: LaneId,
    kind: LaneKind,  // Compute, IO, Network, GPU, etc.
    queue: VecDeque<Job>,
    current: Option<Job>,
    completion_tx: mpsc::Sender<CompletionEvent>,
}

pub enum LaneKind {
    Compute { priority: Priority },
    IO { async_fd: RawFd },
    Network { socket: Socket },
    GPU { device_id: DeviceId },
    Timer { interval: Duration },
}

impl Scheduler {
    pub fn submit(&mut self, job: Job, lane_id: LaneId) -> CompletionHandle {
        let lane = &mut self.lanes[lane_id.0];
        let (tx, rx) = mpsc::channel();
        lane.queue.push_back(Job { job, completion_tx: tx });
        lane.try_advance();
        CompletionHandle::new(rx)
    }
}
```

**Files to read:** `output/amdgpu_drm_analysis.md`, `output/virtio_pci_analysis.md`, `output/virtio_net_analysis.md`

---

### F. Versioned Interfaces (from DRM, BPF, KVM)

**Linux:**
```c
struct drm_version {
    int version_major;
    int version_minor;
    int version_patchlevel;
    // Variable-length strings follow
};

// Feature negotiation
u64 features = ioctl(fd, KVM_GET_SUPPORTED_CPUID);
if (features & KVM_FEATURE_X) {
    // Use feature X
}
```

**Ordo Application:**
```rust
#[derive(Debug, Clone)]
pub struct ProtocolVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProtocolVersion {
    pub const CURRENT: Self = ProtocolVersion {
        major: 0,
        minor: 1,
        patch: 0,
    };
    
    pub fn is_compatible(&self, other: &ProtocolVersion) -> bool {
        self.major == other.major  // Major version must match
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FeatureFlags(u64);

impl FeatureFlags {
    pub const SUBAGENTS: Self = FeatureFlags(1 << 0);
    pub const RAG: Self = FeatureFlags(1 << 1);
    pub const CANVAS: Self = FeatureFlags(1 << 2);
    pub const TTS: Self = FeatureFlags(1 << 3);
    pub const IMAGE_GEN: Self = FeatureFlags(1 << 4);
    
    pub fn negotiate(local: Self, remote: Self) -> Self {
        Self(local.0 & remote.0)  // Intersection of supported features
    }
}
```

**Files to read:** `output/kvm_analysis.md`, `output/bpf_analysis.md`, `output/drm_analysis.md`

---

### G. Error Handling (from errno)

**Linux:**
```c
if (ret == -1) {
    switch (errno) {
        case EAGAIN:  // Try again (non-blocking)
        case EACCES:  // Permission denied
        case EINVAL:  // Invalid argument
        case ENOMEM:  // Out of memory
        case EBUSY:   // Resource busy
        case ENOENT:  // Not found
        case EPERM:   // Operation not permitted
        // ... 130+ errno values
    }
}
```

**Ordo Application:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Error {
    // Standard POSIX errors (mapped from libc::E*)
    Again = 11,           // Try again
    AccessDenied = 13,    // Permission denied
    BadFd = 9,            // Bad file descriptor
    Busy = 16,            // Resource busy
    Interrupted = 4,      // Interrupted system call
    InvalidArg = 22,      // Invalid argument
    Io = 5,               // I/O error
    NoMemory = 12,        // Out of memory
    NotFound = 2,         // No such file or directory
    NotPermitted = 1,     // Operation not permitted
    TimedOut = 110,       // Connection timed out
    
    // Ordo-specific errors (negative values to avoid collision)
    SubagentCrashed = -1,
    ProtocolViolation = -2,
    CapabilityMissing = -3,
    ResourceExhausted = -4,
    InvalidState = -5,
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Files to read:** `output/errno_analysis.md`

---

## Common Primitive Patterns Reference

### 1. Bitmask Flags

```rust
bitflags! {
    pub struct Flags: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
        const NONBLOCK = 1 << 3;
        const CLOEXEC = 1 << 4;
        // ...
    }
}
```

**Examples in extraction:** `output/fcntl_analysis.md`, `output/mman_analysis.md`, `output/if.h_analysis.md`

---

### 2. TLV (Type-Length-Value) Attributes

```rust
#[repr(C)]
pub struct NlAttr {
    pub len: u16,
    pub attr_type: u16,
    // Followed by variable-length payload
}

pub fn parse_attrs(data: &[u8]) -> Result<Vec<(u16, &[u8])>> {
    let mut attrs = Vec::new();
    let mut pos = 0;
    
    while pos + 4 <= data.len() {
        let len = u16::from_ne_bytes([data[pos], data[pos + 1]]) as usize;
        let attr_type = u16::from_ne_bytes([data[pos + 2], data[pos + 3]]);
        
        if len < 4 || pos + len > data.len() {
            return Err(ParseError::InvalidLength);
        }
        
        let payload = &data[pos + 4..pos + len];
        attrs.push((attr_type, payload));
        
        // Align to 4-byte boundary
        pos += (len + 3) & !3;
    }
    
    Ok(attrs)
}
```

**Examples in extraction:** `output/netlink_analysis.md`, `output/nl80211_analysis.md`, `output/nf_tables_analysis.md`

---

### 3. Ring Buffers

```rust
#[repr(C)]
pub struct RingBuffer {
    pub head: AtomicUsize,  // Producer writes here
    pub tail: AtomicUsize,  // Consumer reads here
    pub mask: usize,        // Size - 1 (power of 2)
    pub data: [u8; 0],      // Flexible array member
}

impl RingBuffer {
    pub fn push(&self, item: &[u8]) -> Result<()> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);
        let next_head = (head + 1) & self.mask;
        
        if next_head == tail {
            return Err(Error::Full);
        }
        
        // Write item at head position
        unsafe {
            ptr::copy_nonoverlapping(
                item.as_ptr(),
                self.data.as_mut_ptr().add(head * ITEM_SIZE),
                item.len(),
            );
        }
        
        self.head.store(next_head, Ordering::Release);
        Ok(())
    }
}
```

**Examples in extraction:** `output/perf_event_analysis.md`, `output/virtio_pci_analysis.md`

---

### 4. State Machines

```rust
pub enum ConnectionState {
    Disconnected,
    Connecting { addr: SocketAddr },
    Authenticating { challenge: Vec<u8> },
    Connected { session_key: SessionKey },
    Draining,
    Closed,
}

impl ConnectionState {
    pub fn transition(&mut self, event: ConnectionEvent) -> Result<()> {
        use ConnectionState::*;
        
        match (self, event) {
            (Disconnected, Connect(addr)) => {
                *self = Connecting { addr };
                Ok(())
            }
            (Connecting { .. }, AuthChallenge(challenge)) => {
                *self = Authenticating { challenge };
                Ok(())
            }
            (Authenticating { .. }, AuthSuccess(key)) => {
                *self = Connected { session_key: key };
                Ok(())
            }
            (Connected { .. }, Disconnect) => {
                *self = Draining;
                Ok(())
            }
            (Draining, DrainComplete) => {
                *self = Closed;
                Ok(())
            }
            _ => Err(Error::InvalidStateTransition),
        }
    }
}
```

**Examples in extraction:** `output/nl80211_analysis.md` (WiFi connect state machine), `output/asound_analysis.md` (PCM state machine)

---

## Next Steps After Bootstrap

1. **Pick a pattern** that resonates with an Ordo design problem you're solving
2. **Read the relevant extraction files** to understand how Linux implements it
3. **Draft an Ordo equivalent** using Rust idioms
4. **Test the design** against real use cases
5. **Document in `claw-protocol`** or relevant Ordo crate

---

## Questions to Ask While Reading

As you browse the extracted primitives, ask:

1. **What problem does this solve?** (e.g., "How do we safely expose device capabilities to userspace?")
2. **What are the trade-offs?** (e.g., "ioctl is simple but doesn't scale; netlink scales but is complex")
3. **How does this handle errors?** (e.g., "Return -1 and set errno vs. Result<T, Error>")
4. **How is versioning handled?** (e.g., "Struct size checks vs. explicit version fields")
5. **What would the Rust equivalent look like?** (e.g., "bitflags! for capability bitmasks")
6. **Does this violate Ordo's architecture?** (e.g., "Linux uses global namespaces; Ordo should use capability-based isolation")

---

## Contact / Updates

If you find additional primitives worth extracting or identify patterns that should be added to Ordo's design docs, update this bootstrap file and the relevant summary documents.

**Location:** `F:\OPENCLAW-PROJECTS\linux-primitives-extracted\`

**Tool:** `scripts/parse_uapi.py` - Can be re-run on any new headers as needed
