# nilfs2_api.h

**Source:** `nilfs2_api.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (14 total)


### NILFS_IOCTL (14)

| Name | Value | Comment |
|------|-------|---------|
| `NILFS_IOCTL_IDENT` | `'n'` |  |
| `NILFS_IOCTL_CHANGE_CPMODE` | `` |  |
| `NILFS_IOCTL_DELETE_CHECKPOINT` | `` |  |
| `NILFS_IOCTL_GET_CPINFO` | `` |  |
| `NILFS_IOCTL_GET_CPSTAT` | `` |  |
| `NILFS_IOCTL_GET_SUINFO` | `` |  |
| `NILFS_IOCTL_GET_SUSTAT` | `` |  |
| `NILFS_IOCTL_GET_VINFO` | `` |  |
| `NILFS_IOCTL_GET_BDESCS` | `` |  |
| `NILFS_IOCTL_CLEAN_SEGMENTS` | `` |  |
| `NILFS_IOCTL_SYNC` | `` |  |
| `NILFS_IOCTL_RESIZE` | `` |  |
| `NILFS_IOCTL_SET_ALLOC_RANGE` | `` |  |
| `NILFS_IOCTL_SET_SUINFO` | `` |  |

## Structs (11)


### `struct nilfs_cpinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ci_flags` | `-` |
| `__u32` | `ci_pad` | `-` |
| `__u64` | `ci_cno` | `-` |
| `__u64` | `ci_create` | `-` |
| `__u64` | `ci_nblk_inc` | `-` |
| `__u64` | `ci_inodes_count` | `-` |
| `__u64` | `ci_blocks_count` | `-` |
| `__u64` | `ci_next` | `-` |

### `struct nilfs_suinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `sui_lastmod` | `-` |
| `__u32` | `sui_nblocks` | `-` |
| `__u32` | `sui_flags` | `-` |

### `struct nilfs_suinfo_update`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `sup_segnum` | `-` |
| `__u32` | `sup_flags` | `-` |
| `__u32` | `sup_reserved` | `-` |

### `struct nilfs_cpmode`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cm_cno` | `-` |
| `__u32` | `cm_mode` | `-` |
| `__u32` | `cm_pad` | `-` |

### `struct nilfs_argv`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `v_base` | `-` |
| `__u32` | `v_nmembs` | `-` |
| `__u16` | `v_size` | `-` |
| `__u16` | `v_flags` | `-` |
| `__u64` | `v_index` | `-` |

### `struct nilfs_period`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `p_start` | `-` |
| `__u64` | `p_end` | `-` |

### `struct nilfs_cpstat`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cs_cno` | `-` |
| `__u64` | `cs_ncps` | `-` |
| `__u64` | `cs_nsss` | `-` |

### `struct nilfs_sustat`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ss_nsegs` | `-` |
| `__u64` | `ss_ncleansegs` | `-` |
| `__u64` | `ss_ndirtysegs` | `-` |
| `__u64` | `ss_ctime` | `-` |
| `__u64` | `ss_nongc_ctime` | `-` |
| `__u64` | `ss_prot_seq` | `-` |

### `struct nilfs_vinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `vi_vblocknr` | `-` |
| `__u64` | `vi_start` | `-` |
| `__u64` | `vi_end` | `-` |
| `__u64` | `vi_blocknr` | `-` |

### `struct nilfs_vdesc`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `vd_ino` | `-` |
| `__u64` | `vd_cno` | `-` |
| `__u64` | `vd_vblocknr` | `-` |
| `__u64` | `vd_blocknr` | `-` |
| `__u64` | `vd_offset` | `-` |
| `__u32` | `vd_flags` | `-` |
| `__u32` | `vd_pad` | `-` |

### `struct nilfs_bdesc`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bd_ino` | `-` |
| `__u64` | `bd_oblocknr` | `-` |
| `__u64` | `bd_blocknr` | `-` |
| `__u64` | `bd_offset` | `-` |
| `__u32` | `bd_level` | `-` |
| `__u32` | `bd_pad` | `-` |