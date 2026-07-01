# fuse.h

**Source:** `fuse.h`


## Includes

- `linux/types.h`
- `stdint.h`

## Defines (115 total)


### CUSE_INIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `CUSE_INIT_INFO_MAX` | `4096` |  |

### CUSE_UNRESTRICTED (1)

| Name | Value | Comment |
|------|-------|---------|
| `CUSE_UNRESTRICTED_IOCTL` | `(1 << 0)` |  |

### FATTR_ATIME (1)

| Name | Value | Comment |
|------|-------|---------|
| `FATTR_ATIME_NOW` | `(1 << 7)` |  |

### FATTR_KILL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FATTR_KILL_SUIDGID` | `(1 << 11)` |  |

### FATTR_MTIME (1)

| Name | Value | Comment |
|------|-------|---------|
| `FATTR_MTIME_NOW` | `(1 << 8)` |  |

### FOPEN_CACHE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FOPEN_CACHE_DIR` | `(1 << 3)` |  |

### FOPEN_DIRECT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FOPEN_DIRECT_IO` | `(1 << 0)` |  |

### FOPEN_KEEP (1)

| Name | Value | Comment |
|------|-------|---------|
| `FOPEN_KEEP_CACHE` | `(1 << 1)` |  |

### FOPEN_PARALLEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FOPEN_PARALLEL_DIRECT_WRITES` | `(1 << 6)` |  |

### FUSE_ABORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_ABORT_ERROR` | `(1 << 21)` |  |

### FUSE_ALLOW (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_ALLOW_IDMAP` | `(1ULL << 40)` |  |

### FUSE_ASYNC (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_ASYNC_READ` | `(1 << 0)` |  |
| `FUSE_ASYNC_DIO` | `(1 << 15)` |  |

### FUSE_ATOMIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_ATOMIC_O_TRUNC` | `(1 << 3)` |  |

### FUSE_ATTR (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_ATTR_SUBMOUNT` | `(1 << 0)` |  |
| `FUSE_ATTR_DAX` | `(1 << 1)` |  |

### FUSE_AUTO (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_AUTO_INVAL_DATA` | `(1 << 12)` |  |

### FUSE_BIG (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_BIG_WRITES` | `(1 << 5)` |  |

### FUSE_CACHE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_CACHE_SYMLINKS` | `(1 << 23)` |  |

### FUSE_COMPAT (8)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_COMPAT_ENTRY_OUT_SIZE` | `120` |  |
| `FUSE_COMPAT_ATTR_OUT_SIZE` | `96` |  |
| `FUSE_COMPAT_MKNOD_IN_SIZE` | `8` |  |
| `FUSE_COMPAT_WRITE_IN_SIZE` | `24` |  |
| `FUSE_COMPAT_STATFS_SIZE` | `48` |  |
| `FUSE_COMPAT_SETXATTR_IN_SIZE` | `8` |  |
| `FUSE_COMPAT_INIT_OUT_SIZE` | `8` |  |
| `FUSE_COMPAT_22_INIT_OUT_SIZE` | `24` |  |

### FUSE_CREATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_CREATE_SUPP_GROUP` | `(1ULL << 34)` |  |

### FUSE_DEV (5)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_DEV_IOC_MAGIC` | `229` |  |
| `FUSE_DEV_IOC_CLONE` | `_IOR(FUSE_DEV_IOC_MAGIC, 0, uint32_t)` |  |
| `FUSE_DEV_IOC_BACKING_OPEN` | `_IOW(FUSE_DEV_IOC_MAGIC, 1, ` |  |
| `FUSE_DEV_IOC_BACKING_CLOSE` | `_IOW(FUSE_DEV_IOC_MAGIC, 2, uint32_t)` |  |
| `FUSE_DEV_IOC_SYNC_INIT` | `_IO(FUSE_DEV_IOC_MAGIC, 3)` |  |

### FUSE_DIRECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_DIRECT_IO_ALLOW_MMAP` | `(1ULL << 36)` |  |
| `FUSE_DIRECT_IO_RELAX` | `FUSE_DIRECT_IO_ALLOW_MMAP` |  |

