# btrfs.h

**Source:** `btrfs.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`
- `linux/fs.h`

## Defines (195 total)


### BTRFS_BALANCE (23)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_BALANCE_CTL_PAUSE` | `1` |  |
| `BTRFS_BALANCE_CTL_CANCEL` | `2` |  |
| `BTRFS_BALANCE_DATA` | `(1ULL << 0)` |  |
| `BTRFS_BALANCE_SYSTEM` | `(1ULL << 1)` |  |
| `BTRFS_BALANCE_METADATA` | `(1ULL << 2)` |  |
| `BTRFS_BALANCE_TYPE_MASK` | `(BTRFS_BALANCE_DATA \|	    ` |  |
| `BTRFS_BALANCE_FORCE` | `(1ULL << 3)` |  |
| `BTRFS_BALANCE_RESUME` | `(1ULL << 4)` |  |
| `BTRFS_BALANCE_ARGS_PROFILES` | `(1ULL << 0)` |  |
| `BTRFS_BALANCE_ARGS_USAGE` | `(1ULL << 1)` |  |
| `BTRFS_BALANCE_ARGS_DEVID` | `(1ULL << 2)` |  |
| `BTRFS_BALANCE_ARGS_DRANGE` | `(1ULL << 3)` |  |
| `BTRFS_BALANCE_ARGS_VRANGE` | `(1ULL << 4)` |  |
| `BTRFS_BALANCE_ARGS_LIMIT` | `(1ULL << 5)` |  |
| `BTRFS_BALANCE_ARGS_LIMIT_RANGE` | `(1ULL << 6)` |  |
| `BTRFS_BALANCE_ARGS_STRIPES_RANGE` | `(1ULL << 7)` |  |
| `BTRFS_BALANCE_ARGS_USAGE_RANGE` | `(1ULL << 10)` |  |
| `BTRFS_BALANCE_ARGS_MASK` | `` |  |
| `BTRFS_BALANCE_ARGS_CONVERT` | `(1ULL << 8)` |  |
| `BTRFS_BALANCE_ARGS_SOFT` | `(1ULL << 9)` |  |
| `BTRFS_BALANCE_STATE_RUNNING` | `(1ULL << 0)` |  |
| `BTRFS_BALANCE_STATE_PAUSE_REQ` | `(1ULL << 1)` |  |
| `BTRFS_BALANCE_STATE_CANCEL_REQ` | `(1ULL << 2)` |  |

### BTRFS_DEFRAG (5)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_DEFRAG_RANGE_COMPRESS` | `1` |  |
| `BTRFS_DEFRAG_RANGE_START_IO` | `2` |  |
| `BTRFS_DEFRAG_RANGE_COMPRESS_LEVEL` | `4` |  |
| `BTRFS_DEFRAG_RANGE_NOCOMPRESS` | `8` |  |
| `BTRFS_DEFRAG_RANGE_FLAGS_SUPP` | `(BTRFS_DEFRAG_RANGE_COMPRESS \|		` |  |

### BTRFS_DEV (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_DEV_STATS_RESET` | `(1ULL << 0)` |  |

### BTRFS_DEVICE (3)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_DEVICE_PATH_NAME_MAX` | `1024` |  |
| `BTRFS_DEVICE_SPEC_BY_ID` | `(1ULL << 3)` |  |
| `BTRFS_DEVICE_REMOVE_ARGS_MASK` | `` |  |

### BTRFS_ENCODED (11)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_ENCODED_IO_COMPRESSION_NONE` | `0` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_ZLIB` | `1` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_ZSTD` | `2` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_LZO_4K` | `3` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_LZO_8K` | `4` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_LZO_16K` | `5` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_LZO_32K` | `6` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_LZO_64K` | `7` |  |
| `BTRFS_ENCODED_IO_COMPRESSION_TYPES` | `8` |  |
| `BTRFS_ENCODED_IO_ENCRYPTION_NONE` | `0` |  |
| `BTRFS_ENCODED_IO_ENCRYPTION_TYPES` | `1` |  |

