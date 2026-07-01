# mount.h

**Source:** `mount.h`


## Includes

- `linux/types.h`

## Defines (86 total)


### FSPICK_EMPTY (1)

| Name | Value | Comment |
|------|-------|---------|
| `FSPICK_EMPTY_PATH` | `0x00000008` |  |

### FSPICK_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `FSPICK_NO_AUTOMOUNT` | `0x00000004` |  |

### FSPICK_SYMLINK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FSPICK_SYMLINK_NOFOLLOW` | `0x00000002` |  |

### MNT_ID (2)

| Name | Value | Comment |
|------|-------|---------|
| `MNT_ID_REQ_SIZE_VER0` | `24` | sizeof first published struct |
| `MNT_ID_REQ_SIZE_VER1` | `32` | sizeof second published struct |

### MOUNT_ATTR (12)

| Name | Value | Comment |
|------|-------|---------|
| `MOUNT_ATTR_RDONLY` | `0x00000001` | Mount read-only |
| `MOUNT_ATTR_NOSUID` | `0x00000002` | Ignore suid and sgid bits |
| `MOUNT_ATTR_NODEV` | `0x00000004` | Disallow access to device special files |
| `MOUNT_ATTR_NOEXEC` | `0x00000008` | Disallow program execution |
| `MOUNT_ATTR__ATIME` | `0x00000070` | Setting on how atime should be updated |
| `MOUNT_ATTR_RELATIME` | `0x00000000` | - Update atime relative to mtime/ctime. |
| `MOUNT_ATTR_NOATIME` | `0x00000010` | - Do not update access times. |
| `MOUNT_ATTR_STRICTATIME` | `0x00000020` | - Always perform atime updates |
| `MOUNT_ATTR_NODIRATIME` | `0x00000080` | Do not update directory access times |
| `MOUNT_ATTR_IDMAP` | `0x00100000` | Idmap mount to @userns_fd in struct mount_attr. |
| `MOUNT_ATTR_NOSYMFOLLOW` | `0x00200000` | Do not follow symlinks |
| `MOUNT_ATTR_SIZE_VER0` | `32` | sizeof first published struct |

### MOVE_MOUNT (9)

| Name | Value | Comment |
|------|-------|---------|
| `MOVE_MOUNT_F_SYMLINKS` | `0x00000001` | Follow symlinks on from path |
| `MOVE_MOUNT_F_AUTOMOUNTS` | `0x00000002` | Follow automounts on from path |
| `MOVE_MOUNT_F_EMPTY_PATH` | `0x00000004` | Empty from path permitted |
| `MOVE_MOUNT_T_SYMLINKS` | `0x00000010` | Follow symlinks on to path |
| `MOVE_MOUNT_T_AUTOMOUNTS` | `0x00000020` | Follow automounts on to path |
| `MOVE_MOUNT_T_EMPTY_PATH` | `0x00000040` | Empty to path permitted |
| `MOVE_MOUNT_SET_GROUP` | `0x00000100` | Set sharing group instead |
| `MOVE_MOUNT_BENEATH` | `0x00000200` | Mount beneath top mount |
| `MOVE_MOUNT__MASK` | `0x00000377` |  |

### MS_I (1)

| Name | Value | Comment |
|------|-------|---------|
| `MS_I_VERSION` | `(1<<23)` | Update inode I_version field |

### MS_MGC (2)

| Name | Value | Comment |
|------|-------|---------|
| `MS_MGC_VAL` | `0xC0ED0000` |  |
| `MS_MGC_MSK` | `0xffff0000` |  |

### MS_RMT (1)

| Name | Value | Comment |
|------|-------|---------|
| `MS_RMT_MASK` | `(MS_RDONLY\|MS_SYNCHRONOUS\|MS_MANDLOCK\|MS_I_VERSION\|` |  |

### OPEN_TREE (3)

| Name | Value | Comment |
|------|-------|---------|
| `OPEN_TREE_CLONE` | `(1 << 0)` | Clone the target tree and attach the clone |
| `OPEN_TREE_NAMESPACE` | `(1 << 1)` | Clone the target tree into a new mount namespace |
| `OPEN_TREE_CLOEXEC` | `O_CLOEXEC` | Close the file on execve() |

### STATMOUNT_BY (1)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_BY_FD` | `0x00000001U` | want mountinfo for given fd |

### STATMOUNT_FS (2)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_FS_TYPE` | `0x00000020U` | Want/got fs_type |
| `STATMOUNT_FS_SUBTYPE` | `0x00000100U` | Want/got fs_subtype |

### STATMOUNT_MNT (7)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_MNT_BASIC` | `0x00000002U` | Want/got mnt_... |
| `STATMOUNT_MNT_ROOT` | `0x00000008U` | Want/got mnt_root |
| `STATMOUNT_MNT_POINT` | `0x00000010U` | Want/got mnt_point |
| `STATMOUNT_MNT_NS_ID` | `0x00000040U` | Want/got mnt_ns_id |
| `STATMOUNT_MNT_OPTS` | `0x00000080U` | Want/got mnt_opts |
| `STATMOUNT_MNT_UIDMAP` | `0x00002000U` | Want/got uidmap... |
| `STATMOUNT_MNT_GIDMAP` | `0x00004000U` | Want/got gidmap... |

### STATMOUNT_OPT (2)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_OPT_ARRAY` | `0x00000400U` | Want/got opt_... |
| `STATMOUNT_OPT_SEC_ARRAY` | `0x00000800U` | Want/got opt_sec... |

