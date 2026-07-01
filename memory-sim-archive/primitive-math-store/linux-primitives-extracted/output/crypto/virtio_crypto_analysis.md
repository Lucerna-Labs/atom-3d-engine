# virtio_crypto.h

**Source:** `virtio_crypto.h`


## Includes

- `linux/types.h`
- `linux/virtio_types.h`
- `linux/virtio_ids.h`
- `linux/virtio_config.h`

## Defines (115 total)


### VIRTIO_CRYPTO (115)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_CRYPTO_SERVICE_CIPHER` | `0` |  |
| `VIRTIO_CRYPTO_SERVICE_HASH` | `1` |  |
| `VIRTIO_CRYPTO_SERVICE_MAC` | `2` |  |
| `VIRTIO_CRYPTO_SERVICE_AEAD` | `3` |  |
| `VIRTIO_CRYPTO_SERVICE_AKCIPHER` | `4` |  |
| `VIRTIO_CRYPTO_CIPHER_CREATE_SESSION` | `` |  |
| `VIRTIO_CRYPTO_CIPHER_DESTROY_SESSION` | `` |  |
| `VIRTIO_CRYPTO_HASH_CREATE_SESSION` | `` |  |
| `VIRTIO_CRYPTO_HASH_DESTROY_SESSION` | `` |  |
| `VIRTIO_CRYPTO_MAC_CREATE_SESSION` | `` |  |
| `VIRTIO_CRYPTO_MAC_DESTROY_SESSION` | `` |  |
| `VIRTIO_CRYPTO_AEAD_CREATE_SESSION` | `` |  |
| `VIRTIO_CRYPTO_AEAD_DESTROY_SESSION` | `` |  |
| `VIRTIO_CRYPTO_AKCIPHER_CREATE_SESSION` | `` |  |
| `VIRTIO_CRYPTO_AKCIPHER_DESTROY_SESSION` | `` |  |
| `VIRTIO_CRYPTO_NO_CIPHER` | `0` |  |
| `VIRTIO_CRYPTO_CIPHER_ARC4` | `1` |  |
| `VIRTIO_CRYPTO_CIPHER_AES_ECB` | `2` |  |
| `VIRTIO_CRYPTO_CIPHER_AES_CBC` | `3` |  |
| `VIRTIO_CRYPTO_CIPHER_AES_CTR` | `4` |  |
| `VIRTIO_CRYPTO_CIPHER_DES_ECB` | `5` |  |
| `VIRTIO_CRYPTO_CIPHER_DES_CBC` | `6` |  |
| `VIRTIO_CRYPTO_CIPHER_3DES_ECB` | `7` |  |
| `VIRTIO_CRYPTO_CIPHER_3DES_CBC` | `8` |  |
| `VIRTIO_CRYPTO_CIPHER_3DES_CTR` | `9` |  |
| `VIRTIO_CRYPTO_CIPHER_KASUMI_F8` | `10` |  |
| `VIRTIO_CRYPTO_CIPHER_SNOW3G_UEA2` | `11` |  |
| `VIRTIO_CRYPTO_CIPHER_AES_F8` | `12` |  |
| `VIRTIO_CRYPTO_CIPHER_AES_XTS` | `13` |  |
| `VIRTIO_CRYPTO_CIPHER_ZUC_EEA3` | `14` |  |
| `VIRTIO_CRYPTO_OP_ENCRYPT` | `1` |  |
| `VIRTIO_CRYPTO_OP_DECRYPT` | `2` |  |
| `VIRTIO_CRYPTO_NO_HASH` | `0` |  |
| `VIRTIO_CRYPTO_HASH_MD5` | `1` |  |
| `VIRTIO_CRYPTO_HASH_SHA1` | `2` |  |
| `VIRTIO_CRYPTO_HASH_SHA_224` | `3` |  |
| `VIRTIO_CRYPTO_HASH_SHA_256` | `4` |  |
| `VIRTIO_CRYPTO_HASH_SHA_384` | `5` |  |
| `VIRTIO_CRYPTO_HASH_SHA_512` | `6` |  |
| `VIRTIO_CRYPTO_HASH_SHA3_224` | `7` |  |
| `VIRTIO_CRYPTO_HASH_SHA3_256` | `8` |  |
| `VIRTIO_CRYPTO_HASH_SHA3_384` | `9` |  |
| `VIRTIO_CRYPTO_HASH_SHA3_512` | `10` |  |
| `VIRTIO_CRYPTO_HASH_SHA3_SHAKE128` | `11` |  |
| `VIRTIO_CRYPTO_HASH_SHA3_SHAKE256` | `12` |  |
| `VIRTIO_CRYPTO_NO_MAC` | `0` |  |
| `VIRTIO_CRYPTO_MAC_HMAC_MD5` | `1` |  |
| `VIRTIO_CRYPTO_MAC_HMAC_SHA1` | `2` |  |
| `VIRTIO_CRYPTO_MAC_HMAC_SHA_224` | `3` |  |
| `VIRTIO_CRYPTO_MAC_HMAC_SHA_256` | `4` |  |

