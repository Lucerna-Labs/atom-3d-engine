# taskstats.h

**Source:** `taskstats.h`


## Includes

- `linux/types.h`
- `linux/time_types.h`

## Defines (7 total)


### TASKSTATS_CMD (2)

| Name | Value | Comment |
|------|-------|---------|
| `TASKSTATS_CMD_MAX` | `(__TASKSTATS_CMD_MAX - 1)` |  |
| `TASKSTATS_CMD_ATTR_MAX` | `(__TASKSTATS_CMD_ATTR_MAX - 1)` |  |

### TASKSTATS_GENL (2)

| Name | Value | Comment |
|------|-------|---------|
| `TASKSTATS_GENL_NAME` | `"TASKSTATS"` |  |
| `TASKSTATS_GENL_VERSION` | `0x1` |  |

### TASKSTATS_TYPE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TASKSTATS_TYPE_MAX` | `(__TASKSTATS_TYPE_MAX - 1)` |  |

### TS_COMM (1)

| Name | Value | Comment |
|------|-------|---------|
| `TS_COMM_LEN` | `32	/* should be >= TASK_COMM_LEN` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `TASKSTATS_VERSION` | `17` |  |

## Structs (1)


### `struct taskstats`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `version` | `-` |
| `__u32` | `ac_exitcode` | `-` |
| `__u8` | `ac_flag` | `-` |
| `__u8` | `ac_nice` | `-` |
| `__u64` | `cpu_delay_total` | `-` |
| `__u64` | `blkio_count` | `-` |
| `__u64` | `blkio_delay_total` | `-` |
| `__u64` | `swapin_count` | `-` |
| `__u64` | `swapin_delay_total` | `-` |
| `__u64` | `cpu_run_real_total` | `-` |
| `__u64` | `cpu_run_virtual_total` | `-` |
| `char` | `ac_comm` | `TS_COMM_LEN` |
| `__u8` | `ac_pad` | `3` |
| `__u32` | `ac_gid` | `-` |
| `__u32` | `ac_pid` | `-` |
| `__u32` | `ac_ppid` | `-` |
| `__u32` | `ac_btime` | `-` |
| `__u64` | `ac_utime` | `-` |
| `__u64` | `ac_stime` | `-` |
| `__u64` | `ac_minflt` | `-` |
| `__u64` | `ac_majflt` | `-` |
| `__u64` | `coremem` | `-` |
| `__u64` | `virtmem` | `-` |
| `__u64` | `hiwater_rss` | `-` |
| `__u64` | `hiwater_vm` | `-` |
| `__u64` | `read_char` | `-` |
| `__u64` | `write_char` | `-` |
| `__u64` | `read_syscalls` | `-` |
| `__u64` | `write_syscalls` | `-` |
| `__u64` | `read_bytes` | `-` |
| `__u64` | `write_bytes` | `-` |
| `__u64` | `cancelled_write_bytes` | `-` |
| `__u64` | `nvcsw` | `-` |
| `__u64` | `nivcsw` | `-` |
| `__u64` | `ac_utimescaled` | `-` |
| `__u64` | `ac_stimescaled` | `-` |
| `__u64` | `cpu_scaled_run_real_total` | `-` |
| `__u64` | `freepages_count` | `-` |
| `__u64` | `freepages_delay_total` | `-` |
| `__u64` | `thrashing_count` | `-` |
| `__u64` | `thrashing_delay_total` | `-` |
| `__u64` | `ac_btime64` | `-` |
| `__u64` | `compact_count` | `-` |
| `__u64` | `compact_delay_total` | `-` |
| `__u32` | `ac_tgid` | `-` |
| `__u64` | `ac_exe_dev` | `-` |
| `__u64` | `ac_exe_inode` | `-` |
| `__u64` | `wpcopy_count` | `-` |
| `__u64` | `wpcopy_delay_total` | `-` |
| `__u64` | `irq_count` | `-` |
| `__u64` | `irq_delay_total` | `-` |
| `__u64` | `cpu_delay_max` | `-` |
| `__u64` | `cpu_delay_min` | `-` |
| `__u64` | `blkio_delay_max` | `-` |
| `__u64` | `blkio_delay_min` | `-` |
| `__u64` | `swapin_delay_max` | `-` |
| `__u64` | `swapin_delay_min` | `-` |
| `__u64` | `freepages_delay_max` | `-` |
| `__u64` | `freepages_delay_min` | `-` |
| `__u64` | `thrashing_delay_max` | `-` |
| `__u64` | `thrashing_delay_min` | `-` |
| `__u64` | `compact_delay_max` | `-` |
| `__u64` | `compact_delay_min` | `-` |
| `__u64` | `wpcopy_delay_max` | `-` |
| `__u64` | `wpcopy_delay_min` | `-` |
| `__u64` | `irq_delay_max` | `-` |
| `__u64` | `irq_delay_min` | `-` |