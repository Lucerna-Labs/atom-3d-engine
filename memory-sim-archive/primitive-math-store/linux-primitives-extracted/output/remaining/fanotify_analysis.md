# fanotify.h

**Source:** `fanotify.h`


## Includes

- `linux/types.h`

## Defines (87 total)


### FANOTIFY_METADATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `FANOTIFY_METADATA_VERSION` | `3` |  |

### FAN_ACCESS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_ACCESS_PERM` | `0x00020000` | File accessed in perm check |

### FAN_ALL (6)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_ALL_CLASS_BITS` | `(FAN_CLASS_NOTIF \| FAN_CLASS_CONTENT \| ` |  |
| `FAN_ALL_INIT_FLAGS` | `(FAN_CLOEXEC \| FAN_NONBLOCK \| ` |  |
| `FAN_ALL_MARK_FLAGS` | `(FAN_MARK_ADD \|` |  |
| `FAN_ALL_EVENTS` | `(FAN_ACCESS \|` |  |
| `FAN_ALL_PERM_EVENTS` | `(FAN_OPEN_PERM \|` |  |
| `FAN_ALL_OUTGOING_EVENTS` | `(FAN_ALL_EVENTS \|` |  |

### FAN_CLASS (3)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_CLASS_NOTIF` | `0x00000000` |  |
| `FAN_CLASS_CONTENT` | `0x00000004` |  |
| `FAN_CLASS_PRE_CONTENT` | `0x00000008` |  |

### FAN_CLOSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_CLOSE_WRITE` | `0x00000008` | Writable file closed |
| `FAN_CLOSE_NOWRITE` | `0x00000010` | Unwritable file closed |

### FAN_DELETE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_DELETE_SELF` | `0x00000400` | Self was deleted |

### FAN_ENABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_ENABLE_AUDIT` | `0x00000040` |  |

### FAN_ERRNO (3)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_ERRNO_BITS` | `8` |  |
| `FAN_ERRNO_SHIFT` | `(32 - FAN_ERRNO_BITS)` |  |
| `FAN_ERRNO_MASK` | `((1 << FAN_ERRNO_BITS) - 1)` |  |

### FAN_EVENT (11)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_EVENT_ON_CHILD` | `0x08000000` | Interested in child events |
| `FAN_EVENT_INFO_TYPE_FID` | `1` |  |
| `FAN_EVENT_INFO_TYPE_DFID_NAME` | `2` |  |
| `FAN_EVENT_INFO_TYPE_DFID` | `3` |  |
| `FAN_EVENT_INFO_TYPE_PIDFD` | `4` |  |
| `FAN_EVENT_INFO_TYPE_ERROR` | `5` |  |
| `FAN_EVENT_INFO_TYPE_RANGE` | `6` |  |
| `FAN_EVENT_INFO_TYPE_MNT` | `7` |  |
| `FAN_EVENT_INFO_TYPE_OLD_DFID_NAME` | `10` |  |
| `FAN_EVENT_INFO_TYPE_NEW_DFID_NAME` | `12` |  |
| `FAN_EVENT_METADATA_LEN` | `(sizeof(struct fanotify_event_metadata))` |  |

### FAN_FS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_FS_ERROR` | `0x00008000` | Filesystem error |

### FAN_MARK (14)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_MARK_ADD` | `0x00000001` |  |
| `FAN_MARK_REMOVE` | `0x00000002` |  |
| `FAN_MARK_DONT_FOLLOW` | `0x00000004` |  |
| `FAN_MARK_ONLYDIR` | `0x00000008` |  |
| `FAN_MARK_IGNORED_MASK` | `0x00000020` |  |
| `FAN_MARK_IGNORED_SURV_MODIFY` | `0x00000040` |  |
| `FAN_MARK_FLUSH` | `0x00000080` |  |
| `FAN_MARK_EVICTABLE` | `0x00000200` |  |
| `FAN_MARK_IGNORE` | `0x00000400` |  |
| `FAN_MARK_INODE` | `0x00000000` |  |
| `FAN_MARK_MOUNT` | `0x00000010` |  |
| `FAN_MARK_FILESYSTEM` | `0x00000100` |  |
| `FAN_MARK_MNTNS` | `0x00000110` |  |
| `FAN_MARK_IGNORE_SURV` | `(FAN_MARK_IGNORE \| FAN_MARK_IGNORED_SURV_MODIFY)` |  |

### FAN_MNT (2)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_MNT_ATTACH` | `0x01000000` | Mount was attached |
| `FAN_MNT_DETACH` | `0x02000000` | Mount was detached |

### FAN_MOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_MOVE_SELF` | `0x00000800` | Self was moved |