### FUSE_DO (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_DO_READDIRPLUS` | `(1 << 13)` |  |

### FUSE_DONT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_DONT_MASK` | `(1 << 6)` |  |

### FUSE_EXPIRE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_EXPIRE_ONLY` | `(1 << 0)` |  |

### FUSE_EXPLICIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_EXPLICIT_INVAL_DATA` | `(1 << 25)` |  |

### FUSE_EXPORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_EXPORT_SUPPORT` | `(1 << 4)` |  |

### FUSE_FILE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_FILE_OPS` | `(1 << 2)` |  |

### FUSE_FLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_FLOCK_LOCKS` | `(1 << 10)` |  |

### FUSE_FSYNC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_FSYNC_FDATASYNC` | `(1 << 0)` |  |

### FUSE_GETATTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_GETATTR_FH` | `(1 << 0)` |  |

### FUSE_HANDLE (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_HANDLE_KILLPRIV` | `(1 << 19)` |  |
| `FUSE_HANDLE_KILLPRIV_V2` | `(1 << 28)` |  |

### FUSE_HAS (4)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_HAS_IOCTL_DIR` | `(1 << 11)` |  |
| `FUSE_HAS_INODE_DAX` | `(1ULL << 33)` |  |
| `FUSE_HAS_EXPIRE_ONLY` | `(1ULL << 35)` |  |
| `FUSE_HAS_RESEND` | `(1ULL << 39)` |  |

### FUSE_INIT (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_INIT_EXT` | `(1 << 30)` |  |
| `FUSE_INIT_RESERVED` | `(1 << 31)` |  |

### FUSE_INVALID (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_INVALID_UIDGID` | `((uint32_t)(-1))` |  |

### FUSE_IOCTL (7)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_IOCTL_COMPAT` | `(1 << 0)` |  |
| `FUSE_IOCTL_UNRESTRICTED` | `(1 << 1)` |  |
| `FUSE_IOCTL_RETRY` | `(1 << 2)` |  |
| `FUSE_IOCTL_32BIT` | `(1 << 3)` |  |
| `FUSE_IOCTL_DIR` | `(1 << 4)` |  |
| `FUSE_IOCTL_COMPAT_X32` | `(1 << 5)` |  |
| `FUSE_IOCTL_MAX_IOV` | `256` |  |

### FUSE_KERNEL (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_KERNEL_VERSION` | `7` |  |
| `FUSE_KERNEL_MINOR_VERSION` | `45` |  |

### FUSE_LK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_LK_FLOCK` | `(1 << 0)` |  |

### FUSE_MAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_MAP_ALIGNMENT` | `(1 << 26)` |  |

### FUSE_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_MAX_PAGES` | `(1 << 22)` |  |

### FUSE_MIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_MIN_READ_BUFFER` | `8192` |  |

### FUSE_NAME (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_NAME_OFFSET` | `offsetof(struct fuse_dirent, name)` |  |
| `FUSE_NAME_OFFSET_DIRENTPLUS` | `` |  |

### FUSE_NO (3)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_NO_OPEN_SUPPORT` | `(1 << 17)` |  |
| `FUSE_NO_OPENDIR_SUPPORT` | `(1 << 24)` |  |
| `FUSE_NO_EXPORT_SUPPORT` | `(1ULL << 38)` |  |

### FUSE_OPEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_OPEN_KILL_SUIDGID` | `(1 << 0)` |  |

### FUSE_OVER (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_OVER_IO_URING` | `(1ULL << 41)` |  |

### FUSE_PARALLEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_PARALLEL_DIROPS` | `(1 << 18)` |  |

### FUSE_POLL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_POLL_SCHEDULE_NOTIFY` | `(1 << 0)` |  |

### FUSE_POSIX (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_POSIX_LOCKS` | `(1 << 1)` |  |
| `FUSE_POSIX_ACL` | `(1 << 20)` |  |

### FUSE_READ (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_READ_LOCKOWNER` | `(1 << 1)` |  |

### FUSE_READDIRPLUS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_READDIRPLUS_AUTO` | `(1 << 14)` |  |

### FUSE_RELEASE (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_RELEASE_FLUSH` | `(1 << 0)` |  |
| `FUSE_RELEASE_FLOCK_UNLOCK` | `(1 << 1)` |  |

