# signalfd.h

**Source:** `signalfd.h`


## Includes

- `linux/types.h`
- `linux/fcntl.h`

## Defines (2 total)


### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `SFD_CLOEXEC` | `O_CLOEXEC` |  |
| `SFD_NONBLOCK` | `O_NONBLOCK` |  |

## Structs (1)


### `struct signalfd_siginfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ssi_signo` | `-` |
| `__s32` | `ssi_errno` | `-` |
| `__s32` | `ssi_code` | `-` |
| `__u32` | `ssi_pid` | `-` |
| `__u32` | `ssi_uid` | `-` |
| `__s32` | `ssi_fd` | `-` |
| `__u32` | `ssi_tid` | `-` |
| `__u32` | `ssi_band` | `-` |
| `__u32` | `ssi_overrun` | `-` |
| `__u32` | `ssi_trapno` | `-` |
| `__s32` | `ssi_status` | `-` |
| `__s32` | `ssi_int` | `-` |
| `__u64` | `ssi_ptr` | `-` |
| `__u64` | `ssi_utime` | `-` |
| `__u64` | `ssi_stime` | `-` |
| `__u64` | `ssi_addr` | `-` |
| `__u16` | `ssi_addr_lsb` | `-` |
| `__u16` | `__pad2` | `-` |
| `__s32` | `ssi_syscall` | `-` |
| `__u64` | `ssi_call_addr` | `-` |
| `__u32` | `ssi_arch` | `-` |
| `__u8` | `__pad` | `28` |