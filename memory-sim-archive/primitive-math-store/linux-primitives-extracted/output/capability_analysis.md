# capability.h

**Source:** `capability.h`


## Includes

- `linux/types.h`

## Defines (66 total)


### CAP_AUDIT (3)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_AUDIT_WRITE` | `29` |  |
| `CAP_AUDIT_CONTROL` | `30` |  |
| `CAP_AUDIT_READ` | `37` |  |

### CAP_BLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_BLOCK_SUSPEND` | `36` |  |

### CAP_CHECKPOINT (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_CHECKPOINT_RESTORE` | `40` |  |

### CAP_DAC (2)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_DAC_OVERRIDE` | `1` |  |
| `CAP_DAC_READ_SEARCH` | `2` |  |

### CAP_IPC (2)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_IPC_LOCK` | `14` |  |
| `CAP_IPC_OWNER` | `15` |  |

### CAP_LAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_LAST_CAP` | `CAP_CHECKPOINT_RESTORE` |  |

### CAP_LINUX (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_LINUX_IMMUTABLE` | `9` |  |

### CAP_MAC (2)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_MAC_OVERRIDE` | `32` |  |
| `CAP_MAC_ADMIN` | `33` |  |

### CAP_NET (4)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_NET_BIND_SERVICE` | `10` |  |
| `CAP_NET_BROADCAST` | `11` |  |
| `CAP_NET_ADMIN` | `12` |  |
| `CAP_NET_RAW` | `13` |  |

### CAP_SYS (11)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_SYS_MODULE` | `16` |  |
| `CAP_SYS_RAWIO` | `17` |  |
| `CAP_SYS_CHROOT` | `18` |  |
| `CAP_SYS_PTRACE` | `19` |  |
| `CAP_SYS_PACCT` | `20` |  |
| `CAP_SYS_ADMIN` | `21` |  |
| `CAP_SYS_BOOT` | `22` |  |
| `CAP_SYS_NICE` | `23` |  |
| `CAP_SYS_RESOURCE` | `24` |  |
| `CAP_SYS_TIME` | `25` |  |
| `CAP_SYS_TTY_CONFIG` | `26` |  |

### CAP_WAKE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAP_WAKE_ALARM` | `35` |  |

### UNCATEGORIZED (21)

| Name | Value | Comment |
|------|-------|---------|
| `_LINUX_CAPABILITY_VERSION_1` | `0x19980330` |  |
| `_LINUX_CAPABILITY_U32S_1` | `1` |  |
| `_LINUX_CAPABILITY_VERSION_2` | `0x20071026` | deprecated - use v3 |
| `_LINUX_CAPABILITY_U32S_2` | `2` |  |
| `_LINUX_CAPABILITY_VERSION_3` | `0x20080522` |  |
| `_LINUX_CAPABILITY_U32S_3` | `2` |  |
| `_LINUX_CAPABILITY_VERSION` | `_LINUX_CAPABILITY_VERSION_1` |  |
| `_LINUX_CAPABILITY_U32S` | `_LINUX_CAPABILITY_U32S_1` |  |
| `CAP_CHOWN` | `0` |  |
| `CAP_FOWNER` | `3` |  |
| `CAP_FSETID` | `4` |  |
| `CAP_KILL` | `5` |  |
| `CAP_SETGID` | `6` |  |
| `CAP_SETUID` | `7` |  |
| `CAP_SETPCAP` | `8` |  |
| `CAP_MKNOD` | `27` |  |
| `CAP_LEASE` | `28` |  |
| `CAP_SETFCAP` | `31` |  |
| `CAP_SYSLOG` | `34` |  |
| `CAP_PERFMON` | `38` |  |
| `CAP_BPF` | `39` |  |

### VFS_CAP (12)

| Name | Value | Comment |
|------|-------|---------|
| `VFS_CAP_REVISION_MASK` | `0xFF000000` |  |
| `VFS_CAP_REVISION_SHIFT` | `24` |  |
| `VFS_CAP_FLAGS_MASK` | `~VFS_CAP_REVISION_MASK` |  |
| `VFS_CAP_FLAGS_EFFECTIVE` | `0x000001` |  |
| `VFS_CAP_REVISION_1` | `0x01000000` |  |
| `VFS_CAP_U32_1` | `1` |  |
| `VFS_CAP_REVISION_2` | `0x02000000` |  |
| `VFS_CAP_U32_2` | `2` |  |
| `VFS_CAP_REVISION_3` | `0x03000000` |  |
| `VFS_CAP_U32_3` | `2` |  |
| `VFS_CAP_U32` | `VFS_CAP_U32_3` |  |
| `VFS_CAP_REVISION` | `VFS_CAP_REVISION_3` |  |

### XATTR_CAPS (4)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_CAPS_SZ_1` | `(sizeof(__le32)*(1 + 2*VFS_CAP_U32_1))` |  |
| `XATTR_CAPS_SZ_2` | `(sizeof(__le32)*(1 + 2*VFS_CAP_U32_2))` |  |
| `XATTR_CAPS_SZ_3` | `(sizeof(__le32)*(2 + 2*VFS_CAP_U32_3))` |  |
| `XATTR_CAPS_SZ` | `XATTR_CAPS_SZ_3` |  |

## Structs (6)


### `struct __user_cap_header_struct`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `-` |
| `int` | `pid` | `-` |

### `struct __user_cap_data_struct`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `effective` | `-` |
| `__u32` | `permitted` | `-` |
| `__u32` | `inheritable` | `-` |

### `struct vfs_cap_data`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `magic_etc` | `-` |
| `__le32` | `permitted` | `-` |
| `__le32` | `inheritable` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `permitted` | `-` |
| `__le32` | `inheritable` | `-` |

### `struct vfs_ns_cap_data`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `magic_etc` | `-` |
| `__le32` | `permitted` | `-` |
| `__le32` | `inheritable` | `-` |
| `__le32` | `rootid` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `permitted` | `-` |
| `__le32` | `inheritable` | `-` |

## Typedefs

- `__user_cap_header_struct`
- `__user`