### FUSE_REMOVEMAPPING (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_REMOVEMAPPING_MAX_ENTRY` | `` |  |

### FUSE_REQUEST (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_REQUEST_TIMEOUT` | `(1ULL << 42)` |  |

### FUSE_ROOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_ROOT_ID` | `1` |  |

### FUSE_SECURITY (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_SECURITY_CTX` | `(1ULL << 32)` |  |

### FUSE_SETUPMAPPING (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_SETUPMAPPING_FLAG_WRITE` | `(1ull << 0)` |  |
| `FUSE_SETUPMAPPING_FLAG_READ` | `(1ull << 1)` |  |

### FUSE_SETXATTR (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_SETXATTR_EXT` | `(1 << 29)` |  |
| `FUSE_SETXATTR_ACL_KILL_SGID` | `(1 << 0)` |  |

### FUSE_SPLICE (3)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_SPLICE_WRITE` | `(1 << 7)` |  |
| `FUSE_SPLICE_MOVE` | `(1 << 8)` |  |
| `FUSE_SPLICE_READ` | `(1 << 9)` |  |

### FUSE_UNIQUE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_UNIQUE_RESEND` | `(1ULL << 63)` |  |

### FUSE_URING (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_URING_IN_OUT_HEADER_SZ` | `128` |  |
| `FUSE_URING_OP_IN_OUT_SZ` | `128` |  |

### FUSE_WRITE (4)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_WRITE_CACHE` | `(1 << 0)` |  |
| `FUSE_WRITE_LOCKOWNER` | `(1 << 1)` |  |
| `FUSE_WRITE_KILL_SUIDGID` | `(1 << 2)` |  |
| `FUSE_WRITE_KILL_PRIV` | `FUSE_WRITE_KILL_SUIDGID` |  |

### FUSE_WRITEBACK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUSE_WRITEBACK_CACHE` | `(1 << 16)` |  |

### UNCATEGORIZED (15)

| Name | Value | Comment |
|------|-------|---------|
| `FATTR_MODE` | `(1 << 0)` |  |
| `FATTR_UID` | `(1 << 1)` |  |
| `FATTR_GID` | `(1 << 2)` |  |
| `FATTR_SIZE` | `(1 << 3)` |  |
| `FATTR_ATIME` | `(1 << 4)` |  |
| `FATTR_MTIME` | `(1 << 5)` |  |
| `FATTR_FH` | `(1 << 6)` |  |
| `FATTR_LOCKOWNER` | `(1 << 9)` |  |
| `FATTR_CTIME` | `(1 << 10)` |  |
| `FOPEN_NONSEEKABLE` | `(1 << 2)` |  |
| `FOPEN_STREAM` | `(1 << 4)` |  |
| `FOPEN_NOFLUSH` | `(1 << 5)` |  |
| `FOPEN_PASSTHROUGH` | `(1 << 7)` |  |
| `FUSE_SUBMOUNTS` | `(1 << 27)` |  |
| `FUSE_PASSTHROUGH` | `(1ULL << 37)` |  |

## Structs (76)


### `struct fuse_attr`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `ino` | `-` |
| `uint64_t` | `size` | `-` |
| `uint64_t` | `blocks` | `-` |
| `uint64_t` | `atime` | `-` |
| `uint64_t` | `mtime` | `-` |
| `uint64_t` | `ctime` | `-` |
| `uint32_t` | `atimensec` | `-` |
| `uint32_t` | `mtimensec` | `-` |
| `uint32_t` | `ctimensec` | `-` |
| `uint32_t` | `mode` | `-` |
| `uint32_t` | `nlink` | `-` |
| `uint32_t` | `uid` | `-` |
| `uint32_t` | `gid` | `-` |
| `uint32_t` | `rdev` | `-` |
| `uint32_t` | `blksize` | `-` |
| `uint32_t` | `flags` | `-` |

### `struct fuse_sx_time`

| Type | Field | Array |
|------|-------|-------|
| `int64_t` | `tv_sec` | `-` |
| `uint32_t` | `tv_nsec` | `-` |
| `int32_t` | `__reserved` | `-` |

