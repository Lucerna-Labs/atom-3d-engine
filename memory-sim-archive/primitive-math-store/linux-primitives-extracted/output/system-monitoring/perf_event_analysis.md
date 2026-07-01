# perf_event.h

**Source:** `perf_event.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`
- `asm/byteorder.h`

## Defines (147 total)


### PERF_ATTR (10)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_ATTR_SIZE_VER0` | `64` | Size of first published 'struct perf_event_attr' |
| `PERF_ATTR_SIZE_VER1` | `72` | Add: config2 |
| `PERF_ATTR_SIZE_VER2` | `80` | Add: branch_sample_type |
| `PERF_ATTR_SIZE_VER3` | `96` | Add: sample_regs_user |
| `PERF_ATTR_SIZE_VER4` | `104` | Add: sample_regs_intr |
| `PERF_ATTR_SIZE_VER5` | `112` | Add: aux_watermark |
| `PERF_ATTR_SIZE_VER6` | `120` | Add: aux_sample_size |
| `PERF_ATTR_SIZE_VER7` | `128` | Add: sig_data |
| `PERF_ATTR_SIZE_VER8` | `136` | Add: config3 |
| `PERF_ATTR_SIZE_VER9` | `144` | add: config4 |

### PERF_AUX (7)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_AUX_FLAG_TRUNCATED` | `0x0001` | Record was truncated to fit |
| `PERF_AUX_FLAG_OVERWRITE` | `0x0002` | Snapshot from overwrite mode |
| `PERF_AUX_FLAG_PARTIAL` | `0x0004` | Record contains gaps |
| `PERF_AUX_FLAG_COLLISION` | `0x0008` | Sample collided with another |
| `PERF_AUX_FLAG_PMU_FORMAT_TYPE_MASK` | `0xff00` | PMU specific trace format type |
| `PERF_AUX_FLAG_CORESIGHT_FORMAT_CORESIGHT` | `0x0000` | Default for backward compatibility |
| `PERF_AUX_FLAG_CORESIGHT_FORMAT_RAW` | `0x0100` | Raw format of the source |

### PERF_BR (5)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_BR_ARM64_FIQ` | `PERF_BR_NEW_ARCH_1` |  |
| `PERF_BR_ARM64_DEBUG_HALT` | `PERF_BR_NEW_ARCH_2` |  |
| `PERF_BR_ARM64_DEBUG_EXIT` | `PERF_BR_NEW_ARCH_3` |  |
| `PERF_BR_ARM64_DEBUG_INST` | `PERF_BR_NEW_ARCH_4` |  |
| `PERF_BR_ARM64_DEBUG_DATA` | `PERF_BR_NEW_ARCH_5` |  |

### PERF_BRANCH (1)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_BRANCH_ENTRY_INFO_BITS_MAX` | `33` |  |

### PERF_EVENT (12)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_EVENT_IOC_ENABLE` | `_IO  ('$', 0)` |  |
| `PERF_EVENT_IOC_DISABLE` | `_IO  ('$', 1)` |  |
| `PERF_EVENT_IOC_REFRESH` | `_IO  ('$', 2)` |  |
| `PERF_EVENT_IOC_RESET` | `_IO  ('$', 3)` |  |
| `PERF_EVENT_IOC_PERIOD` | `_IOW ('$', 4, __u64)` |  |
| `PERF_EVENT_IOC_SET_OUTPUT` | `_IO  ('$', 5)` |  |
| `PERF_EVENT_IOC_SET_FILTER` | `_IOW ('$', 6, char *)` |  |
| `PERF_EVENT_IOC_ID` | `_IOR ('$', 7, __u64 *)` |  |
| `PERF_EVENT_IOC_SET_BPF` | `_IOW ('$', 8, __u32)` |  |
| `PERF_EVENT_IOC_PAUSE_OUTPUT` | `_IOW ('$', 9, __u32)` |  |
| `PERF_EVENT_IOC_QUERY_BPF` | `_IOWR('$', 10, struct perf_event_query_bpf *)` |  |
| `PERF_EVENT_IOC_MODIFY_ATTRIBUTES` | `_IOW ('$', 11, struct perf_event_attr *)` |  |

### PERF_FLAG (4)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_FLAG_FD_NO_GROUP` | `(1UL << 0)` |  |
| `PERF_FLAG_FD_OUTPUT` | `(1UL << 1)` |  |
| `PERF_FLAG_PID_CGROUP` | `(1UL << 2)` | pid=cgroup ID, per-CPU mode only |
| `PERF_FLAG_FD_CLOEXEC` | `(1UL << 3)` | O_CLOEXEC |

### PERF_HW (1)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_HW_EVENT_MASK` | `0xffffffff` |  |

### PERF_MAX (2)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_MAX_STACK_DEPTH` | `127` |  |
| `PERF_MAX_CONTEXTS_PER_STACK` | `8` |  |

### PERF_MEM (85)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_MEM_OP_NA` | `0x0001` | Not available |
| `PERF_MEM_OP_LOAD` | `0x0002` | Load instruction |
| `PERF_MEM_OP_STORE` | `0x0004` | Store instruction |
| `PERF_MEM_OP_PFETCH` | `0x0008` | Prefetch |
| `PERF_MEM_OP_EXEC` | `0x0010` | Code (execution) |
| `PERF_MEM_OP_SHIFT` | `0` |  |
| `PERF_MEM_LVL_NA` | `0x0001` | Not available |
| `PERF_MEM_LVL_HIT` | `0x0002` | Hit level |
| `PERF_MEM_LVL_MISS` | `0x0004` | Miss level |
| `PERF_MEM_LVL_L1` | `0x0008` | L1 |
| `PERF_MEM_LVL_LFB` | `0x0010` | Line Fill Buffer |
| `PERF_MEM_LVL_L2` | `0x0020` | L2 |
| `PERF_MEM_LVL_L3` | `0x0040` | L3 |
| `PERF_MEM_LVL_LOC_RAM` | `0x0080` | Local DRAM |
| `PERF_MEM_LVL_REM_RAM1` | `0x0100` | Remote DRAM (1 hop) |
| `PERF_MEM_LVL_REM_RAM2` | `0x0200` | Remote DRAM (2 hops) |
| `PERF_MEM_LVL_REM_CCE1` | `0x0400` | Remote Cache (1 hop) |
| `PERF_MEM_LVL_REM_CCE2` | `0x0800` | Remote Cache (2 hops) |
| `PERF_MEM_LVL_IO` | `0x1000` | I/O memory |
| `PERF_MEM_LVL_UNC` | `0x2000` | Uncached memory |
| `PERF_MEM_LVL_SHIFT` | `5` |  |
| `PERF_MEM_REMOTE_REMOTE` | `0x0001` | Remote |
| `PERF_MEM_REMOTE_SHIFT` | `37` |  |
| `PERF_MEM_LVLNUM_L1` | `0x0001` | L1 |
| `PERF_MEM_LVLNUM_L2` | `0x0002` | L2 |
| `PERF_MEM_LVLNUM_L3` | `0x0003` | L3 |
| `PERF_MEM_LVLNUM_L4` | `0x0004` | L4 |
| `PERF_MEM_LVLNUM_L2_MHB` | `0x0005` | L2 Miss Handling Buffer |
| `PERF_MEM_LVLNUM_MSC` | `0x0006` | Memory-side Cache |
| `PERF_MEM_LVLNUM_L0` | `0x0007` | L0 |
| `PERF_MEM_LVLNUM_UNC` | `0x0008` | Uncached |
| `PERF_MEM_LVLNUM_CXL` | `0x0009` | CXL |
| `PERF_MEM_LVLNUM_IO` | `0x000a` | I/O |
| `PERF_MEM_LVLNUM_ANY_CACHE` | `0x000b` | Any cache |
| `PERF_MEM_LVLNUM_LFB` | `0x000c` | LFB / L1 Miss Handling Buffer |
| `PERF_MEM_LVLNUM_RAM` | `0x000d` | RAM |
| `PERF_MEM_LVLNUM_PMEM` | `0x000e` | PMEM |
| `PERF_MEM_LVLNUM_NA` | `0x000f` | N/A |
| `PERF_MEM_LVLNUM_SHIFT` | `33` |  |
| `PERF_MEM_SNOOP_NA` | `0x0001` | Not available |
| `PERF_MEM_SNOOP_NONE` | `0x0002` | No snoop |
| `PERF_MEM_SNOOP_HIT` | `0x0004` | Snoop hit |
| `PERF_MEM_SNOOP_MISS` | `0x0008` | Snoop miss |
| `PERF_MEM_SNOOP_HITM` | `0x0010` | Snoop hit modified |
| `PERF_MEM_SNOOP_SHIFT` | `19` |  |
| `PERF_MEM_SNOOPX_FWD` | `0x0001` | Forward |
| `PERF_MEM_SNOOPX_PEER` | `0x0002` | Transfer from peer |
| `PERF_MEM_SNOOPX_SHIFT` | `38` |  |
| `PERF_MEM_LOCK_NA` | `0x0001` | Not available |
| `PERF_MEM_LOCK_LOCKED` | `0x0002` | Locked transaction |

*...and 35 more*

### PERF_PMU (1)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_PMU_TYPE_SHIFT` | `32` |  |

