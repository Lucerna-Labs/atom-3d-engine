# quota.h

**Source:** `quota.h`


## Includes

- `linux/types.h`

## Defines (53 total)


### DQF_ROOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `DQF_ROOT_SQUASH` | `(1 << DQF_ROOT_SQUASH_B)` |  |

### DQF_SYS (1)

| Name | Value | Comment |
|------|-------|---------|
| `DQF_SYS_FILE` | `(1 << DQF_SYS_FILE_B)` |  |

### QFMT_VFS (3)

| Name | Value | Comment |
|------|-------|---------|
| `QFMT_VFS_OLD` | `1` |  |
| `QFMT_VFS_V0` | `2` |  |
| `QFMT_VFS_V1` | `4` |  |

### QIF_DQBLKSIZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `QIF_DQBLKSIZE_BITS` | `10` |  |

### QUOTA_NL (13)

| Name | Value | Comment |
|------|-------|---------|
| `QUOTA_NL_NOWARN` | `0` |  |
| `QUOTA_NL_IHARDWARN` | `1` | Inode hardlimit reached |
| `QUOTA_NL_ISOFTLONGWARN` | `2` | Inode grace time expired |
| `QUOTA_NL_ISOFTWARN` | `3` | Inode softlimit reached |
| `QUOTA_NL_BHARDWARN` | `4` | Block hardlimit reached |
| `QUOTA_NL_BSOFTLONGWARN` | `5` | Block grace time expired |
| `QUOTA_NL_BSOFTWARN` | `6` | Block softlimit reached |
| `QUOTA_NL_IHARDBELOW` | `7` | Usage got below inode hardlimit |
| `QUOTA_NL_ISOFTBELOW` | `8` | Usage got below inode softlimit |
| `QUOTA_NL_BHARDBELOW` | `9` | Usage got below block hardlimit |
| `QUOTA_NL_BSOFTBELOW` | `10` | Usage got below block softlimit |
| `QUOTA_NL_C_MAX` | `(__QUOTA_NL_C_MAX - 1)` |  |
| `QUOTA_NL_A_MAX` | `(__QUOTA_NL_A_MAX - 1)` |  |

### UNCATEGORIZED (34)

| Name | Value | Comment |
|------|-------|---------|
| `__DQUOT_VERSION__` | `"dquot_6.6.0"` |  |
| `MAXQUOTAS` | `3` |  |
| `USRQUOTA` | `0` | element used for user quotas |
| `GRPQUOTA` | `1` | element used for group quotas |
| `PRJQUOTA` | `2` | element used for project quotas |
| `INITQFNAMES` | `{ ` |  |
| `SUBCMDMASK` | `0x00ff` |  |
| `SUBCMDSHIFT` | `8` |  |
| `Q_SYNC` | `0x800001` | sync disk copy of a filesystems quotas |
| `Q_QUOTAON` | `0x800002` | turn quotas on |
| `Q_QUOTAOFF` | `0x800003` | turn quotas off |
| `Q_GETFMT` | `0x800004` | get quota format used on given filesystem |
| `Q_GETINFO` | `0x800005` | get information about quota files |
| `Q_SETINFO` | `0x800006` | set information about quota files |
| `Q_GETQUOTA` | `0x800007` | get user quota structure |
| `Q_SETQUOTA` | `0x800008` | set user quota structure |
| `Q_GETNEXTQUOTA` | `0x800009` | get disk limits and usage >= ID |
| `QFMT_OCFS2` | `3` |  |
| `QFMT_SHMEM` | `5` |  |
| `QIF_DQBLKSIZE` | `(1 << QIF_DQBLKSIZE_BITS)` |  |
| `QIF_BLIMITS` | `(1 << QIF_BLIMITS_B)` |  |
| `QIF_SPACE` | `(1 << QIF_SPACE_B)` |  |
| `QIF_ILIMITS` | `(1 << QIF_ILIMITS_B)` |  |
| `QIF_INODES` | `(1 << QIF_INODES_B)` |  |
| `QIF_BTIME` | `(1 << QIF_BTIME_B)` |  |
| `QIF_ITIME` | `(1 << QIF_ITIME_B)` |  |
| `QIF_LIMITS` | `(QIF_BLIMITS \| QIF_ILIMITS)` |  |
| `QIF_USAGE` | `(QIF_SPACE \| QIF_INODES)` |  |
| `QIF_TIMES` | `(QIF_BTIME \| QIF_ITIME)` |  |
| `QIF_ALL` | `(QIF_LIMITS \| QIF_USAGE \| QIF_TIMES)` |  |
| `IIF_BGRACE` | `1` |  |
| `IIF_IGRACE` | `2` |  |
| `IIF_FLAGS` | `4` |  |
| `IIF_ALL` | `(IIF_BGRACE \| IIF_IGRACE \| IIF_FLAGS)` |  |

## Structs (3)


### `struct if_dqblk`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dqb_bhardlimit` | `-` |
| `__u64` | `dqb_bsoftlimit` | `-` |
| `__u64` | `dqb_curspace` | `-` |
| `__u64` | `dqb_ihardlimit` | `-` |
| `__u64` | `dqb_isoftlimit` | `-` |
| `__u64` | `dqb_curinodes` | `-` |
| `__u64` | `dqb_btime` | `-` |
| `__u64` | `dqb_itime` | `-` |
| `__u32` | `dqb_valid` | `-` |

### `struct if_nextdqblk`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dqb_bhardlimit` | `-` |
| `__u64` | `dqb_bsoftlimit` | `-` |
| `__u64` | `dqb_curspace` | `-` |
| `__u64` | `dqb_ihardlimit` | `-` |
| `__u64` | `dqb_isoftlimit` | `-` |
| `__u64` | `dqb_curinodes` | `-` |
| `__u64` | `dqb_btime` | `-` |
| `__u64` | `dqb_itime` | `-` |
| `__u32` | `dqb_valid` | `-` |
| `__u32` | `dqb_id` | `-` |

### `struct if_dqinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dqi_bgrace` | `-` |
| `__u64` | `dqi_igrace` | `-` |
| `__u32` | `dqi_flags` | `-` |
| `__u32` | `dqi_valid` | `-` |