# blk-crypto.h

**Source:** `blk-crypto.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (3 total)


### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `BLKCRYPTOIMPORTKEY` | `_IOWR(0x12, 137, struct blk_crypto_import_key_arg)` |  |
| `BLKCRYPTOGENERATEKEY` | `_IOWR(0x12, 138, struct blk_crypto_generate_key_arg)` |  |
| `BLKCRYPTOPREPAREKEY` | `_IOWR(0x12, 139, struct blk_crypto_prepare_key_arg)` |  |

## Structs (3)


### `struct blk_crypto_import_key_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `raw_key_ptr` | `-` |
| `__u64` | `raw_key_size` | `-` |
| `__u64` | `lt_key_ptr` | `-` |
| `__u64` | `lt_key_size` | `-` |
| `__u64` | `reserved` | `4` |

### `struct blk_crypto_generate_key_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `lt_key_ptr` | `-` |
| `__u64` | `lt_key_size` | `-` |
| `__u64` | `reserved` | `4` |

### `struct blk_crypto_prepare_key_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `lt_key_ptr` | `-` |
| `__u64` | `lt_key_size` | `-` |
| `__u64` | `eph_key_ptr` | `-` |
| `__u64` | `eph_key_size` | `-` |
| `__u64` | `reserved` | `4` |