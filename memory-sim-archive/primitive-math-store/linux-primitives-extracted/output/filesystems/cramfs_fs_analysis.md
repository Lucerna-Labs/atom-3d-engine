# cramfs_fs.h

**Source:** `cramfs_fs.h`


## Includes

- `linux/types.h`
- `linux/magic.h`

## Defines (19 total)


### CRAMFS_BLK (4)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_BLK_FLAG_UNCOMPRESSED` | `(1 << 31)` |  |
| `CRAMFS_BLK_FLAG_DIRECT_PTR` | `(1 << 30)` |  |
| `CRAMFS_BLK_FLAGS` | `( CRAMFS_BLK_FLAG_UNCOMPRESSED ` |  |
| `CRAMFS_BLK_DIRECT_PTR_SHIFT` | `2` |  |

### CRAMFS_FLAG (6)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_FLAG_FSID_VERSION_2` | `0x00000001` | fsid version #2 |
| `CRAMFS_FLAG_SORTED_DIRS` | `0x00000002` | sorted dirs |
| `CRAMFS_FLAG_HOLES` | `0x00000100` | support for holes |
| `CRAMFS_FLAG_WRONG_SIGNATURE` | `0x00000200` | reserved |
| `CRAMFS_FLAG_SHIFTED_ROOT_OFFSET` | `0x00000400` | shifted root fs |
| `CRAMFS_FLAG_EXT_BLOCK_POINTERS` | `0x00000800` | block pointer extensions |

### CRAMFS_GID (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_GID_WIDTH` | `8` |  |

### CRAMFS_MODE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_MODE_WIDTH` | `16` |  |

### CRAMFS_NAMELEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_NAMELEN_WIDTH` | `6` |  |

### CRAMFS_OFFSET (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_OFFSET_WIDTH` | `26` |  |

### CRAMFS_SIZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_SIZE_WIDTH` | `24` |  |

### CRAMFS_SUPPORTED (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_SUPPORTED_FLAGS` | `( 0x000000ff ` |  |

### CRAMFS_UID (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_UID_WIDTH` | `16` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `CRAMFS_SIGNATURE` | `"Compressed ROMFS"` |  |
| `CRAMFS_MAXPATHLEN` | `(((1 << CRAMFS_NAMELEN_WIDTH) - 1) << 2)` |  |

## Structs (3)


### `struct cramfs_inode`

| Type | Field | Array |
|------|-------|-------|

### `struct cramfs_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crc` | `-` |
| `__u32` | `edition` | `-` |
| `__u32` | `blocks` | `-` |
| `__u32` | `files` | `-` |

### `struct cramfs_super`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `magic` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `future` | `-` |
| `__u8` | `signature` | `16` |
| `__u8` | `name` | `16` |