### STATMOUNT_PROPAGATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_PROPAGATE_FROM` | `0x00000004U` | Want/got propagate_from |

### STATMOUNT_SB (2)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_SB_BASIC` | `0x00000001U` | Want/got sb_... |
| `STATMOUNT_SB_SOURCE` | `0x00000200U` | Want/got sb_source |

### STATMOUNT_SUPPORTED (1)

| Name | Value | Comment |
|------|-------|---------|
| `STATMOUNT_SUPPORTED_MASK` | `0x00001000U` | Want/got supported mask flags |

### UNCATEGORIZED (37)

| Name | Value | Comment |
|------|-------|---------|
| `MS_RDONLY` | `1` | Mount read-only |
| `MS_NOSUID` | `2` | Ignore suid and sgid bits |
| `MS_NODEV` | `4` | Disallow access to device special files |
| `MS_NOEXEC` | `8` | Disallow program execution |
| `MS_SYNCHRONOUS` | `16` | Writes are synced at once |
| `MS_REMOUNT` | `32` | Alter flags of a mounted FS |
| `MS_MANDLOCK` | `64` | Allow mandatory locks on an FS |
| `MS_DIRSYNC` | `128` | Directory modifications are synchronous |
| `MS_NOSYMFOLLOW` | `256` | Do not follow symlinks |
| `MS_NOATIME` | `1024` | Do not update access times. |
| `MS_NODIRATIME` | `2048` | Do not update directory access times |
| `MS_BIND` | `4096` |  |
| `MS_MOVE` | `8192` |  |
| `MS_REC` | `16384` |  |
| `MS_VERBOSE` | `32768	/* War is peace. Verbosity is silence.` |  |
| `MS_SILENT` | `32768` |  |
| `MS_POSIXACL` | `(1<<16)` | VFS does not apply the umask |
| `MS_UNBINDABLE` | `(1<<17)` | change to unbindable |
| `MS_PRIVATE` | `(1<<18)` | change to private |
| `MS_SLAVE` | `(1<<19)` | change to slave |
| `MS_SHARED` | `(1<<20)` | change to shared |
| `MS_RELATIME` | `(1<<21)` | Update atime relative to mtime/ctime. |
| `MS_KERNMOUNT` | `(1<<22)` | this is a kern_mount call |
| `MS_STRICTATIME` | `(1<<24)` | Always perform atime updates |
| `MS_LAZYTIME` | `(1<<25)` | Update the on-disk [acm]times lazily |
| `MS_SUBMOUNT` | `(1<<26)` |  |
| `MS_NOREMOTELOCK` | `(1<<27)` |  |
| `MS_NOSEC` | `(1<<28)` |  |
| `MS_BORN` | `(1<<29)` |  |
| `MS_ACTIVE` | `(1<<30)` |  |
| `MS_NOUSER` | `(1<<31)` |  |
| `FSOPEN_CLOEXEC` | `0x00000001` |  |
| `FSPICK_CLOEXEC` | `0x00000001` |  |
| `FSMOUNT_CLOEXEC` | `0x00000001` |  |
| `FSMOUNT_NAMESPACE` | `0x00000002` | Create the mount in a new mount namespace |
| `LSMT_ROOT` | `0xffffffffffffffff` | root mount |
| `LISTMOUNT_REVERSE` | `(1 << 0)` | List later mounts first |

## Structs (3)


### `struct mount_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `attr_set` | `-` |
| `__u64` | `attr_clr` | `-` |
| `__u64` | `propagation` | `-` |
| `__u64` | `userns_fd` | `-` |

### `struct statmount`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `mnt_opts` | `-` |
| `__u64` | `mask` | `-` |
| `__u32` | `sb_dev_major` | `-` |
| `__u32` | `sb_dev_minor` | `-` |
| `__u64` | `sb_magic` | `-` |
| `__u32` | `sb_flags` | `-` |
| `__u32` | `fs_type` | `-` |
| `__u64` | `mnt_id` | `-` |
| `__u64` | `mnt_parent_id` | `-` |
| `__u32` | `mnt_id_old` | `-` |
| `__u32` | `mnt_parent_id_old` | `-` |
| `__u64` | `mnt_attr` | `-` |
| `__u64` | `mnt_propagation` | `-` |
| `__u64` | `mnt_peer_group` | `-` |
| `__u64` | `mnt_master` | `-` |
| `__u64` | `propagate_from` | `-` |
| `__u32` | `mnt_root` | `-` |
| `__u32` | `mnt_point` | `-` |
| `__u64` | `mnt_ns_id` | `-` |
| `__u32` | `fs_subtype` | `-` |
| `__u32` | `sb_source` | `-` |
| `__u32` | `opt_num` | `-` |
| `__u32` | `opt_array` | `-` |
| `__u32` | `opt_sec_num` | `-` |
| `__u32` | `opt_sec_array` | `-` |
| `__u64` | `supported_mask` | `-` |
| `__u32` | `mnt_uidmap_num` | `-` |
| `__u32` | `mnt_uidmap` | `-` |
| `__u32` | `mnt_gidmap_num` | `-` |
| `__u32` | `mnt_gidmap` | `-` |
| `__u64` | `__spare2` | `43` |

### `struct mnt_id_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `mnt_ns_fd` | `-` |
| `__u32` | `mnt_fd` | `-` |
| `__u64` | `mnt_id` | `-` |
| `__u64` | `param` | `-` |
| `__u64` | `mnt_ns_id` | `-` |