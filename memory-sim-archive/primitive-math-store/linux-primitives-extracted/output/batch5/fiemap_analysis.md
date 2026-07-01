# fiemap.h

**Source:** `fiemap.h`


## Includes

- `linux/types.h`

## Defines (16 total)


### FIEMAP_EXTENT (11)

| Name | Value | Comment |
|------|-------|---------|
| `FIEMAP_EXTENT_LAST` | `0x00000001` | Last extent in file. |
| `FIEMAP_EXTENT_UNKNOWN` | `0x00000002` | Data location unknown. |
| `FIEMAP_EXTENT_DELALLOC` | `0x00000004 /* Location still pending.` |  |
| `FIEMAP_EXTENT_ENCODED` | `0x00000008 /* Data can not be read` |  |
| `FIEMAP_EXTENT_DATA_ENCRYPTED` | `0x00000080 /* Data is encrypted by fs.` |  |
| `FIEMAP_EXTENT_NOT_ALIGNED` | `0x00000100 /* Extent offsets may not be` |  |
| `FIEMAP_EXTENT_DATA_INLINE` | `0x00000200 /* Data mixed with metadata.` |  |
| `FIEMAP_EXTENT_DATA_TAIL` | `0x00000400 /* Multiple files in block.` |  |
| `FIEMAP_EXTENT_UNWRITTEN` | `0x00000800 /* Space allocated, but` |  |
| `FIEMAP_EXTENT_MERGED` | `0x00001000 /* File does not natively` |  |
| `FIEMAP_EXTENT_SHARED` | `0x00002000 /* Space shared with other` |  |

### FIEMAP_FLAG (3)

| Name | Value | Comment |
|------|-------|---------|
| `FIEMAP_FLAG_SYNC` | `0x00000001` | sync file data before map |
| `FIEMAP_FLAG_XATTR` | `0x00000002` | map extended attribute tree |
| `FIEMAP_FLAG_CACHE` | `0x00000004` | request caching of the extents |

### FIEMAP_FLAGS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FIEMAP_FLAGS_COMPAT` | `(FIEMAP_FLAG_SYNC \| FIEMAP_FLAG_XATTR)` |  |

### FIEMAP_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FIEMAP_MAX_OFFSET` | `(~0ULL)` |  |

## Structs (2)


### `struct fiemap_extent`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fe_logical` | `-` |
| `__u64` | `fe_physical` | `-` |
| `__u64` | `fe_length` | `-` |
| `__u64` | `fe_reserved64` | `2` |
| `__u32` | `fe_flags` | `-` |
| `__u32` | `fe_reserved` | `3` |

### `struct fiemap`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fm_start` | `-` |
| `__u64` | `fm_length` | `-` |
| `__u32` | `fm_flags` | `-` |
| `__u32` | `fm_mapped_extents` | `-` |
| `__u32` | `fm_extent_count` | `-` |
| `__u32` | `fm_reserved` | `-` |