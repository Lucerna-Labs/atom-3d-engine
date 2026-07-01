# amd-apml.h

**Source:** `amd-apml.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### AMD_SBI (1)

| Name | Value | Comment |
|------|-------|---------|
| `AMD_SBI_MB_DATA_SIZE` | `4` |  |

### SBRMI_IOCTL (4)

| Name | Value | Comment |
|------|-------|---------|
| `SBRMI_IOCTL_MBOX_CMD` | `_IOWR(SB_BASE_IOCTL_NR, 0, struct apml_mbox_msg)` |  |
| `SBRMI_IOCTL_CPUID_CMD` | `_IOWR(SB_BASE_IOCTL_NR, 1, struct apml_cpuid_msg)` |  |
| `SBRMI_IOCTL_MCAMSR_CMD` | `_IOWR(SB_BASE_IOCTL_NR, 2, struct apml_mcamsr_msg)` |  |
| `SBRMI_IOCTL_REG_XFER_CMD` | `_IOWR(SB_BASE_IOCTL_NR, 3, struct apml_reg_xfer_msg)` |  |

### SB_BASE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SB_BASE_IOCTL_NR` | `0xF9` |  |

## Structs (4)


### `struct apml_mbox_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `mb_in_out` | `-` |
| `__u32` | `fw_ret_code` | `-` |

### `struct apml_cpuid_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cpu_in_out` | `-` |
| `__u32` | `fw_ret_code` | `-` |
| `__u32` | `pad` | `-` |

### `struct apml_mcamsr_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mcamsr_in_out` | `-` |
| `__u32` | `fw_ret_code` | `-` |
| `__u32` | `pad` | `-` |

### `struct apml_reg_xfer_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `reg_addr` | `-` |
| `__u8` | `data_in_out` | `-` |
| `__u8` | `rflag` | `-` |