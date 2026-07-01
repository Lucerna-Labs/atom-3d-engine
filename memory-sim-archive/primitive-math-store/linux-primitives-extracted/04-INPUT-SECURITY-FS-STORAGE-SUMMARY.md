# Input, Security, Filesystem, and Storage Primitives

**Date:** 2026-06-20  
**Source:** `F:\OPENCLAW-PROJECTS\linux-master/include/uapi/linux/`  
**Parsed:** 38 additional headers  
**Size:** ~289 KB analysis

---

## Summary by Category

| Category | Headers | Defines | Structs | Size |
|----------|---------|---------|---------|------|
| **Input Devices** | `input.h`, `uinput.h`, `hidraw.h`, `uhid.h`, `joystick.h`, `kd.h` | 231 | 42 | ~40 KB |
| **Security/Sandboxing** | `kvm.h`, `vfio.h`, `seccomp.h`, `landlock.h`, `audit.h` | 1,168 | 150 | ~100 KB |
| **Process Control** | `prctl.h`, `ptrace.h` | 139 | 9 | ~15 KB |
| **Filesystem Monitoring** | `fanotify.h`, `inotify.h`, `quota.h`, `fsmap.h` | 180 | 15 | ~25 KB |
| **Filesystem IOCTLS** | `btrfs.h`, `btrfs_tree.h`, `ext4.h`, `f2fs.h`, `dm-ioctl.h`, `loop.h`, `cdrom.h` | 765 | 145 | ~80 KB |
| **Storage** | `nvme_ioctl.h`, `usbip.h` | 32 | 4 | ~5 KB |
| **Networking (Advanced)** | `mroute.h`, `mroute6.h`, `ip_vs.h`, `nf_tables.h`, `ppp_*.h` | 352 | 33 | ~24 KB |
| **Total** | **38** | **2,867+** | **398+** | **~289 KB** |

---

## Input Device Primitives

### Event Codes (input-event-codes.h)

**Event types:**
```c
#define EV_SYN          0x00  // Synchronization events
#define EV_KEY          0x01  // Key/button events
#define EV_REL          0x02  // Relative axis events
#define EV_ABS          0x03  // Absolute axis events
#define EV_MSC          0x04  // Miscellaneous events
#define EV_SW           0x05  // Switch events
#define EV_LED          0x10  // LED events
#define EV_SND          0x11  // Sound/buzzer events
#define EV_REP          0x14  // Repeat events
#define EV_FF           0x15  // Force feedback events
#define EV_PWR          0x16  // Power management events
#define EV_FF_STATUS    0x17  // Force feedback status
#define EV_MAX          0x1f  // Maximum event type
```

**Key codes (subset):**
```c
#define KEY_RESERVED    0
#define KEY_ESC         1
#define KEY_1           2
#define KEY_ENTER       28
#define KEY_SPACE       57
#define KEY_UP          103
#define KEY_DOWN        108
#define KEY_LEFT        105
#define KEY_RIGHT       106
// ... 700+ key codes total
```

**Absolute axes:**
```c
#define ABS_X           0x00
#define ABS_Y           0x01
#define ABS_Z           0x02
#define ABS_RX          0x03  // Rotation X
#define ABS_RY          0x04  // Rotation Y
#define ABS_RZ          0x05  // Rotation Z
#define ABS_THROTTLE    0x06
#define ABS_RUDDER      0x07
#define ABS_WHEEL       0x08
#define ABS_GAS         0x09
#define ABS_BRAKE       0x0a
#define ABS_HAT0X       0x10  // Hat switch X
#define ABS_HAT0Y       0x11  // Hat switch Y
// ... through ABS_MAX (0x3f)
```

**Force feedback effects:**
```c
#define FF_RUMBLE       0x50
#define FF_PERIODIC     0x51
#define FF_CONSTANT     0x52
#define FF_SPRING       0x53
#define FF_FRICTION     0x54
#define FF_DAMPER       0x55
#define FF_RAMP_FORCE   0x56
#define FF_SINE         0x59
#define FF_SQUARE       0x5a
#define FF_TRIANGLE     0x5b
#define FF_SAW_UP       0x5c
#define FF_SAW_DOWN     0x5d
```

