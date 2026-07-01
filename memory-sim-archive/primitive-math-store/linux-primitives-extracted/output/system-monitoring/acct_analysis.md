# acct.h

**Source:** `acct.h`


## Includes

- `linux/types.h`
- `asm/param.h`
- `asm/byteorder.h`

## Defines (11 total)


### UNCATEGORIZED (11)

| Name | Value | Comment |
|------|-------|---------|
| `ACCT_COMM` | `16` |  |
| `AFORK` | `0x01` | ... executed fork, but did not exec |
| `ASU` | `0x02` | ... used super-user privileges |
| `ACOMPAT` | `0x04` | ... used compatibility mode (VAX only not used) |
| `ACORE` | `0x08` | ... dumped core |
| `AXSIG` | `0x10` | ... was killed by a signal |
| `AGROUP` | `0x20` | ... was the last task of the process (task group) |
| `ACCT_BYTEORDER` | `0x80` | accounting file is big endian |
| `ACCT_BYTEORDER` | `0x00` | accounting file is little endian |
| `ACCT_VERSION` | `2` |  |
| `AHZ` | `(HZ)` |  |

## Structs (2)


### `struct acct`

| Type | Field | Array |
|------|-------|-------|
| `char` | `ac_flag` | `-` |
| `char` | `ac_version` | `-` |
| `__u16` | `ac_uid16` | `-` |
| `__u16` | `ac_gid16` | `-` |
| `__u16` | `ac_tty` | `-` |
| `__u32` | `ac_btime` | `-` |
| `comp_t` | `ac_utime` | `-` |
| `comp_t` | `ac_stime` | `-` |
| `comp_t` | `ac_etime` | `-` |
| `comp_t` | `ac_mem` | `-` |
| `comp_t` | `ac_io` | `-` |
| `comp_t` | `ac_rw` | `-` |
| `comp_t` | `ac_minflt` | `-` |
| `comp_t` | `ac_majflt` | `-` |
| `comp_t` | `ac_swaps` | `-` |
| `__u16` | `ac_ahz` | `-` |
| `__u32` | `ac_exitcode` | `-` |
| `char` | `ac_comm` | `ACCT_COMM + 1` |
| `__u8` | `ac_etime_hi` | `-` |
| `__u16` | `ac_etime_lo` | `-` |
| `__u32` | `ac_uid` | `-` |
| `__u32` | `ac_gid` | `-` |

### `struct acct_v3`

| Type | Field | Array |
|------|-------|-------|
| `char` | `ac_flag` | `-` |
| `char` | `ac_version` | `-` |
| `__u16` | `ac_tty` | `-` |
| `__u32` | `ac_exitcode` | `-` |
| `__u32` | `ac_uid` | `-` |
| `__u32` | `ac_gid` | `-` |
| `__u32` | `ac_pid` | `-` |
| `__u32` | `ac_ppid` | `-` |
| `__u32` | `ac_btime` | `-` |
| `__u32` | `ac_etime` | `-` |
| `float` | `ac_etime` | `-` |
| `comp_t` | `ac_utime` | `-` |
| `comp_t` | `ac_stime` | `-` |
| `comp_t` | `ac_mem` | `-` |
| `comp_t` | `ac_io` | `-` |
| `comp_t` | `ac_rw` | `-` |
| `comp_t` | `ac_minflt` | `-` |
| `comp_t` | `ac_majflt` | `-` |
| `comp_t` | `ac_swaps` | `-` |
| `char` | `ac_comm` | `ACCT_COMM` |