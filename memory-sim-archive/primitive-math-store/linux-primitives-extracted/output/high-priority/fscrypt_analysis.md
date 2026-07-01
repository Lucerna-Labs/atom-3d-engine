# fscrypt.h

**Source:** `fscrypt.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (61 total)


### FSCRYPT_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED` | `0x00000001` |  |

### FSCRYPT_KEY (12)

| Name | Value | Comment |
|------|-------|---------|
| `FSCRYPT_KEY_DESCRIPTOR_SIZE` | `8` |  |
| `FSCRYPT_KEY_DESC_PREFIX` | `"fscrypt:"` |  |
| `FSCRYPT_KEY_DESC_PREFIX_SIZE` | `8` |  |
| `FSCRYPT_KEY_IDENTIFIER_SIZE` | `16` |  |
| `FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR` | `1` |  |
| `FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER` | `2` |  |
| `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY` | `0x00000001` |  |
| `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS` | `0x00000002` |  |
| `FSCRYPT_KEY_STATUS_ABSENT` | `1` |  |
| `FSCRYPT_KEY_STATUS_PRESENT` | `2` |  |
| `FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED` | `3` |  |
| `FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF` | `0x00000001` |  |

### FSCRYPT_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FSCRYPT_MAX_KEY_SIZE` | `64` |  |

### FSCRYPT_MODE (8)

| Name | Value | Comment |
|------|-------|---------|
| `FSCRYPT_MODE_AES_256_XTS` | `1` |  |
| `FSCRYPT_MODE_AES_256_CTS` | `4` |  |
| `FSCRYPT_MODE_AES_128_CBC` | `5` |  |
| `FSCRYPT_MODE_AES_128_CTS` | `6` |  |
| `FSCRYPT_MODE_SM4_XTS` | `7` |  |
| `FSCRYPT_MODE_SM4_CTS` | `8` |  |
| `FSCRYPT_MODE_ADIANTUM` | `9` |  |
| `FSCRYPT_MODE_AES_256_HCTR2` | `10` |  |

### FSCRYPT_POLICY (10)

| Name | Value | Comment |
|------|-------|---------|
| `FSCRYPT_POLICY_FLAGS_PAD_4` | `0x00` |  |
| `FSCRYPT_POLICY_FLAGS_PAD_8` | `0x01` |  |
| `FSCRYPT_POLICY_FLAGS_PAD_16` | `0x02` |  |
| `FSCRYPT_POLICY_FLAGS_PAD_32` | `0x03` |  |
| `FSCRYPT_POLICY_FLAGS_PAD_MASK` | `0x03` |  |
| `FSCRYPT_POLICY_FLAG_DIRECT_KEY` | `0x04` |  |
| `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64` | `0x08` |  |
| `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32` | `0x10` |  |
| `FSCRYPT_POLICY_V1` | `0` |  |
| `FSCRYPT_POLICY_V2` | `2` |  |

### FS_ENCRYPTION (8)

| Name | Value | Comment |
|------|-------|---------|
| `FS_ENCRYPTION_MODE_INVALID` | `0` | never used |
| `FS_ENCRYPTION_MODE_AES_256_XTS` | `FSCRYPT_MODE_AES_256_XTS` |  |
| `FS_ENCRYPTION_MODE_AES_256_GCM` | `2` | never used |
| `FS_ENCRYPTION_MODE_AES_256_CBC` | `3` | never used |
| `FS_ENCRYPTION_MODE_AES_256_CTS` | `FSCRYPT_MODE_AES_256_CTS` |  |
| `FS_ENCRYPTION_MODE_AES_128_CBC` | `FSCRYPT_MODE_AES_128_CBC` |  |
| `FS_ENCRYPTION_MODE_AES_128_CTS` | `FSCRYPT_MODE_AES_128_CTS` |  |
| `FS_ENCRYPTION_MODE_ADIANTUM` | `FSCRYPT_MODE_ADIANTUM` |  |

### FS_IOC (9)

