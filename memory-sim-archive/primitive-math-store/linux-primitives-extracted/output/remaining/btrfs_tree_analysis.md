# btrfs_tree.h

**Source:** `btrfs_tree.h`


## Includes

- `linux/btrfs.h`
- `linux/types.h`
- `linux/stddef.h`
- `stddef.h`

## Defines (168 total)


### BTRFS_AVAIL (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_AVAIL_ALLOC_BIT_SINGLE` | `(1ULL << 48)` |  |

### BTRFS_BACKREF (3)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_BACKREF_REV_MAX` | `256` |  |
| `BTRFS_BACKREF_REV_SHIFT` | `56` |  |
| `BTRFS_BACKREF_REV_MASK` | `(((u64)BTRFS_BACKREF_REV_MAX - 1) << ` |  |

### BTRFS_BALANCE (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_BALANCE_OBJECTID` | `-4ULL` |  |
| `BTRFS_BALANCE_ITEM_KEY` | `248` |  |

### BTRFS_BLOCK (21)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_BLOCK_GROUP_TREE_OBJECTID` | `11ULL` |  |
| `BTRFS_BLOCK_GROUP_ITEM_KEY` | `192` |  |
| `BTRFS_BLOCK_FLAG_FULL_BACKREF` | `(1ULL << 8)` |  |
| `BTRFS_BLOCK_GROUP_DATA` | `(1ULL << 0)` |  |
| `BTRFS_BLOCK_GROUP_SYSTEM` | `(1ULL << 1)` |  |
| `BTRFS_BLOCK_GROUP_METADATA` | `(1ULL << 2)` |  |
| `BTRFS_BLOCK_GROUP_RAID0` | `(1ULL << 3)` |  |
| `BTRFS_BLOCK_GROUP_RAID1` | `(1ULL << 4)` |  |
| `BTRFS_BLOCK_GROUP_DUP` | `(1ULL << 5)` |  |
| `BTRFS_BLOCK_GROUP_RAID10` | `(1ULL << 6)` |  |
| `BTRFS_BLOCK_GROUP_RAID5` | `(1ULL << 7)` |  |
| `BTRFS_BLOCK_GROUP_RAID6` | `(1ULL << 8)` |  |
| `BTRFS_BLOCK_GROUP_RAID1C3` | `(1ULL << 9)` |  |
| `BTRFS_BLOCK_GROUP_RAID1C4` | `(1ULL << 10)` |  |
| `BTRFS_BLOCK_GROUP_REMAPPED` | `(1ULL << 11)` |  |
| `BTRFS_BLOCK_GROUP_METADATA_REMAP` | `(1ULL << 12)` |  |
| `BTRFS_BLOCK_GROUP_RESERVED` | `(BTRFS_AVAIL_ALLOC_BIT_SINGLE \| ` |  |
| `BTRFS_BLOCK_GROUP_TYPE_MASK` | `(BTRFS_BLOCK_GROUP_DATA \|    ` |  |
| `BTRFS_BLOCK_GROUP_PROFILE_MASK` | `(BTRFS_BLOCK_GROUP_RAID0 \|   ` |  |
| `BTRFS_BLOCK_GROUP_RAID56_MASK` | `(BTRFS_BLOCK_GROUP_RAID5 \|   ` |  |
| `BTRFS_BLOCK_GROUP_RAID1_MASK` | `(BTRFS_BLOCK_GROUP_RAID1 \|   ` |  |

### BTRFS_BTREE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_BTREE_INODE_OBJECTID` | `1` |  |

### BTRFS_CHUNK (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_CHUNK_TREE_OBJECTID` | `3ULL` |  |
| `BTRFS_CHUNK_ITEM_KEY` | `228` |  |

### BTRFS_CSUM (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_CSUM_TREE_OBJECTID` | `7ULL` |  |
| `BTRFS_CSUM_SIZE` | `32` |  |

### BTRFS_DATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_DATA_RELOC_TREE_OBJECTID` | `-9ULL` |  |