### FAN_MOVED (2)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_MOVED_FROM` | `0x00000040` | File was moved from X |
| `FAN_MOVED_TO` | `0x00000080` | File was moved to Y |

### FAN_OPEN (3)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_OPEN_EXEC` | `0x00001000` | File was opened for exec |
| `FAN_OPEN_PERM` | `0x00010000` | File open in perm check |
| `FAN_OPEN_EXEC_PERM` | `0x00040000` | File open/exec in perm check |

### FAN_PRE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_PRE_ACCESS` | `0x00100000` | Pre-content access hook |

### FAN_Q (1)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_Q_OVERFLOW` | `0x00004000` | Event queued overflowed |

### FAN_REPORT (10)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_REPORT_PIDFD` | `0x00000080` | Report pidfd for event->pid |
| `FAN_REPORT_TID` | `0x00000100` | event->pid is thread id |
| `FAN_REPORT_FID` | `0x00000200` | Report unique file id |
| `FAN_REPORT_DIR_FID` | `0x00000400` | Report unique directory id |
| `FAN_REPORT_NAME` | `0x00000800` | Report events with name |
| `FAN_REPORT_TARGET_FID` | `0x00001000` | Report dirent target id |
| `FAN_REPORT_FD_ERROR` | `0x00002000` | event->fd can report error |
| `FAN_REPORT_MNT` | `0x00004000` | Report mount events |
| `FAN_REPORT_DFID_NAME` | `(FAN_REPORT_DIR_FID \| FAN_REPORT_NAME)` |  |
| `FAN_REPORT_DFID_NAME_TARGET` | `(FAN_REPORT_DFID_NAME \| ` |  |

### FAN_RESPONSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_RESPONSE_INFO_NONE` | `0` |  |
| `FAN_RESPONSE_INFO_AUDIT_RULE` | `1` |  |

### FAN_UNLIMITED (2)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_UNLIMITED_QUEUE` | `0x00000010` |  |
| `FAN_UNLIMITED_MARKS` | `0x00000020` |  |

### UNCATEGORIZED (19)

| Name | Value | Comment |
|------|-------|---------|
| `FAN_ACCESS` | `0x00000001` | File was accessed |
| `FAN_MODIFY` | `0x00000002` | File was modified |
| `FAN_ATTRIB` | `0x00000004` | Metadata changed |
| `FAN_OPEN` | `0x00000020` | File was opened |
| `FAN_CREATE` | `0x00000100` | Subfile was created |
| `FAN_DELETE` | `0x00000200` | Subfile was deleted |
| `FAN_RENAME` | `0x10000000` | File was renamed |
| `FAN_ONDIR` | `0x40000000` | Event occurred against dir |
| `FAN_CLOSE` | `(FAN_CLOSE_WRITE \| FAN_CLOSE_NOWRITE)` | close |
| `FAN_MOVE` | `(FAN_MOVED_FROM \| FAN_MOVED_TO)` | moves |
| `FAN_CLOEXEC` | `0x00000001` |  |
| `FAN_NONBLOCK` | `0x00000002` |  |
| `FAN_ALLOW` | `0x01` |  |
| `FAN_DENY` | `0x02` |  |
| `FAN_AUDIT` | `0x10` | Bitmask to create audit record for result |
| `FAN_INFO` | `0x20` | Bitmask to indicate additional information |
| `FAN_NOFD` | `-1` |  |
| `FAN_NOPIDFD` | `FAN_NOFD` |  |
| `FAN_EPIDFD` | `-2` |  |

## Structs (10)


### `struct fanotify_event_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `event_len` | `-` |
| `__u8` | `vers` | `-` |
| `__u8` | `reserved` | `-` |
| `__u16` | `metadata_len` | `-` |
| `__aligned_u64` | `mask` | `-` |
| `__s32` | `fd` | `-` |
| `__s32` | `pid` | `-` |

### `struct fanotify_event_info_header`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `info_type` | `-` |
| `__u8` | `pad` | `-` |
| `__u16` | `len` | `-` |

### `struct fanotify_event_info_fid`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_fsid_t` | `fsid` | `-` |

### `struct fanotify_event_info_pidfd`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `pidfd` | `-` |

### `struct fanotify_event_info_error`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `error` | `-` |
| `__u32` | `error_count` | `-` |

### `struct fanotify_event_info_range`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `count` | `-` |

### `struct fanotify_event_info_mnt`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mnt_id` | `-` |

### `struct fanotify_response`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `fd` | `-` |
| `__u32` | `response` | `-` |

### `struct fanotify_response_info_header`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `pad` | `-` |
| `__u16` | `len` | `-` |

### `struct fanotify_response_info_audit_rule`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rule_number` | `-` |
| `__u32` | `subj_trust` | `-` |
| `__u32` | `obj_trust` | `-` |