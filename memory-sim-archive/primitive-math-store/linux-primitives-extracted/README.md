# Linux Kernel Primitives Extraction

**Source:** `F:\OPENCLAW-PROJECTS\linux-master` (Linux kernel v6.x source tree)

**Purpose:** Extract conceptual primitives from Linux kernel's user API headers to inform Ordo's architecture design.

---

## What We're Extracting

Not code. Not functions. **Design patterns and interface contracts:**

- How Linux encodes commands (ioctl scheme)
- How Linux represents capabilities (bit vectors, enums)
- How Linux handles versioning (struct sizes, feature flags)
- How Linux signals errors (negative errno)
- How Linux manages resources (file descriptor model)
- How Linux exposes subsystems (netlink, sysfs, debugfs)

These become **design inspiration** for:
- `claw-protocol` - Message encoding and routing
- `ordo-protocol` - Subsystem interface contracts
- `cap-*` crates - Capability primitives

---

## Methodology

1. **Read uapi headers** - User-visible ABI contracts in `include/uapi/`
2. **Categorize primitives** - Group by function (IPC, sync, memory, device, etc.)
3. **Extract patterns** - Common encoding schemes, struct layouts, error handling
4. **Map to Ordo** - Propose equivalent primitives for Ordo's architecture

---

## Extracted Categories

| Category | Source Files | Output Doc | Status |
|----------|-------------|------------|--------|
| Syscall Encoding | `ioctl.h`, `syscall.h`, `unistd.h` | `01-syscall-encoding.md` | Pending |
| Resource Handles | `fcntl.h`, `file.h`, `fd.h` | `02-resource-handles.md` | Pending |
| IPC Primitives | `ipc.h`, `socket.h`, `netlink.h`, `shm.h` | `03-ipc-primitives.md` | Pending |
| Sync Primitives | `futex.h`, `mutex.h`, `semaphore.h` | `04-sync-primitives.md` | Pending |
| Capability Model | `capabilities.h`, `prctl.h` | `05-capability-model.md` | Pending |
| Device Model | `device.h`, `driver.h`, `platform_device.h` | `06-device-model.md` | Pending |
| Event Streaming | `inotify.h`, `perf_event.h`, `signalfd.h`, `eventfd.h` | `07-event-streaming.md` | Pending |
| Memory Primitives | `mman.h`, `dma-buf.h`, `memfd.h` | `08-memory-primitives.md` | Pending |
| Network Primitives | `socket.h`, `netlink.h`, `packet.h`, `ethtool.h` | `09-network-primitives.md` | Pending |
| Filesystem Primitives | `stat.h`, `fcntl.h`, `inotify.h`, `fanotify.h` | `10-filesystem-primitives.md` | Pending |
| Graphics/Media | `drm.h`, `videodev2.h`, `asound.h` | `11-graphics-media.md` | Pending |
| Ordo Mappings | Analysis + proposals | `99-ordo-mappings.md` | Pending |

---

## Key Insights (So Far)

### 1. ioctl Command Encoding (32-bit scheme)

```
 31                  16 15                   8 7                   0
 +----------------------+----------------------+----------------------+
 |   Direction (2b)     |   Size (14b)         |   Type (8b) | Nr (8b)|
 +----------------------+----------------------+----------------------+
```

- **Direction:** None/Read/Write/ReadWrite (userland ↔ kernel)
- **Size:** Parameter struct size (max 16KB)
- **Type:** Subsystem identifier (magic number)
- **Nr:** Command number within subsystem

**Ordo Application:** Message type encoding in `claw-protocol`

### 2. File Descriptor Model

Everything is a fd:
- Files → `open()`
- Sockets → `socket()`
- Events → `eventfd()`, `signalfd()`, `timerfd()`
- Monitoring → `inotify_init()`, `perf_event_open()`
- Devices → `open(/dev/...)`

Operations: `read()`, `write()`, `ioctl()`, `mmap()`, `close()`

**Ordo Application:** Capability tokens as handle-based resources

### 3. Error Handling Convention

- Return `-1` on error
- Set `errno` to positive error code
- Error codes are negative in kernel (`-EINVAL`, `-ENOMEM`)
- Userland sees positive `errno`

Common errors: `EAGAIN`, `EINTR`, `EINVAL`, `ENOMEM`, `EACCES`, `EPERM`, `ENOENT`, `EBUSY`

**Ordo Application:** Result types with explicit error variants

### 4. Capability Model

Fine-grained permissions (vs. root/non-root):
- `CAP_NET_ADMIN` - Network configuration
- `CAP_SYS_ADMIN` - Catch-all admin
- `CAP_DAC_OVERRIDE` - Bypass file permissions
- `CAP_KILL` - Send signals to other processes
- ~40 total capabilities~

**Ordo Application:** Permission grants for MCP servers

### 5. Netlink Socket Messaging

Kernel-userspace messaging via sockets:
- Family ID (e.g., `NETLINK_ROUTE`, `NETLINK_FIREWALL`)
- Message type within family
- Sequence numbers for request/response matching
- Multipart messages for large data

**Ordo Application:** Bus message topics with request/response correlation

---

## Directory Structure

```
F:\OPENCLAW-PROJECTS\linux-primitives-extracted\
├── README.md                    # This file
├── 01-syscall-encoding.md       # ioctl, syscall numbers
├── 02-resource-handles.md       # fd model, reference counting
├── 03-ipc-primitives.md         # IPC mechanisms
├── 04-sync-primitives.md        # Futex, mutex patterns
├── 05-capability-model.md       # Capabilities, namespaces
├── 06-device-model.md           # Driver model, sysfs
├── 07-event-streaming.md        # Event fds, monitoring
├── 08-memory-primitives.md      # mmap, dma-buf
├── 09-network-primitives.md     # Sockets, netlink
├── 10-filesystem-primitives.md  # VFS, notifications
├── 11-graphics-media.md         # DRM, V4L2, ALSA
└── 99-ordo-mappings.md          # Proposed Ordo equivalents
```

---

## Tools

### Header Parser Script

Location: `scripts/parse_uapi.py` (to be written)

Extracts:
- `#define` constants by category
- Struct definitions
- Ioctl command encodings
- Include dependencies

### Cross-Reference Analyzer

Location: `scripts/analyze_deps.py` (to be written)

Generates:
- Header dependency graph
- Related primitive groupings

---

## Next Steps

1. ✅ Create extraction directory
2. ✅ Write extraction plan (this file)
3. ⏳ Parse key uapi headers
4. ⏳ Generate category documents
5. ⏳ Write Ordo mapping proposals

---

## References

- Linux Kernel Docs: https://www.kernel.org/doc/html/latest/
- LWN Kernel Page: https://lwn.net/Kernel/
- Rust for Linux: https://rust-for-linux.com/
- Linux UAPI Headers: `F:\OPENCLAW-PROJECTS\linux-master\include\uapi\`
