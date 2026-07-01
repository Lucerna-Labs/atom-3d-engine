# keyctl.h

**Source:** `keyctl.h`


## Includes

- `linux/types.h`

## Defines (66 total)


### KEYCTL_ASSUME (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_ASSUME_AUTHORITY` | `16` | assume request_key() authorisation |

### KEYCTL_DH (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_DH_COMPUTE` | `23` | Compute Diffie-Hellman values |

### KEYCTL_GET (3)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_GET_KEYRING_ID` | `0` | ask for a keyring's ID |
| `KEYCTL_GET_SECURITY` | `17` | get key security label |
| `KEYCTL_GET_PERSISTENT` | `22` | get a user's persistent keyring |

### KEYCTL_INSTANTIATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_INSTANTIATE_IOV` | `20` | instantiate a partially constructed key |

### KEYCTL_JOIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_JOIN_SESSION_KEYRING` | `1` | join or start named session keyring |

### KEYCTL_MOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_MOVE_EXCL` | `0x00000001` | Do not displace from the to-keyring |

### KEYCTL_PKEY (5)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_PKEY_QUERY` | `24` | Query public key parameters |
| `KEYCTL_PKEY_ENCRYPT` | `25` | Encrypt a blob using a public key |
| `KEYCTL_PKEY_DECRYPT` | `26` | Decrypt a blob using a public key |
| `KEYCTL_PKEY_SIGN` | `27` | Create a public key signature |
| `KEYCTL_PKEY_VERIFY` | `28` | Verify a public key signature |

### KEYCTL_RESTRICT (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_RESTRICT_KEYRING` | `29` | Restrict keys allowed to link to a keyring |

### KEYCTL_SESSION (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_SESSION_TO_PARENT` | `18` | apply session keyring to parent process |

### KEYCTL_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_SET_REQKEY_KEYRING` | `14` | set default request-key keyring |
| `KEYCTL_SET_TIMEOUT` | `15` | set key timeout |

### KEYCTL_SUPPORTS (4)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_SUPPORTS_ENCRYPT` | `0x01` |  |
| `KEYCTL_SUPPORTS_DECRYPT` | `0x02` |  |
| `KEYCTL_SUPPORTS_SIGN` | `0x04` |  |
| `KEYCTL_SUPPORTS_VERIFY` | `0x08` |  |

### KEYCTL_WATCH (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_WATCH_KEY` | `32` | Watch a key or ring of keys for changes |

### KEY_REQKEY (9)

| Name | Value | Comment |
|------|-------|---------|
| `KEY_REQKEY_DEFL_NO_CHANGE` | `-1` |  |
| `KEY_REQKEY_DEFL_DEFAULT` | `0` |  |
| `KEY_REQKEY_DEFL_THREAD_KEYRING` | `1` |  |
| `KEY_REQKEY_DEFL_PROCESS_KEYRING` | `2` |  |
| `KEY_REQKEY_DEFL_SESSION_KEYRING` | `3` |  |
| `KEY_REQKEY_DEFL_USER_KEYRING` | `4` |  |
| `KEY_REQKEY_DEFL_USER_SESSION_KEYRING` | `5` |  |
| `KEY_REQKEY_DEFL_GROUP_KEYRING` | `6` |  |
| `KEY_REQKEY_DEFL_REQUESTOR_KEYRING` | `7` |  |

### KEY_SPEC (8)

| Name | Value | Comment |
|------|-------|---------|
| `KEY_SPEC_THREAD_KEYRING` | `-1` | - key ID for thread-specific keyring |
| `KEY_SPEC_PROCESS_KEYRING` | `-2` | - key ID for process-specific keyring |
| `KEY_SPEC_SESSION_KEYRING` | `-3` | - key ID for session-specific keyring |
| `KEY_SPEC_USER_KEYRING` | `-4` | - key ID for UID-specific keyring |
| `KEY_SPEC_USER_SESSION_KEYRING` | `-5` | - key ID for UID-session keyring |
| `KEY_SPEC_GROUP_KEYRING` | `-6` | - key ID for GID-specific keyring |
| `KEY_SPEC_REQKEY_AUTH_KEY` | `-7` | - key ID for assumed request_key auth key |
| `KEY_SPEC_REQUESTOR_KEYRING` | `-8` | - key ID for request_key() dest keyring |

