# gfs2_ondisk.h

**Source:** `gfs2_ondisk.h`


## Includes

- `linux/types.h`

## Defines (117 total)


### UNCATEGORIZED (117)

| Name | Value | Comment |
|------|-------|---------|
| `GFS2_MAGIC` | `0x01161970` |  |
| `GFS2_BASIC_BLOCK` | `512` |  |
| `GFS2_BASIC_BLOCK_SHIFT` | `9` |  |
| `GFS2_MOUNT_LOCK` | `0` |  |
| `GFS2_LIVE_LOCK` | `1` |  |
| `GFS2_FREEZE_LOCK` | `2` |  |
| `GFS2_RENAME_LOCK` | `3` |  |
| `GFS2_CONTROL_LOCK` | `4` |  |
| `GFS2_MOUNTED_LOCK` | `5` |  |
| `GFS2_FORMAT_NONE` | `0` |  |
| `GFS2_FORMAT_SB` | `100` |  |
| `GFS2_FORMAT_RG` | `200` |  |
| `GFS2_FORMAT_RB` | `300` |  |
| `GFS2_FORMAT_DI` | `400` |  |
| `GFS2_FORMAT_IN` | `500` |  |
| `GFS2_FORMAT_LF` | `600` |  |
| `GFS2_FORMAT_JD` | `700` |  |
| `GFS2_FORMAT_LH` | `800` |  |
| `GFS2_FORMAT_LD` | `900` |  |
| `GFS2_FORMAT_LB` | `1000` |  |
| `GFS2_FORMAT_EA` | `1600` |  |
| `GFS2_FORMAT_ED` | `1700` |  |
| `GFS2_FORMAT_QC` | `1400` |  |
| `GFS2_FORMAT_RI` | `1100` |  |
| `GFS2_FORMAT_DE` | `1200` |  |
| `GFS2_FORMAT_QU` | `1500` |  |
| `GFS2_FORMAT_FS` | `1802` |  |
| `GFS2_FORMAT_MULTI` | `1900` |  |
| `GFS2_METATYPE_NONE` | `0` |  |
| `GFS2_METATYPE_SB` | `1` |  |
| `GFS2_METATYPE_RG` | `2` |  |
| `GFS2_METATYPE_RB` | `3` |  |
| `GFS2_METATYPE_DI` | `4` |  |
| `GFS2_METATYPE_IN` | `5` |  |
| `GFS2_METATYPE_LF` | `6` |  |
| `GFS2_METATYPE_JD` | `7` |  |
| `GFS2_METATYPE_LH` | `8` |  |
| `GFS2_METATYPE_LD` | `9` |  |
| `GFS2_METATYPE_LB` | `12` |  |
| `GFS2_METATYPE_EA` | `10` |  |
| `GFS2_METATYPE_ED` | `11` |  |
| `GFS2_METATYPE_QC` | `14` |  |
| `GFS2_SB_ADDR` | `128` |  |
| `GFS2_SB_LOCK` | `0` |  |
| `GFS2_LOCKNAME_LEN` | `64` |  |
| `GFS2_HAS_UUID` | `1` |  |
| `GFS2_NBBY` | `4` |  |
| `GFS2_BIT_SIZE` | `2` |  |
| `GFS2_BIT_MASK` | `0x00000003` |  |
| `GFS2_BLKST_FREE` | `0` |  |

*...and 67 more*

## Structs (20)


### `struct gfs2_inum`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `no_formal_ino` | `-` |
| `__be64` | `no_addr` | `-` |

### `struct gfs2_meta_header`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `mh_magic` | `-` |
| `__be32` | `mh_type` | `-` |
| `__be64` | `__pad0` | `-` |
| `__be32` | `mh_format` | `-` |
| `__be32` | `mh_jid` | `-` |
| `__be32` | `__pad1` | `-` |

### `struct gfs2_sb`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `sb_fs_format` | `-` |
| `__be32` | `sb_multihost_format` | `-` |
| `__u32` | `__pad0` | `-` |
| `__be32` | `sb_bsize` | `-` |
| `__be32` | `sb_bsize_shift` | `-` |
| `__u32` | `__pad1` | `-` |
| `char` | `sb_lockproto` | `GFS2_LOCKNAME_LEN` |
| `char` | `sb_locktable` | `GFS2_LOCKNAME_LEN` |
| `__u8` | `sb_uuid` | `16` |

### `struct gfs2_rindex`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `ri_addr` | `-` |
| `__be32` | `ri_length` | `-` |
| `__u32` | `__pad` | `-` |
| `__be64` | `ri_data0` | `-` |
| `__be32` | `ri_data` | `-` |
| `__be32` | `ri_bitbytes` | `-` |
| `__u8` | `ri_reserved` | `64` |

### `struct gfs2_inode_lvb`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ri_magic` | `-` |
| `__be32` | `__pad` | `-` |
| `__be64` | `ri_generation_deleted` | `-` |

### `struct gfs2_rgrp_lvb`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `rl_magic` | `-` |
| `__be32` | `rl_flags` | `-` |
| `__be32` | `rl_free` | `-` |
| `__be32` | `rl_dinodes` | `-` |
| `__be64` | `rl_igeneration` | `-` |
| `__be32` | `rl_unlinked` | `-` |
| `__be32` | `__pad` | `-` |