### BTRFS_FEATURE (21)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FEATURE_COMPAT_RO_FREE_SPACE_TREE` | `(1ULL << 0)` |  |
| `BTRFS_FEATURE_COMPAT_RO_FREE_SPACE_TREE_VALID` | `(1ULL << 1)` |  |
| `BTRFS_FEATURE_COMPAT_RO_VERITY` | `(1ULL << 2)` |  |
| `BTRFS_FEATURE_COMPAT_RO_BLOCK_GROUP_TREE` | `(1ULL << 3)` |  |
| `BTRFS_FEATURE_INCOMPAT_MIXED_BACKREF` | `(1ULL << 0)` |  |
| `BTRFS_FEATURE_INCOMPAT_DEFAULT_SUBVOL` | `(1ULL << 1)` |  |
| `BTRFS_FEATURE_INCOMPAT_MIXED_GROUPS` | `(1ULL << 2)` |  |
| `BTRFS_FEATURE_INCOMPAT_COMPRESS_LZO` | `(1ULL << 3)` |  |
| `BTRFS_FEATURE_INCOMPAT_COMPRESS_ZSTD` | `(1ULL << 4)` |  |
| `BTRFS_FEATURE_INCOMPAT_BIG_METADATA` | `(1ULL << 5)` |  |
| `BTRFS_FEATURE_INCOMPAT_EXTENDED_IREF` | `(1ULL << 6)` |  |
| `BTRFS_FEATURE_INCOMPAT_RAID56` | `(1ULL << 7)` |  |
| `BTRFS_FEATURE_INCOMPAT_SKINNY_METADATA` | `(1ULL << 8)` |  |
| `BTRFS_FEATURE_INCOMPAT_NO_HOLES` | `(1ULL << 9)` |  |
| `BTRFS_FEATURE_INCOMPAT_METADATA_UUID` | `(1ULL << 10)` |  |
| `BTRFS_FEATURE_INCOMPAT_RAID1C34` | `(1ULL << 11)` |  |
| `BTRFS_FEATURE_INCOMPAT_ZONED` | `(1ULL << 12)` |  |
| `BTRFS_FEATURE_INCOMPAT_EXTENT_TREE_V2` | `(1ULL << 13)` |  |
| `BTRFS_FEATURE_INCOMPAT_RAID_STRIPE_TREE` | `(1ULL << 14)` |  |
| `BTRFS_FEATURE_INCOMPAT_SIMPLE_QUOTA` | `(1ULL << 16)` |  |
| `BTRFS_FEATURE_INCOMPAT_REMAP_TREE` | `(1ULL << 17)` |  |

### BTRFS_FS (3)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FS_INFO_FLAG_CSUM_INFO` | `(1 << 0)` |  |
| `BTRFS_FS_INFO_FLAG_GENERATION` | `(1 << 1)` |  |
| `BTRFS_FS_INFO_FLAG_METADATA_UUID` | `(1 << 2)` |  |

### BTRFS_FSID (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FSID_SIZE` | `16` |  |

### BTRFS_INO (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_INO_LOOKUP_PATH_MAX` | `4080` |  |
| `BTRFS_INO_LOOKUP_USER_PATH_MAX` | `(4080 - BTRFS_VOL_NAME_MAX - 1)` |  |