### BTRFS_DEV (10)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_DEV_TREE_OBJECTID` | `4ULL` |  |
| `BTRFS_DEV_STATS_OBJECTID` | `0ULL` |  |
| `BTRFS_DEV_ITEMS_OBJECTID` | `1ULL` |  |
| `BTRFS_DEV_REPLACE_DEVID` | `0ULL` |  |
| `BTRFS_DEV_EXTENT_KEY` | `204` |  |
| `BTRFS_DEV_ITEM_KEY` | `216` |  |
| `BTRFS_DEV_STATS_KEY` | `249` |  |
| `BTRFS_DEV_REPLACE_KEY` | `250` |  |
| `BTRFS_DEV_REPLACE_ITEM_CONT_READING_FROM_SRCDEV_MODE_ALWAYS` | `0` |  |
| `BTRFS_DEV_REPLACE_ITEM_CONT_READING_FROM_SRCDEV_MODE_AVOID` | `1` |  |

### BTRFS_DIR (4)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_DIR_LOG_ITEM_KEY` | `60` |  |
| `BTRFS_DIR_LOG_INDEX_KEY` | `72` |  |
| `BTRFS_DIR_ITEM_KEY` | `84` |  |
| `BTRFS_DIR_INDEX_KEY` | `96` |  |

### BTRFS_EMPTY (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_EMPTY_SUBVOL_DIR_OBJECTID` | `2` |  |

### BTRFS_EXTENDED (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_EXTENDED_PROFILE_MASK` | `(BTRFS_BLOCK_GROUP_PROFILE_MASK \| ` |  |

### BTRFS_EXTENT (10)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_EXTENT_TREE_OBJECTID` | `2ULL` |  |
| `BTRFS_EXTENT_CSUM_OBJECTID` | `-10ULL` |  |
| `BTRFS_EXTENT_DATA_KEY` | `108` |  |
| `BTRFS_EXTENT_CSUM_KEY` | `128` |  |
| `BTRFS_EXTENT_ITEM_KEY` | `168` |  |
| `BTRFS_EXTENT_OWNER_REF_KEY` | `172` |  |
| `BTRFS_EXTENT_DATA_REF_KEY` | `178` |  |
| `BTRFS_EXTENT_FLAG_DATA` | `(1ULL << 0)` |  |
| `BTRFS_EXTENT_FLAG_TREE_BLOCK` | `(1ULL << 1)` |  |
| `BTRFS_EXTENT_FLAG_SUPER` | `(1ULL << 48)` |  |

### BTRFS_FIRST (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FIRST_FREE_OBJECTID` | `256ULL` |  |
| `BTRFS_FIRST_CHUNK_TREE_OBJECTID` | `256ULL` |  |

### BTRFS_FREE (10)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FREE_SPACE_TREE_OBJECTID` | `10ULL` |  |
| `BTRFS_FREE_SPACE_OBJECTID` | `-11ULL` |  |
| `BTRFS_FREE_INO_OBJECTID` | `-12ULL` |  |
| `BTRFS_FREE_SPACE_INFO_KEY` | `198` |  |
| `BTRFS_FREE_SPACE_EXTENT_KEY` | `199` |  |
| `BTRFS_FREE_SPACE_BITMAP_KEY` | `200` |  |
| `BTRFS_FREE_SPACE_EXTENT` | `1` |  |
| `BTRFS_FREE_SPACE_BITMAP` | `2` |  |
| `BTRFS_FREE_SPACE_USING_BITMAPS` | `(1UL << 0)` |  |
| `BTRFS_FREE_SPACE_FLAGS_MASK` | `(BTRFS_FREE_SPACE_USING_BITMAPS)` |  |

### BTRFS_FS (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FS_TREE_OBJECTID` | `5ULL` |  |

### BTRFS_FT (11)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_FT_UNKNOWN` | `0` |  |
| `BTRFS_FT_REG_FILE` | `1` |  |
| `BTRFS_FT_DIR` | `2` |  |
| `BTRFS_FT_CHRDEV` | `3` |  |
| `BTRFS_FT_BLKDEV` | `4` |  |
| `BTRFS_FT_FIFO` | `5` |  |
| `BTRFS_FT_SOCK` | `6` |  |
| `BTRFS_FT_SYMLINK` | `7` |  |
| `BTRFS_FT_XATTR` | `8` |  |
| `BTRFS_FT_MAX` | `9` |  |
| `BTRFS_FT_ENCRYPTED` | `0x80` |  |

### BTRFS_HEADER (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_HEADER_FLAG_WRITTEN` | `(1ULL << 0)` |  |
| `BTRFS_HEADER_FLAG_RELOC` | `(1ULL << 1)` |  |