### Input Event Structure

```c
struct input_event {
    struct timeval time;      // Event timestamp
    __u16 type;               // Event type (EV_*)
    __u16 code;               // Event code (KEY_*, ABS_*, etc.)
    __s32 value;              // Event value
};
```

**Value semantics by type:**
- `EV_KEY`: 0 = release, 1 = press, 2 = auto-repeat
- `EV_REL`: Relative movement (-128 to 127 typical)
- `EV_ABS`: Absolute position (0 to resolution)
- `EV_FF`: Effect ID or command

### uinput - Userspace Input Injection

**Device setup:**
```c
struct uinput_setup {
    char name[UINPUT_MAX_NAME_SIZE];  // Device name
    struct input_id id;                // Vendor/product IDs
    int ff_effects_max;                // Max FF effects
};

struct input_id {
    __u16 bustype;    // BUS_USB, BUS_BLUETOOTH, etc.
    __u16 vendor;
    __u16 product;
    __u16 version;
};
```

**Workflow:**
1. `open("/dev/uinput")` → fd
2. `ioctl(fd, UI_SET_EVBIT, EV_KEY)` - Enable event types
3. `ioctl(fd, UI_SET_KEYBIT, KEY_ENTER)` - Enable specific codes
4. `ioctl(fd, UI_DEV_SETUP, &setup)` - Configure device
5. `ioctl(fd, UI_DEV_CREATE)` - Register device
6. Write `input_event` structs to fd
7. `ioctl(fd, UI_DEV_DESTROY)` - Unregister

**Ordo Application:** Virtual input device creation for automation/testing.

---

## Security & Sandboxing Primitives

### KVM (Virtualization) - 606 defines, 87 structs

**VM lifecycle:**
```c
KVM_CREATE_VM() → vm_fd
  ├─ KVM_CREATE_VCPU() → vcpu_fd
  ├─ KVM_SET_USER_MEMORY_REGION() → Map guest RAM
  ├─ KVM_CREATE_IRQCHIP() → Interrupt controller
  ├─ KVM_CREATE_PIT() → Programmable interval timer
  └─ KVM_RUN() → Execute vCPU until exit
```

**Memory slots:**
```c
struct kvm_userspace_memory_region {
    __u32 slot;              // Slot ID (0-31)
    __u32 flags;             // KVM_MEM_* flags
    __u64 guest_phys_addr;   // Guest physical address
    __u64 memory_size;       // Region size
    __u64 userspace_addr;    // Userspace pointer
};

#define KVM_MEM_LOG_DIRTY_PAGES   (1UL << 0)  // Track dirty pages
#define KVM_MEM_READONLY          (1UL << 1)  // Read-only region
```

**vCPU exit reasons:**
```c
#define KVM_EXIT_UNKNOWN          0
#define KVM_EXIT_EXCEPTION        1
#define KVM_EXIT_IO               2
#define KVM_EXIT_HYPERCALL        3
#define KVM_EXIT_DEBUG            4
#define KVM_EXIT_HLT              5
#define KVM_EXIT_MMIO             6
#define KVM_EXIT_IRQ_WINDOW_OPEN  7
#define KVM_EXIT_SHUTDOWN         8
#define KVM_EXIT_FAIL_ENTRY       9
#define KVM_EXIT_INTR             10
#define KVM_EXIT_SET_TPR          11
#define KVM_EXIT_TPR_ACCESS       12
#define KVM_EXIT_S390_SIEIC       13
#define KVM_EXIT_S390_RESET       14
#define KVM_EXIT_DCR              15
#define KVM_EXIT_NMI              16
#define KVM_EXIT_INTERNAL_ERROR   17
#define KVM_EXIT_OSI              18
#define KVM_EXIT_PAPR_HCALL       19
#define KVM_EXIT_WATCHDOG         20
#define KVM_EXIT_SYSTEM_EVENT     21
// ... 30+ exit reasons total
```

