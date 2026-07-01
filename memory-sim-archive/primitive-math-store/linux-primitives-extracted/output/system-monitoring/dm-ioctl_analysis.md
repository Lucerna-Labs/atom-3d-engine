# dm-ioctl.h

**Source:** `dm-ioctl.h`


## Includes

- `linux/types.h`

## Defines (49 total)


### DM_ACTIVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_ACTIVE_PRESENT_FLAG` | `(1 << 5)` | Out |

### DM_BUFFER (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_BUFFER_FULL_FLAG` | `(1 << 8)` | Out |

### DM_CONTROL (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_CONTROL_NODE` | `"control"` |  |

### DM_DATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_DATA_OUT_FLAG` | `(1 << 16)` | Out |

### DM_DEFERRED (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_DEFERRED_REMOVE` | `(1 << 17)` | In/Out |

### DM_DEV (8)

| Name | Value | Comment |
|------|-------|---------|
| `DM_DEV_CREATE` | `_IOWR(DM_IOCTL, DM_DEV_CREATE_CMD, struct dm_ioctl)` |  |
| `DM_DEV_REMOVE` | `_IOWR(DM_IOCTL, DM_DEV_REMOVE_CMD, struct dm_ioctl)` |  |
| `DM_DEV_RENAME` | `_IOWR(DM_IOCTL, DM_DEV_RENAME_CMD, struct dm_ioctl)` |  |
| `DM_DEV_SUSPEND` | `_IOWR(DM_IOCTL, DM_DEV_SUSPEND_CMD, struct dm_ioctl)` |  |
| `DM_DEV_STATUS` | `_IOWR(DM_IOCTL, DM_DEV_STATUS_CMD, struct dm_ioctl)` |  |
| `DM_DEV_WAIT` | `_IOWR(DM_IOCTL, DM_DEV_WAIT_CMD, struct dm_ioctl)` |  |
| `DM_DEV_ARM_POLL` | `_IOWR(DM_IOCTL, DM_DEV_ARM_POLL_CMD, struct dm_ioctl)` |  |
| `DM_DEV_SET_GEOMETRY` | `_IOWR(DM_IOCTL, DM_DEV_SET_GEOMETRY_CMD, struct dm_ioctl)` |  |

### DM_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_GET_TARGET_VERSION` | `_IOWR(DM_IOCTL, DM_GET_TARGET_VERSION_CMD, struct dm_ioctl)` |  |

### DM_IMA (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_IMA_MEASUREMENT_FLAG` | `(1 << 19)` | In |

### DM_INACTIVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_INACTIVE_PRESENT_FLAG` | `(1 << 6)` | Out |

### DM_INTERNAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_INTERNAL_SUSPEND_FLAG` | `(1 << 18)` | Out |

### DM_LIST (2)

| Name | Value | Comment |
|------|-------|---------|
| `DM_LIST_DEVICES` | `_IOWR(DM_IOCTL, DM_LIST_DEVICES_CMD, struct dm_ioctl)` |  |
| `DM_LIST_VERSIONS` | `_IOWR(DM_IOCTL, DM_LIST_VERSIONS_CMD, struct dm_ioctl)` |  |

### DM_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_MAX_TYPE_NAME` | `16` |  |

### DM_MPATH (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_MPATH_PROBE_PATHS` | `_IO(DM_IOCTL, DM_MPATH_PROBE_PATHS_CMD)` |  |

### DM_NAME (3)

| Name | Value | Comment |
|------|-------|---------|
| `DM_NAME_LEN` | `128` |  |
| `DM_NAME_LIST_FLAG_HAS_UUID` | `1` |  |
| `DM_NAME_LIST_FLAG_DOESNT_HAVE_UUID` | `2` |  |

### DM_NOFLUSH (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_NOFLUSH_FLAG` | `(1 << 11)` | In |

### DM_PERSISTENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_PERSISTENT_DEV_FLAG` | `(1 << 3)` | In |

