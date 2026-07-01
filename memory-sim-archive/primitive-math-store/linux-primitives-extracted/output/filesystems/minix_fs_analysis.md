# minix_fs.h

**Source:** `minix_fs.h`


## Includes

- `linux/types.h`
- `linux/magic.h`

## Defines (8 total)


### MINIX_ERROR (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_ERROR_FS` | `0x0002` | fs has errors. |

### MINIX_I (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_I_MAP_SLOTS` | `8` |  |

### MINIX_INODES (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_INODES_PER_BLOCK` | `((BLOCK_SIZE)/(sizeof (struct minix_inode)))` |  |

### MINIX_LINK (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_LINK_MAX` | `250` |  |

### MINIX_ROOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_ROOT_INO` | `1` |  |

### MINIX_VALID (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_VALID_FS` | `0x0001` | Clean fs. |

### MINIX_Z (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX_Z_MAP_SLOTS` | `64` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `MINIX2_LINK_MAX` | `65530` |  |

## Structs (6)


### `struct minix_inode`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `i_mode` | `-` |
| `__u16` | `i_uid` | `-` |
| `__u32` | `i_size` | `-` |
| `__u32` | `i_time` | `-` |
| `__u8` | `i_gid` | `-` |
| `__u8` | `i_nlinks` | `-` |
| `__u16` | `i_zone` | `9` |

### `struct minix2_inode`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `i_mode` | `-` |
| `__u16` | `i_nlinks` | `-` |
| `__u16` | `i_uid` | `-` |
| `__u16` | `i_gid` | `-` |
| `__u32` | `i_size` | `-` |
| `__u32` | `i_atime` | `-` |
| `__u32` | `i_mtime` | `-` |
| `__u32` | `i_ctime` | `-` |
| `__u32` | `i_zone` | `10` |

### `struct minix_super_block`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `s_ninodes` | `-` |
| `__u16` | `s_nzones` | `-` |
| `__u16` | `s_imap_blocks` | `-` |
| `__u16` | `s_zmap_blocks` | `-` |
| `__u16` | `s_firstdatazone` | `-` |
| `__u16` | `s_log_zone_size` | `-` |
| `__u32` | `s_max_size` | `-` |
| `__u16` | `s_magic` | `-` |
| `__u16` | `s_state` | `-` |
| `__u32` | `s_zones` | `-` |

### `struct minix3_super_block`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `s_ninodes` | `-` |
| `__u16` | `s_pad0` | `-` |
| `__u16` | `s_imap_blocks` | `-` |
| `__u16` | `s_zmap_blocks` | `-` |
| `__u16` | `s_firstdatazone` | `-` |
| `__u16` | `s_log_zone_size` | `-` |
| `__u16` | `s_pad1` | `-` |
| `__u32` | `s_max_size` | `-` |
| `__u32` | `s_zones` | `-` |
| `__u16` | `s_magic` | `-` |
| `__u16` | `s_pad2` | `-` |
| `__u16` | `s_blocksize` | `-` |
| `__u8` | `s_disk_version` | `-` |

### `struct minix_dir_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `inode` | `-` |

### `struct minix3_dir_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `inode` | `-` |