### BTRFS_IOC (65)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_IOC_SNAP_CREATE` | `_IOW(BTRFS_IOCTL_MAGIC, 1, ` |  |
| `BTRFS_IOC_DEFRAG` | `_IOW(BTRFS_IOCTL_MAGIC, 2, ` |  |
| `BTRFS_IOC_RESIZE` | `_IOW(BTRFS_IOCTL_MAGIC, 3, ` |  |
| `BTRFS_IOC_SCAN_DEV` | `_IOW(BTRFS_IOCTL_MAGIC, 4, ` |  |
| `BTRFS_IOC_FORGET_DEV` | `_IOW(BTRFS_IOCTL_MAGIC, 5, ` |  |
| `BTRFS_IOC_TRANS_START` | `_IO(BTRFS_IOCTL_MAGIC, 6)` |  |
| `BTRFS_IOC_TRANS_END` | `_IO(BTRFS_IOCTL_MAGIC, 7)` |  |
| `BTRFS_IOC_SYNC` | `_IO(BTRFS_IOCTL_MAGIC, 8)` |  |
| `BTRFS_IOC_CLONE` | `_IOW(BTRFS_IOCTL_MAGIC, 9, int)` |  |
| `BTRFS_IOC_ADD_DEV` | `_IOW(BTRFS_IOCTL_MAGIC, 10, ` |  |
| `BTRFS_IOC_RM_DEV` | `_IOW(BTRFS_IOCTL_MAGIC, 11, ` |  |
| `BTRFS_IOC_BALANCE` | `_IOW(BTRFS_IOCTL_MAGIC, 12, ` |  |
| `BTRFS_IOC_CLONE_RANGE` | `_IOW(BTRFS_IOCTL_MAGIC, 13, ` |  |
| `BTRFS_IOC_SUBVOL_CREATE` | `_IOW(BTRFS_IOCTL_MAGIC, 14, ` |  |
| `BTRFS_IOC_SNAP_DESTROY` | `_IOW(BTRFS_IOCTL_MAGIC, 15, ` |  |
| `BTRFS_IOC_DEFRAG_RANGE` | `_IOW(BTRFS_IOCTL_MAGIC, 16, ` |  |
| `BTRFS_IOC_TREE_SEARCH` | `_IOWR(BTRFS_IOCTL_MAGIC, 17, ` |  |
| `BTRFS_IOC_TREE_SEARCH_V2` | `_IOWR(BTRFS_IOCTL_MAGIC, 17, ` |  |
| `BTRFS_IOC_INO_LOOKUP` | `_IOWR(BTRFS_IOCTL_MAGIC, 18, ` |  |
| `BTRFS_IOC_DEFAULT_SUBVOL` | `_IOW(BTRFS_IOCTL_MAGIC, 19, __u64)` |  |
| `BTRFS_IOC_SPACE_INFO` | `_IOWR(BTRFS_IOCTL_MAGIC, 20, ` |  |
| `BTRFS_IOC_START_SYNC` | `_IOR(BTRFS_IOCTL_MAGIC, 24, __u64)` |  |
| `BTRFS_IOC_WAIT_SYNC` | `_IOW(BTRFS_IOCTL_MAGIC, 22, __u64)` |  |
| `BTRFS_IOC_SNAP_CREATE_V2` | `_IOW(BTRFS_IOCTL_MAGIC, 23, ` |  |
| `BTRFS_IOC_SUBVOL_CREATE_V2` | `_IOW(BTRFS_IOCTL_MAGIC, 24, ` |  |
| `BTRFS_IOC_SUBVOL_GETFLAGS` | `_IOR(BTRFS_IOCTL_MAGIC, 25, __u64)` |  |
| `BTRFS_IOC_SUBVOL_SETFLAGS` | `_IOW(BTRFS_IOCTL_MAGIC, 26, __u64)` |  |
| `BTRFS_IOC_SCRUB` | `_IOWR(BTRFS_IOCTL_MAGIC, 27, ` |  |
| `BTRFS_IOC_SCRUB_CANCEL` | `_IO(BTRFS_IOCTL_MAGIC, 28)` |  |
| `BTRFS_IOC_SCRUB_PROGRESS` | `_IOWR(BTRFS_IOCTL_MAGIC, 29, ` |  |
| `BTRFS_IOC_DEV_INFO` | `_IOWR(BTRFS_IOCTL_MAGIC, 30, ` |  |
| `BTRFS_IOC_FS_INFO` | `_IOR(BTRFS_IOCTL_MAGIC, 31, ` |  |
| `BTRFS_IOC_BALANCE_V2` | `_IOWR(BTRFS_IOCTL_MAGIC, 32, ` |  |
| `BTRFS_IOC_BALANCE_CTL` | `_IOW(BTRFS_IOCTL_MAGIC, 33, int)` |  |
| `BTRFS_IOC_BALANCE_PROGRESS` | `_IOR(BTRFS_IOCTL_MAGIC, 34, ` |  |
| `BTRFS_IOC_INO_PATHS` | `_IOWR(BTRFS_IOCTL_MAGIC, 35, ` |  |
| `BTRFS_IOC_LOGICAL_INO` | `_IOWR(BTRFS_IOCTL_MAGIC, 36, ` |  |
| `BTRFS_IOC_SET_RECEIVED_SUBVOL` | `_IOWR(BTRFS_IOCTL_MAGIC, 37, ` |  |
| `BTRFS_IOC_SEND` | `_IOW(BTRFS_IOCTL_MAGIC, 38, struct btrfs_ioctl_send_args)` |  |
| `BTRFS_IOC_DEVICES_READY` | `_IOR(BTRFS_IOCTL_MAGIC, 39, ` |  |
| `BTRFS_IOC_QUOTA_CTL` | `_IOWR(BTRFS_IOCTL_MAGIC, 40, ` |  |
| `BTRFS_IOC_QGROUP_ASSIGN` | `_IOW(BTRFS_IOCTL_MAGIC, 41, ` |  |
| `BTRFS_IOC_QGROUP_CREATE` | `_IOW(BTRFS_IOCTL_MAGIC, 42, ` |  |
| `BTRFS_IOC_QGROUP_LIMIT` | `_IOR(BTRFS_IOCTL_MAGIC, 43, ` |  |
| `BTRFS_IOC_QUOTA_RESCAN` | `_IOW(BTRFS_IOCTL_MAGIC, 44, ` |  |
| `BTRFS_IOC_QUOTA_RESCAN_STATUS` | `_IOR(BTRFS_IOCTL_MAGIC, 45, ` |  |
| `BTRFS_IOC_QUOTA_RESCAN_WAIT` | `_IO(BTRFS_IOCTL_MAGIC, 46)` |  |
| `BTRFS_IOC_GET_FSLABEL` | `FS_IOC_GETFSLABEL` |  |
| `BTRFS_IOC_SET_FSLABEL` | `FS_IOC_SETFSLABEL` |  |
| `BTRFS_IOC_GET_DEV_STATS` | `_IOWR(BTRFS_IOCTL_MAGIC, 52, ` |  |