### BTRFS_IDENTITY (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_IDENTITY_REMAP_KEY` | `234` |  |

### BTRFS_INODE (19)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_INODE_ITEM_KEY` | `1` |  |
| `BTRFS_INODE_REF_KEY` | `12` |  |
| `BTRFS_INODE_EXTREF_KEY` | `13` |  |
| `BTRFS_INODE_NODATASUM` | `(1U << 0)` |  |
| `BTRFS_INODE_NODATACOW` | `(1U << 1)` |  |
| `BTRFS_INODE_READONLY` | `(1U << 2)` |  |
| `BTRFS_INODE_NOCOMPRESS` | `(1U << 3)` |  |
| `BTRFS_INODE_PREALLOC` | `(1U << 4)` |  |
| `BTRFS_INODE_SYNC` | `(1U << 5)` |  |
| `BTRFS_INODE_IMMUTABLE` | `(1U << 6)` |  |
| `BTRFS_INODE_APPEND` | `(1U << 7)` |  |
| `BTRFS_INODE_NODUMP` | `(1U << 8)` |  |
| `BTRFS_INODE_NOATIME` | `(1U << 9)` |  |
| `BTRFS_INODE_DIRSYNC` | `(1U << 10)` |  |
| `BTRFS_INODE_COMPRESS` | `(1U << 11)` |  |
| `BTRFS_INODE_ROOT_ITEM_INIT` | `(1U << 31)` |  |
| `BTRFS_INODE_FLAG_MASK` | `` |  |
| `BTRFS_INODE_RO_VERITY` | `(1U << 0)` |  |
| `BTRFS_INODE_RO_FLAG_MASK` | `(BTRFS_INODE_RO_VERITY)` |  |

### BTRFS_LAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_LAST_FREE_OBJECTID` | `-256ULL` |  |

### BTRFS_LINK (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_LINK_MAX` | `65535U` |  |

### BTRFS_MAX (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_MAX_LEVEL` | `8` |  |
| `BTRFS_MAX_METADATA_BLOCKSIZE` | `65536` |  |

### BTRFS_METADATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_METADATA_ITEM_KEY` | `169` |  |

### BTRFS_MIXED (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_MIXED_BACKREF_REV` | `1` |  |

### BTRFS_MULTIPLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_MULTIPLE_OBJECTIDS` | `-255ULL` |  |

### BTRFS_NAME (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_NAME_LEN` | `255` |  |

### BTRFS_NUM (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_NUM_BACKUP_ROOTS` | `4` |  |

### BTRFS_OLD (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_OLD_BACKREF_REV` | `0` |  |

### BTRFS_ORPHAN (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_ORPHAN_OBJECTID` | `-5ULL` |  |
| `BTRFS_ORPHAN_ITEM_KEY` | `48` |  |

### BTRFS_PERSISTENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_PERSISTENT_ITEM_KEY` | `249` |  |

### BTRFS_QGROUP (11)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_QGROUP_STATUS_KEY` | `240` |  |
| `BTRFS_QGROUP_INFO_KEY` | `242` |  |
| `BTRFS_QGROUP_LIMIT_KEY` | `244` |  |
| `BTRFS_QGROUP_RELATION_KEY` | `246` |  |
| `BTRFS_QGROUP_LEVEL_SHIFT` | `48` |  |
| `BTRFS_QGROUP_STATUS_FLAG_ON` | `(1ULL << 0)` |  |
| `BTRFS_QGROUP_STATUS_FLAG_RESCAN` | `(1ULL << 1)` |  |
| `BTRFS_QGROUP_STATUS_FLAG_INCONSISTENT` | `(1ULL << 2)` |  |
| `BTRFS_QGROUP_STATUS_FLAG_SIMPLE_MODE` | `(1ULL << 3)` |  |
| `BTRFS_QGROUP_STATUS_FLAGS_MASK` | `(BTRFS_QGROUP_STATUS_FLAG_ON \|		` |  |
| `BTRFS_QGROUP_STATUS_VERSION` | `1` |  |

### BTRFS_QUOTA (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_QUOTA_TREE_OBJECTID` | `8ULL` |  |

### BTRFS_RAID (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_RAID_STRIPE_TREE_OBJECTID` | `12ULL` |  |
| `BTRFS_RAID_STRIPE_KEY` | `230` |  |

### BTRFS_REMAP (3)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_REMAP_TREE_OBJECTID` | `13ULL` |  |
| `BTRFS_REMAP_KEY` | `235` |  |
| `BTRFS_REMAP_BACKREF_KEY` | `236` |  |

### BTRFS_ROOT (7)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_ROOT_TREE_OBJECTID` | `1ULL` |  |
| `BTRFS_ROOT_TREE_DIR_OBJECTID` | `6ULL` |  |
| `BTRFS_ROOT_ITEM_KEY` | `132` |  |
| `BTRFS_ROOT_BACKREF_KEY` | `144` |  |
| `BTRFS_ROOT_REF_KEY` | `156` |  |
| `BTRFS_ROOT_SUBVOL_RDONLY` | `(1ULL << 0)` |  |
| `BTRFS_ROOT_SUBVOL_DEAD` | `(1ULL << 48)` |  |

### BTRFS_SHARED (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SHARED_BLOCK_REF_KEY` | `182` |  |
| `BTRFS_SHARED_DATA_REF_KEY` | `184` |  |

### BTRFS_SPACE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SPACE_INFO_GLOBAL_RSV` | `(1ULL << 49)` |  |

### BTRFS_STRING (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_STRING_ITEM_KEY` | `253` |  |

### BTRFS_SUPER (9)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SUPER_FLAG_ERROR` | `(1ULL << 2)` |  |
| `BTRFS_SUPER_FLAG_SEEDING` | `(1ULL << 32)` |  |
| `BTRFS_SUPER_FLAG_METADUMP` | `(1ULL << 33)` |  |
| `BTRFS_SUPER_FLAG_METADUMP_V2` | `(1ULL << 34)` |  |
| `BTRFS_SUPER_FLAG_CHANGING_FSID` | `(1ULL << 35)` |  |
| `BTRFS_SUPER_FLAG_CHANGING_FSID_V2` | `(1ULL << 36)` |  |
| `BTRFS_SUPER_FLAG_CHANGING_BG_TREE` | `(1ULL << 38)` |  |
| `BTRFS_SUPER_FLAG_CHANGING_DATA_CSUM` | `(1ULL << 39)` |  |
| `BTRFS_SUPER_FLAG_CHANGING_META_CSUM` | `(1ULL << 40)` |  |

### BTRFS_SYSTEM (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_SYSTEM_CHUNK_ARRAY_SIZE` | `2048` |  |

### BTRFS_TEMPORARY (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_TEMPORARY_ITEM_KEY` | `248` |  |

### BTRFS_TREE (4)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_TREE_LOG_OBJECTID` | `-6ULL` |  |
| `BTRFS_TREE_LOG_FIXUP_OBJECTID` | `-7ULL` |  |
| `BTRFS_TREE_RELOC_OBJECTID` | `-8ULL` |  |
| `BTRFS_TREE_BLOCK_REF_KEY` | `176` |  |

### BTRFS_UUID (3)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_UUID_TREE_OBJECTID` | `9ULL` |  |
| `BTRFS_UUID_KEY_SUBVOL` | `251` | for UUIDs assigned to subvols |
| `BTRFS_UUID_KEY_RECEIVED_SUBVOL` | `252	/* for UUIDs assigned to` |  |

### BTRFS_VERITY (2)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_VERITY_DESC_ITEM_KEY` | `36` |  |
| `BTRFS_VERITY_MERKLE_ITEM_KEY` | `37` |  |

### BTRFS_XATTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_XATTR_ITEM_KEY` | `24` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `BTRFS_MAGIC` | `0x4D5F53665248425FULL` |  |

## Structs (48)


### `struct btrfs_disk_key`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `objectid` | `-` |
| `__u8` | `type` | `-` |
| `__le64` | `offset` | `-` |

### `struct btrfs_key`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `objectid` | `-` |
| `__u8` | `type` | `-` |
| `__u64` | `offset` | `-` |

### `struct btrfs_header`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `csum` | `BTRFS_CSUM_SIZE` |
| `__u8` | `fsid` | `BTRFS_FSID_SIZE` |
| `__le64` | `bytenr` | `-` |
| `__le64` | `flags` | `-` |
| `__u8` | `chunk_tree_uuid` | `BTRFS_UUID_SIZE` |
| `__le64` | `generation` | `-` |
| `__le64` | `owner` | `-` |
| `__le32` | `nritems` | `-` |
| `__u8` | `level` | `-` |

