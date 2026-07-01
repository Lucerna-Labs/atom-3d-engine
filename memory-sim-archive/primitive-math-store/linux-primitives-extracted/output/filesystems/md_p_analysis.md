# md_p.h

**Source:** `md_p.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`

## Defines (59 total)


### MD_DISK (13)

| Name | Value | Comment |
|------|-------|---------|
| `MD_DISK_FAULTY` | `0` | disk is faulty / operational |
| `MD_DISK_ACTIVE` | `1` | disk is running or spare disk |
| `MD_DISK_SYNC` | `2` | disk is in sync with the raid set |
| `MD_DISK_REMOVED` | `3` | disk is in sync with the raid set |
| `MD_DISK_CLUSTER_ADD` | `4 /* Initiate a disk add across the cluster` |  |
| `MD_DISK_CANDIDATE` | `5 /* disk is added as spare (local) until confirmed` |  |
| `MD_DISK_FAILFAST` | `10 /* Send REQ_FAILFAST if there are multiple` |  |
| `MD_DISK_WRITEMOSTLY` | `9 /* disk is "write-mostly" is RAID1 config.` |  |
| `MD_DISK_JOURNAL` | `18` | disk is used as the write journal in RAID-5/6 |
| `MD_DISK_ROLE_SPARE` | `0xffff` |  |
| `MD_DISK_ROLE_FAULTY` | `0xfffe` |  |
| `MD_DISK_ROLE_JOURNAL` | `0xfffd` |  |
| `MD_DISK_ROLE_MAX` | `0xff00` | max value of regular disk role |

### MD_FEATURE (14)

| Name | Value | Comment |
|------|-------|---------|
| `MD_FEATURE_BITMAP_OFFSET` | `1` |  |
| `MD_FEATURE_RECOVERY_OFFSET` | `2 /* recovery_offset is present and` |  |
| `MD_FEATURE_RESHAPE_ACTIVE` | `4` |  |
| `MD_FEATURE_BAD_BLOCKS` | `8` | badblock list is not empty |
| `MD_FEATURE_REPLACEMENT` | `16 /* This device is replacing an` |  |
| `MD_FEATURE_RESHAPE_BACKWARDS` | `32 /* Reshape doesn't change number` |  |
| `MD_FEATURE_NEW_OFFSET` | `64` | new_offset must be honoured |
| `MD_FEATURE_RECOVERY_BITMAP` | `128 /* recovery that is happening` |  |
| `MD_FEATURE_CLUSTERED` | `256` | clustered MD |
| `MD_FEATURE_JOURNAL` | `512` | support write cache |
| `MD_FEATURE_PPL` | `1024` | support PPL |
| `MD_FEATURE_MULTIPLE_PPLS` | `2048` | support for multiple PPLs |
| `MD_FEATURE_RAID0_LAYOUT` | `4096` | layout is meaningful for RAID0 |
| `MD_FEATURE_ALL` | `(MD_FEATURE_BITMAP_OFFSET	` |  |

### MD_RESERVED (2)

| Name | Value | Comment |
|------|-------|---------|
| `MD_RESERVED_BYTES` | `(64 * 1024)` |  |
| `MD_RESERVED_SECTORS` | `(MD_RESERVED_BYTES / 512)` |  |

### MD_SB (21)