### `struct fuse_statx`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `mask` | `-` |
| `uint32_t` | `blksize` | `-` |
| `uint64_t` | `attributes` | `-` |
| `uint32_t` | `nlink` | `-` |
| `uint32_t` | `uid` | `-` |
| `uint32_t` | `gid` | `-` |
| `uint16_t` | `mode` | `-` |
| `uint16_t` | `__spare0` | `1` |
| `uint64_t` | `ino` | `-` |
| `uint64_t` | `size` | `-` |
| `uint64_t` | `blocks` | `-` |
| `uint64_t` | `attributes_mask` | `-` |
| `uint32_t` | `rdev_major` | `-` |
| `uint32_t` | `rdev_minor` | `-` |
| `uint32_t` | `dev_major` | `-` |
| `uint32_t` | `dev_minor` | `-` |
| `uint64_t` | `__spare2` | `14` |

### `struct fuse_kstatfs`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `blocks` | `-` |
| `uint64_t` | `bfree` | `-` |
| `uint64_t` | `bavail` | `-` |
| `uint64_t` | `files` | `-` |
| `uint64_t` | `ffree` | `-` |
| `uint32_t` | `bsize` | `-` |
| `uint32_t` | `namelen` | `-` |
| `uint32_t` | `frsize` | `-` |
| `uint32_t` | `padding` | `-` |
| `uint32_t` | `spare` | `6` |

### `struct fuse_file_lock`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `start` | `-` |
| `uint64_t` | `end` | `-` |
| `uint32_t` | `type` | `-` |
| `uint32_t` | `pid` | `-` |

### `struct fuse_entry_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `nodeid` | `-` |
| `uint64_t` | `generation` | `-` |
| `uint64_t` | `entry_valid` | `-` |
| `uint64_t` | `attr_valid` | `-` |
| `uint32_t` | `entry_valid_nsec` | `-` |
| `uint32_t` | `attr_valid_nsec` | `-` |

### `struct fuse_forget_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `nlookup` | `-` |

### `struct fuse_forget_one`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `nodeid` | `-` |
| `uint64_t` | `nlookup` | `-` |

### `struct fuse_batch_forget_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `count` | `-` |
| `uint32_t` | `dummy` | `-` |

### `struct fuse_getattr_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `getattr_flags` | `-` |
| `uint32_t` | `dummy` | `-` |
| `uint64_t` | `fh` | `-` |

### `struct fuse_attr_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `attr_valid` | `-` |
| `uint32_t` | `attr_valid_nsec` | `-` |
| `uint32_t` | `dummy` | `-` |

### `struct fuse_statx_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `getattr_flags` | `-` |
| `uint32_t` | `reserved` | `-` |
| `uint64_t` | `fh` | `-` |
| `uint32_t` | `sx_flags` | `-` |
| `uint32_t` | `sx_mask` | `-` |

### `struct fuse_statx_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `attr_valid` | `-` |
| `uint32_t` | `attr_valid_nsec` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint64_t` | `spare` | `2` |

### `struct fuse_mknod_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `mode` | `-` |
| `uint32_t` | `rdev` | `-` |
| `uint32_t` | `umask` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_mkdir_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `mode` | `-` |
| `uint32_t` | `umask` | `-` |

### `struct fuse_rename_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `newdir` | `-` |

### `struct fuse_rename2_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `newdir` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_link_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `oldnodeid` | `-` |

### `struct fuse_setattr_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `valid` | `-` |
| `uint32_t` | `padding` | `-` |
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `size` | `-` |
| `uint64_t` | `lock_owner` | `-` |
| `uint64_t` | `atime` | `-` |
| `uint64_t` | `mtime` | `-` |
| `uint64_t` | `ctime` | `-` |
| `uint32_t` | `atimensec` | `-` |
| `uint32_t` | `mtimensec` | `-` |
| `uint32_t` | `ctimensec` | `-` |
| `uint32_t` | `mode` | `-` |
| `uint32_t` | `unused4` | `-` |
| `uint32_t` | `uid` | `-` |
| `uint32_t` | `gid` | `-` |
| `uint32_t` | `unused5` | `-` |

