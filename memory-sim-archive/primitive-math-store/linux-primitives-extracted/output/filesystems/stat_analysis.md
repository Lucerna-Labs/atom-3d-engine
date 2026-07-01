# stat.h

**Source:** `stat.h`


## Includes

- `linux/types.h`

## Defines (54 total)


### STATX_ATTR (10)

| Name | Value | Comment |
|------|-------|---------|
| `STATX_ATTR_COMPRESSED` | `0x00000004` | [I] File is compressed by the fs |
| `STATX_ATTR_IMMUTABLE` | `0x00000010` | [I] File is marked immutable |
| `STATX_ATTR_APPEND` | `0x00000020` | [I] File is append-only |
| `STATX_ATTR_NODUMP` | `0x00000040` | [I] File is not to be dumped |
| `STATX_ATTR_ENCRYPTED` | `0x00000800` | [I] File requires key to decrypt in fs |
| `STATX_ATTR_AUTOMOUNT` | `0x00001000` | Dir: Automount trigger |
| `STATX_ATTR_MOUNT_ROOT` | `0x00002000` | Root of a mount |
| `STATX_ATTR_VERITY` | `0x00100000` | [I] Verity protected file |
| `STATX_ATTR_DAX` | `0x00200000` | File is currently in DAX state |
| `STATX_ATTR_WRITE_ATOMIC` | `0x00400000` | File supports atomic write operations |

### STATX_BASIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `STATX_BASIC_STATS` | `0x000007ffU` | The stuff in the normal stat struct |

### STATX_DIO (1)

| Name | Value | Comment |
|------|-------|---------|
| `STATX_DIO_READ_ALIGN` | `0x00020000U` | Want/got dio read alignment info |

### STATX_MNT (2)

| Name | Value | Comment |
|------|-------|---------|
| `STATX_MNT_ID` | `0x00001000U` | Got stx_mnt_id |
| `STATX_MNT_ID_UNIQUE` | `0x00004000U` | Want/got extended stx_mount_id |

### STATX_WRITE (1)

| Name | Value | Comment |
|------|-------|---------|
| `STATX_WRITE_ATOMIC` | `0x00010000U` | Want/got atomic_write_* fields |

### UNCATEGORIZED (39)

| Name | Value | Comment |
|------|-------|---------|
| `S_IFMT` | `00170000` |  |
| `S_IFSOCK` | `0140000` |  |
| `S_IFLNK` | `0120000` |  |
| `S_IFREG` | `0100000` |  |
| `S_IFBLK` | `0060000` |  |
| `S_IFDIR` | `0040000` |  |
| `S_IFCHR` | `0020000` |  |
| `S_IFIFO` | `0010000` |  |
| `S_ISUID` | `0004000` |  |
| `S_ISGID` | `0002000` |  |
| `S_ISVTX` | `0001000` |  |
| `S_IRWXU` | `00700` |  |
| `S_IRUSR` | `00400` |  |
| `S_IWUSR` | `00200` |  |
| `S_IXUSR` | `00100` |  |
| `S_IRWXG` | `00070` |  |
| `S_IRGRP` | `00040` |  |
| `S_IWGRP` | `00020` |  |
| `S_IXGRP` | `00010` |  |
| `S_IRWXO` | `00007` |  |
| `S_IROTH` | `00004` |  |
| `S_IWOTH` | `00002` |  |
| `S_IXOTH` | `00001` |  |
| `STATX_TYPE` | `0x00000001U` | Want/got stx_mode & S_IFMT |
| `STATX_MODE` | `0x00000002U` | Want/got stx_mode & ~S_IFMT |
| `STATX_NLINK` | `0x00000004U` | Want/got stx_nlink |
| `STATX_UID` | `0x00000008U` | Want/got stx_uid |
| `STATX_GID` | `0x00000010U` | Want/got stx_gid |
| `STATX_ATIME` | `0x00000020U` | Want/got stx_atime |
| `STATX_MTIME` | `0x00000040U` | Want/got stx_mtime |
| `STATX_CTIME` | `0x00000080U` | Want/got stx_ctime |
| `STATX_INO` | `0x00000100U` | Want/got stx_ino |
| `STATX_SIZE` | `0x00000200U` | Want/got stx_size |
| `STATX_BLOCKS` | `0x00000400U` | Want/got stx_blocks |
| `STATX_BTIME` | `0x00000800U` | Want/got stx_btime |
| `STATX_DIOALIGN` | `0x00002000U` | Want/got direct I/O alignment info |
| `STATX_SUBVOL` | `0x00008000U` | Want/got stx_subvol |
| `STATX__RESERVED` | `0x80000000U` | Reserved for future struct statx expansion |
| `STATX_ALL` | `0x00000fffU` |  |

## Structs (2)


### `struct statx_timestamp`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `tv_sec` | `-` |
| `__u32` | `tv_nsec` | `-` |
| `__s32` | `__reserved` | `-` |

### `struct statx`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `stx_mask` | `-` |
| `__u32` | `stx_blksize` | `-` |
| `__u64` | `stx_attributes` | `-` |
| `__u32` | `stx_nlink` | `-` |
| `__u32` | `stx_uid` | `-` |
| `__u32` | `stx_gid` | `-` |
| `__u16` | `stx_mode` | `-` |
| `__u16` | `__spare0` | `1` |
| `__u64` | `stx_ino` | `-` |
| `__u64` | `stx_size` | `-` |
| `__u64` | `stx_blocks` | `-` |
| `__u64` | `stx_attributes_mask` | `-` |
| `__u32` | `stx_rdev_major` | `-` |
| `__u32` | `stx_rdev_minor` | `-` |
| `__u32` | `stx_dev_major` | `-` |
| `__u32` | `stx_dev_minor` | `-` |
| `__u64` | `stx_mnt_id` | `-` |
| `__u32` | `stx_dio_mem_align` | `-` |
| `__u32` | `stx_dio_offset_align` | `-` |
| `__u64` | `stx_subvol` | `-` |
| `__u32` | `stx_atomic_write_unit_min` | `-` |
| `__u32` | `stx_atomic_write_unit_max` | `-` |
| `__u32` | `stx_atomic_write_segments_max` | `-` |
| `__u32` | `stx_dio_read_offset_align` | `-` |
| `__u32` | `stx_atomic_write_unit_max_opt` | `-` |
| `__u32` | `__spare2` | `1` |
| `__u64` | `__spare3` | `8` |