### UNCATEGORIZED (27)

| Name | Value | Comment |
|------|-------|---------|
| `KEYCTL_UPDATE` | `2` | update a key |
| `KEYCTL_REVOKE` | `3` | revoke a key |
| `KEYCTL_CHOWN` | `4` | set ownership of a key |
| `KEYCTL_SETPERM` | `5` | set perms on a key |
| `KEYCTL_DESCRIBE` | `6` | describe a key |
| `KEYCTL_CLEAR` | `7` | clear contents of a keyring |
| `KEYCTL_LINK` | `8` | link a key into a keyring |
| `KEYCTL_UNLINK` | `9` | unlink a key from a keyring |
| `KEYCTL_SEARCH` | `10` | search for a key in a keyring |
| `KEYCTL_READ` | `11` | read a key or keyring's contents |
| `KEYCTL_INSTANTIATE` | `12` | instantiate a partially constructed key |
| `KEYCTL_NEGATE` | `13` | negate a partially constructed key |
| `KEYCTL_REJECT` | `19` | reject a partially constructed key |
| `KEYCTL_INVALIDATE` | `21` | invalidate a key |
| `KEYCTL_MOVE` | `30` | Move keys between keyrings |
| `KEYCTL_CAPABILITIES` | `31` | Find capabilities of keyrings subsystem |
| `KEYCTL_CAPS0_CAPABILITIES` | `0x01` | KEYCTL_CAPABILITIES supported |
| `KEYCTL_CAPS0_PERSISTENT_KEYRINGS` | `0x02` | Persistent keyrings enabled |
| `KEYCTL_CAPS0_DIFFIE_HELLMAN` | `0x04` | Diffie-Hellman computation enabled |
| `KEYCTL_CAPS0_PUBLIC_KEY` | `0x08` | Public key ops enabled |
| `KEYCTL_CAPS0_BIG_KEY` | `0x10` | big_key-type enabled |
| `KEYCTL_CAPS0_INVALIDATE` | `0x20` | KEYCTL_INVALIDATE supported |
| `KEYCTL_CAPS0_RESTRICT_KEYRING` | `0x40` | KEYCTL_RESTRICT_KEYRING supported |
| `KEYCTL_CAPS0_MOVE` | `0x80` | KEYCTL_MOVE supported |
| `KEYCTL_CAPS1_NS_KEYRING_NAME` | `0x01` | Keyring names are per-user_namespace |
| `KEYCTL_CAPS1_NS_KEY_TAG` | `0x02` | Key indexing can include a namespace tag |
| `KEYCTL_CAPS1_NOTIFICATIONS` | `0x04` | Keys generate watchable notifications |

## Structs (4)


### `struct keyctl_dh_params`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `private` | `-` |
| `__s32` | `priv` | `-` |
| `__s32` | `prime` | `-` |
| `__s32` | `base` | `-` |

### `struct keyctl_kdf_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `otherinfolen` | `-` |
| `__u32` | `__spare` | `8` |

### `struct keyctl_pkey_query`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `supported_ops` | `-` |
| `__u32` | `key_size` | `-` |
| `__u16` | `max_data_size` | `-` |
| `__u16` | `max_sig_size` | `-` |
| `__u16` | `max_enc_size` | `-` |
| `__u16` | `max_dec_size` | `-` |
| `__u32` | `__spare` | `10` |

### `struct keyctl_pkey_params`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `key_id` | `-` |
| `__u32` | `in_len` | `-` |
| `__u32` | `out_len` | `-` |
| `__u32` | `in2_len` | `-` |
| `__u32` | `__spare` | `7` |