*...and 15 more*

### BTRFS_IOCTL (15)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_IOCTL_MAGIC` | `0x94` |  |
| `BTRFS_IOCTL_DEV_REPLACE_CONT_READING_FROM_SRCDEV_MODE_ALWAYS` | `0` |  |
| `BTRFS_IOCTL_DEV_REPLACE_CONT_READING_FROM_SRCDEV_MODE_AVOID` | `1` |  |
| `BTRFS_IOCTL_DEV_REPLACE_STATE_NEVER_STARTED` | `0` |  |
| `BTRFS_IOCTL_DEV_REPLACE_STATE_STARTED` | `1` |  |
| `BTRFS_IOCTL_DEV_REPLACE_STATE_FINISHED` | `2` |  |
| `BTRFS_IOCTL_DEV_REPLACE_STATE_CANCELED` | `3` |  |
| `BTRFS_IOCTL_DEV_REPLACE_STATE_SUSPENDED` | `4` |  |
| `BTRFS_IOCTL_DEV_REPLACE_CMD_START` | `0` |  |
| `BTRFS_IOCTL_DEV_REPLACE_CMD_STATUS` | `1` |  |
| `BTRFS_IOCTL_DEV_REPLACE_CMD_CANCEL` | `2` |  |
| `BTRFS_IOCTL_DEV_REPLACE_RESULT_NO_ERROR` | `0` |  |
| `BTRFS_IOCTL_DEV_REPLACE_RESULT_NOT_STARTED` | `1` |  |
| `BTRFS_IOCTL_DEV_REPLACE_RESULT_ALREADY_STARTED` | `2` |  |
| `BTRFS_IOCTL_DEV_REPLACE_RESULT_SCRUB_INPROGRESS` | `3` |  |

### BTRFS_LABEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_LABEL_SIZE` | `256` |  |

### BTRFS_LOGICAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_LOGICAL_INO_ARGS_IGNORE_OFFSET` | `(1ULL << 0)` |  |

### BTRFS_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_MAX_ROOTREF_BUFFER_NUM` | `255` |  |