### `struct btrfs_root_backup`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `tree_root` | `-` |
| `__le64` | `tree_root_gen` | `-` |
| `__le64` | `chunk_root` | `-` |
| `__le64` | `chunk_root_gen` | `-` |
| `__le64` | `extent_root` | `-` |
| `__le64` | `extent_root_gen` | `-` |
| `__le64` | `fs_root` | `-` |
| `__le64` | `fs_root_gen` | `-` |
| `__le64` | `dev_root` | `-` |
| `__le64` | `dev_root_gen` | `-` |
| `__le64` | `csum_root` | `-` |
| `__le64` | `csum_root_gen` | `-` |
| `__le64` | `total_bytes` | `-` |
| `__le64` | `bytes_used` | `-` |
| `__le64` | `num_devices` | `-` |
| `__le64` | `unused_64` | `4` |
| `__u8` | `tree_root_level` | `-` |
| `__u8` | `chunk_root_level` | `-` |
| `__u8` | `extent_root_level` | `-` |
| `__u8` | `fs_root_level` | `-` |
| `__u8` | `dev_root_level` | `-` |
| `__u8` | `csum_root_level` | `-` |
| `__u8` | `unused_8` | `10` |

### `struct btrfs_item`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `offset` | `-` |
| `__le32` | `size` | `-` |

### `struct btrfs_leaf`

| Type | Field | Array |
|------|-------|-------|

### `struct btrfs_key_ptr`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `blockptr` | `-` |
| `__le64` | `generation` | `-` |

### `struct btrfs_node`

| Type | Field | Array |
|------|-------|-------|

### `struct btrfs_dev_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `devid` | `-` |
| `__le64` | `total_bytes` | `-` |
| `__le64` | `bytes_used` | `-` |
| `__le32` | `io_align` | `-` |
| `__le32` | `io_width` | `-` |
| `__le32` | `sector_size` | `-` |
| `__le64` | `type` | `-` |
| `__le64` | `generation` | `-` |
| `__le64` | `start_offset` | `-` |
| `__le32` | `dev_group` | `-` |
| `__u8` | `seek_speed` | `-` |
| `__u8` | `bandwidth` | `-` |
| `__u8` | `uuid` | `BTRFS_UUID_SIZE` |
| `__u8` | `fsid` | `BTRFS_UUID_SIZE` |

### `struct btrfs_stripe`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `devid` | `-` |
| `__le64` | `offset` | `-` |
| `__u8` | `dev_uuid` | `BTRFS_UUID_SIZE` |

### `struct btrfs_chunk`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `length` | `-` |
| `__le64` | `owner` | `-` |
| `__le64` | `stripe_len` | `-` |
| `__le64` | `type` | `-` |
| `__le32` | `io_align` | `-` |
| `__le32` | `io_width` | `-` |
| `__le32` | `sector_size` | `-` |
| `__le16` | `num_stripes` | `-` |
| `__le16` | `sub_stripes` | `-` |

### `struct btrfs_super_block`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `csum` | `BTRFS_CSUM_SIZE` |
| `__u8` | `fsid` | `BTRFS_FSID_SIZE` |
| `__le64` | `bytenr` | `-` |
| `__le64` | `flags` | `-` |
| `__le64` | `magic` | `-` |
| `__le64` | `generation` | `-` |
| `__le64` | `root` | `-` |
| `__le64` | `chunk_root` | `-` |
| `__le64` | `log_root` | `-` |
| `__le64` | `__unused_log_root_transid` | `-` |
| `__le64` | `total_bytes` | `-` |
| `__le64` | `bytes_used` | `-` |
| `__le64` | `root_dir_objectid` | `-` |
| `__le64` | `num_devices` | `-` |
| `__le32` | `sectorsize` | `-` |
| `__le32` | `nodesize` | `-` |
| `__le32` | `__unused_leafsize` | `-` |
| `__le32` | `stripesize` | `-` |
| `__le32` | `sys_chunk_array_size` | `-` |
| `__le64` | `chunk_root_generation` | `-` |
| `__le64` | `compat_flags` | `-` |
| `__le64` | `compat_ro_flags` | `-` |
| `__le64` | `incompat_flags` | `-` |
| `__le16` | `csum_type` | `-` |
| `__u8` | `root_level` | `-` |
| `__u8` | `chunk_root_level` | `-` |
| `__u8` | `log_root_level` | `-` |
| `char` | `label` | `BTRFS_LABEL_SIZE` |
| `__le64` | `cache_generation` | `-` |
| `__le64` | `uuid_tree_generation` | `-` |
| `__u8` | `metadata_uuid` | `BTRFS_FSID_SIZE` |
| `__u64` | `nr_global_roots` | `-` |
| `__le64` | `remap_root` | `-` |
| `__le64` | `remap_root_generation` | `-` |
| `__u8` | `remap_root_level` | `-` |
| `__u8` | `reserved` | `199` |
| `__u8` | `sys_chunk_array` | `BTRFS_SYSTEM_CHUNK_ARRAY_SIZE` |
| `__u8` | `padding` | `565` |

