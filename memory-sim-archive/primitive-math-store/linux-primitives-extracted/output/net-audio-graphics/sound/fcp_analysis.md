# fcp.h

**Source:** `fcp.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (9 total)


### FCP_HWDEP (4)

| Name | Value | Comment |
|------|-------|---------|
| `FCP_HWDEP_MAJOR` | `2` |  |
| `FCP_HWDEP_MINOR` | `0` |  |
| `FCP_HWDEP_SUBMINOR` | `0` |  |
| `FCP_HWDEP_VERSION` | `` |  |

### FCP_IOCTL (5)

| Name | Value | Comment |
|------|-------|---------|
| `FCP_IOCTL_PVERSION` | `_IOR('S', 0x60, int)` |  |
| `FCP_IOCTL_INIT` | `_IOWR('S', 0x64, struct fcp_init)` |  |
| `FCP_IOCTL_CMD` | `_IOWR('S', 0x65, struct fcp_cmd)` |  |
| `FCP_IOCTL_SET_METER_MAP` | `_IOW('S', 0x66, struct fcp_meter_map)` |  |
| `FCP_IOCTL_SET_METER_LABELS` | `_IOW('S', 0x67, struct fcp_meter_labels)` |  |

## Structs (4)


### `struct fcp_init`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `step0_resp_size` | `-` |
| `__u16` | `step2_resp_size` | `-` |
| `__u32` | `init1_opcode` | `-` |
| `__u32` | `init2_opcode` | `-` |

### `struct fcp_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `opcode` | `-` |
| `__u16` | `req_size` | `-` |
| `__u16` | `resp_size` | `-` |

### `struct fcp_meter_map`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `map_size` | `-` |
| `__u16` | `meter_slots` | `-` |

### `struct fcp_meter_labels`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `labels_size` | `-` |