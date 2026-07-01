# fwctl.h

**Source:** `fwctl.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (3 total)


### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `FWCTL_TYPE` | `0x9A` |  |
| `FWCTL_INFO` | `_IO(FWCTL_TYPE, FWCTL_CMD_INFO)` |  |
| `FWCTL_RPC` | `_IO(FWCTL_TYPE, FWCTL_CMD_RPC)` |  |

## Structs (2)


### `struct fwctl_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `out_device_type` | `-` |
| `__u32` | `device_data_len` | `-` |
| `__aligned_u64` | `out_device_data` | `-` |

### `struct fwctl_rpc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `scope` | `-` |
| `__u32` | `in_len` | `-` |
| `__u32` | `out_len` | `-` |
| `__aligned_u64` | `in` | `-` |
| `__aligned_u64` | `out` | `-` |