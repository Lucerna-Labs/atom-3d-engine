# ptrace.h

**Source:** `ptrace.h`


## Includes

- `linux/types.h`
- `asm/ptrace.h`

## Defines (58 total)


### PTRACE_EVENT (8)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_EVENT_FORK` | `1` |  |
| `PTRACE_EVENT_VFORK` | `2` |  |
| `PTRACE_EVENT_CLONE` | `3` |  |
| `PTRACE_EVENT_EXEC` | `4` |  |
| `PTRACE_EVENT_VFORK_DONE` | `5` |  |
| `PTRACE_EVENT_EXIT` | `6` |  |
| `PTRACE_EVENT_SECCOMP` | `7` |  |
| `PTRACE_EVENT_STOP` | `128` |  |

### PTRACE_EVENTMSG (2)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_EVENTMSG_SYSCALL_ENTRY` | `1` |  |
| `PTRACE_EVENTMSG_SYSCALL_EXIT` | `2` |  |

### PTRACE_GET (3)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_GET_SYSCALL_INFO` | `0x420e` |  |
| `PTRACE_GET_RSEQ_CONFIGURATION` | `0x420f` |  |
| `PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG` | `0x4211` |  |

### PTRACE_O (11)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_O_TRACESYSGOOD` | `1` |  |
| `PTRACE_O_TRACEFORK` | `(1 << PTRACE_EVENT_FORK)` |  |
| `PTRACE_O_TRACEVFORK` | `(1 << PTRACE_EVENT_VFORK)` |  |
| `PTRACE_O_TRACECLONE` | `(1 << PTRACE_EVENT_CLONE)` |  |
| `PTRACE_O_TRACEEXEC` | `(1 << PTRACE_EVENT_EXEC)` |  |
| `PTRACE_O_TRACEVFORKDONE` | `(1 << PTRACE_EVENT_VFORK_DONE)` |  |
| `PTRACE_O_TRACEEXIT` | `(1 << PTRACE_EVENT_EXIT)` |  |
| `PTRACE_O_TRACESECCOMP` | `(1 << PTRACE_EVENT_SECCOMP)` |  |
| `PTRACE_O_EXITKILL` | `(1 << 20)` |  |
| `PTRACE_O_SUSPEND_SECCOMP` | `(1 << 21)` |  |
| `PTRACE_O_MASK` | `(` |  |

### PTRACE_PEEKSIGINFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_PEEKSIGINFO_SHARED` | `(1 << 0)` |  |

### PTRACE_SECCOMP (2)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_SECCOMP_GET_FILTER` | `0x420c` |  |
| `PTRACE_SECCOMP_GET_METADATA` | `0x420d` |  |

### PTRACE_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_SET_SYSCALL_INFO` | `0x4212` |  |
| `PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG` | `0x4210` |  |

### PTRACE_SYSCALL (4)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_SYSCALL_INFO_NONE` | `0` |  |
| `PTRACE_SYSCALL_INFO_ENTRY` | `1` |  |
| `PTRACE_SYSCALL_INFO_EXIT` | `2` |  |
| `PTRACE_SYSCALL_INFO_SECCOMP` | `3` |  |

### UNCATEGORIZED (25)

| Name | Value | Comment |
|------|-------|---------|
| `PTRACE_TRACEME` | `0` |  |
| `PTRACE_PEEKTEXT` | `1` |  |
| `PTRACE_PEEKDATA` | `2` |  |
| `PTRACE_PEEKUSR` | `3` |  |
| `PTRACE_POKETEXT` | `4` |  |
| `PTRACE_POKEDATA` | `5` |  |
| `PTRACE_POKEUSR` | `6` |  |
| `PTRACE_CONT` | `7` |  |
| `PTRACE_KILL` | `8` |  |
| `PTRACE_SINGLESTEP` | `9` |  |
| `PTRACE_ATTACH` | `16` |  |
| `PTRACE_DETACH` | `17` |  |
| `PTRACE_SYSCALL` | `24` |  |
| `PTRACE_SETOPTIONS` | `0x4200` |  |
| `PTRACE_GETEVENTMSG` | `0x4201` |  |
| `PTRACE_GETSIGINFO` | `0x4202` |  |
| `PTRACE_SETSIGINFO` | `0x4203` |  |
| `PTRACE_GETREGSET` | `0x4204` |  |
| `PTRACE_SETREGSET` | `0x4205` |  |
| `PTRACE_SEIZE` | `0x4206` |  |
| `PTRACE_INTERRUPT` | `0x4207` |  |
| `PTRACE_LISTEN` | `0x4208` |  |
| `PTRACE_PEEKSIGINFO` | `0x4209` |  |
| `PTRACE_GETSIGMASK` | `0x420a` |  |
| `PTRACE_SETSIGMASK` | `0x420b` |  |

## Structs (8)


### `struct ptrace_peeksiginfo_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `off` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `nr` | `-` |

### `struct seccomp_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `filter_off` | `-` |
| `__u64` | `flags` | `-` |

### `struct ptrace_syscall_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `op` | `-` |
| `__u8` | `reserved` | `-` |
| `__u16` | `flags` | `-` |
| `__u32` | `arch` | `-` |
| `__u64` | `instruction_pointer` | `-` |
| `__u64` | `stack_pointer` | `-` |
| `__u64` | `nr` | `-` |
| `__u64` | `args` | `6` |
| `__s64` | `rval` | `-` |
| `__u8` | `is_error` | `-` |
| `__u64` | `nr` | `-` |
| `__u64` | `args` | `6` |
| `__u32` | `ret_data` | `-` |
| `__u32` | `reserved2` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `nr` | `-` |
| `__u64` | `args` | `6` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `rval` | `-` |
| `__u8` | `is_error` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `nr` | `-` |
| `__u64` | `args` | `6` |
| `__u32` | `ret_data` | `-` |
| `__u32` | `reserved2` | `-` |

### `struct ptrace_rseq_configuration`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rseq_abi_pointer` | `-` |
| `__u32` | `rseq_abi_size` | `-` |
| `__u32` | `signature` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct ptrace_sud_config`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mode` | `-` |
| `__u64` | `selector` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `len` | `-` |