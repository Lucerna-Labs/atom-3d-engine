# cxl.h

**Source:** `cxl.h`


## Includes

- `linux/types.h`
- `linux/stddef.h`
- `cxl/features.h`

## Structs (2)


### `struct fwctl_rpc_cxl`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `opcode` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `op_size` | `-` |
| `__u32` | `reserved1` | `-` |

### `struct fwctl_rpc_cxl_out`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `retval` | `-` |