**vCPU registers (x86):**
```c
struct kvm_regs {
    __u64 rax, rbx, rcx, rdx;
    __u64 rsi, rdi, rsp, rbp;
    __u64 r8, r9, r10, r11;
    __u64 r12, r13, r14, r15;
    __u64 rip, rflags;
};

struct kvm_sregs {
    struct kvm_segment cs, ds, es, fs, gs, ss;
    struct kvm_segment tr, gdt, ldt, idt;
    __u64 cr0, cr2, cr3, cr4, cr8;
    __u64 efer;
    __u64 apic_base;
    // ... more control registers
};
```

**MSR access:**
```c
struct kvm_msr_entry {
    __u32 index;    // MSR number
    __u32 reserved;
    __u64 data;     // MSR value
};

struct kvm_msrs {
    __u32 nmsrs;    // Number of MSRs
    __u32 pad;
    struct kvm_msr_entry entries[];
};
```

**Ordo Application:** VM/subagent lifecycle with explicit exit reason handling, memory slot management.

---

### VFIO (Virtual Function I/O) - Device Passthrough

**Container → Group → Device model:**
```
/dev/vfio/vfio (container fd)
  └─ /dev/vfio/<group_id> (IOMMU group fd)
      └─ Device passthrough (PCI, platform, CCW, etc.)
```

**Group status:**
```c
struct vfio_group_status {
    __u32 argsz;
    __u32 flags;
    // VFIO_GROUP_FLAGS_VIABLE: Group can be used
    // VFIO_GROUP_FLAGS_CONTAINER_SET: Container assigned
};
```

**Device regions:**
```c
struct vfio_region_info {
    __u32 argsz;
    __u32 flags;
    __u32 index;          // Region index
    __u32 cap_offset;     // Capability chain offset
    __u64 size;           // Region size
    __u64 offset;         // Region offset for mmap
};

#define VFIO_REGION_INFO_FLAG_READ      (1 << 0)
#define VFIO_REGION_INFO_FLAG_WRITE     (1 << 1)
#define VFIO_REGION_INFO_FLAG_MMAP      (1 << 2)
#define VFIO_REGION_INFO_FLAG_CAPS      (1 << 3)
```

**PCI-specific:**
```c
#define VFIO_PCI_CONFIG_REGION_INDEX    0  // Config space
#define VFIO_PCI_BAR0_REGION_INDEX      1  // BAR0
#define VFIO_PCI_BAR1_REGION_INDEX      2  // BAR1
// ... through BAR5
#define VFIO_PCI_ROM_REGION_INDEX       7  // Option ROM
#define VFIO_PCI_VGA_REGION_INDEX       8  // VGA memory
#define VFIO_PCI_NUM_REGIONS            9
```

**IRQ handling:**
```c
struct vfio_irq_set {
    __u32 argsz;
    __u32 flags;
    __u32 index;          // IRQ index (INTX, MSI, MSI-X)
    __u32 start;          // Starting IRQ number
    __u32 count;          // Number of IRQs
    __u8  data[];         // Eventfd data or IRQ indices
};

#define VFIO_IRQ_SET_DATA_NONE      (1 << 0)  // No data
#define VFIO_IRQ_SET_DATA_BOOL      (1 << 1)  // Boolean trigger
#define VFIO_IRQ_SET_DATA_EVENTFD   (1 << 2)  // Eventfd signaling
#define VFIO_IRQ_SET_ACTION_TRIGGER (1 << 3)  // Trigger IRQ
#define VFIO_IRQ_SET_ACTION_MASK    (1 << 4)  // Mask IRQ
#define VFIO_IRQ_SET_ACTION_UNMASK  (1 << 5)  // Unmask IRQ
```

**DMA mapping:**
```c
struct vfio_iommu_type1_dma_map {
    __u32 argsz;
    __u32 flags;
    __u64 vaddr;          // Userspace virtual address
    __u64 iova;           // I/O virtual address (guest physical)
    __u64 size;           // Mapping size
};

#define VFIO_DMA_MAP_FLAG_READ    (1 << 0)
#define VFIO_DMA_MAP_FLAG_WRITE   (1 << 1)
```

