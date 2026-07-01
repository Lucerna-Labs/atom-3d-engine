# if_macsec.h

**Source:** `if_macsec.h`


## Includes

- `linux/types.h`

## Defines (14 total)


### MACSEC_CIPHER (4)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_CIPHER_ID_GCM_AES_128` | `0x0080C20001000001ULL` |  |
| `MACSEC_CIPHER_ID_GCM_AES_256` | `0x0080C20001000002ULL` |  |
| `MACSEC_CIPHER_ID_GCM_AES_XPN_128` | `0x0080C20001000003ULL` |  |
| `MACSEC_CIPHER_ID_GCM_AES_XPN_256` | `0x0080C20001000004ULL` |  |

### MACSEC_DEFAULT (2)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_DEFAULT_CIPHER_ID` | `0x0080020001000001ULL` |  |
| `MACSEC_DEFAULT_CIPHER_ALT` | `MACSEC_CIPHER_ID_GCM_AES_128` |  |

### MACSEC_GENL (2)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_GENL_NAME` | `"macsec"` |  |
| `MACSEC_GENL_VERSION` | `1` |  |

### MACSEC_KEYID (1)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_KEYID_LEN` | `16` |  |

### MACSEC_MAX (2)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_MAX_KEY_LEN` | `128` |  |
| `MACSEC_MAX_ICV_LEN` | `32` |  |

### MACSEC_MIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_MIN_ICV_LEN` | `8` |  |

### MACSEC_SALT (1)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_SALT_LEN` | `12` |  |

### MACSEC_STD (1)

| Name | Value | Comment |
|------|-------|---------|
| `MACSEC_STD_ICV_LEN` | `16` |  |