*...and 65 more*

## Structs (36)


### `struct virtio_crypto_ctrl_header`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `opcode` | `-` |
| `__le32` | `algo` | `-` |
| `__le32` | `flag` | `-` |
| `__le32` | `queue_id` | `-` |

### `struct virtio_crypto_cipher_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `algo` | `-` |
| `__le32` | `keylen` | `-` |
| `__le32` | `op` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_session_input`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `session_id` | `-` |
| `__le32` | `status` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_cipher_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `32` |

### `struct virtio_crypto_hash_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `algo` | `-` |
| `__le32` | `hash_result_len` | `-` |
| `__u8` | `padding` | `8` |

### `struct virtio_crypto_hash_create_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `40` |

### `struct virtio_crypto_mac_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `algo` | `-` |
| `__le32` | `hash_result_len` | `-` |
| `__le32` | `auth_key_len` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_mac_create_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `40` |

### `struct virtio_crypto_aead_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `algo` | `-` |
| `__le32` | `key_len` | `-` |
| `__le32` | `hash_result_len` | `-` |
| `__le32` | `aad_len` | `-` |
| `__le32` | `op` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_aead_create_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `32` |

### `struct virtio_crypto_rsa_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `padding_algo` | `-` |
| `__le32` | `hash_algo` | `-` |

### `struct virtio_crypto_ecdsa_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `curve_id` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_akcipher_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `algo` | `-` |
| `__le32` | `keytype` | `-` |
| `__le32` | `keylen` | `-` |

### `struct virtio_crypto_akcipher_create_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `36` |

### `struct virtio_crypto_alg_chain_session_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `alg_chain_order` | `-` |
| `__le32` | `hash_mode` | `-` |
| `__u8` | `padding` | `16` |
| `__le32` | `aad_len` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_alg_chain_session_req`

| Type | Field | Array |
|------|-------|-------|

### `struct virtio_crypto_sym_create_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `48` |
| `__le32` | `op_type` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_destroy_session_req`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `session_id` | `-` |
| `__u8` | `padding` | `48` |

### `struct virtio_crypto_op_ctrl_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `56` |

### `struct virtio_crypto_op_header`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `opcode` | `-` |
| `__le32` | `algo` | `-` |
| `__le64` | `session_id` | `-` |
| `__le32` | `flag` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_cipher_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `iv_len` | `-` |
| `__le32` | `src_data_len` | `-` |
| `__le32` | `dst_data_len` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_hash_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `src_data_len` | `-` |
| `__le32` | `hash_result_len` | `-` |

### `struct virtio_crypto_mac_para`

| Type | Field | Array |
|------|-------|-------|

### `struct virtio_crypto_aead_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `iv_len` | `-` |
| `__le32` | `aad_len` | `-` |
| `__le32` | `src_data_len` | `-` |
| `__le32` | `dst_data_len` | `-` |

### `struct virtio_crypto_cipher_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `24` |

### `struct virtio_crypto_hash_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `40` |

### `struct virtio_crypto_mac_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `40` |

### `struct virtio_crypto_alg_chain_data_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `iv_len` | `-` |
| `__le32` | `src_data_len` | `-` |
| `__le32` | `dst_data_len` | `-` |
| `__le32` | `cipher_start_src_offset` | `-` |
| `__le32` | `len_to_cipher` | `-` |
| `__le32` | `hash_start_src_offset` | `-` |
| `__le32` | `len_to_hash` | `-` |
| `__le32` | `aad_len` | `-` |
| `__le32` | `hash_result_len` | `-` |
| `__le32` | `reserved` | `-` |

### `struct virtio_crypto_alg_chain_data_req`

| Type | Field | Array |
|------|-------|-------|

### `struct virtio_crypto_sym_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `40` |
| `__le32` | `op_type` | `-` |
| `__le32` | `padding` | `-` |

### `struct virtio_crypto_aead_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `32` |

### `struct virtio_crypto_akcipher_para`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `src_data_len` | `-` |
| `__le32` | `dst_data_len` | `-` |

### `struct virtio_crypto_akcipher_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `40` |

### `struct virtio_crypto_op_data_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `48` |

### `struct virtio_crypto_config`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `status` | `-` |
| `__le32` | `max_dataqueues` | `-` |
| `__le32` | `crypto_services` | `-` |
| `__le32` | `cipher_algo_l` | `-` |
| `__le32` | `cipher_algo_h` | `-` |
| `__le32` | `hash_algo` | `-` |
| `__le32` | `mac_algo_l` | `-` |
| `__le32` | `mac_algo_h` | `-` |
| `__le32` | `aead_algo` | `-` |
| `__le32` | `max_cipher_key_len` | `-` |
| `__le32` | `max_auth_key_len` | `-` |
| `__le32` | `akcipher_algo` | `-` |
| `__le64` | `max_size` | `-` |

### `struct virtio_crypto_inhdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `status` | `-` |