| Name | Value | Comment |
|------|-------|---------|
| `MD_SB_BYTES` | `4096` |  |
| `MD_SB_WORDS` | `(MD_SB_BYTES / 4)` |  |
| `MD_SB_SECTORS` | `(MD_SB_BYTES / 512)` |  |
| `MD_SB_GENERIC_OFFSET` | `0` |  |
| `MD_SB_PERSONALITY_OFFSET` | `64` |  |
| `MD_SB_DISKS_OFFSET` | `128` |  |
| `MD_SB_DESCRIPTOR_OFFSET` | `992` |  |
| `MD_SB_GENERIC_CONSTANT_WORDS` | `32` |  |
| `MD_SB_GENERIC_STATE_WORDS` | `32` |  |
| `MD_SB_GENERIC_WORDS` | `(MD_SB_GENERIC_CONSTANT_WORDS + MD_SB_GENERIC_STATE_WORDS)` |  |
| `MD_SB_PERSONALITY_WORDS` | `64` |  |
| `MD_SB_DESCRIPTOR_WORDS` | `32` |  |
| `MD_SB_DISKS` | `27` |  |
| `MD_SB_DISKS_WORDS` | `(MD_SB_DISKS*MD_SB_DESCRIPTOR_WORDS)` |  |
| `MD_SB_RESERVED_WORDS` | `(1024 - MD_SB_GENERIC_WORDS - MD_SB_PERSONALITY_WORDS - MD_S` |  |
| `MD_SB_EQUAL_WORDS` | `(MD_SB_GENERIC_WORDS + MD_SB_PERSONALITY_WORDS + MD_SB_DISKS` |  |
| `MD_SB_MAGIC` | `0xa92b4efc` |  |
| `MD_SB_CLEAN` | `0` |  |
| `MD_SB_ERRORS` | `1` |  |
| `MD_SB_CLUSTERED` | `5` | MD is clustered |
| `MD_SB_BITMAP_PRESENT` | `8` | bitmap may be present nearby |

### MD_SUPERBLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `MD_SUPERBLOCK_1_TIME_SEC_MASK` | `((1ULL<<40) - 1)` |  |

### PPL_HDR (3)

| Name | Value | Comment |
|------|-------|---------|
| `PPL_HDR_RESERVED` | `512` |  |
| `PPL_HDR_ENTRY_SPACE` | `` |  |
| `PPL_HDR_MAX_ENTRIES` | `` |  |

### PPL_HEADER (1)

| Name | Value | Comment |
|------|-------|---------|
| `PPL_HEADER_SIZE` | `4096` |  |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `WriteMostly1` | `1` | mask for writemostly flag in above |
| `FailFast1` | `2` | Should avoid retries and fixups and just fail |
| `R5LOG_VERSION` | `0x1` |  |
| `R5LOG_MAGIC` | `0x6433c509` |  |

## Structs (10)


### `struct mdp_device_descriptor_s`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `number` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u32` | `raid_disk` | `-` |
| `__u32` | `state` | `-` |
| `__u32` | `reserved` | `MD_SB_DESCRIPTOR_WORDS - 5` |

### `struct mdp_superblock_s`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `md_magic` | `-` |
| `__u32` | `major_version` | `-` |
| `__u32` | `minor_version` | `-` |
| `__u32` | `patch_version` | `-` |
| `__u32` | `gvalid_words` | `-` |
| `__u32` | `set_uuid0` | `-` |
| `__u32` | `ctime` | `-` |
| `__u32` | `level` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `nr_disks` | `-` |
| `__u32` | `raid_disks` | `-` |
| `__u32` | `md_minor` | `-` |
| `__u32` | `not_persistent` | `-` |
| `__u32` | `set_uuid1` | `-` |
| `__u32` | `set_uuid2` | `-` |
| `__u32` | `set_uuid3` | `-` |
| `__u32` | `gstate_creserved` | `MD_SB_GENERIC_CONSTANT_WORDS - 16` |
| `__u32` | `utime` | `-` |
| `__u32` | `state` | `-` |
| `__u32` | `active_disks` | `-` |
| `__u32` | `working_disks` | `-` |
| `__u32` | `failed_disks` | `-` |
| `__u32` | `spare_disks` | `-` |
| `__u32` | `sb_csum` | `-` |
| `__u32` | `events_hi` | `-` |
| `__u32` | `events_lo` | `-` |
| `__u32` | `cp_events_hi` | `-` |
| `__u32` | `cp_events_lo` | `-` |
| `__u32` | `events_lo` | `-` |
| `__u32` | `events_hi` | `-` |
| `__u32` | `cp_events_lo` | `-` |
| `__u32` | `cp_events_hi` | `-` |
| `__u32` | `recovery_cp` | `-` |
| `__u64` | `reshape_position` | `-` |
| `__u32` | `new_level` | `-` |
| `__u32` | `delta_disks` | `-` |
| `__u32` | `new_layout` | `-` |
| `__u32` | `new_chunk` | `-` |
| `__u32` | `gstate_sreserved` | `MD_SB_GENERIC_STATE_WORDS - 18` |
| `__u32` | `layout` | `-` |
| `__u32` | `chunk_size` | `-` |
| `__u32` | `root_pv` | `-` |
| `__u32` | `root_block` | `-` |
| `__u32` | `pstate_reserved` | `MD_SB_PERSONALITY_WORDS - 4` |
| `mdp_disk_t` | `disks` | `MD_SB_DISKS` |
| `__u32` | `reserved` | `MD_SB_RESERVED_WORDS` |
| `mdp_disk_t` | `this_disk` | `-` |