**Ordo Application:** Secure device passthrough with IOMMU isolation, eventfd-based interrupt signaling.

---

### Seccomp (Secure Computing Mode)

**Filter modes:**
```c
#define SECCOMP_MODE_DISABLED     0
#define SECCOMP_MODE_STRICT       1  // Strict mode (predefined syscalls only)
#define SECCOMP_MODE_FILTER       2  // BPF filter mode
```

**BPF filter actions:**
```c
#define SECCOMP_RET_KILL_PROCESS  0x80000000U  // Kill process
#define SECCOMP_RET_KILL_THREAD   0x00000000U  // Kill thread
#define SECCOMP_RET_TRAP          0x00030000U  // SIGSYS trap
#define SECCOMP_RET_ERRNO         0x00050000U  // Return errno
#define SECCOMP_RET_TRACE         0x7ff00000U  // Tracee notification
#define SECCOMP_RET_LOG           0x7ffc0000U  // Log and allow
#define SECCOMP_RET_ALLOW         0x7fff0000U  // Allow syscall
```

**Data return mask:**
```c
#define SECCOMP_RET_DATA          0x0000ffffU  // Action data bits
```

**Filter installation:**
```c
struct seccomp_data {
    int nr;                 // Syscall number
    __u32 arch;             // AUDIT_ARCH_*
    __u64 instruction_pointer;
    __u64 args[6];          // Syscall arguments
};

struct sock_fprog {
    unsigned short len;     // Number of BPF instructions
    struct sock_filter *filter;
};
```

**Ordo Application:** Syscall filtering for sandboxing MCP servers/plugins.

---

### Landlock (Landlock LSM)

**Access rights:**
```c
#define LANDLOCK_ACCESS_FS_EXECUTE      (1ULL << 0)
#define LANDLOCK_ACCESS_FS_WRITE_FILE   (1ULL << 1)
#define LANDLOCK_ACCESS_FS_READ_FILE    (1ULL << 2)
#define LANDLOCK_ACCESS_FS_READ_DIR     (1ULL << 3)
#define LANDLOCK_ACCESS_FS_REMOVE_DIR   (1ULL << 4)
#define LANDLOCK_ACCESS_FS_REMOVE_FILE  (1ULL << 5)
#define LANDLOCK_ACCESS_FS_MAKE_CHAR    (1ULL << 6)
#define LANDLOCK_ACCESS_FS_MAKE_DIR     (1ULL << 7)
#define LANDLOCK_ACCESS_FS_MAKE_REG     (1ULL << 8)
#define LANDLOCK_ACCESS_FS_MAKE_SOCK    (1ULL << 9)
#define LANDLOCK_ACCESS_FS_MAKE_FIFO    (1ULL << 10)
#define LANDLOCK_ACCESS_FS_MAKE_BLOCK   (1ULL << 11)
#define LANDLOCK_ACCESS_FS_MAKE_SYM     (1ULL << 12)
#define LANDLOCK_ACCESS_FS_REFER        (1ULL << 13)  // Rename/link
#define LANDLOCK_ACCESS_FS_TRUNCATE     (1ULL << 14)  // Truncate file
```

**Ruleset creation:**
```c
struct landlock_ruleset_attr {
    __u64 handled_access_fs;    // Handled filesystem rights
};

int landlock_create_ruleset(
    const struct landlock_ruleset_attr *attr,
    size_t size,
    __u32 flags
);
```

**Add rule:**
```c
enum landlock_rule_type {
    LANDLOCK_RULE_PATH_BENEATH = 1,
};

struct landlock_path_beneath_attr {
    __u64 allowed_access;
    __s32 parent_fd;
};
```

**Ordo Application:** Fine-grained filesystem sandboxing beyond seccomp.

---

### Audit System