### BTRFS_PATH (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_PATH_NAME_MAX` | `4087` |  |

### BTRFS_QGROUP (8)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_QGROUP_LIMIT_MAX_RFER` | `(1ULL << 0)` |  |
| `BTRFS_QGROUP_LIMIT_MAX_EXCL` | `(1ULL << 1)` |  |
| `BTRFS_QGROUP_LIMIT_RSV_RFER` | `(1ULL << 2)` |  |
| `BTRFS_QGROUP_LIMIT_RSV_EXCL` | `(1ULL << 3)` |  |
| `BTRFS_QGROUP_LIMIT_RFER_CMPR` | `(1ULL << 4)` |  |
| `BTRFS_QGROUP_LIMIT_EXCL_CMPR` | `(1ULL << 5)` |  |
| `BTRFS_QGROUP_INHERIT_SET_LIMITS` | `(1ULL << 0)` |  |
| `BTRFS_QGROUP_INHERIT_FLAGS_SUPP` | `(BTRFS_QGROUP_INHERIT_SET_LIMITS)` |  |

### BTRFS_QUOTA (4)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_QUOTA_CTL_ENABLE` | `1` |  |
| `BTRFS_QUOTA_CTL_DISABLE` | `2` |  |
| `BTRFS_QUOTA_CTL_RESCAN__NOTUSED` | `3` |  |
| `BTRFS_QUOTA_CTL_ENABLE_SIMPLE_QUOTA` | `4` |  |

### BTRFS_SAME (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SAME_DATA_DIFFERS` | `1` |  |

### BTRFS_SCRUB (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SCRUB_READONLY` | `1` |  |
| `BTRFS_SCRUB_SUPPORTED_FLAGS` | `(BTRFS_SCRUB_READONLY)` |  |

### BTRFS_SEARCH (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SEARCH_ARGS_BUFSIZE` | `(4096 - sizeof(struct btrfs_ioctl_search_key))` |  |

### BTRFS_SEND (6)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SEND_FLAG_NO_FILE_DATA` | `0x1` |  |
| `BTRFS_SEND_FLAG_OMIT_STREAM_HEADER` | `0x2` |  |
| `BTRFS_SEND_FLAG_OMIT_END_CMD` | `0x4` |  |
| `BTRFS_SEND_FLAG_VERSION` | `0x8` |  |
| `BTRFS_SEND_FLAG_COMPRESSED` | `0x10` |  |
| `BTRFS_SEND_FLAG_MASK` | `` |  |

### BTRFS_SHUTDOWN (4)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SHUTDOWN_FLAGS_DEFAULT` | `0x0` |  |
| `BTRFS_SHUTDOWN_FLAGS_LOGFLUSH` | `0x1` |  |
| `BTRFS_SHUTDOWN_FLAGS_NOLOGFLUSH` | `0x2` |  |
| `BTRFS_SHUTDOWN_FLAGS_LAST` | `0x3` |  |

### BTRFS_SUBVOL (11)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SUBVOL_NAME_MAX` | `4039` |  |
| `BTRFS_SUBVOL_RDONLY` | `(1ULL << 1)` |  |
| `BTRFS_SUBVOL_QGROUP_INHERIT` | `(1ULL << 2)` |  |
| `BTRFS_SUBVOL_SPEC_BY_ID` | `(1ULL << 4)` |  |
| `BTRFS_SUBVOL_CREATE_ARGS_MASK` | `` |  |
| `BTRFS_SUBVOL_DELETE_ARGS_MASK` | `` |  |
| `BTRFS_SUBVOL_SYNC_WAIT_FOR_ONE` | `(0)` |  |
| `BTRFS_SUBVOL_SYNC_WAIT_FOR_QUEUED` | `(1)` |  |
| `BTRFS_SUBVOL_SYNC_COUNT` | `(2)` |  |
| `BTRFS_SUBVOL_SYNC_PEEK_FIRST` | `(3)` |  |
| `BTRFS_SUBVOL_SYNC_PEEK_LAST` | `(4)` |  |

