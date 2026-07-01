# pds.h

**Source:** `pds.h`


## Includes

- `linux/types.h`

## Structs (4)


### `struct fwctl_info_pds`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `uctx_caps` | `-` |

### `struct fwctl_rpc_pds`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `ep` | `-` |
| `__u32` | `rsvd` | `-` |
| `__u32` | `len` | `-` |
| `__aligned_u64` | `payload` | `-` |
| `__u32` | `retval` | `-` |
| `__u32` | `rsvd` | `2` |
| `__u32` | `len` | `-` |
| `__aligned_u64` | `payload` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `ep` | `-` |
| `__u32` | `rsvd` | `-` |
| `__u32` | `len` | `-` |
| `__aligned_u64` | `payload` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `retval` | `-` |
| `__u32` | `rsvd` | `2` |
| `__u32` | `len` | `-` |
| `__aligned_u64` | `payload` | `-` |