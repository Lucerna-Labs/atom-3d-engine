# fsverity.h

**Source:** `fsverity.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (8 total)


### FS_IOC (3)

| Name | Value | Comment |
|------|-------|---------|
| `FS_IOC_ENABLE_VERITY` | `_IOW('f', 133, struct fsverity_enable_arg)` |  |
| `FS_IOC_MEASURE_VERITY` | `_IOWR('f', 134, struct fsverity_digest)` |  |
| `FS_IOC_READ_VERITY_METADATA` | `` |  |

### FS_VERITY (5)

| Name | Value | Comment |
|------|-------|---------|
| `FS_VERITY_HASH_ALG_SHA256` | `1` |  |
| `FS_VERITY_HASH_ALG_SHA512` | `2` |  |
| `FS_VERITY_METADATA_TYPE_MERKLE_TREE` | `1` |  |
| `FS_VERITY_METADATA_TYPE_DESCRIPTOR` | `2` |  |
| `FS_VERITY_METADATA_TYPE_SIGNATURE` | `3` |  |

## Structs (5)


### `struct fsverity_enable_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `-` |
| `__u32` | `hash_algorithm` | `-` |
| `__u32` | `block_size` | `-` |
| `__u32` | `salt_size` | `-` |
| `__u64` | `salt_ptr` | `-` |
| `__u32` | `sig_size` | `-` |
| `__u32` | `__reserved1` | `-` |
| `__u64` | `sig_ptr` | `-` |
| `__u64` | `__reserved2` | `11` |

### `struct fsverity_digest`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `digest_algorithm` | `-` |
| `__u16` | `digest_size` | `-` |

### `struct fsverity_descriptor`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `version` | `-` |
| `__u8` | `hash_algorithm` | `-` |
| `__u8` | `log_blocksize` | `-` |
| `__u8` | `salt_size` | `-` |
| `__le32` | `sig_size` | `-` |
| `__le32` | `__reserved_0x04` | `-` |
| `__le64` | `data_size` | `-` |
| `__u8` | `root_hash` | `64` |
| `__u8` | `salt` | `32` |
| `__u8` | `__reserved` | `144` |

### `struct fsverity_formatted_digest`

| Type | Field | Array |
|------|-------|-------|
| `char` | `magic` | `8` |
| `__le16` | `digest_algorithm` | `-` |
| `__le16` | `digest_size` | `-` |

### `struct fsverity_read_metadata_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `metadata_type` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `length` | `-` |
| `__u64` | `buf_ptr` | `-` |
| `__u64` | `__reserved` | `-` |