### BTRFS_UUID (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_UUID_SIZE` | `16` |  |
| `BTRFS_UUID_UNPARSED_SIZE` | `37` |  |

### BTRFS_VOL (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_VOL_NAME_MAX` | `255` |  |
| `BTRFS_VOL_ARG_V2_FLAGS_SUPPORTED` | `` |  |

## Structs (48)


### `struct btrfs_ioctl_vol_args`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `fd` | `-` |
| `char` | `name` | `BTRFS_PATH_NAME_MAX + 1` |

### `struct btrfs_qgroup_limit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `max_rfer` | `-` |
| `__u64` | `max_excl` | `-` |
| `__u64` | `rsv_rfer` | `-` |
| `__u64` | `rsv_excl` | `-` |

### `struct btrfs_qgroup_inherit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `num_qgroups` | `-` |
| `__u64` | `num_ref_copies` | `-` |
| `__u64` | `num_excl_copies` | `-` |

### `struct btrfs_ioctl_qgroup_limit_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `qgroupid` | `-` |

### `struct btrfs_ioctl_vol_args_v2`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `fd` | `-` |
| `__u64` | `transid` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `unused` | `4` |
| `char` | `name` | `BTRFS_SUBVOL_NAME_MAX + 1` |
| `__u64` | `devid` | `-` |
| `__u64` | `subvolid` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |

### `struct btrfs_scrub_progress`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data_extents_scrubbed` | `-` |
| `__u64` | `tree_extents_scrubbed` | `-` |
| `__u64` | `data_bytes_scrubbed` | `-` |
| `__u64` | `tree_bytes_scrubbed` | `-` |
| `__u64` | `read_errors` | `-` |
| `__u64` | `csum_errors` | `-` |
| `__u64` | `verify_errors` | `-` |
| `__u64` | `no_csum` | `-` |
| `__u64` | `csum_discards` | `-` |
| `__u64` | `super_errors` | `-` |
| `__u64` | `malloc_errors` | `-` |
| `__u64` | `uncorrectable_errors` | `-` |
| `__u64` | `corrected_errors` | `-` |
| `__u64` | `last_physical` | `-` |
| `__u64` | `unverified_errors` | `-` |

### `struct btrfs_ioctl_scrub_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `devid` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `unused` | `(1024-32-sizeof(struct btrfs_scrub_progress))/8` |

### `struct btrfs_ioctl_dev_replace_start_params`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `srcdevid` | `-` |
| `__u64` | `cont_reading_from_srcdev_mode` | `-` |
| `__u8` | `srcdev_name` | `BTRFS_DEVICE_PATH_NAME_MAX + 1` |
| `__u8` | `tgtdev_name` | `BTRFS_DEVICE_PATH_NAME_MAX + 1` |

### `struct btrfs_ioctl_dev_replace_status_params`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `replace_state` | `-` |
| `__u64` | `progress_1000` | `-` |
| `__u64` | `time_started` | `-` |
| `__u64` | `time_stopped` | `-` |
| `__u64` | `num_write_errors` | `-` |
| `__u64` | `num_uncorrectable_read_errors` | `-` |

### `struct btrfs_ioctl_dev_replace_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cmd` | `-` |
| `__u64` | `result` | `-` |
| `__u64` | `spare` | `64` |

### `struct btrfs_ioctl_dev_info_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `devid` | `-` |
| `__u8` | `uuid` | `BTRFS_UUID_SIZE` |
| `__u64` | `bytes_used` | `-` |
| `__u64` | `total_bytes` | `-` |
| `__u8` | `fsid` | `BTRFS_UUID_SIZE` |
| `__u64` | `unused` | `377` |
| `__u8` | `path` | `BTRFS_DEVICE_PATH_NAME_MAX` |

### `struct btrfs_ioctl_fs_info_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `max_id` | `-` |
| `__u64` | `num_devices` | `-` |
| `__u8` | `fsid` | `BTRFS_FSID_SIZE` |
| `__u32` | `nodesize` | `-` |
| `__u32` | `sectorsize` | `-` |
| `__u32` | `clone_alignment` | `-` |
| `__u16` | `csum_type` | `-` |
| `__u16` | `csum_size` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `generation` | `-` |
| `__u8` | `metadata_uuid` | `BTRFS_FSID_SIZE` |
| `__u8` | `reserved` | `944` |

