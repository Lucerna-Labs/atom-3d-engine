# bfs_fs.h

**Source:** `bfs_fs.h`


## Includes

- `linux/types.h`

## Defines (10 total)


### BFS_BSIZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BFS_BSIZE_BITS` | `9` |  |

### BFS_DIRENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `BFS_DIRENT_SIZE` | `16` |  |

### BFS_DIRS (1)

| Name | Value | Comment |
|------|-------|---------|
| `BFS_DIRS_PER_BLOCK` | `32` |  |

### BFS_INODES (1)

| Name | Value | Comment |
|------|-------|---------|
| `BFS_INODES_PER_BLOCK` | `8` |  |

### BFS_ROOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `BFS_ROOT_INO` | `2` |  |

### UNCATEGORIZED (5)

| Name | Value | Comment |
|------|-------|---------|
| `BFS_BSIZE` | `(1<<BFS_BSIZE_BITS)` |  |
| `BFS_MAGIC` | `0x1BADFACE` |  |
| `BFS_VDIR` | `2L` |  |
| `BFS_VREG` | `1L` |  |
| `BFS_NAMELEN` | `14` |  |

## Structs (3)


### `struct bfs_inode`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `i_ino` | `-` |
| `__u16` | `i_unused` | `-` |
| `__le32` | `i_sblock` | `-` |
| `__le32` | `i_eblock` | `-` |
| `__le32` | `i_eoffset` | `-` |
| `__le32` | `i_vtype` | `-` |
| `__le32` | `i_mode` | `-` |
| `__le32` | `i_uid` | `-` |
| `__le32` | `i_gid` | `-` |
| `__le32` | `i_nlink` | `-` |
| `__le32` | `i_atime` | `-` |
| `__le32` | `i_mtime` | `-` |
| `__le32` | `i_ctime` | `-` |
| `__u32` | `i_padding` | `4` |

### `struct bfs_dirent`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `ino` | `-` |
| `char` | `name` | `BFS_NAMELEN` |

### `struct bfs_super_block`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `s_magic` | `-` |
| `__le32` | `s_start` | `-` |
| `__le32` | `s_end` | `-` |
| `__le32` | `s_from` | `-` |
| `__le32` | `s_to` | `-` |
| `__s32` | `s_bfrom` | `-` |
| `__s32` | `s_bto` | `-` |
| `char` | `s_fsname` | `6` |
| `char` | `s_volume` | `6` |
| `__u32` | `s_padding` | `118` |