### `struct gfs2_rgrp`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `rg_flags` | `-` |
| `__be32` | `rg_free` | `-` |
| `__be32` | `rg_dinodes` | `-` |
| `__be32` | `__pad` | `-` |
| `__be32` | `rg_skip` | `-` |
| `__be64` | `rg_igeneration` | `-` |
| `__be64` | `rg_data0` | `-` |
| `__be32` | `rg_data` | `-` |
| `__be32` | `rg_bitbytes` | `-` |
| `__be32` | `rg_crc` | `-` |
| `__u8` | `rg_reserved` | `60` |

### `struct gfs2_quota`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `qu_limit` | `-` |
| `__be64` | `qu_warn` | `-` |
| `__be64` | `qu_value` | `-` |
| `__u8` | `qu_reserved` | `64` |

### `struct gfs2_dinode`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `di_mode` | `-` |
| `__be32` | `di_uid` | `-` |
| `__be32` | `di_gid` | `-` |
| `__be32` | `di_nlink` | `-` |
| `__be64` | `di_size` | `-` |
| `__be64` | `di_blocks` | `-` |
| `__be64` | `di_atime` | `-` |
| `__be64` | `di_mtime` | `-` |
| `__be64` | `di_ctime` | `-` |
| `__be32` | `di_major` | `-` |
| `__be32` | `di_minor` | `-` |
| `__be64` | `di_goal_meta` | `-` |
| `__be64` | `di_goal_data` | `-` |
| `__be64` | `di_generation` | `-` |
| `__be32` | `di_flags` | `-` |
| `__be32` | `di_payload_format` | `-` |
| `__u16` | `__pad1` | `-` |
| `__be16` | `di_height` | `-` |
| `__u32` | `__pad2` | `-` |
| `__u16` | `__pad3` | `-` |
| `__be16` | `di_depth` | `-` |
| `__be32` | `di_entries` | `-` |
| `__be64` | `di_eattr` | `-` |
| `__be32` | `di_atime_nsec` | `-` |
| `__be32` | `di_mtime_nsec` | `-` |
| `__be32` | `di_ctime_nsec` | `-` |
| `__u8` | `di_reserved` | `44` |

### `struct gfs2_dirent`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `de_hash` | `-` |
| `__be16` | `de_rec_len` | `-` |
| `__be16` | `de_name_len` | `-` |
| `__be16` | `de_type` | `-` |
| `__be16` | `de_rahead` | `-` |
| `__u8` | `__pad` | `12` |
| `__u32` | `de_cookie` | `-` |
| `__u8` | `pad3` | `8` |

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `de_cookie` | `-` |
| `__u8` | `pad3` | `8` |

### `struct gfs2_leaf`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `lf_depth` | `-` |
| `__be16` | `lf_entries` | `-` |
| `__be32` | `lf_dirent_format` | `-` |
| `__be64` | `lf_next` | `-` |
| `__u8` | `lf_reserved` | `64` |
| `__be64` | `lf_inode` | `-` |
| `__be32` | `lf_dist` | `-` |
| `__be32` | `lf_nsec` | `-` |
| `__be64` | `lf_sec` | `-` |
| `__u8` | `lf_reserved2` | `40` |

### `struct anonymous_12`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `lf_inode` | `-` |
| `__be32` | `lf_dist` | `-` |
| `__be32` | `lf_nsec` | `-` |
| `__be64` | `lf_sec` | `-` |
| `__u8` | `lf_reserved2` | `40` |

### `struct gfs2_ea_header`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ea_rec_len` | `-` |
| `__be32` | `ea_data_len` | `-` |
| `__u8` | `ea_name_len` | `-` |
| `__u8` | `ea_type` | `-` |
| `__u8` | `ea_flags` | `-` |
| `__u8` | `ea_num_ptrs` | `-` |
| `__u32` | `__pad` | `-` |

### `struct gfs2_log_header`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `lh_sequence` | `-` |
| `__be32` | `lh_flags` | `-` |
| `__be32` | `lh_tail` | `-` |
| `__be32` | `lh_blkno` | `-` |
| `__be32` | `lh_hash` | `-` |
| `__be32` | `lh_crc` | `-` |
| `__be32` | `lh_nsec` | `-` |
| `__be64` | `lh_sec` | `-` |
| `__be64` | `lh_addr` | `-` |
| `__be64` | `lh_jinode` | `-` |
| `__be64` | `lh_statfs_addr` | `-` |
| `__be64` | `lh_quota_addr` | `-` |
| `__be64` | `lh_local_total` | `-` |
| `__be64` | `lh_local_free` | `-` |
| `__be64` | `lh_local_dinodes` | `-` |

### `struct gfs2_log_descriptor`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ld_type` | `-` |
| `__be32` | `ld_length` | `-` |
| `__be32` | `ld_data1` | `-` |
| `__be32` | `ld_data2` | `-` |
| `__u8` | `ld_reserved` | `32` |

### `struct gfs2_inum_range`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `ir_start` | `-` |
| `__be64` | `ir_length` | `-` |

### `struct gfs2_statfs_change`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `sc_total` | `-` |
| `__be64` | `sc_free` | `-` |
| `__be64` | `sc_dinodes` | `-` |

### `struct gfs2_quota_change`

| Type | Field | Array |
|------|-------|-------|
| `__be64` | `qc_change` | `-` |
| `__be32` | `qc_flags` | `-` |
| `__be32` | `qc_id` | `-` |

### `struct gfs2_quota_lvb`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `qb_magic` | `-` |
| `__u32` | `__pad` | `-` |
| `__be64` | `qb_limit` | `-` |
| `__be64` | `qb_warn` | `-` |
| `__be64` | `qb_value` | `-` |