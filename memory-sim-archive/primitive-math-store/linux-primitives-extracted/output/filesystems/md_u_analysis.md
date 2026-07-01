# md_u.h

**Source:** `md_u.h`


## Defines (28 total)


### ADD_NEW (1)

| Name | Value | Comment |
|------|-------|---------|
| `ADD_NEW_DISK` | `_IOW (MD_MAJOR, 0x21, mdu_disk_info_t)` |  |

### CLUSTERED_DISK (1)

| Name | Value | Comment |
|------|-------|---------|
| `CLUSTERED_DISK_NACK` | `_IO (MD_MAJOR, 0x35)` |  |

### GET_ARRAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `GET_ARRAY_INFO` | `_IOR (MD_MAJOR, 0x11, mdu_array_info_t)` |  |

### GET_BITMAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `GET_BITMAP_FILE` | `_IOR (MD_MAJOR, 0x15, mdu_bitmap_file_t)` |  |

### GET_DISK (1)

| Name | Value | Comment |
|------|-------|---------|
| `GET_DISK_INFO` | `_IOR (MD_MAJOR, 0x12, mdu_disk_info_t)` |  |

### HOT_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `HOT_ADD_DISK` | `_IO (MD_MAJOR, 0x28)` |  |

### HOT_GENERATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `HOT_GENERATE_ERROR` | `_IO (MD_MAJOR, 0x2a)` |  |

### HOT_REMOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `HOT_REMOVE_DISK` | `_IO (MD_MAJOR, 0x22)` |  |

### MD_MAJOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `MD_MAJOR_VERSION` | `0` |  |

### MD_MINOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `MD_MINOR_VERSION` | `90` |  |

### MD_PATCHLEVEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `MD_PATCHLEVEL_VERSION` | `3` |  |

### RESTART_ARRAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `RESTART_ARRAY_RW` | `_IO (MD_MAJOR, 0x34)` |  |

### SET_ARRAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `SET_ARRAY_INFO` | `_IOW (MD_MAJOR, 0x23, mdu_array_info_t)` |  |

### SET_BITMAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `SET_BITMAP_FILE` | `_IOW (MD_MAJOR, 0x2b, int)` |  |

### SET_DISK (2)

| Name | Value | Comment |
|------|-------|---------|
| `SET_DISK_INFO` | `_IO (MD_MAJOR, 0x24)` |  |
| `SET_DISK_FAULTY` | `_IO (MD_MAJOR, 0x29)` |  |

### STOP_ARRAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `STOP_ARRAY_RO` | `_IO (MD_MAJOR, 0x33)` |  |

### UNCATEGORIZED (10)

| Name | Value | Comment |
|------|-------|---------|
| `RAID_VERSION` | `_IOR (MD_MAJOR, 0x10, mdu_version_t)` |  |
| `RAID_AUTORUN` | `_IO (MD_MAJOR, 0x14)` |  |
| `CLEAR_ARRAY` | `_IO (MD_MAJOR, 0x20)` |  |
| `UNPROTECT_ARRAY` | `_IO (MD_MAJOR, 0x26)` |  |
| `PROTECT_ARRAY` | `_IO (MD_MAJOR, 0x27)` |  |
| `RUN_ARRAY` | `_IOW (MD_MAJOR, 0x30, mdu_param_t)` |  |
| `STOP_ARRAY` | `_IO (MD_MAJOR, 0x32)` |  |
| `MdpMinorShift` | `6` |  |
| `LEVEL_LINEAR` | `(-1)` |  |
| `LEVEL_NONE` | `(-1000000)` |  |

### WRITE_RAID (1)

| Name | Value | Comment |
|------|-------|---------|
| `WRITE_RAID_INFO` | `_IO (MD_MAJOR, 0x25)` |  |

## Structs (6)


### `struct mdu_version_s`

| Type | Field | Array |
|------|-------|-------|
| `int` | `major` | `-` |
| `int` | `minor` | `-` |
| `int` | `patchlevel` | `-` |

### `struct mdu_array_info_s`

| Type | Field | Array |
|------|-------|-------|
| `int` | `major_version` | `-` |
| `int` | `minor_version` | `-` |
| `int` | `patch_version` | `-` |
| `int` | `level` | `-` |
| `int` | `size` | `-` |
| `int` | `nr_disks` | `-` |
| `int` | `raid_disks` | `-` |
| `int` | `md_minor` | `-` |
| `int` | `not_persistent` | `-` |
| `int` | `state` | `-` |
| `int` | `active_disks` | `-` |
| `int` | `working_disks` | `-` |
| `int` | `failed_disks` | `-` |
| `int` | `spare_disks` | `-` |
| `int` | `layout` | `-` |
| `int` | `chunk_size` | `-` |

### `struct mdu_disk_info_s`

| Type | Field | Array |
|------|-------|-------|
| `int` | `number` | `-` |
| `int` | `major` | `-` |
| `int` | `minor` | `-` |
| `int` | `raid_disk` | `-` |
| `int` | `state` | `-` |

### `struct mdu_start_info_s`

| Type | Field | Array |
|------|-------|-------|
| `int` | `major` | `-` |
| `int` | `minor` | `-` |
| `int` | `raid_disk` | `-` |
| `int` | `state` | `-` |

### `struct mdu_bitmap_file_s`

| Type | Field | Array |
|------|-------|-------|
| `char` | `pathname` | `4096` |

### `struct mdu_param_s`

| Type | Field | Array |
|------|-------|-------|
| `int` | `personality` | `-` |
| `int` | `chunk_size` | `-` |
| `int` | `max_fault` | `-` |

## Typedefs

- `mdu_version_s`
- `mdu_array_info_s`
- `mdu_disk_info_s`
- `mdu_start_info_s`
- `mdu_bitmap_file_s`
- `mdu_param_s`