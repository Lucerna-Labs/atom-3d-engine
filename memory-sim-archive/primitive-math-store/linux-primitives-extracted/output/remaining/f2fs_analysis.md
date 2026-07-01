# f2fs.h

**Source:** `f2fs.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (37 total)


### UNCATEGORIZED (37)

| Name | Value | Comment |
|------|-------|---------|
| `F2FS_IOCTL_MAGIC` | `0xf5` |  |
| `F2FS_IOC_START_ATOMIC_WRITE` | `_IO(F2FS_IOCTL_MAGIC, 1)` |  |
| `F2FS_IOC_COMMIT_ATOMIC_WRITE` | `_IO(F2FS_IOCTL_MAGIC, 2)` |  |
| `F2FS_IOC_START_VOLATILE_WRITE` | `_IO(F2FS_IOCTL_MAGIC, 3)` |  |
| `F2FS_IOC_RELEASE_VOLATILE_WRITE` | `_IO(F2FS_IOCTL_MAGIC, 4)` |  |
| `F2FS_IOC_ABORT_ATOMIC_WRITE` | `_IO(F2FS_IOCTL_MAGIC, 5)` |  |
| `F2FS_IOC_GARBAGE_COLLECT` | `_IOW(F2FS_IOCTL_MAGIC, 6, __u32)` |  |
| `F2FS_IOC_WRITE_CHECKPOINT` | `_IO(F2FS_IOCTL_MAGIC, 7)` |  |
| `F2FS_IOC_DEFRAGMENT` | `_IOWR(F2FS_IOCTL_MAGIC, 8,	` |  |
| `F2FS_IOC_MOVE_RANGE` | `_IOWR(F2FS_IOCTL_MAGIC, 9,	` |  |
| `F2FS_IOC_FLUSH_DEVICE` | `_IOW(F2FS_IOCTL_MAGIC, 10,	` |  |
| `F2FS_IOC_GARBAGE_COLLECT_RANGE` | `_IOW(F2FS_IOCTL_MAGIC, 11,	` |  |
| `F2FS_IOC_GET_FEATURES` | `_IOR(F2FS_IOCTL_MAGIC, 12, __u32)` |  |
| `F2FS_IOC_SET_PIN_FILE` | `_IOW(F2FS_IOCTL_MAGIC, 13, __u32)` |  |
| `F2FS_IOC_GET_PIN_FILE` | `_IOR(F2FS_IOCTL_MAGIC, 14, __u32)` |  |
| `F2FS_IOC_PRECACHE_EXTENTS` | `_IO(F2FS_IOCTL_MAGIC, 15)` |  |
| `F2FS_IOC_RESIZE_FS` | `_IOW(F2FS_IOCTL_MAGIC, 16, __u64)` |  |
| `F2FS_IOC_GET_COMPRESS_BLOCKS` | `_IOR(F2FS_IOCTL_MAGIC, 17, __u64)` |  |
| `F2FS_IOC_RELEASE_COMPRESS_BLOCKS` | `` |  |
| `F2FS_IOC_RESERVE_COMPRESS_BLOCKS` | `` |  |
| `F2FS_IOC_SEC_TRIM_FILE` | `_IOW(F2FS_IOCTL_MAGIC, 20,	` |  |
| `F2FS_IOC_GET_COMPRESS_OPTION` | `_IOR(F2FS_IOCTL_MAGIC, 21,	` |  |
| `F2FS_IOC_SET_COMPRESS_OPTION` | `_IOW(F2FS_IOCTL_MAGIC, 22,	` |  |
| `F2FS_IOC_DECOMPRESS_FILE` | `_IO(F2FS_IOCTL_MAGIC, 23)` |  |
| `F2FS_IOC_COMPRESS_FILE` | `_IO(F2FS_IOCTL_MAGIC, 24)` |  |
| `F2FS_IOC_START_ATOMIC_REPLACE` | `_IO(F2FS_IOCTL_MAGIC, 25)` |  |
| `F2FS_IOC_GET_DEV_ALIAS_FILE` | `_IOR(F2FS_IOCTL_MAGIC, 26, __u32)` |  |
| `F2FS_IOC_IO_PRIO` | `_IOW(F2FS_IOCTL_MAGIC, 27, __u32)` |  |
| `F2FS_IOC_SHUTDOWN` | `_IOR('X', 125, __u32)` | Shutdown |
| `F2FS_GOING_DOWN_FULLSYNC` | `0x0` | going down with full sync |
| `F2FS_GOING_DOWN_METASYNC` | `0x1` | going down with metadata |
| `F2FS_GOING_DOWN_NOSYNC` | `0x2` | going down |
| `F2FS_GOING_DOWN_METAFLUSH` | `0x3` | going down with meta flush |
| `F2FS_GOING_DOWN_NEED_FSCK` | `0x4` | going down to trigger fsck |
| `F2FS_TRIM_FILE_DISCARD` | `0x1` | send discard command |
| `F2FS_TRIM_FILE_ZEROOUT` | `0x2` | zero out |
| `F2FS_TRIM_FILE_MASK` | `0x3` |  |

## Structs (6)


### `struct f2fs_gc_range`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `sync` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `len` | `-` |

### `struct f2fs_defragment`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `len` | `-` |

### `struct f2fs_move_range`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dst_fd` | `-` |
| `__u64` | `pos_in` | `-` |
| `__u64` | `pos_out` | `-` |
| `__u64` | `len` | `-` |

### `struct f2fs_flush_device`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dev_num` | `-` |
| `__u32` | `segments` | `-` |

### `struct f2fs_sectrim_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `flags` | `-` |

### `struct f2fs_comp_option`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `algorithm` | `-` |
| `__u8` | `log_cluster_size` | `-` |