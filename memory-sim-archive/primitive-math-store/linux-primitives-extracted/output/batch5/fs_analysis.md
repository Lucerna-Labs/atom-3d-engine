# fs.h

**Source:** `fs.h`


## Includes

- `linux/limits.h`
- `linux/ioctl.h`
- `linux/types.h`
- `linux/fscrypt.h`
- `linux/mount.h`

## Defines (170 total)


### BLOCK_SIZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BLOCK_SIZE_BITS` | `10` |  |

### FILE_ATTR (2)

| Name | Value | Comment |
|------|-------|---------|
| `FILE_ATTR_SIZE_VER0` | `24` |  |
| `FILE_ATTR_SIZE_LATEST` | `FILE_ATTR_SIZE_VER0` |  |

### FILE_DEDUPE (2)

| Name | Value | Comment |
|------|-------|---------|
| `FILE_DEDUPE_RANGE_SAME` | `0` |  |
| `FILE_DEDUPE_RANGE_DIFFERS` | `1` |  |

### FS_APPEND (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_APPEND_FL` | `0x00000020` | writes to file may only append |

### FS_BTREE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_BTREE_FL` | `0x00001000` | btree format dir |

### FS_CASEFOLD (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_CASEFOLD_FL` | `0x40000000` | Folder is case insensitive |

### FS_COMPR (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_COMPR_FL` | `0x00000004` | Compress file |

### FS_COMPRBLK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_COMPRBLK_FL` | `0x00000200` | One or more compressed clusters |

### FS_DAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_DAX_FL` | `0x02000000` | Inode is DAX |

### FS_DIRSYNC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_DIRSYNC_FL` | `0x00010000` | dirsync behaviour (directories only) |

### FS_DIRTY (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_DIRTY_FL` | `0x00000100` |  |

### FS_EA (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_EA_INODE_FL` | `0x00200000` | Inode used for large EA |

### FS_ENCRYPT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_ENCRYPT_FL` | `0x00000800` | Encrypted file |

### FS_EOFBLOCKS (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_EOFBLOCKS_FL` | `0x00400000` | Reserved for ext4 |

### FS_EXTENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_EXTENT_FL` | `0x00080000` | Extents |

### FS_FL (2)

| Name | Value | Comment |
|------|-------|---------|
| `FS_FL_USER_VISIBLE` | `0x0003DFFF` | User visible flags |
| `FS_FL_USER_MODIFIABLE` | `0x000380FF` | User modifiable flags |

### FS_HUGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_HUGE_FILE_FL` | `0x00040000` | Reserved for ext4 |

### FS_IMAGIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_IMAGIC_FL` | `0x00002000` | AFS directory |

### FS_IMMUTABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_IMMUTABLE_FL` | `0x00000010` | Immutable file |

### FS_INDEX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_INDEX_FL` | `0x00001000` | hash-indexed directory |

### FS_INLINE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_INLINE_DATA_FL` | `0x10000000` | Reserved for ext4 |

### FS_IOC (13)

| Name | Value | Comment |
|------|-------|---------|
| `FS_IOC_GETFLAGS` | `_IOR('f', 1, long)` |  |
| `FS_IOC_SETFLAGS` | `_IOW('f', 2, long)` |  |
| `FS_IOC_GETVERSION` | `_IOR('v', 1, long)` |  |
| `FS_IOC_SETVERSION` | `_IOW('v', 2, long)` |  |
| `FS_IOC_FIEMAP` | `_IOWR('f', 11, struct fiemap)` |  |
| `FS_IOC_FSGETXATTR` | `_IOR('X', 31, struct fsxattr)` |  |
| `FS_IOC_FSSETXATTR` | `_IOW('X', 32, struct fsxattr)` |  |
| `FS_IOC_GETFSLABEL` | `_IOR(0x94, 49, char[FSLABEL_MAX])` |  |
| `FS_IOC_SETFSLABEL` | `_IOW(0x94, 50, char[FSLABEL_MAX])` |  |
| `FS_IOC_GETFSUUID` | `_IOR(0x15, 0, struct fsuuid2)` |  |
| `FS_IOC_GETFSSYSFSPATH` | `_IOR(0x15, 1, struct fs_sysfs_path)` |  |
| `FS_IOC_GETLBMD_CAP` | `_IOWR(0x15, 2, struct logical_block_metadata_cap)` |  |
| `FS_IOC_SHUTDOWN` | `_IOR('X', 125, __u32)` |  |

### FS_JOURNAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_JOURNAL_DATA_FL` | `0x00004000` | Reserved for ext3 |

### FS_NOATIME (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_NOATIME_FL` | `0x00000080` | do not update atime |

### FS_NOCOMP (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_NOCOMP_FL` | `0x00000400` | Don't compress |

### FS_NOCOW (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_NOCOW_FL` | `0x00800000` | Do not cow file |