**Message types:**
```c
#define AUDIT_GET             1000
#define AUDIT_SET             1001
#define AUDIT_LIST            1002
#define AUDIT_ADD             1003
#define AUDIT_DEL             1004
#define AUDIT_USER            1100
#define AUDIT_LOGIN           1101
#define AUDIT_KERNEL          1102
#define AUDIT_FIRST_EVENT     1103
#define AUDIT_SYSCALL         1104  // Syscall audit record
#define AUDIT_PATH            1105  // Path access
#define AUDIT_EXECVE          1106  // execve arguments
#define AUDIT_SOCKET          1107  // Socket creation
#define AUDIT_NETFILTER_CFG   1108  // Netfilter config change
#define AUDIT_TTY             1109  // TTY input
#define AUDIT_MAC_CHECK       1110  // MAC check
#define AUDIT_ANOM_PROMISCUOUS 1111
#define AUDIT_ANOM_ABEND      1112
#define AUDIT_INTEGRITY_DATA  1113
#define AUDIT_INTEGRITY_METADATA 1114
// ... 300+ message types
```

**Record format:**
```c
struct audit_message {
    struct nlmsghdr nlh;
    struct audit_reply reply;
};

struct audit_reply {
    int type;                     // Message type
    void *data;                   // Record data
    size_t len;                   // Data length
};
```

**Rule fields:**
```c
#define AUDIT_FIELD_UID           0
#define AUDIT_FIELD_GID           1
#define AUDIT_FIELD_PID           2
#define AUDIT_FIELD_PPID          3
#define AUDIT_FIELD_SYSCALL       4
#define AUDIT_FIELD_EXIT          5
#define AUDIT_FIELD_SUCCESS       6
#define AUDIT_FIELD_PATH          7
#define AUDIT_FIELD_FILENAME      8
#define AUDIT_FIELD_ARGV          9
// ... many more fields
```

**Ordo Application:** Comprehensive activity logging for compliance/debugging.

---

## Filesystem Primitives

### Btrfs (B-tree Filesystem) - 195 defines, 48 structs

**Subvolume operations:**
```c
#define BTRFS_IOC_SNAP_SUBVOL       _IOW(BTRFS_IOCTL_MAGIC, 19, struct btrfs_ioctl_vol_args)
#define BTRFS_IOC_SUBVOL_CREATE     _IOW(BTRFS_IOCTL_MAGIC, 20, struct btrfs_ioctl_vol_args)
#define BTRFS_IOC_SUBVOL_DESTROY    _IOW(BTRFS_IOCTL_MAGIC, 21, struct btrfs_ioctl_vol_args)
#define BTRFS_IOC_SUBVOL_GETFLAGS   _IOR(BTRFS_IOCTL_MAGIC, 22, __u64)
#define BTRFS_IOC_SUBVOL_SETFLAGS   _IOW(BTRFS_IOCTL_MAGIC, 23, __u64)
```

**Defragmentation:**
```c
struct btrfs_ioctl_defrag_range_args {
    __u64 start;          // Start offset
    __u64 len;            // Length
    __u64 extent_thresh;  // Extent size threshold
    __u64 compress_type;  // Compression type
    __u64 flags;          // Flags
};

#define BTRFS_DEFRAG_RANGE_COMPRESS (1 << 0)
#define BTRFS_DEFRAG_RANGE_START_IO (1 << 1)
```

**Balance operations:**
```c
struct btrfs_balance_args {
    __u64 profiles;       // Profiles to balance
    __u64 usage;          // Usage filter (%)
    __u64 devid;          // Device ID filter
    __u64 pstart;         // Physical start filter
    __u64 pend;           // Physical end filter
    __u64 vstart;         // Virtual start filter
    __u64 vend;           // Virtual end filter
    __u64 target;         // Target profile
    __u64 flags;          // Balance flags
    __u64 limit;          // Limit chunks to relocate
    __u64 unused[2];
};

#define BTRFS_BALANCE_ARGS_PROFILES     (1 << 0)
#define BTRFS_BALANCE_ARGS_USAGE        (1 << 1)
#define BTRFS_BALANCE_ARGS_DEVID        (1 << 2)
#define BTRFS_BALANCE_ARGS_DRANGE       (1 << 3)
#define BTRFS_BALANCE_ARGS_VRANGE       (1 << 4)
#define BTRFS_BALANCE_ARGS_CONVERT      (1 << 5)
#define BTRFS_BALANCE_ARGS_SOFT         (1 << 6)
#define BTRFS_BALANCE_ARGS_LIMIT        (1 << 7)
```