### DM_QUERY (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_QUERY_INACTIVE_TABLE_FLAG` | `(1 << 12)` | In |

### DM_READONLY (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_READONLY_FLAG` | `(1 << 0)` | In/Out |

### DM_REMOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_REMOVE_ALL` | `_IOWR(DM_IOCTL, DM_REMOVE_ALL_CMD, struct dm_ioctl)` |  |

### DM_SECURE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_SECURE_DATA_FLAG` | `(1 << 15)` | In |

### DM_SKIP (2)

| Name | Value | Comment |
|------|-------|---------|
| `DM_SKIP_BDGET_FLAG` | `(1 << 9)` | In |
| `DM_SKIP_LOCKFS_FLAG` | `(1 << 10)` | In |

### DM_STATUS (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_STATUS_TABLE_FLAG` | `(1 << 4)` | In |

### DM_SUSPEND (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_SUSPEND_FLAG` | `(1 << 1)` | In/Out |

### DM_TABLE (4)

| Name | Value | Comment |
|------|-------|---------|
| `DM_TABLE_LOAD` | `_IOWR(DM_IOCTL, DM_TABLE_LOAD_CMD, struct dm_ioctl)` |  |
| `DM_TABLE_CLEAR` | `_IOWR(DM_IOCTL, DM_TABLE_CLEAR_CMD, struct dm_ioctl)` |  |
| `DM_TABLE_DEPS` | `_IOWR(DM_IOCTL, DM_TABLE_DEPS_CMD, struct dm_ioctl)` |  |
| `DM_TABLE_STATUS` | `_IOWR(DM_IOCTL, DM_TABLE_STATUS_CMD, struct dm_ioctl)` |  |

### DM_TARGET (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_TARGET_MSG` | `_IOWR(DM_IOCTL, DM_TARGET_MSG_CMD, struct dm_ioctl)` |  |

### DM_UEVENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `DM_UEVENT_GENERATED_FLAG` | `(1 << 13)` | Out |

### DM_UUID (2)

| Name | Value | Comment |
|------|-------|---------|
| `DM_UUID_LEN` | `129` |  |
| `DM_UUID_FLAG` | `(1 << 14)` | In |

### DM_VERSION (4)

| Name | Value | Comment |
|------|-------|---------|
| `DM_VERSION_MAJOR` | `4` |  |
| `DM_VERSION_MINOR` | `50` |  |
| `DM_VERSION_PATCHLEVEL` | `0` |  |
| `DM_VERSION_EXTRA` | `"-ioctl (2025-04-28)"` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `DM_DIR` | `"mapper"` | Slashes not supported |
| `DM_IOCTL` | `0xfd` |  |
| `DM_VERSION` | `_IOWR(DM_IOCTL, DM_VERSION_CMD, struct dm_ioctl)` |  |

## Structs (6)


### `struct dm_ioctl`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `3` |
| `__u32` | `data_size` | `-` |
| `__u32` | `data_start` | `-` |
| `__u32` | `target_count` | `-` |
| `__s32` | `open_count` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `event_nr` | `-` |
| `__u32` | `padding` | `-` |
| `__u64` | `dev` | `-` |
| `char` | `name` | `DM_NAME_LEN` |
| `char` | `uuid` | `DM_UUID_LEN` |
| `char` | `data` | `7` |

### `struct dm_target_spec`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `sector_start` | `-` |
| `__u64` | `length` | `-` |
| `__s32` | `status` | `-` |
| `__u32` | `next` | `-` |
| `char` | `target_type` | `DM_MAX_TYPE_NAME` |

### `struct dm_target_deps`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |
| `__u32` | `padding` | `-` |

### `struct dm_name_list`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dev` | `-` |
| `__u32` | `next` | `-` |

### `struct dm_target_versions`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `next` | `-` |
| `__u32` | `version` | `3` |

### `struct dm_target_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `sector` | `-` |