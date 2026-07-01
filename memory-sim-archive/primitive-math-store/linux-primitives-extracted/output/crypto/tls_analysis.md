# tls.h

**Source:** `tls.h`


## Includes

- `linux/types.h`

## Defines (66 total)


### TLS_CIPHER (48)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_CIPHER_AES_GCM_128` | `51` |  |
| `TLS_CIPHER_AES_GCM_128_IV_SIZE` | `8` |  |
| `TLS_CIPHER_AES_GCM_128_KEY_SIZE` | `16` |  |
| `TLS_CIPHER_AES_GCM_128_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_AES_GCM_128_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_AES_GCM_256` | `52` |  |
| `TLS_CIPHER_AES_GCM_256_IV_SIZE` | `8` |  |
| `TLS_CIPHER_AES_GCM_256_KEY_SIZE` | `32` |  |
| `TLS_CIPHER_AES_GCM_256_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_AES_GCM_256_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_AES_CCM_128` | `53` |  |
| `TLS_CIPHER_AES_CCM_128_IV_SIZE` | `8` |  |
| `TLS_CIPHER_AES_CCM_128_KEY_SIZE` | `16` |  |
| `TLS_CIPHER_AES_CCM_128_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_AES_CCM_128_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_CHACHA20_POLY1305` | `54` |  |
| `TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE` | `12` |  |
| `TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE` | `32` |  |
| `TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE` | `0` |  |
| `TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_SM4_GCM` | `55` |  |
| `TLS_CIPHER_SM4_GCM_IV_SIZE` | `8` |  |
| `TLS_CIPHER_SM4_GCM_KEY_SIZE` | `16` |  |
| `TLS_CIPHER_SM4_GCM_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_SM4_GCM_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_SM4_CCM` | `56` |  |
| `TLS_CIPHER_SM4_CCM_IV_SIZE` | `8` |  |
| `TLS_CIPHER_SM4_CCM_KEY_SIZE` | `16` |  |
| `TLS_CIPHER_SM4_CCM_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_SM4_CCM_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_ARIA_GCM_128` | `57` |  |
| `TLS_CIPHER_ARIA_GCM_128_IV_SIZE` | `8` |  |
| `TLS_CIPHER_ARIA_GCM_128_KEY_SIZE` | `16` |  |
| `TLS_CIPHER_ARIA_GCM_128_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_ARIA_GCM_128_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE` | `8` |  |
| `TLS_CIPHER_ARIA_GCM_256` | `58` |  |
| `TLS_CIPHER_ARIA_GCM_256_IV_SIZE` | `8` |  |
| `TLS_CIPHER_ARIA_GCM_256_KEY_SIZE` | `32` |  |
| `TLS_CIPHER_ARIA_GCM_256_SALT_SIZE` | `4` |  |
| `TLS_CIPHER_ARIA_GCM_256_TAG_SIZE` | `16` |  |
| `TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE` | `8` |  |

### TLS_CONF (4)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_CONF_BASE` | `1` |  |
| `TLS_CONF_SW` | `2` |  |
| `TLS_CONF_HW` | `3` |  |
| `TLS_CONF_HW_RECORD` | `4` |  |

### TLS_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_GET_RECORD_TYPE` | `2` |  |

### TLS_INFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_INFO_MAX` | `(__TLS_INFO_MAX - 1)` |  |

### TLS_RX (1)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_RX_EXPECT_NO_PAD` | `4` | Attempt opportunistic zero-copy |

### TLS_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_SET_RECORD_TYPE` | `1` |  |

### TLS_TX (2)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_TX_ZEROCOPY_RO` | `3` | TX zerocopy (only sendfile now) |
| `TLS_TX_MAX_PAYLOAD_LEN` | `5` | Maximum plaintext size |

### UNCATEGORIZED (8)

| Name | Value | Comment |
|------|-------|---------|
| `TLS_TX` | `1` | Set transmit parameters |
| `TLS_RX` | `2` | Set receive parameters |
| `TLS_1_2_VERSION_MAJOR` | `0x3` |  |
| `TLS_1_2_VERSION_MINOR` | `0x3` |  |
| `TLS_1_2_VERSION` | `TLS_VERSION_NUMBER(TLS_1_2)` |  |
| `TLS_1_3_VERSION_MAJOR` | `0x3` |  |
| `TLS_1_3_VERSION_MINOR` | `0x4` |  |
| `TLS_1_3_VERSION` | `TLS_VERSION_NUMBER(TLS_1_3)` |  |

## Structs (9)


### `struct tls_crypto_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `version` | `-` |
| `__u16` | `cipher_type` | `-` |

### `struct tls12_crypto_info_aes_gcm_128`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_aes_gcm_256`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_aes_ccm_128`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_chacha20_poly1305`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_sm4_gcm`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_sm4_ccm`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_aria_gcm_128`

| Type | Field | Array |
|------|-------|-------|

### `struct tls12_crypto_info_aria_gcm_256`

| Type | Field | Array |
|------|-------|-------|