### `struct btrfs_ioctl_feature_flags`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `compat_flags` | `-` |
| `__u64` | `compat_ro_flags` | `-` |
| `__u64` | `incompat_flags` | `-` |

### `struct btrfs_balance_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `profiles` | `-` |
| `__u64` | `usage` | `-` |
| `__u32` | `usage_min` | `-` |
| `__u32` | `usage_max` | `-` |
| `__u64` | `devid` | `-` |
| `__u64` | `pstart` | `-` |
| `__u64` | `pend` | `-` |
| `__u64` | `vstart` | `-` |
| `__u64` | `vend` | `-` |
| `__u64` | `target` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `limit` | `-` |
| `__u32` | `limit_min` | `-` |
| `__u32` | `limit_max` | `-` |
| `__u32` | `stripes_min` | `-` |
| `__u32` | `stripes_max` | `-` |
| `__u64` | `unused` | `6` |

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `usage_min` | `-` |
| `__u32` | `usage_max` | `-` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `limit_min` | `-` |
| `__u32` | `limit_max` | `-` |

### `struct btrfs_balance_progress`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `expected` | `-` |
| `__u64` | `considered` | `-` |
| `__u64` | `completed` | `-` |

### `struct btrfs_ioctl_balance_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `state` | `-` |
| `__u64` | `unused` | `72` |

### `struct btrfs_ioctl_ino_lookup_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `treeid` | `-` |
| `__u64` | `objectid` | `-` |
| `char` | `name` | `BTRFS_INO_LOOKUP_PATH_MAX` |

### `struct btrfs_ioctl_ino_lookup_user_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dirid` | `-` |
| `__u64` | `treeid` | `-` |
| `char` | `name` | `BTRFS_VOL_NAME_MAX + 1` |
| `char` | `path` | `BTRFS_INO_LOOKUP_USER_PATH_MAX` |

### `struct btrfs_ioctl_search_key`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `tree_id` | `-` |
| `__u64` | `min_objectid` | `-` |
| `__u64` | `max_objectid` | `-` |
| `__u64` | `min_offset` | `-` |
| `__u64` | `max_offset` | `-` |
| `__u64` | `min_transid` | `-` |
| `__u64` | `max_transid` | `-` |
| `__u32` | `min_type` | `-` |
| `__u32` | `max_type` | `-` |
| `__u32` | `nr_items` | `-` |
| `__u32` | `unused` | `-` |
| `__u64` | `unused1` | `-` |
| `__u64` | `unused2` | `-` |
| `__u64` | `unused3` | `-` |
| `__u64` | `unused4` | `-` |

### `struct btrfs_ioctl_search_header`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `transid` | `-` |
| `__u64` | `objectid` | `-` |
| `__u64` | `offset` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `len` | `-` |

### `struct btrfs_ioctl_search_args`

| Type | Field | Array |
|------|-------|-------|
| `char` | `buf` | `BTRFS_SEARCH_ARGS_BUFSIZE` |

### `struct btrfs_ioctl_search_args_v2`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buf_size` | `-` |

### `struct btrfs_ioctl_clone_range_args`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `src_fd` | `-` |
| `__u64` | `dest_offset` | `-` |

### `struct btrfs_ioctl_defrag_range_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `flags` | `-` |
| `__u32` | `extent_thresh` | `-` |
| `__u32` | `compress_type` | `-` |
| `__u8` | `type` | `-` |
| `__s8` | `level` | `-` |
| `__u32` | `unused` | `4` |

### `struct anonymous_27`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__s8` | `level` | `-` |

### `struct btrfs_ioctl_same_extent_info`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `fd` | `-` |
| `__u64` | `logical_offset` | `-` |
| `__u64` | `bytes_deduped` | `-` |
| `__s32` | `status` | `-` |
| `__u32` | `reserved` | `-` |

### `struct btrfs_ioctl_same_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `logical_offset` | `-` |
| `__u64` | `length` | `-` |
| `__u16` | `dest_count` | `-` |
| `__u16` | `reserved1` | `-` |
| `__u32` | `reserved2` | `-` |