| Name | Value | Comment |
|------|-------|---------|
| `FS_IOC_SET_ENCRYPTION_POLICY` | `_IOR('f', 19, struct fscrypt_policy_v1)` |  |
| `FS_IOC_GET_ENCRYPTION_PWSALT` | `_IOW('f', 20, __u8[16])` |  |
| `FS_IOC_GET_ENCRYPTION_POLICY` | `_IOW('f', 21, struct fscrypt_policy_v1)` |  |
| `FS_IOC_GET_ENCRYPTION_POLICY_EX` | `_IOWR('f', 22, __u8[9])` | size + version |
| `FS_IOC_ADD_ENCRYPTION_KEY` | `_IOWR('f', 23, struct fscrypt_add_key_arg)` |  |
| `FS_IOC_REMOVE_ENCRYPTION_KEY` | `_IOWR('f', 24, struct fscrypt_remove_key_arg)` |  |
| `FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS` | `_IOWR('f', 25, struct fscrypt_remove_key_arg)` |  |
| `FS_IOC_GET_ENCRYPTION_KEY_STATUS` | `_IOWR('f', 26, struct fscrypt_get_key_status_arg)` |  |
| `FS_IOC_GET_ENCRYPTION_NONCE` | `_IOR('f', 27, __u8[16])` |  |

### FS_KEY (3)

| Name | Value | Comment |
|------|-------|---------|
| `FS_KEY_DESCRIPTOR_SIZE` | `FSCRYPT_KEY_DESCRIPTOR_SIZE` |  |
| `FS_KEY_DESC_PREFIX` | `FSCRYPT_KEY_DESC_PREFIX` |  |
| `FS_KEY_DESC_PREFIX_SIZE` | `FSCRYPT_KEY_DESC_PREFIX_SIZE` |  |

### FS_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FS_MAX_KEY_SIZE` | `FSCRYPT_MAX_KEY_SIZE` |  |

### FS_POLICY (7)

| Name | Value | Comment |
|------|-------|---------|
| `FS_POLICY_FLAGS_PAD_4` | `FSCRYPT_POLICY_FLAGS_PAD_4` |  |
| `FS_POLICY_FLAGS_PAD_8` | `FSCRYPT_POLICY_FLAGS_PAD_8` |  |
| `FS_POLICY_FLAGS_PAD_16` | `FSCRYPT_POLICY_FLAGS_PAD_16` |  |
| `FS_POLICY_FLAGS_PAD_32` | `FSCRYPT_POLICY_FLAGS_PAD_32` |  |
| `FS_POLICY_FLAGS_PAD_MASK` | `FSCRYPT_POLICY_FLAGS_PAD_MASK` |  |
| `FS_POLICY_FLAG_DIRECT_KEY` | `FSCRYPT_POLICY_FLAG_DIRECT_KEY` |  |
| `FS_POLICY_FLAGS_VALID` | `0x07` | contains old flags only |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `fscrypt_policy` | `fscrypt_policy_v1` |  |

## Structs (9)


### `struct fscrypt_policy_v1`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `version` | `-` |
| `__u8` | `contents_encryption_mode` | `-` |
| `__u8` | `filenames_encryption_mode` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `master_key_descriptor` | `FSCRYPT_KEY_DESCRIPTOR_SIZE` |

### `struct fscrypt_key`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mode` | `-` |
| `__u8` | `raw` | `FSCRYPT_MAX_KEY_SIZE` |
| `__u32` | `size` | `-` |

### `struct fscrypt_policy_v2`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `version` | `-` |
| `__u8` | `contents_encryption_mode` | `-` |
| `__u8` | `filenames_encryption_mode` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `log2_data_unit_size` | `-` |
| `__u8` | `__reserved` | `3` |
| `__u8` | `master_key_identifier` | `FSCRYPT_KEY_IDENTIFIER_SIZE` |

### `struct fscrypt_get_policy_ex_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `policy_size` | `-` |
| `__u8` | `version` | `-` |

### `struct fscrypt_key_specifier`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `__reserved` | `-` |
| `__u8` | `__reserved` | `32` |
| `__u8` | `descriptor` | `FSCRYPT_KEY_DESCRIPTOR_SIZE` |
| `__u8` | `identifier` | `FSCRYPT_KEY_IDENTIFIER_SIZE` |

### `struct fscrypt_provisioning_key_payload`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `flags` | `-` |

### `struct fscrypt_add_key_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `raw_size` | `-` |
| `__u32` | `key_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `__reserved` | `7` |

### `struct fscrypt_remove_key_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `removal_status_flags` | `-` |
| `__u32` | `__reserved` | `5` |

### `struct fscrypt_get_key_status_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `__reserved` | `6` |
| `__u32` | `status` | `-` |
| `__u32` | `status_flags` | `-` |
| `__u32` | `user_count` | `-` |
| `__u32` | `__out_reserved` | `13` |