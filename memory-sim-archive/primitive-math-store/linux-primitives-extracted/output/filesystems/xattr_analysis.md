# xattr.h

**Source:** `xattr.h`


## Includes

- `linux/libc-compat.h`
- `linux/types.h`

## Defines (47 total)


### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_CREATE` | `0x1` | set value, fail if attr already exists |
| `XATTR_REPLACE` | `0x2` | set value, fail if attr does not exist |
| `XATTR_OS2_PREFIX` | `"os2."` |  |
| `XATTR_OS2_PREFIX_LEN` | `(sizeof(XATTR_OS2_PREFIX) - 1)` |  |

### XATTR_APPARMOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_APPARMOR_SUFFIX` | `"apparmor"` |  |

### XATTR_BPF (1)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_BPF_LSM_SUFFIX` | `"bpf."` |  |

### XATTR_BTRFS (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_BTRFS_PREFIX` | `"btrfs."` |  |
| `XATTR_BTRFS_PREFIX_LEN` | `(sizeof(XATTR_BTRFS_PREFIX) - 1)` |  |

### XATTR_CAPS (1)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_CAPS_SUFFIX` | `"capability"` |  |

### XATTR_EVM (1)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_EVM_SUFFIX` | `"evm"` |  |

### XATTR_HURD (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_HURD_PREFIX` | `"gnu."` |  |
| `XATTR_HURD_PREFIX_LEN` | `(sizeof(XATTR_HURD_PREFIX) - 1)` |  |

### XATTR_IMA (1)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_IMA_SUFFIX` | `"ima"` |  |

### XATTR_MAC (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_MAC_OSX_PREFIX` | `"osx."` |  |
| `XATTR_MAC_OSX_PREFIX_LEN` | `(sizeof(XATTR_MAC_OSX_PREFIX) - 1)` |  |

### XATTR_NAME (15)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_NAME_EVM` | `XATTR_SECURITY_PREFIX XATTR_EVM_SUFFIX` |  |
| `XATTR_NAME_IMA` | `XATTR_SECURITY_PREFIX XATTR_IMA_SUFFIX` |  |
| `XATTR_NAME_SELINUX` | `XATTR_SECURITY_PREFIX XATTR_SELINUX_SUFFIX` |  |
| `XATTR_NAME_SMACK` | `XATTR_SECURITY_PREFIX XATTR_SMACK_SUFFIX` |  |
| `XATTR_NAME_SMACKIPIN` | `XATTR_SECURITY_PREFIX XATTR_SMACK_IPIN` |  |
| `XATTR_NAME_SMACKIPOUT` | `XATTR_SECURITY_PREFIX XATTR_SMACK_IPOUT` |  |
| `XATTR_NAME_SMACKEXEC` | `XATTR_SECURITY_PREFIX XATTR_SMACK_EXEC` |  |
| `XATTR_NAME_SMACKTRANSMUTE` | `XATTR_SECURITY_PREFIX XATTR_SMACK_TRANSMUTE` |  |
| `XATTR_NAME_SMACKMMAP` | `XATTR_SECURITY_PREFIX XATTR_SMACK_MMAP` |  |
| `XATTR_NAME_APPARMOR` | `XATTR_SECURITY_PREFIX XATTR_APPARMOR_SUFFIX` |  |
| `XATTR_NAME_CAPS` | `XATTR_SECURITY_PREFIX XATTR_CAPS_SUFFIX` |  |
| `XATTR_NAME_BPF_LSM` | `(XATTR_SECURITY_PREFIX XATTR_BPF_LSM_SUFFIX)` |  |
| `XATTR_NAME_BPF_LSM_LEN` | `(sizeof(XATTR_NAME_BPF_LSM) - 1)` |  |
| `XATTR_NAME_POSIX_ACL_ACCESS` | `XATTR_SYSTEM_PREFIX XATTR_POSIX_ACL_ACCESS` |  |
| `XATTR_NAME_POSIX_ACL_DEFAULT` | `XATTR_SYSTEM_PREFIX XATTR_POSIX_ACL_DEFAULT` |  |

### XATTR_POSIX (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_POSIX_ACL_ACCESS` | `"posix_acl_access"` |  |
| `XATTR_POSIX_ACL_DEFAULT` | `"posix_acl_default"` |  |

### XATTR_SECURITY (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_SECURITY_PREFIX` | `"security."` |  |
| `XATTR_SECURITY_PREFIX_LEN` | `(sizeof(XATTR_SECURITY_PREFIX) - 1)` |  |

### XATTR_SELINUX (1)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_SELINUX_SUFFIX` | `"selinux"` |  |

### XATTR_SMACK (6)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_SMACK_SUFFIX` | `"SMACK64"` |  |
| `XATTR_SMACK_IPIN` | `"SMACK64IPIN"` |  |
| `XATTR_SMACK_IPOUT` | `"SMACK64IPOUT"` |  |
| `XATTR_SMACK_EXEC` | `"SMACK64EXEC"` |  |
| `XATTR_SMACK_TRANSMUTE` | `"SMACK64TRANSMUTE"` |  |
| `XATTR_SMACK_MMAP` | `"SMACK64MMAP"` |  |

### XATTR_SYSTEM (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_SYSTEM_PREFIX` | `"system."` |  |
| `XATTR_SYSTEM_PREFIX_LEN` | `(sizeof(XATTR_SYSTEM_PREFIX) - 1)` |  |

### XATTR_TRUSTED (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_TRUSTED_PREFIX` | `"trusted."` |  |
| `XATTR_TRUSTED_PREFIX_LEN` | `(sizeof(XATTR_TRUSTED_PREFIX) - 1)` |  |

### XATTR_USER (2)

| Name | Value | Comment |
|------|-------|---------|
| `XATTR_USER_PREFIX` | `"user."` |  |
| `XATTR_USER_PREFIX_LEN` | `(sizeof(XATTR_USER_PREFIX) - 1)` |  |

## Structs (1)


### `struct xattr_args`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `value` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |