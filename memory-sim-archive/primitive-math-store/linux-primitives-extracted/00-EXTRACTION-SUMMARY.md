# Linux Kernel Primitives Extraction Summary

**Date:** 2026-06-20  
**Source:** `F:\OPENCLAW-PROJECTS\linux-master` (Linux kernel v6.x)  
**Tool:** `scripts/parse_uapi.py`

---

## Extraction Results

**Headers Parsed:** 19 key uapi headers  
**Total Output:** ~137 KB of structured analysis

| Category | Headers | Defines | Structs | Key Insights |
|----------|---------|---------|---------|--------------|
| **IPC & Messaging** | `netlink.h`, `socket.h` | 76 | 10 | Netlink: family/type/seq multipart messages |
| **Synchronization** | `futex.h` | 58 | 4 | FUTEX_WAIT/WAKE/REQUEUE, priority inheritance |
| **Event FDs** | `eventfd.h`, `signalfd.h`, `timerfd.h`, `inotify.h` | 38 | 2 | Everything as waitable fd |
| **Memory** | `mman.h`, `dma-buf.h` | 39 | 5 | mmap flags, DMA buffer sharing protocol |
| **Capabilities** | `capability.h` | 66 | 6 | 40+ capability bits, versioned structs |
| **Process Control** | `prctl.h`, `ptrace.h`, `resource.h` | 151 | 12 | Process attributes, debugging, limits |
| **File Operations** | `fcntl.h`, `stat.h`, `sysctl.h` | 116 | 4 | File flags, metadata, hierarchical params |
| **Observability** | `perf_event.h` | 147 | 39 | Hardware counters, tracepoints, sampling |
| **Sandboxing** | `bpf.h` | 65 | 115 | BPF program types, maps, helpers |
| **Virtualization** | `kvm.h` | 606 | 87 | VM/vCPU lifecycle, memory slots, interrupts |
| **Command Encoding** | `ioctl.h` (asm-generic) | 15 | 0 | 32-bit ioctl scheme (dir/type/nr/size) |

---

## Key Primitive Patterns Extracted

### 1. Command Encoding (ioctl)

**32-bit encoding scheme:**
```
Bits 31-30: Direction (NONE/READ/WRITE/INOUT)
Bits 29-16: Size of parameter struct (max 16KB)
Bits 15-8:  Type ( subsystem magic number)
Bits 7-0:   Nr (command number within subsystem)
```

**Macros:**
- `_IO(type, nr)` - No data transfer
- `_IOR(type, nr, argtype)` - Read from kernel
- `_IOW(type, nr, argtype)` - Write to kernel
- `_IOWR(type, nr, argtype)` - Bidirectional

**Ordo Application:** Message type encoding in `claw-protocol` could use similar bit-field scheme for routing + dispatch.

---

### 2. Capability Model

**40+ discrete capabilities** (vs. binary root/non-root):

| Category | Capabilities |
|----------|-------------|
| `CAP_NET_*` | `BIND_SERVICE`, `BROADCAST`, `ADMIN`, `RAW` |
| `CAP_SYS_*` | `MODULE`, `RAWIO`, `CHROOT`, `PTRACE`, `ADMIN`, `BOOT`, `NICE`, `RESOURCE`, `TIME`, `TTY_CONFIG` |
| `CAP_DAC_*` | `OVERRIDE`, `READ_SEARCH` |
| `CAP_IPC_*` | `LOCK`, `OWNER` |
| `CAP_AUDIT_*` | `WRITE`, `CONTROL`, `READ` |
| `CAP_MAC_*` | `OVERRIDE`, `ADMIN` |
| Standalone | `CHOWN`, `FOWNER`, `FSETID`, `KILL`, `SETGID`, `SETUID`, `SETPCAP`, `MKNOD`, `LEASE`, `SETFCAP`, `SYSLOG`, `PERFMON`, `BPF`, `CHECKPOINT_RESTORE`, `BLOCK_SUSPEND`, `WAKE_ALARM` |

**Data structures:**
```c
struct __user_cap_header_struct {
    __u32 version;
    int pid;
};

struct __user_cap_data_struct {
    __u32 effective;    // Bit vector
    __u32 permitted;    // Bit vector
    __u32 inheritable;  // Bit vector
};
```

**Versioning:** `_LINUX_CAPABILITY_VERSION_3` (current), with UAPI version constants for backward compat.

**Ordo Application:** Permission grants for MCP servers could use similar bit-vector approach with explicit capability names.

---

### 3. Event FDs (Everything is Waitable)

Linux turns everything into file descriptors that can be `poll()`/`select()`/`epoll()`'d:

| Interface | Purpose | Key Struct/Macro |
|-----------|---------|------------------|
| `eventfd.h` | Counter-based event signaling | `EFD_SEMAPHORE`, `EFD_CLOEXEC`, `EFD_NONBLOCK` |
| `signalfd.h` | Signals as fd | `struct signalfd_siginfo` (128 bytes) |
| `timerfd.h` | Timers as fd | `TFD_CLOEXEC`, `TFD_NONBLOCK`, `TFD_TIMER_ABSTIME` |
| `inotify.h` | Filesystem monitoring | `struct inotify_event`, `IN_ACCESS`, `IN_MODIFY`, etc. (28 events) |

**Pattern:** Single fd represents a stream of events. Read returns struct array. Non-blocking mode with `EAGAIN` for no data.

**Ordo Application:** Bus subscriptions could expose event streams as handle-based resources with similar non-blocking read semantics.

---

### 4. Futex (Fast Userspace Mutex)

**Core operations:**
- `FUTEX_WAIT` - Sleep if value unchanged
- `FUTEX_WAKE` - Wake waiters
- `FUTEX_REQUEUE` - Move waiters between futexes
- `FUTEX_CMP_REQUEUE` - Conditional requeue
- `FUTEX_WAIT_BITSET` - Wait with filter
- `FUTEX_LOCK_PI` - Priority inheritance lock
- `FUTEX_UNLOCK_PI` - Priority inheritance unlock

**Key insight:** Fast path is purely userspace (atomic compare-and-swap). Kernel only involved on contention.

**Ordo Application:** Async wait primitives in Ordo could use similar "wait if unchanged" semantics for efficient state polling.

---

### 5. Netlink Sockets

**Kernel-userspace messaging via sockets:**

**Families:**
- `NETLINK_ROUTE` (0) - Routing/device config
- `NETLINK_FIREWALL` (3) - Firewall rules
- `NETLINK_NFLOG` (7) - Netfilter logging
- `NETLINK_XFRM` (6) - IPsec config
- `NETLINK_AUDIT` (9) - Audit subsystem
- `NETLINK_INET_DIAG` (22) - Socket stats
- `NETLINK_BPF` (29) - BPF events
- ...and 30+ more

**Message structure:**
```c
struct nlmsghdr {
    __u32 nlmsg_len;     // Length including header
    __u16 nlmsg_type;    // Type within family
    __u16 nlmsg_flags;   // NLM_F_REQUEST, NLM_F_MULTI, etc.
    __u32 nlmsg_seq;     // Sequence number (request/response matching)
    __u32 nlmsg_pid;     // Port ID (sender ID)
};
```

**Flags:** `NLM_F_REQUEST`, `NLM_F_MULTI` (multipart), `NLM_F_ACK`, `NLM_F_DUMP` (dump all)

**Ordo Application:** Bus message topics with `seq` for request/response correlation, `NLM_F_MULTI` for streaming responses.

---

### 6. BPF (Sandboxed Bytecode)

**Massive API surface:**
- **115 struct definitions** - Program types, map types, instructions
- **Program types:** `BPF_PROG_TYPE_SOCKET_FILTER`, `KPROBE`, `TRACEPOINT`, `XDP`, `CGROUP_SOCK`, `LWT_IN`, `LWT_OUT`, `LWT_XMIT`, `SK_MSG`, `SK_SKB`, `CGROUP_DEVICE`, `SK_REUSEPORT`, `FLOW_DISSECTOR`, `CGROUP_SYSCTL`, `CGROUP_SOCKOPT`, `TRACING`, `STRUCT_OPS`, `EXT`, `LSM`, `SK_LOOKUP`, `SYSCALL`
- **Map types:** `ARRAY`, `HASH`, `LRU_HASH`, `LRU_PERCPU_HASH`, `RINGBUF`, `BLOOM_FILTER`, `STACK_TRACE`, `CGROUP_ARRAY`, `DEVMAP`, `SOCKMAP`, `SOCKHASH`, `OFFLOADMAP`, `QUEUE`, `PERCPU_ARRAY`, `PERCPU_HASH`, `REUSEPORT_SOCKARRAY`, `CPUMAP`, `XSKMAP`, `HSHTABLE`
- **Helper functions:** 100+ helpers for packet manipulation, map access, printing, time, crypto, bpf2bpf calls