**Send/receive:**
```c
#define BTRFS_IOC_SEND                  _IOW(BTRFS_IOCTL_MAGIC, 38, struct btrfs_ioctl_send_args)
#define BTRFS_IOC_RECEIVE               _IOW(BTRFS_IOCTL_MAGIC, 39, int)
#define BTRFS_IOC_SEND_GET_PARAMS       _IOR(BTRFS_IOCTL_MAGIC, 53, struct btrfs_ioctl_send_params)

struct btrfs_ioctl_send_args {
    __s64 send_fd;                      // Send file descriptor
    __u64 clone_sources_count;          // Number of clone sources
    __u64 clone_sources;                // Clone source subvol IDs
    __u64 parent_root;                  // Parent subvol ID for incremental
    __u64 flags;                        // Send flags
};

#define BTRFS_SEND_FLAG_NO_FILE_DATA    (1 << 0)
#define BTRFS_SEND_FLAG_OMIT_STREAM_HEADER (1 << 1)
#define BTRFS_SEND_FLAG_OMIT_END_CMD    (1 << 2)
```

**Ordo Application:** Snapshot-based checkpointing, incremental backup patterns.

---

### ext4 (Fourth Extended Filesystem) - 61 defines, 4 structs

**Extended attributes:**
```c
#define EXT4_IOC_GETVERSION         _IOR('f', 3, long)
#define EXT4_IOC_SETVERSION         _IOW('f', 4, long)
#define EXT4_IOC_GETFLAGS           _IOR('f', 1, long)
#define EXT4_IOC_SETFLAGS           _IOW('f', 2, long)
#define EXT4_IOC_GET_ENCRYPTION_POLICY _IOW('f', 21, struct fscrypt_policy)
#define EXT4_IOC_SET_ENCRYPTION_POLICY _IOW('f', 22, struct fscrypt_policy)
```

**File flags:**
```c
#define EXT4_SECRM_FLAG             0x00000001  // Secure deletion
#define EXT4_UNRM_FLAG              0x00000002  // Undelete on deletion
#define EXT4_COMPR_FLAG             0x00000004  // Compress file
#define EXT4_SYNC_FLAG              0x00000008  // Synchronous updates
#define EXT4_IMMUTABLE_FLAG         0x00000010  // Immutable file
#define EXT4_APPEND_FLAG            0x00000020  // Append-only
#define EXT4_NODUMP_FLAG            0x00000040  // Don't dump
#define EXT4_NOATIME_FLAG           0x00000080  // Don't update atime
#define EXT4_DIRTY_FLAG             0x00000100  // Dirty (modified)
#define EXT4_COMPRBLK_FLAG          0x00000200  // Compressed blocks
#define EXT4_NOCOMPR_FLAG           0x00000400  // Don't compress
#define EXT4_ENCRYPT_FLAG           0x00000800  // Encrypted file
#define EXT4_INDEX_FLAG             0x00001000  // Hash-indexed directory
#define EXT4_JOURNAL_DATA_FLAG      0x00004000  // Journal data
#define EXT4_NOTAIL_FLAG            0x00008000  // No tail merging
#define EXT4_DIRSYNC_FLAG           0x00010000  // Dirsync behavior
#define EXT4_TOPDIR_FLAG            0x00020000  // Top of directory hierarchy
#define EXT4_EXTENTS_FLAG           0x00080000  // Extents used
#define EXT4_EA_INODE_FLAG          0x00200000  // EA inode
#define EXT4_EOFBLOCKS_FLAG         0x00400000  // EOF blocks flag
#define EXT4_INLINE_DATA_FLAG       0x10000000  // Inline data
#define EXT4_PROJINHERIT_FLAG       0x20000000  // Project inheritance
#define EXT4_CASEFOLD_FLAG          0x40000000  // Casefold directory
```