### `struct mdp_superblock_1`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `magic` | `-` |
| `__le32` | `major_version` | `-` |
| `__le32` | `feature_map` | `-` |
| `__le32` | `pad0` | `-` |
| `__u8` | `set_uuid` | `16` |
| `char` | `set_name` | `32` |
| `__le64` | `ctime` | `-` |
| `__le32` | `level` | `-` |
| `__le32` | `layout` | `-` |
| `__le64` | `size` | `-` |
| `__le32` | `chunksize` | `-` |
| `__le32` | `raid_disks` | `-` |
| `__le32` | `bitmap_offset` | `-` |
| `__le16` | `offset` | `-` |
| `__le16` | `size` | `-` |
| `__le32` | `new_level` | `-` |
| `__le64` | `reshape_position` | `-` |
| `__le32` | `delta_disks` | `-` |
| `__le32` | `new_layout` | `-` |
| `__le32` | `new_chunk` | `-` |
| `__le32` | `new_offset` | `-` |
| `__le64` | `data_offset` | `-` |
| `__le64` | `data_size` | `-` |
| `__le64` | `super_offset` | `-` |
| `__le64` | `recovery_offset` | `-` |
| `__le64` | `journal_tail` | `-` |
| `__le32` | `dev_number` | `-` |
| `__le32` | `cnt_corrected_read` | `-` |
| `__u8` | `device_uuid` | `16` |
| `__u8` | `devflags` | `-` |
| `__u8` | `bblog_shift` | `-` |
| `__le16` | `bblog_size` | `-` |
| `__le32` | `bblog_offset` | `-` |
| `__le64` | `utime` | `-` |
| `__le64` | `events` | `-` |
| `__le64` | `resync_offset` | `-` |
| `__le32` | `sb_csum` | `-` |
| `__le32` | `max_dev` | `-` |
| `__le32` | `logical_block_size` | `-` |
| `__u8` | `pad3` | `64-36` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `offset` | `-` |
| `__le16` | `size` | `-` |

### `struct r5l_payload_header`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `type` | `-` |
| `__le16` | `flags` | `-` |

### `struct r5l_payload_data_parity`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le64` | `location` | `-` |

### `struct r5l_payload_flush`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |

### `struct r5l_meta_block`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `magic` | `-` |
| `__le32` | `checksum` | `-` |
| `__u8` | `version` | `-` |
| `__u8` | `__zero_pading_1` | `-` |
| `__le16` | `__zero_pading_2` | `-` |
| `__le32` | `meta_size` | `-` |
| `__le64` | `seq` | `-` |
| `__le64` | `position` | `-` |

### `struct ppl_header_entry`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `data_sector` | `-` |
| `__le32` | `pp_size` | `-` |
| `__le32` | `data_size` | `-` |
| `__le32` | `parity_disk` | `-` |
| `__le32` | `checksum` | `-` |

### `struct ppl_header`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `reserved` | `PPL_HDR_RESERVED` |
| `__le32` | `signature` | `-` |
| `__le32` | `padding` | `-` |
| `__le64` | `generation` | `-` |
| `__le32` | `entries_count` | `-` |
| `__le32` | `checksum` | `-` |

## Typedefs

- `mdp_device_descriptor_s`
- `mdp_superblock_s`