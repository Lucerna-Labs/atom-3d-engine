# fsmap.h

**Source:** `fsmap.h`


## Includes

- `linux/types.h`

## Defines (12 total)


### FMH_IF (1)

| Name | Value | Comment |
|------|-------|---------|
| `FMH_IF_VALID` | `0` |  |

### FMH_OF (1)

| Name | Value | Comment |
|------|-------|---------|
| `FMH_OF_DEV_T` | `0x1` | fmr_device values will be dev_t |

### FMR_OF (6)

| Name | Value | Comment |
|------|-------|---------|
| `FMR_OF_PREALLOC` | `0x1` | segment = unwritten pre-allocation |
| `FMR_OF_ATTR_FORK` | `0x2` | segment = attribute fork |
| `FMR_OF_EXTENT_MAP` | `0x4` | segment = extent map |
| `FMR_OF_SHARED` | `0x8` | segment = shared with another file |
| `FMR_OF_SPECIAL_OWNER` | `0x10` | owner is a special value |
| `FMR_OF_LAST` | `0x20` | segment is the last in the dataset |

### FMR_OWN (3)

| Name | Value | Comment |
|------|-------|---------|
| `FMR_OWN_FREE` | `FMR_OWNER(0, 1)` | free space |
| `FMR_OWN_UNKNOWN` | `FMR_OWNER(0, 2)` | unknown owner |
| `FMR_OWN_METADATA` | `FMR_OWNER(0, 3)` | metadata |

### FS_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_IOC_GETFSMAP` | `_IOWR('X', 59, struct fsmap_head)` |  |

## Structs (2)


### `struct fsmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fmr_device` | `-` |
| `__u32` | `fmr_flags` | `-` |
| `__u64` | `fmr_physical` | `-` |
| `__u64` | `fmr_owner` | `-` |
| `__u64` | `fmr_offset` | `-` |
| `__u64` | `fmr_length` | `-` |
| `__u64` | `fmr_reserved` | `3` |

### `struct fsmap_head`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fmh_iflags` | `-` |
| `__u32` | `fmh_oflags` | `-` |
| `__u32` | `fmh_count` | `-` |
| `__u32` | `fmh_entries` | `-` |
| `__u64` | `fmh_reserved` | `6` |