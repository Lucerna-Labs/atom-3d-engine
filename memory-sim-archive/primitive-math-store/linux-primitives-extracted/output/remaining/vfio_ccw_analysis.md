# vfio_ccw.h

**Source:** `vfio_ccw.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### IRB_AREA (1)

| Name | Value | Comment |
|------|-------|---------|
| `IRB_AREA_SIZE` | `96` |  |

### ORB_AREA (1)

| Name | Value | Comment |
|------|-------|---------|
| `ORB_AREA_SIZE` | `12` |  |

### SCHIB_AREA (1)

| Name | Value | Comment |
|------|-------|---------|
| `SCHIB_AREA_SIZE` | `52` |  |

### SCSW_AREA (1)

| Name | Value | Comment |
|------|-------|---------|
| `SCSW_AREA_SIZE` | `12` |  |

### VFIO_CCW (2)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_CCW_ASYNC_CMD_HSCH` | `(1 << 0)` |  |
| `VFIO_CCW_ASYNC_CMD_CSCH` | `(1 << 1)` |  |

## Structs (4)


### `struct ccw_io_region`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `orb_area` | `ORB_AREA_SIZE` |
| `__u8` | `scsw_area` | `SCSW_AREA_SIZE` |
| `__u8` | `irb_area` | `IRB_AREA_SIZE` |
| `__u32` | `ret_code` | `-` |

### `struct ccw_cmd_region`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `command` | `-` |
| `__u32` | `ret_code` | `-` |

### `struct ccw_schib_region`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `schib_area` | `SCHIB_AREA_SIZE` |

### `struct ccw_crw_region`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crw` | `-` |
| `__u32` | `pad` | `-` |