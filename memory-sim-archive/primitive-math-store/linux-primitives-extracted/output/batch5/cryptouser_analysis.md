# cryptouser.h

**Source:** `cryptouser.h`


## Includes

- `linux/types.h`

## Defines (5 total)


### CRYPTO_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRYPTO_MAX_NAME` | `64` |  |

### CRYPTO_MSG (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRYPTO_MSG_MAX` | `(__CRYPTO_MSG_MAX - 1)` |  |

### CRYPTO_NR (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRYPTO_NR_MSGTYPES` | `(CRYPTO_MSG_MAX + 1 - CRYPTO_MSG_BASE)` |  |

### CRYPTO_REPORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRYPTO_REPORT_MAXSIZE` | `(sizeof(struct crypto_user_alg) + ` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `CRYPTOCFGA_MAX` | `(__CRYPTOCFGA_MAX - 1)` |  |

## Structs (20)


### `struct crypto_user_alg`

| Type | Field | Array |
|------|-------|-------|
| `char` | `cru_name` | `CRYPTO_MAX_NAME` |
| `char` | `cru_driver_name` | `CRYPTO_MAX_NAME` |
| `char` | `cru_module_name` | `CRYPTO_MAX_NAME` |
| `__u32` | `cru_type` | `-` |
| `__u32` | `cru_mask` | `-` |
| `__u32` | `cru_refcnt` | `-` |
| `__u32` | `cru_flags` | `-` |

### `struct crypto_stat_aead`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_encrypt_cnt` | `-` |
| `__u64` | `stat_encrypt_tlen` | `-` |
| `__u64` | `stat_decrypt_cnt` | `-` |
| `__u64` | `stat_decrypt_tlen` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_akcipher`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_encrypt_cnt` | `-` |
| `__u64` | `stat_encrypt_tlen` | `-` |
| `__u64` | `stat_decrypt_cnt` | `-` |
| `__u64` | `stat_decrypt_tlen` | `-` |
| `__u64` | `stat_verify_cnt` | `-` |
| `__u64` | `stat_sign_cnt` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_cipher`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_encrypt_cnt` | `-` |
| `__u64` | `stat_encrypt_tlen` | `-` |
| `__u64` | `stat_decrypt_cnt` | `-` |
| `__u64` | `stat_decrypt_tlen` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_compress`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_compress_cnt` | `-` |
| `__u64` | `stat_compress_tlen` | `-` |
| `__u64` | `stat_decompress_cnt` | `-` |
| `__u64` | `stat_decompress_tlen` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_hash`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_hash_cnt` | `-` |
| `__u64` | `stat_hash_tlen` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_kpp`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_setsecret_cnt` | `-` |
| `__u64` | `stat_generate_public_key_cnt` | `-` |
| `__u64` | `stat_compute_shared_secret_cnt` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_rng`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `__u64` | `stat_generate_cnt` | `-` |
| `__u64` | `stat_generate_tlen` | `-` |
| `__u64` | `stat_seed_cnt` | `-` |
| `__u64` | `stat_err_cnt` | `-` |

### `struct crypto_stat_larval`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_larval`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_hash`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_cipher`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_blkcipher`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `char` | `geniv` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_aead`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |
| `char` | `geniv` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_comp`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_rng`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_akcipher`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_kpp`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_acomp`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |

### `struct crypto_report_sig`

| Type | Field | Array |
|------|-------|-------|
| `char` | `type` | `CRYPTO_MAX_NAME` |