### `struct btrfs_free_space_entry`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `offset` | `-` |
| `__le64` | `bytes` | `-` |
| `__u8` | `type` | `-` |

### `struct btrfs_free_space_header`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `generation` | `-` |
| `__le64` | `num_entries` | `-` |
| `__le64` | `num_bitmaps` | `-` |

### `struct btrfs_raid_stride`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `devid` | `-` |
| `__le64` | `physical` | `-` |

### `struct btrfs_stripe_extent`

| Type | Field | Array |
|------|-------|-------|

### `struct btrfs_extent_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `refs` | `-` |
| `__le64` | `generation` | `-` |
| `__le64` | `flags` | `-` |

### `struct btrfs_extent_item_v0`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `refs` | `-` |

### `struct btrfs_tree_block_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `level` | `-` |

### `struct btrfs_extent_data_ref`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `root` | `-` |
| `__le64` | `objectid` | `-` |
| `__le64` | `offset` | `-` |
| `__le32` | `count` | `-` |

### `struct btrfs_shared_data_ref`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `count` | `-` |

### `struct btrfs_extent_owner_ref`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `root_id` | `-` |

### `struct btrfs_extent_inline_ref`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__le64` | `offset` | `-` |

### `struct btrfs_dev_extent`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `chunk_tree` | `-` |
| `__le64` | `chunk_objectid` | `-` |
| `__le64` | `chunk_offset` | `-` |
| `__le64` | `length` | `-` |
| `__u8` | `chunk_tree_uuid` | `BTRFS_UUID_SIZE` |

### `struct btrfs_inode_ref`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `index` | `-` |
| `__le16` | `name_len` | `-` |

### `struct btrfs_inode_extref`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `parent_objectid` | `-` |
| `__le64` | `index` | `-` |
| `__le16` | `name_len` | `-` |

### `struct btrfs_timespec`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `sec` | `-` |
| `__le32` | `nsec` | `-` |

### `struct btrfs_inode_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `generation` | `-` |
| `__le64` | `transid` | `-` |
| `__le64` | `size` | `-` |
| `__le64` | `nbytes` | `-` |
| `__le64` | `block_group` | `-` |
| `__le32` | `nlink` | `-` |
| `__le32` | `uid` | `-` |
| `__le32` | `gid` | `-` |
| `__le32` | `mode` | `-` |
| `__le64` | `rdev` | `-` |
| `__le64` | `flags` | `-` |
| `__le64` | `sequence` | `-` |
| `__le64` | `reserved` | `4` |

### `struct btrfs_dir_log_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `end` | `-` |

### `struct btrfs_dir_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `transid` | `-` |
| `__le16` | `data_len` | `-` |
| `__le16` | `name_len` | `-` |
| `__u8` | `type` | `-` |

### `struct btrfs_root_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `generation` | `-` |
| `__le64` | `root_dirid` | `-` |
| `__le64` | `bytenr` | `-` |
| `__le64` | `byte_limit` | `-` |
| `__le64` | `bytes_used` | `-` |
| `__le64` | `last_snapshot` | `-` |
| `__le64` | `flags` | `-` |
| `__le32` | `refs` | `-` |
| `__u8` | `drop_level` | `-` |
| `__u8` | `level` | `-` |
| `__le64` | `generation_v2` | `-` |
| `__u8` | `uuid` | `BTRFS_UUID_SIZE` |
| `__u8` | `parent_uuid` | `BTRFS_UUID_SIZE` |
| `__u8` | `received_uuid` | `BTRFS_UUID_SIZE` |
| `__le64` | `ctransid` | `-` |
| `__le64` | `otransid` | `-` |
| `__le64` | `stransid` | `-` |
| `__le64` | `rtransid` | `-` |
| `__le64` | `reserved` | `8` |

