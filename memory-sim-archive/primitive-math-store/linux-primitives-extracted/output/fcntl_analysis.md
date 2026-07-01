# fcntl.h

**Source:** `fcntl.h`


## Includes

- `asm/fcntl.h`
- `linux/openat2.h`
- `linux/types.h`

## Defines (61 total)


### AT_EMPTY (1)

| Name | Value | Comment |
|------|-------|---------|
| `AT_EMPTY_PATH` | `0x1000	/* Allow empty relative` |  |

### AT_EXECVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `AT_EXECVE_CHECK` | `0x10000	/* Only perform a check if execution` |  |

### AT_HANDLE (3)

| Name | Value | Comment |
|------|-------|---------|
| `AT_HANDLE_FID` | `0x200	/* File handle is needed to compare` |  |
| `AT_HANDLE_MNT_ID_UNIQUE` | `0x001` | Return the u64 unique mount ID. |
| `AT_HANDLE_CONNECTABLE` | `0x002` | Request a connectable file handle |

### AT_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `AT_NO_AUTOMOUNT` | `0x800	/* Suppress terminal automount` |  |

### AT_RENAME (3)

| Name | Value | Comment |
|------|-------|---------|
| `AT_RENAME_NOREPLACE` | `0x0001` |  |
| `AT_RENAME_EXCHANGE` | `0x0002` |  |
| `AT_RENAME_WHITEOUT` | `0x0004` |  |

### AT_STATX (4)

| Name | Value | Comment |
|------|-------|---------|
| `AT_STATX_SYNC_TYPE` | `0x6000` | Type of synchronisation required from statx() |
| `AT_STATX_SYNC_AS_STAT` | `0x0000` | - Do whatever stat() does |
| `AT_STATX_FORCE_SYNC` | `0x2000` | - Force the attributes to be sync'd with the server |
| `AT_STATX_DONT_SYNC` | `0x4000` | - Don't sync attributes with the server |

### AT_SYMLINK (2)

| Name | Value | Comment |
|------|-------|---------|
| `AT_SYMLINK_NOFOLLOW` | `0x100   /* Do not follow symbolic` |  |
| `AT_SYMLINK_FOLLOW` | `0x400` | Follow symbolic links. |

### FD_NSFS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_NSFS_ROOT` | `-10003` | Root of the nsfs filesystem |

### FD_PIDFS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_PIDFS_ROOT` | `-10002` | Root of the pidfs filesystem |

### F_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `F_ADD_SEALS` | `(F_LINUX_SPECIFIC_BASE + 9)` |  |

### F_CREATED (1)

| Name | Value | Comment |
|------|-------|---------|
| `F_CREATED_QUERY` | `(F_LINUX_SPECIFIC_BASE + 4)` |  |

### F_DUPFD (2)

| Name | Value | Comment |
|------|-------|---------|
| `F_DUPFD_QUERY` | `(F_LINUX_SPECIFIC_BASE + 3)` |  |
| `F_DUPFD_CLOEXEC` | `(F_LINUX_SPECIFIC_BASE + 6)` |  |

### F_GET (3)

| Name | Value | Comment |
|------|-------|---------|
| `F_GET_SEALS` | `(F_LINUX_SPECIFIC_BASE + 10)` |  |
| `F_GET_RW_HINT` | `(F_LINUX_SPECIFIC_BASE + 11)` |  |
| `F_GET_FILE_RW_HINT` | `(F_LINUX_SPECIFIC_BASE + 13)` |  |

### F_GETPIPE (1)

| Name | Value | Comment |
|------|-------|---------|
| `F_GETPIPE_SZ` | `(F_LINUX_SPECIFIC_BASE + 8)` |  |

### F_SEAL (6)

| Name | Value | Comment |
|------|-------|---------|
| `F_SEAL_SEAL` | `0x0001` | prevent further seals from being set |
| `F_SEAL_SHRINK` | `0x0002` | prevent file from shrinking |
| `F_SEAL_GROW` | `0x0004` | prevent file from growing |
| `F_SEAL_WRITE` | `0x0008` | prevent writes |
| `F_SEAL_FUTURE_WRITE` | `0x0010` | prevent future writes while mapped |
| `F_SEAL_EXEC` | `0x0020` | prevent chmod modifying exec bits |

### F_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `F_SET_RW_HINT` | `(F_LINUX_SPECIFIC_BASE + 12)` |  |
| `F_SET_FILE_RW_HINT` | `(F_LINUX_SPECIFIC_BASE + 14)` |  |

### F_SETPIPE (1)

| Name | Value | Comment |
|------|-------|---------|
| `F_SETPIPE_SZ` | `(F_LINUX_SPECIFIC_BASE + 7)` |  |

### PIDFD_SELF (2)

| Name | Value | Comment |
|------|-------|---------|
| `PIDFD_SELF_THREAD` | `-10000` | Current thread. |
| `PIDFD_SELF_THREAD_GROUP` | `-10001` | Current thread group leader. |

### RWF_WRITE (1)

| Name | Value | Comment |
|------|-------|---------|
| `RWF_WRITE_LIFE_NOT_SET` | `RWH_WRITE_LIFE_NOT_SET` |  |

### RWH_WRITE (6)

| Name | Value | Comment |
|------|-------|---------|
| `RWH_WRITE_LIFE_NOT_SET` | `0` |  |
| `RWH_WRITE_LIFE_NONE` | `1` |  |
| `RWH_WRITE_LIFE_SHORT` | `2` |  |
| `RWH_WRITE_LIFE_MEDIUM` | `3` |  |
| `RWH_WRITE_LIFE_LONG` | `4` |  |
| `RWH_WRITE_LIFE_EXTREME` | `5` |  |

### UNCATEGORIZED (18)

| Name | Value | Comment |
|------|-------|---------|
| `F_SETLEASE` | `(F_LINUX_SPECIFIC_BASE + 0)` |  |
| `F_GETLEASE` | `(F_LINUX_SPECIFIC_BASE + 1)` |  |
| `F_NOTIFY` | `(F_LINUX_SPECIFIC_BASE + 2)` |  |
| `F_CANCELLK` | `(F_LINUX_SPECIFIC_BASE + 5)` |  |
| `F_GETDELEG` | `(F_LINUX_SPECIFIC_BASE + 15)` |  |
| `F_SETDELEG` | `(F_LINUX_SPECIFIC_BASE + 16)` |  |
| `DN_ACCESS` | `0x00000001` | File accessed |
| `DN_MODIFY` | `0x00000002` | File modified |
| `DN_CREATE` | `0x00000004` | File created |
| `DN_DELETE` | `0x00000008` | File removed |
| `DN_RENAME` | `0x00000010` | File renamed |
| `DN_ATTRIB` | `0x00000020` | File changed attibutes |
| `DN_MULTISHOT` | `0x80000000` | Don't remove notifier |
| `AT_FDCWD` | `-100    /* Special value for dirfd used to` |  |
| `FD_INVALID` | `-10009` | Invalid file descriptor: -10000 - EBADF = -10009 |
| `AT_RECURSIVE` | `0x8000` | Apply to the entire subtree |
| `AT_EACCESS` | `0x200	/* Test access permitted for` |  |
| `AT_REMOVEDIR` | `0x200   /* Remove directory instead of` |  |

## Structs (1)


### `struct delegation`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `d_flags` | `-` |
| `__u16` | `d_type` | `-` |
| `__u16` | `__pad` | `-` |