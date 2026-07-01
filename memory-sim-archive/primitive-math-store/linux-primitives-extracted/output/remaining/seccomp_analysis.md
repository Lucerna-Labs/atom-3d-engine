# seccomp.h

**Source:** `seccomp.h`


## Includes

- `linux/compiler.h`
- `linux/types.h`

## Defines (35 total)


### SECCOMP_ADDFD (2)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_ADDFD_FLAG_SETFD` | `(1UL << 0)` | Specify remote fd |
| `SECCOMP_ADDFD_FLAG_SEND` | `(1UL << 1)` | Addfd and return it, atomically |

### SECCOMP_FILTER (6)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_FILTER_FLAG_TSYNC` | `(1UL << 0)` |  |
| `SECCOMP_FILTER_FLAG_LOG` | `(1UL << 1)` |  |
| `SECCOMP_FILTER_FLAG_SPEC_ALLOW` | `(1UL << 2)` |  |
| `SECCOMP_FILTER_FLAG_NEW_LISTENER` | `(1UL << 3)` |  |
| `SECCOMP_FILTER_FLAG_TSYNC_ESRCH` | `(1UL << 4)` |  |
| `SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV` | `(1UL << 5)` |  |

### SECCOMP_GET (2)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_GET_ACTION_AVAIL` | `2` |  |
| `SECCOMP_GET_NOTIF_SIZES` | `3` |  |

### SECCOMP_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_IOC_MAGIC` | `'!'` |  |

### SECCOMP_IOCTL (5)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_IOCTL_NOTIF_RECV` | `SECCOMP_IOWR(0, struct seccomp_notif)` |  |
| `SECCOMP_IOCTL_NOTIF_SEND` | `SECCOMP_IOWR(1,	` |  |
| `SECCOMP_IOCTL_NOTIF_ID_VALID` | `SECCOMP_IOW(2, __u64)` |  |
| `SECCOMP_IOCTL_NOTIF_ADDFD` | `SECCOMP_IOW(3, ` |  |
| `SECCOMP_IOCTL_NOTIF_SET_FLAGS` | `SECCOMP_IOW(4, __u64)` |  |

### SECCOMP_MODE (3)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_MODE_DISABLED` | `0` | seccomp is not in use. |
| `SECCOMP_MODE_STRICT` | `1` | uses hard-coded filter. |
| `SECCOMP_MODE_FILTER` | `2` | uses user-supplied filter. |

### SECCOMP_RET (12)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_RET_KILL_PROCESS` | `0x80000000U` | kill the process |
| `SECCOMP_RET_KILL_THREAD` | `0x00000000U` | kill the thread |
| `SECCOMP_RET_KILL` | `SECCOMP_RET_KILL_THREAD` |  |
| `SECCOMP_RET_TRAP` | `0x00030000U` | disallow and force a SIGSYS |
| `SECCOMP_RET_ERRNO` | `0x00050000U` | returns an errno |
| `SECCOMP_RET_USER_NOTIF` | `0x7fc00000U` | notifies userspace |
| `SECCOMP_RET_TRACE` | `0x7ff00000U` | pass to a tracer or disallow |
| `SECCOMP_RET_LOG` | `0x7ffc0000U` | allow after logging |
| `SECCOMP_RET_ALLOW` | `0x7fff0000U` | allow |
| `SECCOMP_RET_ACTION_FULL` | `0xffff0000U` |  |
| `SECCOMP_RET_ACTION` | `0x7fff0000U` |  |
| `SECCOMP_RET_DATA` | `0x0000ffffU` |  |

### SECCOMP_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_SET_MODE_STRICT` | `0` |  |
| `SECCOMP_SET_MODE_FILTER` | `1` |  |

### SECCOMP_USER (2)

| Name | Value | Comment |
|------|-------|---------|
| `SECCOMP_USER_NOTIF_FLAG_CONTINUE` | `(1UL << 0)` |  |
| `SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP` | `(1UL << 0)` |  |

## Structs (5)


### `struct seccomp_data`

| Type | Field | Array |
|------|-------|-------|
| `int` | `nr` | `-` |
| `__u32` | `arch` | `-` |
| `__u64` | `instruction_pointer` | `-` |
| `__u64` | `args` | `6` |

### `struct seccomp_notif_sizes`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `seccomp_notif` | `-` |
| `__u16` | `seccomp_notif_resp` | `-` |
| `__u16` | `seccomp_data` | `-` |

### `struct seccomp_notif`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `flags` | `-` |

### `struct seccomp_notif_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |
| `__s64` | `val` | `-` |
| `__s32` | `error` | `-` |
| `__u32` | `flags` | `-` |

### `struct seccomp_notif_addfd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `srcfd` | `-` |
| `__u32` | `newfd` | `-` |
| `__u32` | `newfd_flags` | `-` |