### `struct fuse_open_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `open_flags` | `-` |

### `struct fuse_create_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `mode` | `-` |
| `uint32_t` | `umask` | `-` |
| `uint32_t` | `open_flags` | `-` |

### `struct fuse_open_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint32_t` | `open_flags` | `-` |
| `int32_t` | `backing_id` | `-` |

### `struct fuse_release_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `release_flags` | `-` |
| `uint64_t` | `lock_owner` | `-` |

### `struct fuse_flush_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint32_t` | `unused` | `-` |
| `uint32_t` | `padding` | `-` |
| `uint64_t` | `lock_owner` | `-` |

### `struct fuse_read_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint32_t` | `size` | `-` |
| `uint32_t` | `read_flags` | `-` |
| `uint64_t` | `lock_owner` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_write_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint32_t` | `size` | `-` |
| `uint32_t` | `write_flags` | `-` |
| `uint64_t` | `lock_owner` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_write_out`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_statfs_out`

| Type | Field | Array |
|------|-------|-------|

### `struct fuse_fsync_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint32_t` | `fsync_flags` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_setxattr_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `setxattr_flags` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_getxattr_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_getxattr_out`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_lk_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `owner` | `-` |
| `uint32_t` | `lk_flags` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_lk_out`

| Type | Field | Array |
|------|-------|-------|

### `struct fuse_access_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `mask` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_init_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `major` | `-` |
| `uint32_t` | `minor` | `-` |
| `uint32_t` | `max_readahead` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `flags2` | `-` |
| `uint32_t` | `unused` | `11` |

### `struct fuse_init_out`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `major` | `-` |
| `uint32_t` | `minor` | `-` |
| `uint32_t` | `max_readahead` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint16_t` | `max_background` | `-` |
| `uint16_t` | `congestion_threshold` | `-` |
| `uint32_t` | `max_write` | `-` |
| `uint32_t` | `time_gran` | `-` |
| `uint16_t` | `max_pages` | `-` |
| `uint16_t` | `map_alignment` | `-` |
| `uint32_t` | `flags2` | `-` |
| `uint32_t` | `max_stack_depth` | `-` |
| `uint16_t` | `request_timeout` | `-` |
| `uint16_t` | `unused` | `11` |

### `struct cuse_init_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `major` | `-` |
| `uint32_t` | `minor` | `-` |
| `uint32_t` | `unused` | `-` |
| `uint32_t` | `flags` | `-` |

### `struct cuse_init_out`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `major` | `-` |
| `uint32_t` | `minor` | `-` |
| `uint32_t` | `unused` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `max_read` | `-` |
| `uint32_t` | `max_write` | `-` |
| `uint32_t` | `dev_major` | `-` |
| `uint32_t` | `dev_minor` | `-` |
| `uint32_t` | `spare` | `10` |

### `struct fuse_interrupt_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `unique` | `-` |

### `struct fuse_bmap_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `block` | `-` |
| `uint32_t` | `blocksize` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_bmap_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `block` | `-` |

### `struct fuse_ioctl_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `cmd` | `-` |
| `uint64_t` | `arg` | `-` |
| `uint32_t` | `in_size` | `-` |
| `uint32_t` | `out_size` | `-` |

### `struct fuse_ioctl_iovec`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `base` | `-` |
| `uint64_t` | `len` | `-` |

### `struct fuse_ioctl_out`

| Type | Field | Array |
|------|-------|-------|
| `int32_t` | `result` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `in_iovs` | `-` |
| `uint32_t` | `out_iovs` | `-` |

### `struct fuse_poll_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `kh` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint32_t` | `events` | `-` |

### `struct fuse_poll_out`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `revents` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_notify_poll_wakeup_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `kh` | `-` |