### FS_NODUMP (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_NODUMP_FL` | `0x00000040` | do not dump file |

### FS_NOTAIL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_NOTAIL_FL` | `0x00008000` | file tail should not be merged |

### FS_PROJINHERIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_PROJINHERIT_FL` | `0x20000000` | Create with parents projid |

### FS_RESERVED (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_RESERVED_FL` | `0x80000000` | reserved for ext2 lib |

### FS_SECRM (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_SECRM_FL` | `0x00000001` | Secure deletion |

### FS_SHUTDOWN (3)

| Name | Value | Comment |
|------|-------|---------|
| `FS_SHUTDOWN_FLAGS_DEFAULT` | `0x0` |  |
| `FS_SHUTDOWN_FLAGS_LOGFLUSH` | `0x1` | flush log but not data |
| `FS_SHUTDOWN_FLAGS_NOLOGFLUSH` | `0x2` | don't flush log nor data |

### FS_SYNC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_SYNC_FL` | `0x00000008` | Synchronous updates |

### FS_TOPDIR (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_TOPDIR_FL` | `0x00020000` | Top of directory hierarchies |

### FS_UNRM (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_UNRM_FL` | `0x00000002` | Undelete |

### FS_VERITY (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_VERITY_FL` | `0x00100000` | Verity protected inode |

### FS_XFLAG (18)

| Name | Value | Comment |
|------|-------|---------|
| `FS_XFLAG_REALTIME` | `0x00000001` | data in realtime volume |
| `FS_XFLAG_PREALLOC` | `0x00000002` | preallocated file extents |
| `FS_XFLAG_IMMUTABLE` | `0x00000008` | file cannot be modified |
| `FS_XFLAG_APPEND` | `0x00000010` | all writes append |
| `FS_XFLAG_SYNC` | `0x00000020` | all writes synchronous |
| `FS_XFLAG_NOATIME` | `0x00000040` | do not update access time |
| `FS_XFLAG_NODUMP` | `0x00000080` | do not include in backups |
| `FS_XFLAG_RTINHERIT` | `0x00000100` | create with rt bit set |
| `FS_XFLAG_PROJINHERIT` | `0x00000200` | create with parents projid |
| `FS_XFLAG_NOSYMLINKS` | `0x00000400` | disallow symlink creation |
| `FS_XFLAG_EXTSIZE` | `0x00000800` | extent size allocator hint |
| `FS_XFLAG_EXTSZINHERIT` | `0x00001000` | inherit inode extent size |
| `FS_XFLAG_NODEFRAG` | `0x00002000` | do not defragment |
| `FS_XFLAG_FILESTREAM` | `0x00004000` | use filestream allocator |
| `FS_XFLAG_DAX` | `0x00008000` | use DAX for IO |
| `FS_XFLAG_COWEXTSIZE` | `0x00010000` | CoW extent size allocator hint |
| `FS_XFLAG_VERITY` | `0x00020000` | fs-verity enabled |
| `FS_XFLAG_HASATTR` | `0x80000000` | no DIFLAG for this |

### INR_OPEN (2)

| Name | Value | Comment |
|------|-------|---------|
| `INR_OPEN_CUR` | `1024` | Initial setting for nfile rlimits |
| `INR_OPEN_MAX` | `4096` | Hard limit for nfile rlimits |

### IO_INTEGRITY (4)

| Name | Value | Comment |
|------|-------|---------|
| `IO_INTEGRITY_CHK_GUARD` | `(1U << 0)` | enforce guard check |
| `IO_INTEGRITY_CHK_REFTAG` | `(1U << 1)` | enforce ref check |
| `IO_INTEGRITY_CHK_APPTAG` | `(1U << 2)` | enforce app check |
| `IO_INTEGRITY_VALID_FLAGS` | `(IO_INTEGRITY_CHK_GUARD \| ` |  |

### LBMD_PI (6)

| Name | Value | Comment |
|------|-------|---------|
| `LBMD_PI_CAP_INTEGRITY` | `(1 << 0)` |  |
| `LBMD_PI_CAP_REFTAG` | `(1 << 1)` |  |
| `LBMD_PI_CSUM_NONE` | `0` |  |
| `LBMD_PI_CSUM_IP` | `1` |  |
| `LBMD_PI_CSUM_CRC16_T10DIF` | `2` |  |
| `LBMD_PI_CSUM_CRC64_NVME` | `4` |  |

### LBMD_SIZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `LBMD_SIZE_VER0` | `16` |  |

### PAGE_IS (9)

| Name | Value | Comment |
|------|-------|---------|
| `PAGE_IS_WPALLOWED` | `(1 << 0)` |  |
| `PAGE_IS_WRITTEN` | `(1 << 1)` |  |
| `PAGE_IS_FILE` | `(1 << 2)` |  |
| `PAGE_IS_PRESENT` | `(1 << 3)` |  |
| `PAGE_IS_SWAPPED` | `(1 << 4)` |  |
| `PAGE_IS_PFNZERO` | `(1 << 5)` |  |
| `PAGE_IS_HUGE` | `(1 << 6)` |  |
| `PAGE_IS_SOFT_DIRTY` | `(1 << 7)` |  |
| `PAGE_IS_GUARD` | `(1 << 8)` |  |

### PM_SCAN (2)

| Name | Value | Comment |
|------|-------|---------|
| `PM_SCAN_WP_MATCHING` | `(1 << 0)` | Write protect the pages matched. |
| `PM_SCAN_CHECK_WPASYNC` | `(1 << 1)` | Abort the scan when a non-WP-enabled page is found. |

### PROCFS_IOCTL (1)

| Name | Value | Comment |
|------|-------|---------|
| `PROCFS_IOCTL_MAGIC` | `'f'` |  |

### SYNC_FILE (4)

| Name | Value | Comment |
|------|-------|---------|
| `SYNC_FILE_RANGE_WAIT_BEFORE` | `1` |  |
| `SYNC_FILE_RANGE_WRITE` | `2` |  |
| `SYNC_FILE_RANGE_WAIT_AFTER` | `4` |  |
| `SYNC_FILE_RANGE_WRITE_AND_WAIT` | `(SYNC_FILE_RANGE_WRITE \| ` |  |

### UNCATEGORIZED (70)

| Name | Value | Comment |
|------|-------|---------|
| `BLOCK_SIZE` | `(1<<BLOCK_SIZE_BITS)` |  |
| `SEEK_SET` | `0` | seek relative to beginning of file |
| `SEEK_CUR` | `1` | seek relative to current file position |
| `SEEK_END` | `2` | seek relative to end of file |
| `SEEK_DATA` | `3` | seek to the next data |
| `SEEK_HOLE` | `4` | seek to the next hole |
| `SEEK_MAX` | `SEEK_HOLE` |  |
| `RENAME_NOREPLACE` | `(1 << 0)` | Don't overwrite target |
| `RENAME_EXCHANGE` | `(1 << 1)` | Exchange source and dest |
| `RENAME_WHITEOUT` | `(1 << 2)` | Whiteout source |
| `NR_FILE` | `8192` | this can well be larger on a larger system |
| `BLKROSET` | `_IO(0x12,93)` | set device read-only (0 = read-write) |
| `BLKROGET` | `_IO(0x12,94)` | get read-only status (0 = read_write) |
| `BLKRRPART` | `_IO(0x12,95)` | re-read partition table |
| `BLKGETSIZE` | `_IO(0x12,96)` | return device size /512 (long *arg) |
| `BLKFLSBUF` | `_IO(0x12,97)` | flush buffer cache |
| `BLKRASET` | `_IO(0x12,98)` | set read ahead for block device |
| `BLKRAGET` | `_IO(0x12,99)` | get current read ahead setting |
| `BLKFRASET` | `_IO(0x12,100)` | set filesystem (mm/filemap.c) read-ahead |
| `BLKFRAGET` | `_IO(0x12,101)` | get filesystem (mm/filemap.c) read-ahead |
| `BLKSECTSET` | `_IO(0x12,102)` | set max sectors per request (ll_rw_blk.c) |
| `BLKSECTGET` | `_IO(0x12,103)` | get max sectors per request (ll_rw_blk.c) |
| `BLKSSZGET` | `_IO(0x12,104)` | get block device sector size |
| `BLKPG` | `_IO(0x12,105)` | See blkpg.h |
| `BLKELVGET` | `_IOR(0x12,106,size_t)` | elevator get |
| `BLKELVSET` | `_IOW(0x12,107,size_t)` | elevator set |
| `BLKBSZGET` | `_IOR(0x12,112,size_t)` |  |
| `BLKBSZSET` | `_IOW(0x12,113,size_t)` |  |
| `BLKGETSIZE64` | `_IOR(0x12,114,size_t)` | return device size in bytes (u64 *arg) |
| `BLKTRACESETUP` | `_IOWR(0x12,115,struct blk_user_trace_setup)` |  |
| `BLKTRACESTART` | `_IO(0x12,116)` |  |
| `BLKTRACESTOP` | `_IO(0x12,117)` |  |
| `BLKTRACETEARDOWN` | `_IO(0x12,118)` |  |
| `BLKDISCARD` | `_IO(0x12,119)` |  |
| `BLKIOMIN` | `_IO(0x12,120)` |  |
| `BLKIOOPT` | `_IO(0x12,121)` |  |
| `BLKALIGNOFF` | `_IO(0x12,122)` |  |
| `BLKPBSZGET` | `_IO(0x12,123)` |  |
| `BLKDISCARDZEROES` | `_IO(0x12,124)` |  |
| `BLKSECDISCARD` | `_IO(0x12,125)` |  |
| `BLKROTATIONAL` | `_IO(0x12,126)` |  |
| `BLKZEROOUT` | `_IO(0x12,127)` |  |
| `BLKGETDISKSEQ` | `_IOR(0x12,128,__u64)` |  |
| `BLKTRACESETUP2` | `_IOWR(0x12, 142, struct blk_user_trace_setup2)` |  |
| `BMAP_IOCTL` | `1` | obsolete - kept for compatibility |
| `FIBMAP` | `_IO(0x00,1)` | bmap access |
| `FIGETBSZ` | `_IO(0x00,2)` | get the block size used for bmap |
| `FIFREEZE` | `_IOWR('X', 119, int)` | Freeze |
| `FITHAW` | `_IOWR('X', 120, int)` | Thaw |
| `FITRIM` | `_IOWR('X', 121, struct fstrim_range)` | Trim |

*...and 20 more*

## Structs (14)


### `struct file_clone_range`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `src_fd` | `-` |
| `__u64` | `src_offset` | `-` |
| `__u64` | `src_length` | `-` |
| `__u64` | `dest_offset` | `-` |

### `struct fstrim_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `minlen` | `-` |

### `struct fsuuid2`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `len` | `-` |
| `__u8` | `uuid` | `16` |

### `struct fs_sysfs_path`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `len` | `-` |
| `__u8` | `name` | `128` |

### `struct logical_block_metadata_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `lbmd_flags` | `-` |
| `__u16` | `lbmd_interval` | `-` |
| `__u8` | `lbmd_size` | `-` |
| `__u8` | `lbmd_opaque_size` | `-` |
| `__u8` | `lbmd_opaque_offset` | `-` |
| `__u8` | `lbmd_pi_size` | `-` |
| `__u8` | `lbmd_pi_offset` | `-` |
| `__u8` | `lbmd_guard_tag_type` | `-` |
| `__u8` | `lbmd_app_tag_size` | `-` |
| `__u8` | `lbmd_ref_tag_size` | `-` |
| `__u8` | `lbmd_storage_tag_size` | `-` |
| `__u8` | `pad` | `-` |

### `struct file_dedupe_range_info`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `dest_fd` | `-` |
| `__u64` | `dest_offset` | `-` |
| `__u64` | `bytes_deduped` | `-` |
| `__s32` | `status` | `-` |
| `__u32` | `reserved` | `-` |

### `struct file_dedupe_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `src_offset` | `-` |
| `__u64` | `src_length` | `-` |
| `__u16` | `dest_count` | `-` |
| `__u16` | `reserved1` | `-` |
| `__u32` | `reserved2` | `-` |

### `struct files_stat_struct`

| Type | Field | Array |
|------|-------|-------|

### `struct inodes_stat_t`

| Type | Field | Array |
|------|-------|-------|
| `long` | `nr_inodes` | `-` |
| `long` | `nr_unused` | `-` |
| `long` | `dummy` | `5` |

### `struct fsxattr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fsx_xflags` | `-` |
| `__u32` | `fsx_extsize` | `-` |
| `__u32` | `fsx_nextents` | `-` |
| `__u32` | `fsx_projid` | `-` |
| `__u32` | `fsx_cowextsize` | `-` |

### `struct file_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fa_xflags` | `-` |
| `__u32` | `fa_extsize` | `-` |
| `__u32` | `fa_nextents` | `-` |
| `__u32` | `fa_projid` | `-` |
| `__u32` | `fa_cowextsize` | `-` |

### `struct page_region`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |
| `__u64` | `categories` | `-` |

### `struct pm_scan_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |
| `__u64` | `walk_end` | `-` |
| `__u64` | `vec` | `-` |
| `__u64` | `vec_len` | `-` |
| `__u64` | `max_pages` | `-` |
| `__u64` | `category_inverted` | `-` |
| `__u64` | `category_mask` | `-` |
| `__u64` | `category_anyof_mask` | `-` |
| `__u64` | `return_mask` | `-` |

### `struct procmap_query`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u64` | `query_flags` | `-` |
| `__u64` | `query_addr` | `-` |
| `__u64` | `vma_start` | `-` |
| `__u64` | `vma_end` | `-` |
| `__u64` | `vma_flags` | `-` |
| `__u64` | `vma_page_size` | `-` |
| `__u64` | `vma_offset` | `-` |
| `__u64` | `inode` | `-` |
| `__u32` | `dev_major` | `-` |
| `__u32` | `dev_minor` | `-` |
| `__u32` | `vma_name_size` | `-` |
| `__u32` | `build_id_size` | `-` |
| `__u64` | `vma_name_addr` | `-` |
| `__u64` | `build_id_addr` | `-` |