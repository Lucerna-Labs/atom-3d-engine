# prctl.h

**Source:** `prctl.h`


## Includes

- `linux/types.h`

## Defines (81 total)


### PR_CAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_CAP_AMBIENT` | `47` |  |

### PR_CAPBSET (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_CAPBSET_READ` | `23` |  |
| `PR_CAPBSET_DROP` | `24` |  |

### PR_CFI (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_CFI_BRANCH_LANDING_PADS` | `0` |  |

### PR_FUTEX (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_FUTEX_HASH` | `78` |  |

### PR_GET (26)

| Name | Value | Comment |
|------|-------|---------|
| `PR_GET_PDEATHSIG` | `2` | Second arg is a ptr to return the signal |
| `PR_GET_DUMPABLE` | `3` |  |
| `PR_GET_UNALIGN` | `5` |  |
| `PR_GET_KEEPCAPS` | `7` |  |
| `PR_GET_FPEMU` | `9` |  |
| `PR_GET_FPEXC` | `11` |  |
| `PR_GET_TIMING` | `13` |  |
| `PR_GET_NAME` | `16` | Get process name |
| `PR_GET_ENDIAN` | `19` |  |
| `PR_GET_SECCOMP` | `21` |  |
| `PR_GET_TSC` | `25` |  |
| `PR_GET_SECUREBITS` | `27` |  |
| `PR_GET_TIMERSLACK` | `30` |  |
| `PR_GET_CHILD_SUBREAPER` | `37` |  |
| `PR_GET_NO_NEW_PRIVS` | `39` |  |
| `PR_GET_TID_ADDRESS` | `40` |  |
| `PR_GET_THP_DISABLE` | `42` |  |
| `PR_GET_FP_MODE` | `46` |  |
| `PR_GET_SPECULATION_CTRL` | `52` |  |
| `PR_GET_TAGGED_ADDR_CTRL` | `56` |  |
| `PR_GET_IO_FLUSHER` | `58` |  |
| `PR_GET_MDWE` | `66` |  |
| `PR_GET_AUXV` | `0x41555856` |  |
| `PR_GET_MEMORY_MERGE` | `68` |  |
| `PR_GET_SHADOW_STACK_STATUS` | `74` |  |
| `PR_GET_CFI` | `80` |  |

### PR_LOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_LOCK_SHADOW_STACK_STATUS` | `76` |  |

### PR_MCE (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_MCE_KILL` | `33` |  |
| `PR_MCE_KILL_GET` | `34` |  |

### PR_MPX (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_MPX_ENABLE_MANAGEMENT` | `43` |  |
| `PR_MPX_DISABLE_MANAGEMENT` | `44` |  |

### PR_PAC (3)

| Name | Value | Comment |
|------|-------|---------|
| `PR_PAC_RESET_KEYS` | `54` |  |
| `PR_PAC_SET_ENABLED_KEYS` | `60` |  |
| `PR_PAC_GET_ENABLED_KEYS` | `61` |  |

### PR_PPC (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_PPC_GET_DEXCR` | `72` |  |
| `PR_PPC_SET_DEXCR` | `73` |  |

### PR_RISCV (3)

| Name | Value | Comment |
|------|-------|---------|
| `PR_RISCV_V_SET_CONTROL` | `69` |  |
| `PR_RISCV_V_GET_CONTROL` | `70` |  |
| `PR_RISCV_SET_ICACHE_FLUSH_CTX` | `71` |  |

### PR_RSEQ (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_RSEQ_SLICE_EXTENSION` | `79` |  |

### PR_SCHED (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_SCHED_CORE` | `62` |  |

### PR_SET (28)

| Name | Value | Comment |
|------|-------|---------|
| `PR_SET_PDEATHSIG` | `1` | Second arg is a signal |
| `PR_SET_DUMPABLE` | `4` |  |
| `PR_SET_UNALIGN` | `6` |  |
| `PR_SET_KEEPCAPS` | `8` |  |
| `PR_SET_FPEMU` | `10` |  |
| `PR_SET_FPEXC` | `12` |  |
| `PR_SET_TIMING` | `14` |  |
| `PR_SET_NAME` | `15` | Set process name |
| `PR_SET_ENDIAN` | `20` |  |
| `PR_SET_SECCOMP` | `22` |  |
| `PR_SET_TSC` | `26` |  |
| `PR_SET_SECUREBITS` | `28` |  |
| `PR_SET_TIMERSLACK` | `29` |  |
| `PR_SET_MM` | `35` |  |
| `PR_SET_PTRACER` | `0x59616d61` |  |
| `PR_SET_CHILD_SUBREAPER` | `36` |  |
| `PR_SET_NO_NEW_PRIVS` | `38` |  |
| `PR_SET_THP_DISABLE` | `41` |  |
| `PR_SET_FP_MODE` | `45` |  |
| `PR_SET_SPECULATION_CTRL` | `53` |  |
| `PR_SET_TAGGED_ADDR_CTRL` | `55` |  |
| `PR_SET_IO_FLUSHER` | `57` |  |
| `PR_SET_SYSCALL_USER_DISPATCH` | `59` |  |
| `PR_SET_MDWE` | `65` |  |
| `PR_SET_VMA` | `0x53564d41` |  |
| `PR_SET_MEMORY_MERGE` | `67` |  |
| `PR_SET_SHADOW_STACK_STATUS` | `75` |  |
| `PR_SET_CFI` | `81` |  |

### PR_SME (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_SME_SET_VL` | `63` | set task vector length |
| `PR_SME_GET_VL` | `64` | get task vector length |

### PR_SVE (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_SVE_SET_VL` | `50` | set task vector length |
| `PR_SVE_GET_VL` | `51` | get task vector length |

### PR_TASK (2)

| Name | Value | Comment |
|------|-------|---------|
| `PR_TASK_PERF_EVENTS_DISABLE` | `31` |  |
| `PR_TASK_PERF_EVENTS_ENABLE` | `32` |  |

### PR_TIMER (1)

| Name | Value | Comment |
|------|-------|---------|
| `PR_TIMER_CREATE_RESTORE_IDS` | `77` |  |

## Structs (1)


### `struct prctl_mm_map`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start_code` | `-` |
| `__u64` | `end_code` | `-` |
| `__u64` | `start_data` | `-` |
| `__u64` | `end_data` | `-` |
| `__u64` | `start_brk` | `-` |
| `__u64` | `brk` | `-` |
| `__u64` | `start_stack` | `-` |
| `__u64` | `arg_start` | `-` |
| `__u64` | `arg_end` | `-` |
| `__u64` | `env_start` | `-` |
| `__u64` | `env_end` | `-` |
| `__u32` | `auxv_size` | `-` |
| `__u32` | `exe_fd` | `-` |