### PERF_RECORD (17)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_RECORD_MISC_CPUMODE_MASK` | `(7 << 0)` |  |
| `PERF_RECORD_MISC_CPUMODE_UNKNOWN` | `(0 << 0)` |  |
| `PERF_RECORD_MISC_KERNEL` | `(1 << 0)` |  |
| `PERF_RECORD_MISC_USER` | `(2 << 0)` |  |
| `PERF_RECORD_MISC_HYPERVISOR` | `(3 << 0)` |  |
| `PERF_RECORD_MISC_GUEST_KERNEL` | `(4 << 0)` |  |
| `PERF_RECORD_MISC_GUEST_USER` | `(5 << 0)` |  |
| `PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT` | `(1 << 12)` |  |
| `PERF_RECORD_MISC_MMAP_DATA` | `(1 << 13)` |  |
| `PERF_RECORD_MISC_COMM_EXEC` | `(1 << 13)` |  |
| `PERF_RECORD_MISC_FORK_EXEC` | `(1 << 13)` |  |
| `PERF_RECORD_MISC_SWITCH_OUT` | `(1 << 13)` |  |
| `PERF_RECORD_MISC_EXACT_IP` | `(1 << 14)` |  |
| `PERF_RECORD_MISC_SWITCH_OUT_PREEMPT` | `(1 << 14)` |  |
| `PERF_RECORD_MISC_MMAP_BUILD_ID` | `(1 << 14)` |  |
| `PERF_RECORD_MISC_EXT_RESERVED` | `(1 << 15)` |  |
| `PERF_RECORD_KSYMBOL_FLAGS_UNREGISTER` | `(1 << 0)` |  |

### PERF_SAMPLE (2)

| Name | Value | Comment |
|------|-------|---------|
| `PERF_SAMPLE_WEIGHT_TYPE` | `(PERF_SAMPLE_WEIGHT \| PERF_SAMPLE_WEIGHT_STRUCT)` |  |
| `PERF_SAMPLE_BRANCH_PLM_ALL` | `` |  |

## Structs (39)


### `struct read_format`

| Type | Field | Array |
|------|-------|-------|

### `struct perf_event_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `size` | `-` |
| `__u64` | `config` | `-` |
| `__u64` | `sample_period` | `-` |
| `__u64` | `sample_freq` | `-` |
| `__u64` | `sample_type` | `-` |
| `__u64` | `read_format` | `-` |
| `__u32` | `wakeup_events` | `-` |
| `__u32` | `wakeup_watermark` | `-` |
| `__u32` | `bp_type` | `-` |
| `__u64` | `bp_addr` | `-` |
| `__u64` | `kprobe_func` | `-` |
| `__u64` | `uprobe_path` | `-` |
| `__u64` | `config1` | `-` |
| `__u64` | `bp_len` | `-` |
| `__u64` | `kprobe_addr` | `-` |
| `__u64` | `probe_offset` | `-` |
| `__u64` | `config2` | `-` |
| `__u64` | `branch_sample_type` | `-` |
| `__u64` | `sample_regs_user` | `-` |
| `__u32` | `sample_stack_user` | `-` |
| `__s32` | `clockid` | `-` |
| `__u64` | `sample_regs_intr` | `-` |
| `__u32` | `aux_watermark` | `-` |
| `__u16` | `sample_max_stack` | `-` |
| `__u16` | `__reserved_2` | `-` |
| `__u32` | `aux_sample_size` | `-` |
| `__u32` | `aux_action` | `-` |
| `__u64` | `sig_data` | `-` |
| `__u64` | `config3` | `-` |
| `__u64` | `config4` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|

### `struct perf_event_query_bpf`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ids_len` | `-` |
| `__u32` | `prog_cnt` | `-` |

### `struct perf_event_mmap_page`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `-` |
| `__u32` | `compat_version` | `-` |
| `__u32` | `lock` | `-` |
| `__u32` | `index` | `-` |
| `__s64` | `offset` | `-` |
| `__u64` | `time_enabled` | `-` |
| `__u64` | `time_running` | `-` |
| `__u64` | `capabilities` | `-` |
| `__u16` | `pmc_width` | `-` |
| `__u16` | `time_shift` | `-` |
| `__u32` | `time_mult` | `-` |
| `__u64` | `time_offset` | `-` |
| `__u64` | `time_zero` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `__reserved_1` | `-` |
| `__u64` | `time_cycles` | `-` |
| `__u64` | `time_mask` | `-` |
| `__u8` | `__reserved` | `116*8` |
| `__u64` | `data_head` | `-` |
| `__u64` | `data_tail` | `-` |
| `__u64` | `data_offset` | `-` |
| `__u64` | `data_size` | `-` |
| `__u64` | `aux_head` | `-` |
| `__u64` | `aux_tail` | `-` |
| `__u64` | `aux_offset` | `-` |
| `__u64` | `aux_size` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|

### `struct perf_event_header`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `misc` | `-` |
| `__u16` | `size` | `-` |

### `struct perf_ns_link_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dev` | `-` |
| `__u64` | `ino` | `-` |

### `struct sample_id`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_9`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_12`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_14`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_20`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_21`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_23`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_24`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_25`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_26`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_27`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_28`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_29`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_30`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_31`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_32`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_33`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_34`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_35`

| Type | Field | Array |
|------|-------|-------|

### `struct perf_branch_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `from` | `-` |
| `__u64` | `to` | `-` |

### `struct anonymous_37`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `var1_dw` | `-` |
| `__u16` | `var2_w` | `-` |
| `__u16` | `var3_w` | `-` |

### `struct anonymous_38`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `var3_w` | `-` |
| `__u16` | `var2_w` | `-` |
| `__u32` | `var1_dw` | `-` |