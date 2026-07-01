# loop.h

**Source:** `loop.h`


## Includes

- `asm/posix_types.h`
- `linux/types.h`

## Defines (30 total)


### LOOP_CHANGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_CHANGE_FD` | `0x4C06` |  |

### LOOP_CLR (1)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_CLR_FD` | `0x4C01` |  |

### LOOP_CONFIGURE (1)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_CONFIGURE_SETTABLE_FLAGS` | `(LO_FLAGS_READ_ONLY \| LO_FLAGS_AUTOCLEAR ` |  |

### LOOP_CTL (3)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_CTL_ADD` | `0x4C80` |  |
| `LOOP_CTL_REMOVE` | `0x4C81` |  |
| `LOOP_CTL_GET_FREE` | `0x4C82` |  |

### LOOP_GET (2)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_GET_STATUS` | `0x4C03` |  |
| `LOOP_GET_STATUS64` | `0x4C05` |  |

### LOOP_SET (8)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_SET_STATUS_SETTABLE_FLAGS` | `(LO_FLAGS_AUTOCLEAR \| LO_FLAGS_PARTSCAN)` |  |
| `LOOP_SET_STATUS_CLEARABLE_FLAGS` | `(LO_FLAGS_AUTOCLEAR)` |  |
| `LOOP_SET_FD` | `0x4C00` |  |
| `LOOP_SET_STATUS` | `0x4C02` |  |
| `LOOP_SET_STATUS64` | `0x4C04` |  |
| `LOOP_SET_CAPACITY` | `0x4C07` |  |
| `LOOP_SET_DIRECT_IO` | `0x4C08` |  |
| `LOOP_SET_BLOCK_SIZE` | `0x4C09` |  |

### LO_CRYPT (10)

| Name | Value | Comment |
|------|-------|---------|
| `LO_CRYPT_NONE` | `0` |  |
| `LO_CRYPT_XOR` | `1` |  |
| `LO_CRYPT_DES` | `2` |  |
| `LO_CRYPT_FISH2` | `3` | Twofish encryption |
| `LO_CRYPT_BLOW` | `4` |  |
| `LO_CRYPT_CAST128` | `5` |  |
| `LO_CRYPT_IDEA` | `6` |  |
| `LO_CRYPT_DUMMY` | `9` |  |
| `LO_CRYPT_SKIPJACK` | `10` |  |
| `LO_CRYPT_CRYPTOAPI` | `18` |  |

### LO_KEY (1)

| Name | Value | Comment |
|------|-------|---------|
| `LO_KEY_SIZE` | `32` |  |

### LO_NAME (1)

| Name | Value | Comment |
|------|-------|---------|
| `LO_NAME_SIZE` | `64` |  |

### MAX_LO (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_LO_CRYPT` | `20` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `LOOP_CONFIGURE` | `0x4C0A` |  |

## Structs (3)


### `struct loop_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `lo_number` | `-` |
| `__kernel_old_dev_t` | `lo_device` | `-` |
| `__kernel_old_dev_t` | `lo_rdevice` | `-` |
| `int` | `lo_offset` | `-` |
| `int` | `lo_encrypt_type` | `-` |
| `int` | `lo_encrypt_key_size` | `-` |
| `int` | `lo_flags` | `-` |
| `char` | `lo_name` | `LO_NAME_SIZE` |
| `char` | `reserved` | `4` |

### `struct loop_info64`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `lo_device` | `-` |
| `__u64` | `lo_inode` | `-` |
| `__u64` | `lo_rdevice` | `-` |
| `__u64` | `lo_offset` | `-` |
| `__u64` | `lo_sizelimit` | `-` |
| `__u32` | `lo_number` | `-` |
| `__u32` | `lo_encrypt_type` | `-` |
| `__u32` | `lo_encrypt_key_size` | `-` |
| `__u32` | `lo_flags` | `-` |
| `__u8` | `lo_file_name` | `LO_NAME_SIZE` |
| `__u8` | `lo_crypt_name` | `LO_NAME_SIZE` |
| `__u8` | `lo_encrypt_key` | `LO_KEY_SIZE` |
| `__u64` | `lo_init` | `2` |

### `struct loop_config`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fd` | `-` |
| `__u32` | `block_size` | `-` |
| `__u64` | `__reserved` | `8` |