**Group descriptors:**
```c
struct ext4_group_desc {
    __le32 bg_block_bitmap_lo;      // Block bitmap block
    __le32 bg_inode_bitmap_lo;      // Inode bitmap block
    __le32 bg_inode_table_lo;       // Inode table block
    __le16 bg_free_blocks_count_lo; // Free blocks count
    __le16 bg_free_inodes_count_lo; // Free inodes count
    __le16 bg_used_dirs_count_lo;   // Directory count
    __le16 bg_flags;                // Group flags
    __le32 bg_reserved[2];
    __le16 bg_itable_unused_lo;     // Unused inodes
    __le16 bg_checksum;             // CRC16 checksum
};

#define EXT4_BG_BLOCK_UNINIT        0x0001  // Block bitmap uninitialized
#define EXT4_BG_INODE_UNINIT        0x0002  // Inode bitmap uninitialized
#define EXT4_BG_INODE_ZEROED        0x0004  // Inode table zeroed
```

**Ordo Application:** File-level encryption policies, immutable/append-only flags for audit trails.

---

### Device Mapper (dm-ioctl.h) - 49 defines, 6 structs

**Target types:**
- `linear` - Linear mapping
- `striped` - RAID0 striping
- `mirror` - RAID1 mirroring
- `snapshot` - COW snapshots
- `crypt` - dm-crypt encryption
- `verity` - dm-verity integrity
- `thin` - Thin provisioning
- `cache` - SSD caching
- `writecache` - Write-back cache
- `multipath` - Multipath I/O

**IOCTL commands:**
```c
#define DM_VERSION                  _IOWR(DM_IOCTL, DM_VERSION_CMD, struct dm_ioctl)
#define DM_REMOVE_ALL               _IOWR(DM_IOCTL, DM_REMOVE_ALL_CMD, struct dm_ioctl)
#define DM_LIST_DEVICES             _IOWR(DM_IOCTL, DM_LIST_DEVICES_CMD, struct dm_ioctl)
#define DM_DEV_CREATE               _IOWR(DM_IOCTL, DM_DEV_CREATE_CMD, struct dm_ioctl)
#define DM_DEV_REMOVE               _IOWR(DM_IOCTL, DM_DEV_REMOVE_CMD, struct dm_ioctl)
#define DM_DEV_RENAME               _IOWR(DM_IOCTL, DM_DEV_RENAME_CMD, struct dm_target_spec)
#define DM_SUSPEND                  _IOWR(DM_IOCTL, DM_SUSPEND_CMD, struct dm_ioctl)
#define DM_STATUS                   _IOWR(DM_IOCTL, DM_STATUS_CMD, struct dm_target_spec)
#define DM_TABLE                    _IOWR(DM_IOCTL, DM_TABLE_CMD, struct dm_target_spec)
#define DM_ARM_POLL                 _IOWR(DM_IOCTL, DM_ARM_POLL_CMD, struct dm_ioctl)
#define DM_LIST_VERSIONS            _IOWR(DM_IOCTL, DM_LIST_VERSIONS_CMD, struct dm_target_versions)
#define DM_TARGET_MSG               _IOWR(DM_IOCTL, DM_TARGET_MSG_CMD, struct dm_target_msg)
#define DM_DEV_SET_GEOMETRY         _IOWR(DM_IOCTL, DM_DEV_SET_GEOMETRY_CMD, struct dm_ioctl)
```

**Table specification:**
```c
struct dm_target_spec {
    __u64 sector_start;         // Start sector
    __u64 length;               // Length in sectors
    int32_t status;             // Status (used for returns)
    char target_type[DM_TARGET_TYPE_LEN];  // Target type name
    char params[DM_TARGET_PARAMS_LEN];     // Target parameters
};
```

**Example: Create encrypted volume:**
```bash
# Create device
dmsetup create cryptvol --table "0 $(blockdev --getsz /dev/sda1) crypt aes-xts-plain64 :64:logon /dev/sda1"

# Equivalent ioctl sequence:
ioctl(fd, DM_DEV_CREATE, &dmi);
ioctl(fd, DM_TABLE, &table_spec);  // "0 N crypt ..."
ioctl(fd, DM_DEV_SUSPEND, &dmi);   // Resume device
```

**Ordo Application:** Storage abstraction layer with composable transformations (encrypt, snapshot, cache).

---

## Total Extraction Status

| Category | Headers | Size | Defines | Structs |
|----------|---------|------|---------|---------|
| Core uapi | 19 | ~137 KB | 1,200+ | 280+ |
| Generic drivers | 13 | ~126 KB | 500+ | 120+ |
| GPU drivers | 15 | ~200 KB | 1,880+ | 500+ |
| Network/Audio | 25 | ~153 KB | 2,124+ | 284+ |
| **Input/Security/FS/Storage** | **38** | **~289 KB** | **2,867+** | **398+** |
| **GRAND TOTAL** | **110** | **~905 KB** | **8,571+** | **1,582+** |

---

## Remaining Headers (if any)

- Accelerators: `habanalabs_accel.h`, `qaic_accel.h`, `ivpu_accel.h`
- More networking: `bridge.h`, `bonding.h`, `tun.h`, `if_bridge.h`, etc.
- Bluetooth: Not in uapi (kernel-internal only?)
- Misc: `watchdog.h`, `rtc.h`, `random.h` (may have been parsed earlier)
- Additional filesystems: XFS headers in `include/uapi/linux/xfs/`?

---

## Files Generated

```
F:\OPENCLAW-PROJECTS\linux-primitives-extracted\output\remaining\
├── input_analysis.md              (Input event codes, structures)
├── uinput_analysis.md             (Userspace input injection)
├── hidraw_analysis.md             (HID raw access)
├── uhid_analysis.md               (Userspace HID transport)
├── joystick_analysis.md           (Joystick API)
├── kd_analysis.md                 (Keyboard/console)
├── kvm_analysis.md                (Virtualization - 606 defines!)
├── vfio_analysis.md               (Device passthrough)
├── vfio_ccw_analysis.md           (VFIO channel I/O)
├── userfaultfd_analysis.md        (Page fault handling)
├── memfd_analysis.md              (Anonymous file descriptors)
├── prctl_analysis.md              (Process control)
├── ptrace_analysis.md             (Process tracing/debugging)
├── seccomp_analysis.md            (Syscall filtering)
├── landlock_analysis.md           (Filesystem sandboxing)
├── audit_analysis.md              (Audit logging - 312 defines)
├── fanotify_analysis.md           (Filesystem fanotify)
├── inotify_analysis.md            (Inode monitoring)
├── quota_analysis.md              (Disk quota)
├── fsmap_analysis.md              (File extent mapping)
├── cdrom_analysis.md              (CD-ROM ioctls)
├── loop_analysis.md               (Loopback devices)
├── dm-ioctl_analysis.md           (Device mapper)
├── btrfs_analysis.md              (Btrfs filesystem)
├── btrfs_tree_analysis.md         (Btrfs tree structures)
├── ext4_analysis.md               (ext4 filesystem)
├── f2fs_analysis.md               (F2FS flash filesystem)
├── nvme_ioctl_analysis.md         (NVMe passthrough)
├── usbip_analysis.md              (USB over IP)
├── mroute_analysis.md             (IPv4 multicast routing)
├── mroute6_analysis.md            (IPv6 multicast routing)
├── ip_vs_analysis.md              (IP Virtual Server - L4 load balancing)
├── nf_tables_analysis.md          (Netfilter tables/nftables)
├── nfnetlink_analysis.md          (Netfilter netlink)
├── ppp_defs_analysis.md           (PPP definitions)
├── ppp-comp_analysis.md           (PPP compression)
├── if_ppp_analysis.md             (PPP interface)
└── ppp-ioctl_analysis.md          (PPP ioctls)
```

**Total:** 38 files → ~289 KB