**Ioctl commands:** `BPF_MAP_CREATE`, `BPF_PROG_LOAD`, `BPF_OBJ_PIN`, `BPF_OBJ_GET`, `BPF_PROG_ATTACH`, `BPF_PROG_DETACH`, `BPF_PROG_TEST_RUN`, `BPF_PROG_GET_NEXT_ID`, `BPF_MAP_GET_NEXT_ID`, `BPF_PROG_GET_FD_BY_ID`, `BPF_MAP_GET_FD_BY_ID`, `BPF_OBJ_GET_INFO_BY_FD`, `BPF_PROG_QUERY`, `BPF_RAW_TRACEPOINT_OPEN`, `BPF_BTF_LOAD`, `BPF_BTF_GET_FD_BY_ID`, `BPF_TASK_FD_QUERY`, `BPF_MAP_LOOKUP_AND_DELETE_ELEM`, `BPF_MAP_FREEZE`, `BPF_BTF_GET_NEXT_ID`, `BPF_ITER_CREATE`, `BPF_LINK_CREATE`, `BPF_LINK_UPDATE`, `BPF_LINK_GET_FD_BY_ID`, `BPF_LINK_GET_NEXT_ID`, `BPF_ENABLE_STATS`, `BPF_ITER_CREATE`, `BPF_PROG_BIND_MAP`

**Ordo Application:** Sandboxed plugin system could use similar "program type + map type + helper allowlist" model for extensibility without compromising safety.

---

### 7. KVM (Virtualization)

**Largest header:** 606 defines, 87 structs

**Key primitives:**
- **VM lifecycle:** `KVM_CREATE_VM`, `KVM_DESTROY_VM`
- **vCPU lifecycle:** `KVM_CREATE_VCPU`, `KVM_RUN`, `KVM_GET_REGS`, `KVM_SET_REGS`
- **Memory management:** `KVM_SET_USER_MEMORY_REGION` (memory slots), `KVM_SET_TSS_ADDR`
- **Interrupt handling:** `KVM_IRQ_LINE`, `KVM_INTERRUPT`, `KVM_IOEVENTFD`
- **Device assignment:** `KVM_ASSIGN_DEV_IRQ`, `KVM_SET_PCI_MSIX_ENTRY`
- **MSI/MSI-X:** `KVM_SIGNAL_MSI`, `KVM_ASSIGN_SET_MSIX_ENTRY`

**Exit reasons** (vCPU → userspace):
- `KVM_EXIT_UNKNOWN`, `KVM_EXIT_EXCEPTION`, `KVM_EXIT_IO`, `KVM_EXIT_HYPERCALL`, `KVM_EXIT_DEBUG`, `KVM_EXIT_MMIO`, `KVM_EXIT_IRQ_WINDOW_OPEN`, `KVM_EXIT_SHUTDOWN`, `KVM_EXIT_FAIL_ENTRY`, `KVM_EXIT_INTR`, `KVM_EXIT_SET_TPR`, `KVM_EXIT_TPR_ACCESS`, `KVM_EXIT_S390_SIEIC`, `KVM_EXIT_S390_RESET`, `KVM_EXIT_DCR`, `KVM_EXIT_NMI`, `KVM_EXIT_INTERNAL_ERROR`, `KVM_EXIT_OSI`, `KVM_EXIT_PAPR_HCALL`, `KVM_EXIT_EPR`, `KVM_EXIT_WATCHDOG`, `KVM_EXIT_SYSTEM_EVENT`, `KVM_EXIT_S390_STSI`, `KVM_EXIT_IOAPIC_EOI`, `KVM_EXIT_HYPERV`, `KVM_EXIT_ARM_NISV`, `KVM_EXIT_X86_RDMSR`, `KVM_EXIT_X86_WRMSR`, `KVM_EXIT_DIRTY_RING_FULL`, `KVM_EXIT_AP_RESET_HOLD`, `KVM_EXIT_X86_BUS_LOCK`, `KVM_EXIT_XEN`, `KVM_EXIT_RISCV_SBI`, `KVM_EXIT_RISCV_INSNS`, `KVM_EXIT_LOONGARCH_IOCSR`, `KVM_EXIT_LOONGARCH_CPUCFG`

**Ordo Application:** Subagent lifecycle management could mirror KVM's VM/vCPU model with explicit run → exit → handle → resume loop.

---

### 8. Perf Event (Observability)

**Hardware + software event sampling:**

**Event types:**
- Hardware: `PERF_TYPE_HARDWARE` (CPU cycles, instructions, cache refs, cache misses, branches, branch misses, bus cycles, stalled cycles frontend/backend)
- Software: `PERF_TYPE_SOFTWARE` (CPU clock, task clock, page faults, context switches, minor/major faults, alignment faults, emulation faults, wakeup, bpf output, cgroup switch)
- Tracepoints: `PERF_TYPE_TRACEPOINT`
- HW cache: `PERF_TYPE_HW_CACHE` (L1-D/L1-I/LLC/DTLB/ITLB/BPU caches, ops: read/write/prefetch, results: access/miss/prefetch_access)
- Raw: `PERF_TYPE_RAW` (architecture-specific PMU events)
- Core power: `PERF_TYPE_POWER`

