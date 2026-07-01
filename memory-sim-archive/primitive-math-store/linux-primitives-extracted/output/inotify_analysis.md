# inotify.h

**Source:** `inotify.h`


## Includes

- `linux/fcntl.h`
- `linux/types.h`

## Defines (28 total)


### INOTIFY_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `INOTIFY_IOC_SETNEXTWD` | `_IOW('I', 0, __s32)` |  |

### IN_ALL (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_ALL_EVENTS` | `(IN_ACCESS \| IN_MODIFY \| IN_ATTRIB \| IN_CLOSE_WRITE \| ` |  |

### IN_CLOSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `IN_CLOSE_WRITE` | `0x00000008` | Writable file was closed |
| `IN_CLOSE_NOWRITE` | `0x00000010` | Unwritable file closed |

### IN_DELETE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_DELETE_SELF` | `0x00000400` | Self was deleted |

### IN_DONT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_DONT_FOLLOW` | `0x02000000` | don't follow a sym link |

### IN_EXCL (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_EXCL_UNLINK` | `0x04000000` | exclude events on unlinked objects |

### IN_MASK (2)

| Name | Value | Comment |
|------|-------|---------|
| `IN_MASK_CREATE` | `0x10000000` | only create watches |
| `IN_MASK_ADD` | `0x20000000` | add to the mask of an already existing watch |

### IN_MOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_MOVE_SELF` | `0x00000800` | Self was moved |

### IN_MOVED (2)

| Name | Value | Comment |
|------|-------|---------|
| `IN_MOVED_FROM` | `0x00000040` | File was moved from X |
| `IN_MOVED_TO` | `0x00000080` | File was moved to Y |

### IN_Q (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_Q_OVERFLOW` | `0x00004000` | Event queued overflowed |

### UNCATEGORIZED (15)

| Name | Value | Comment |
|------|-------|---------|
| `IN_ACCESS` | `0x00000001` | File was accessed |
| `IN_MODIFY` | `0x00000002` | File was modified |
| `IN_ATTRIB` | `0x00000004` | Metadata changed |
| `IN_OPEN` | `0x00000020` | File was opened |
| `IN_CREATE` | `0x00000100` | Subfile was created |
| `IN_DELETE` | `0x00000200` | Subfile was deleted |
| `IN_UNMOUNT` | `0x00002000` | Backing fs was unmounted |
| `IN_IGNORED` | `0x00008000` | File was ignored |
| `IN_CLOSE` | `(IN_CLOSE_WRITE \| IN_CLOSE_NOWRITE)` | close |
| `IN_MOVE` | `(IN_MOVED_FROM \| IN_MOVED_TO)` | moves |
| `IN_ONLYDIR` | `0x01000000` | only watch the path if it is a directory |
| `IN_ISDIR` | `0x40000000` | event occurred against dir |
| `IN_ONESHOT` | `0x80000000` | only send event once |
| `IN_CLOEXEC` | `O_CLOEXEC` |  |
| `IN_NONBLOCK` | `O_NONBLOCK` |  |

## Structs (1)


### `struct inotify_event`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `wd` | `-` |
| `__u32` | `mask` | `-` |
| `__u32` | `cookie` | `-` |
| `__u32` | `len` | `-` |