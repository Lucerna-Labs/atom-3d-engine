# openat2.h

**Source:** `openat2.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### RESOLVE_IN (1)

| Name | Value | Comment |
|------|-------|---------|
| `RESOLVE_IN_ROOT` | `0x10 /* Make all jumps to "/" and ".."` |  |

### RESOLVE_NO (3)

| Name | Value | Comment |
|------|-------|---------|
| `RESOLVE_NO_XDEV` | `0x01 /* Block mount-point crossings` |  |
| `RESOLVE_NO_MAGICLINKS` | `0x02 /* Block traversal through procfs-style` |  |
| `RESOLVE_NO_SYMLINKS` | `0x04 /* Block traversal through all symlinks` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `RESOLVE_BENEATH` | `0x08 /* Block "lexical" trickery like` |  |
| `RESOLVE_CACHED` | `0x20 /* Only complete if resolution can be` |  |

## Structs (1)


### `struct open_how`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `mode` | `-` |
| `__u64` | `resolve` | `-` |