### `struct btrfs_ioctl_space_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `total_bytes` | `-` |
| `__u64` | `used_bytes` | `-` |

### `struct btrfs_ioctl_space_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `space_slots` | `-` |
| `__u64` | `total_spaces` | `-` |

### `struct btrfs_data_container`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bytes_left` | `-` |
| `__u32` | `bytes_missing` | `-` |
| `__u32` | `elem_cnt` | `-` |
| `__u32` | `elem_missed` | `-` |

### `struct btrfs_ioctl_ino_path_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `inum` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `reserved` | `4` |
| `__u64` | `fspath` | `-` |

### `struct btrfs_ioctl_logical_ino_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `logical` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `reserved` | `3` |
| `__u64` | `flags` | `-` |
| `__u64` | `inodes` | `-` |

### `struct btrfs_ioctl_get_dev_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `devid` | `-` |
| `__u64` | `nr_items` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `values` | `BTRFS_DEV_STAT_VALUES_MAX` |
| `__u64` | `unused` | `128 - 2 - BTRFS_DEV_STAT_VALUES_MAX` |

### `struct btrfs_ioctl_quota_ctl_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cmd` | `-` |
| `__u64` | `status` | `-` |

### `struct btrfs_ioctl_quota_rescan_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `progress` | `-` |
| `__u64` | `reserved` | `6` |

### `struct btrfs_ioctl_qgroup_assign_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `assign` | `-` |
| `__u64` | `src` | `-` |
| `__u64` | `dst` | `-` |

### `struct btrfs_ioctl_qgroup_create_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `create` | `-` |
| `__u64` | `qgroupid` | `-` |

### `struct btrfs_ioctl_timespec`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `sec` | `-` |
| `__u32` | `nsec` | `-` |

### `struct btrfs_ioctl_received_subvol_args`

| Type | Field | Array |
|------|-------|-------|
| `char` | `uuid` | `BTRFS_UUID_SIZE` |
| `__u64` | `stransid` | `-` |
| `__u64` | `rtransid` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `reserved` | `16` |

### `struct btrfs_ioctl_send_args`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `send_fd` | `-` |
| `__u64` | `clone_sources_count` | `-` |
| `__u64` | `parent_root` | `-` |
| `__u64` | `flags` | `-` |
| `__u32` | `version` | `-` |
| `__u8` | `reserved` | `28` |

### `struct btrfs_ioctl_get_subvol_info_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `treeid` | `-` |
| `char` | `name` | `BTRFS_VOL_NAME_MAX + 1` |
| `__u64` | `parent_id` | `-` |
| `__u64` | `dirid` | `-` |
| `__u64` | `generation` | `-` |
| `__u64` | `flags` | `-` |
| `__u8` | `uuid` | `BTRFS_UUID_SIZE` |
| `__u8` | `parent_uuid` | `BTRFS_UUID_SIZE` |
| `__u8` | `received_uuid` | `BTRFS_UUID_SIZE` |
| `__u64` | `ctransid` | `-` |
| `__u64` | `otransid` | `-` |
| `__u64` | `stransid` | `-` |
| `__u64` | `rtransid` | `-` |
| `__u64` | `reserved` | `8` |

### `struct btrfs_ioctl_get_subvol_rootref_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `min_treeid` | `-` |
| `__u64` | `treeid` | `-` |
| `__u64` | `dirid` | `-` |
| `__u8` | `num_items` | `-` |
| `__u8` | `align` | `7` |

### `struct anonymous_45`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `treeid` | `-` |
| `__u64` | `dirid` | `-` |

### `struct btrfs_ioctl_encoded_io_args`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `offset` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `unencoded_len` | `-` |
| `__u64` | `unencoded_offset` | `-` |
| `__u32` | `compression` | `-` |
| `__u32` | `encryption` | `-` |
| `__u8` | `reserved` | `64` |

### `struct btrfs_ioctl_subvol_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `subvolid` | `-` |
| `__u32` | `mode` | `-` |
| `__u32` | `count` | `-` |