### `struct fuse_fallocate_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint64_t` | `length` | `-` |
| `uint32_t` | `mode` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_in_header`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `len` | `-` |
| `uint32_t` | `opcode` | `-` |
| `uint64_t` | `unique` | `-` |
| `uint64_t` | `nodeid` | `-` |
| `uint32_t` | `uid` | `-` |
| `uint32_t` | `gid` | `-` |
| `uint32_t` | `pid` | `-` |
| `uint16_t` | `total_extlen` | `-` |
| `uint16_t` | `padding` | `-` |

### `struct fuse_out_header`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `len` | `-` |
| `int32_t` | `error` | `-` |
| `uint64_t` | `unique` | `-` |

### `struct fuse_dirent`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `ino` | `-` |
| `uint64_t` | `off` | `-` |
| `uint32_t` | `namelen` | `-` |
| `uint32_t` | `type` | `-` |

### `struct fuse_direntplus`

| Type | Field | Array |
|------|-------|-------|

### `struct fuse_notify_inval_inode_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `ino` | `-` |
| `int64_t` | `off` | `-` |
| `int64_t` | `len` | `-` |

### `struct fuse_notify_inval_entry_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `parent` | `-` |
| `uint32_t` | `namelen` | `-` |
| `uint32_t` | `flags` | `-` |

### `struct fuse_notify_delete_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `parent` | `-` |
| `uint64_t` | `child` | `-` |
| `uint32_t` | `namelen` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_notify_store_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `nodeid` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint32_t` | `size` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_notify_retrieve_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `notify_unique` | `-` |
| `uint64_t` | `nodeid` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint32_t` | `size` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_notify_retrieve_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `dummy1` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint32_t` | `size` | `-` |
| `uint32_t` | `dummy2` | `-` |
| `uint64_t` | `dummy3` | `-` |
| `uint64_t` | `dummy4` | `-` |

### `struct fuse_notify_prune_out`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `count` | `-` |
| `uint32_t` | `padding` | `-` |
| `uint64_t` | `spare` | `-` |

### `struct fuse_backing_map`

| Type | Field | Array |
|------|-------|-------|
| `int32_t` | `fd` | `-` |
| `uint32_t` | `flags` | `-` |
| `uint64_t` | `padding` | `-` |

### `struct fuse_lseek_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `offset` | `-` |
| `uint32_t` | `whence` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_lseek_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `offset` | `-` |

### `struct fuse_copy_file_range_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh_in` | `-` |
| `uint64_t` | `off_in` | `-` |
| `uint64_t` | `nodeid_out` | `-` |
| `uint64_t` | `fh_out` | `-` |
| `uint64_t` | `off_out` | `-` |
| `uint64_t` | `len` | `-` |
| `uint64_t` | `flags` | `-` |

### `struct fuse_copy_file_range_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `bytes_copied` | `-` |

### `struct fuse_setupmapping_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `fh` | `-` |
| `uint64_t` | `foffset` | `-` |
| `uint64_t` | `len` | `-` |
| `uint64_t` | `flags` | `-` |
| `uint64_t` | `moffset` | `-` |

### `struct fuse_removemapping_in`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `count` | `-` |

### `struct fuse_removemapping_one`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `moffset` | `-` |
| `uint64_t` | `len` | `-` |

### `struct fuse_syncfs_in`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `padding` | `-` |

### `struct fuse_secctx`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `padding` | `-` |

### `struct fuse_secctx_header`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `nr_secctx` | `-` |

### `struct fuse_ext_header`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `size` | `-` |
| `uint32_t` | `type` | `-` |

### `struct fuse_supp_groups`

| Type | Field | Array |
|------|-------|-------|
| `uint32_t` | `nr_groups` | `-` |

### `struct fuse_uring_ent_in_out`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `flags` | `-` |
| `uint64_t` | `commit_id` | `-` |
| `uint32_t` | `payload_sz` | `-` |
| `uint32_t` | `padding` | `-` |
| `uint64_t` | `reserved` | `-` |

### `struct fuse_uring_req_header`

| Type | Field | Array |
|------|-------|-------|
| `char` | `in_out` | `FUSE_URING_IN_OUT_HEADER_SZ` |
| `char` | `op_in` | `FUSE_URING_OP_IN_OUT_SZ` |

### `struct fuse_uring_cmd_req`

| Type | Field | Array |
|------|-------|-------|
| `uint64_t` | `flags` | `-` |
| `uint64_t` | `commit_id` | `-` |
| `uint16_t` | `qid` | `-` |
| `uint8_t` | `padding` | `6` |