### `struct btrfs_root_ref`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `dirid` | `-` |
| `__le64` | `sequence` | `-` |
| `__le16` | `name_len` | `-` |

### `struct btrfs_disk_balance_args`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `profiles` | `-` |
| `__le64` | `usage` | `-` |
| `__le32` | `usage_min` | `-` |
| `__le32` | `usage_max` | `-` |
| `__le64` | `devid` | `-` |
| `__le64` | `pstart` | `-` |
| `__le64` | `pend` | `-` |
| `__le64` | `vstart` | `-` |
| `__le64` | `vend` | `-` |
| `__le64` | `target` | `-` |
| `__le64` | `flags` | `-` |
| `__le64` | `limit` | `-` |
| `__le32` | `limit_min` | `-` |
| `__le32` | `limit_max` | `-` |
| `__le32` | `stripes_min` | `-` |
| `__le32` | `stripes_max` | `-` |
| `__le64` | `unused` | `6` |

### `struct anonymous_33`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `usage_min` | `-` |
| `__le32` | `usage_max` | `-` |

### `struct anonymous_34`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `limit_min` | `-` |
| `__le32` | `limit_max` | `-` |

### `struct btrfs_balance_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `flags` | `-` |
| `__le64` | `unused` | `4` |

### `struct btrfs_file_extent_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `generation` | `-` |
| `__le64` | `ram_bytes` | `-` |
| `__u8` | `compression` | `-` |
| `__u8` | `encryption` | `-` |
| `__le16` | `other_encoding` | `-` |
| `__u8` | `type` | `-` |
| `__le64` | `disk_bytenr` | `-` |
| `__le64` | `disk_num_bytes` | `-` |
| `__le64` | `offset` | `-` |
| `__le64` | `num_bytes` | `-` |

### `struct btrfs_csum_item`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `csum` | `-` |

### `struct btrfs_dev_stats_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `values` | `BTRFS_DEV_STAT_VALUES_MAX` |

### `struct btrfs_dev_replace_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `src_devid` | `-` |
| `__le64` | `cursor_left` | `-` |
| `__le64` | `cursor_right` | `-` |
| `__le64` | `cont_reading_from_srcdev_mode` | `-` |
| `__le64` | `replace_state` | `-` |
| `__le64` | `time_started` | `-` |
| `__le64` | `time_stopped` | `-` |
| `__le64` | `num_write_errors` | `-` |
| `__le64` | `num_uncorrectable_read_errors` | `-` |

### `struct btrfs_block_group_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `used` | `-` |
| `__le64` | `chunk_objectid` | `-` |
| `__le64` | `flags` | `-` |

### `struct btrfs_block_group_item_v2`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `used` | `-` |
| `__le64` | `chunk_objectid` | `-` |
| `__le64` | `flags` | `-` |
| `__le64` | `remap_bytes` | `-` |
| `__le32` | `identity_remap_count` | `-` |

### `struct btrfs_free_space_info`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `extent_count` | `-` |
| `__le32` | `flags` | `-` |

### `struct btrfs_qgroup_status_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `version` | `-` |
| `__le64` | `generation` | `-` |
| `__le64` | `flags` | `-` |
| `__le64` | `rescan` | `-` |
| `__le64` | `enable_gen` | `-` |

### `struct btrfs_qgroup_info_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `generation` | `-` |
| `__le64` | `rfer` | `-` |
| `__le64` | `rfer_cmpr` | `-` |
| `__le64` | `excl` | `-` |
| `__le64` | `excl_cmpr` | `-` |

### `struct btrfs_qgroup_limit_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `flags` | `-` |
| `__le64` | `max_rfer` | `-` |
| `__le64` | `max_excl` | `-` |
| `__le64` | `rsv_rfer` | `-` |
| `__le64` | `rsv_excl` | `-` |

### `struct btrfs_verity_descriptor_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `size` | `-` |
| `__le64` | `reserved` | `2` |
| `__u8` | `encryption` | `-` |

### `struct btrfs_remap_item`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `address` | `-` |