**Structures:** 39 structs including `perf_event_attr` (72 bytes), `perf_event_mmap_page`, `perf_event_header`, `perf_sample_id`, various event records (`PERF_RECORD_MMAP`, `PERF_RECORD_LOST`, `PERF_RECORD_COMM`, `PERF_RECORD_EXIT`, `PERF_RECORD_THROTTLE`, `PERF_RECORD_UNTHROTTLE`, `PERF_RECORD_FORK`, `PERF_RECORD_READ`, `PERF_RECORD_SAMPLE`, `PERF_RECORD_SWITCH`, `PERF_RECORD_NAMESPACES`, `PERF_RECORD_KSYMBOL`, `PERF_RECORD_BPF_EVENT`, `PERF_RECORD_CGROUP`, `PERF_RECORD_TEXT_POKE`)

**Ordo Application:** Runtime observability could use similar event categorization (hardware metrics, software events, tracepoints) with unified sampling interface.

---

## Patterns for Ordo

### A. Handle-Based Resources

Linux: File descriptors (integers) with `read()`, `write()`, `ioctl()`, `mmap()`, `close()`

Ordo equivalent: Capability tokens (UUIDs or opaque handles) with explicit operations:
```rust
enum CapabilityOp {
    Read { buf: Vec<u8> },
    Write { data: Vec<u8> },
    Ioctl { command: u32, payload: Vec<u8> },
    Close,
}
```

### B. Versioned Interfaces

Linux: `_LINUX_CAPABILITY_VERSION_3`, struct size checks, feature flags

Ordo equivalent: Protocol versioning in `claw-protocol`:
```rust
#[repr(C)]
struct MessageHeader {
    version: u32,
    message_type: u16,
    flags: u16,
    payload_size: u32,
    sequence: u32,
    correlation_id: u64,
}
```

### C. Multipart Responses

Linux: `NLM_F_MULTI` flag + `NLMSG_DONE` terminator

Ordo equivalent: Streaming bus messages with continuation flag:
```rust
struct BusMessage {
    topic: String,
    seq: u32,
    flags: MessageFlags, // CONTINUATION, FINAL, ERROR
    payload: Vec<u8>,
}
```

### D. Error Handling

Linux: Return `-1`, set `errno` (positive). Kernel uses `-EINVAL`, `-ENOMEM`, etc.

Ordo equivalent: `Result<T, ErrorCode>` with explicit error enum:
```rust
enum ErrorCode {
    InvalidArg = 22,      // EINVAL
    NoMemory = 12,        // ENOMEM
    AccessDenied = 13,    // EACCES
    NotPermitted = 1,     // EPERM
    NotFound = 2,         // ENOENT
    Busy = 16,            // EBUSY
    Again = 11,           // EAGAIN
    Interrupted = 4,      // EINTR
    // ...map all relevant errno values
}
```

### E. Ioctl-like Command Encoding

Linux: 32-bit encoding (dir/type/nr/size)

Ordo equivalent: Message type field with bit-fields:
```
Bits 31-30: Direction (Request/Response/Notification)
Bits 29-20: Subsystem ID (capability domain)
Bits 19-12: Operation code
Bits 11-0:  Flags
```

---

## Next Steps

1. ✅ Parse initial 19 headers
2. ⏳ Expand to full uapi catalog (~575 headers in `include/uapi/linux/`)
3. ⏳ Write category summary documents (`01-syscall-encoding.md`, etc.)
4. ⏳ Draft `99-ordo-mappings.md` with concrete proposals for `claw-protocol`
5. ⏳ Review with Jesse: which patterns resonate with Ordo's architecture?

---

## Files Generated

```
F:\OPENCLAW-PROJECTS\linux-primitives-extracted\
├── README.md                      # Overview
├── 00-EXTRACTION-SUMMARY.md       # This file
├── scripts/
│   └── parse_uapi.py              # Header parser
└── output/
    ├── bpf_analysis.md            # 37 KB, 115 structs
    ├── capability_analysis.md     # 40 capabilities
    ├── dma-buf_analysis.md
    ├── eventfd_analysis.md
    ├── fcntl_analysis.md
    ├── futex_analysis.md
    ├── inotify_analysis.md
    ├── ioctl_analysis.md          # Command encoding
    ├── kvm_analysis.md            # 45 KB, 87 structs
    ├── mman_analysis.md
    ├── netlink_analysis.md        # Messaging protocol
    ├── perf_event_analysis.md     # 12 KB, 39 structs
    ├── prctl_analysis.md
    ├── ptrace_analysis.md
    ├── resource_analysis.md
    ├── signalfd_analysis.md
    ├── socket_analysis.md
    ├── stat_analysis.md
    ├── sysctl_analysis.md
    ├── timerfd_analysis.md
    └── *_data.json                # Machine-readable extracts
```

---

**Total primitives extracted:** 1,200+ defines, 280+ structs, 20+ distinct primitive categories
