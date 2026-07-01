# pfkeyv2.h

**Source:** `pfkeyv2.h`


## Includes

- `linux/types.h`

## Defines (116 total)


### PF_KEY (1)

| Name | Value | Comment |
|------|-------|---------|
| `PF_KEY_V2` | `2` |  |

### SADB_AALG (4)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_AALG_NONE` | `0` |  |
| `SADB_AALG_MD5HMAC` | `2` |  |
| `SADB_AALG_SHA1HMAC` | `3` |  |
| `SADB_AALG_MAX` | `251` |  |

### SADB_EALG (5)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_EALG_NONE` | `0` |  |
| `SADB_EALG_DESCBC` | `2` |  |
| `SADB_EALG_3DESCBC` | `3` |  |
| `SADB_EALG_NULL` | `11` |  |
| `SADB_EALG_MAX` | `253` | last EALG |

### SADB_EXT (18)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_EXT_RESERVED` | `0` |  |
| `SADB_EXT_SA` | `1` |  |
| `SADB_EXT_LIFETIME_CURRENT` | `2` |  |
| `SADB_EXT_LIFETIME_HARD` | `3` |  |
| `SADB_EXT_LIFETIME_SOFT` | `4` |  |
| `SADB_EXT_ADDRESS_SRC` | `5` |  |
| `SADB_EXT_ADDRESS_DST` | `6` |  |
| `SADB_EXT_ADDRESS_PROXY` | `7` |  |
| `SADB_EXT_KEY_AUTH` | `8` |  |
| `SADB_EXT_KEY_ENCRYPT` | `9` |  |
| `SADB_EXT_IDENTITY_SRC` | `10` |  |
| `SADB_EXT_IDENTITY_DST` | `11` |  |
| `SADB_EXT_SENSITIVITY` | `12` |  |
| `SADB_EXT_PROPOSAL` | `13` |  |
| `SADB_EXT_SUPPORTED_AUTH` | `14` |  |
| `SADB_EXT_SUPPORTED_ENCRYPT` | `15` |  |
| `SADB_EXT_SPIRANGE` | `16` |  |
| `SADB_EXT_MAX` | `26` |  |

### SADB_IDENTTYPE (5)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_IDENTTYPE_RESERVED` | `0` |  |
| `SADB_IDENTTYPE_PREFIX` | `1` |  |
| `SADB_IDENTTYPE_FQDN` | `2` |  |
| `SADB_IDENTTYPE_USERFQDN` | `3` |  |
| `SADB_IDENTTYPE_MAX` | `3` |  |

### SADB_SAFLAGS (4)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_SAFLAGS_PFS` | `1` |  |
| `SADB_SAFLAGS_NOPMTUDISC` | `0x20000000` |  |
| `SADB_SAFLAGS_DECAP_DSCP` | `0x40000000` |  |
| `SADB_SAFLAGS_NOECN` | `0x80000000` |  |

### SADB_SASTATE (5)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_SASTATE_LARVAL` | `0` |  |
| `SADB_SASTATE_MATURE` | `1` |  |
| `SADB_SASTATE_DYING` | `2` |  |
| `SADB_SASTATE_DEAD` | `3` |  |
| `SADB_SASTATE_MAX` | `3` |  |

### SADB_SATYPE (8)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_SATYPE_UNSPEC` | `0` |  |
| `SADB_SATYPE_AH` | `2` |  |
| `SADB_SATYPE_ESP` | `3` |  |
| `SADB_SATYPE_RSVP` | `5` |  |
| `SADB_SATYPE_OSPFV2` | `6` |  |
| `SADB_SATYPE_RIPV2` | `7` |  |
| `SADB_SATYPE_MIP` | `8` |  |
| `SADB_SATYPE_MAX` | `9` |  |

### SADB_X (53)

| Name | Value | Comment |
|------|-------|---------|
| `SADB_X_PROMISC` | `11` |  |
| `SADB_X_PCHANGE` | `12` |  |
| `SADB_X_SPDUPDATE` | `13` |  |
| `SADB_X_SPDADD` | `14` |  |
| `SADB_X_SPDDELETE` | `15` |  |
| `SADB_X_SPDGET` | `16` |  |
| `SADB_X_SPDACQUIRE` | `17` |  |
| `SADB_X_SPDDUMP` | `18` |  |
| `SADB_X_SPDFLUSH` | `19` |  |
| `SADB_X_SPDSETIDX` | `20` |  |
| `SADB_X_SPDEXPIRE` | `21` |  |
| `SADB_X_SPDDELETE2` | `22` |  |
| `SADB_X_NAT_T_NEW_MAPPING` | `23` |  |
| `SADB_X_MIGRATE` | `24` |  |
| `SADB_X_SATYPE_IPCOMP` | `9` |  |
| `SADB_X_AALG_SHA2_256HMAC` | `5` |  |
| `SADB_X_AALG_SHA2_384HMAC` | `6` |  |
| `SADB_X_AALG_SHA2_512HMAC` | `7` |  |
| `SADB_X_AALG_RIPEMD160HMAC` | `8` |  |
| `SADB_X_AALG_AES_XCBC_MAC` | `9` |  |
| `SADB_X_AALG_SM3_256HMAC` | `10` |  |
| `SADB_X_AALG_NULL` | `251` | kame |
| `SADB_X_EALG_CASTCBC` | `6` |  |
| `SADB_X_EALG_BLOWFISHCBC` | `7` |  |
| `SADB_X_EALG_AESCBC` | `12` |  |
| `SADB_X_EALG_AESCTR` | `13` |  |
| `SADB_X_EALG_AES_CCM_ICV8` | `14` |  |
| `SADB_X_EALG_AES_CCM_ICV12` | `15` |  |
| `SADB_X_EALG_AES_CCM_ICV16` | `16` |  |
| `SADB_X_EALG_AES_GCM_ICV8` | `18` |  |
| `SADB_X_EALG_AES_GCM_ICV12` | `19` |  |
| `SADB_X_EALG_AES_GCM_ICV16` | `20` |  |
| `SADB_X_EALG_CAMELLIACBC` | `22` |  |
| `SADB_X_EALG_NULL_AES_GMAC` | `23` |  |
| `SADB_X_EALG_SM4CBC` | `24` |  |
| `SADB_X_EALG_SERPENTCBC` | `252` | draft-ietf-ipsec-ciph-aes-cbc-00 |
| `SADB_X_EALG_TWOFISHCBC` | `253` | draft-ietf-ipsec-ciph-aes-cbc-00 |
| `SADB_X_CALG_NONE` | `0` |  |
| `SADB_X_CALG_OUI` | `1` |  |
| `SADB_X_CALG_DEFLATE` | `2` |  |
| `SADB_X_CALG_LZS` | `3` |  |
| `SADB_X_CALG_LZJH` | `4` |  |
| `SADB_X_CALG_MAX` | `4` |  |
| `SADB_X_EXT_KMPRIVATE` | `17` |  |
| `SADB_X_EXT_POLICY` | `18` |  |
| `SADB_X_EXT_SA2` | `19` |  |
| `SADB_X_EXT_NAT_T_TYPE` | `20` |  |
| `SADB_X_EXT_NAT_T_SPORT` | `21` |  |
| `SADB_X_EXT_NAT_T_DPORT` | `22` |  |
| `SADB_X_EXT_NAT_T_OA` | `23` |  |

*...and 3 more*

### UNCATEGORIZED (13)

| Name | Value | Comment |
|------|-------|---------|
| `PFKEYV2_REVISION` | `199806L` |  |
| `SADB_RESERVED` | `0` |  |
| `SADB_GETSPI` | `1` |  |
| `SADB_UPDATE` | `2` |  |
| `SADB_ADD` | `3` |  |
| `SADB_DELETE` | `4` |  |
| `SADB_GET` | `5` |  |
| `SADB_ACQUIRE` | `6` |  |
| `SADB_REGISTER` | `7` |  |
| `SADB_EXPIRE` | `8` |  |
| `SADB_FLUSH` | `9` |  |
| `SADB_DUMP` | `10` |  |
| `SADB_MAX` | `24` |  |

## Structs (22)


### `struct sadb_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sadb_msg_version` | `-` |
| `__u8` | `sadb_msg_type` | `-` |
| `__u8` | `sadb_msg_errno` | `-` |
| `__u8` | `sadb_msg_satype` | `-` |
| `__u16` | `sadb_msg_len` | `-` |
| `__u16` | `sadb_msg_reserved` | `-` |
| `__u32` | `sadb_msg_seq` | `-` |
| `__u32` | `sadb_msg_pid` | `-` |

### `struct sadb_ext`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_ext_len` | `-` |
| `__u16` | `sadb_ext_type` | `-` |

### `struct sadb_sa`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_sa_len` | `-` |
| `__u16` | `sadb_sa_exttype` | `-` |
| `__be32` | `sadb_sa_spi` | `-` |
| `__u8` | `sadb_sa_replay` | `-` |
| `__u8` | `sadb_sa_state` | `-` |
| `__u8` | `sadb_sa_auth` | `-` |
| `__u8` | `sadb_sa_encrypt` | `-` |
| `__u32` | `sadb_sa_flags` | `-` |

### `struct sadb_lifetime`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_lifetime_len` | `-` |
| `__u16` | `sadb_lifetime_exttype` | `-` |
| `__u32` | `sadb_lifetime_allocations` | `-` |
| `__u64` | `sadb_lifetime_bytes` | `-` |
| `__u64` | `sadb_lifetime_addtime` | `-` |
| `__u64` | `sadb_lifetime_usetime` | `-` |

### `struct sadb_address`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_address_len` | `-` |
| `__u16` | `sadb_address_exttype` | `-` |
| `__u8` | `sadb_address_proto` | `-` |
| `__u8` | `sadb_address_prefixlen` | `-` |
| `__u16` | `sadb_address_reserved` | `-` |

### `struct sadb_key`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_key_len` | `-` |
| `__u16` | `sadb_key_exttype` | `-` |
| `__u16` | `sadb_key_bits` | `-` |
| `__u16` | `sadb_key_reserved` | `-` |

### `struct sadb_ident`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_ident_len` | `-` |
| `__u16` | `sadb_ident_exttype` | `-` |
| `__u16` | `sadb_ident_type` | `-` |
| `__u16` | `sadb_ident_reserved` | `-` |
| `__u64` | `sadb_ident_id` | `-` |

### `struct sadb_sens`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_sens_len` | `-` |
| `__u16` | `sadb_sens_exttype` | `-` |
| `__u32` | `sadb_sens_dpd` | `-` |
| `__u8` | `sadb_sens_sens_level` | `-` |
| `__u8` | `sadb_sens_sens_len` | `-` |
| `__u8` | `sadb_sens_integ_level` | `-` |
| `__u8` | `sadb_sens_integ_len` | `-` |
| `__u32` | `sadb_sens_reserved` | `-` |

### `struct sadb_prop`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_prop_len` | `-` |
| `__u16` | `sadb_prop_exttype` | `-` |
| `__u8` | `sadb_prop_replay` | `-` |
| `__u8` | `sadb_prop_reserved` | `3` |

### `struct sadb_comb`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sadb_comb_auth` | `-` |
| `__u8` | `sadb_comb_encrypt` | `-` |
| `__u16` | `sadb_comb_flags` | `-` |
| `__u16` | `sadb_comb_auth_minbits` | `-` |
| `__u16` | `sadb_comb_auth_maxbits` | `-` |
| `__u16` | `sadb_comb_encrypt_minbits` | `-` |
| `__u16` | `sadb_comb_encrypt_maxbits` | `-` |
| `__u32` | `sadb_comb_reserved` | `-` |
| `__u32` | `sadb_comb_soft_allocations` | `-` |
| `__u32` | `sadb_comb_hard_allocations` | `-` |
| `__u64` | `sadb_comb_soft_bytes` | `-` |
| `__u64` | `sadb_comb_hard_bytes` | `-` |
| `__u64` | `sadb_comb_soft_addtime` | `-` |
| `__u64` | `sadb_comb_hard_addtime` | `-` |
| `__u64` | `sadb_comb_soft_usetime` | `-` |
| `__u64` | `sadb_comb_hard_usetime` | `-` |

### `struct sadb_supported`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_supported_len` | `-` |
| `__u16` | `sadb_supported_exttype` | `-` |
| `__u32` | `sadb_supported_reserved` | `-` |

### `struct sadb_alg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sadb_alg_id` | `-` |
| `__u8` | `sadb_alg_ivlen` | `-` |
| `__u16` | `sadb_alg_minbits` | `-` |
| `__u16` | `sadb_alg_maxbits` | `-` |
| `__u16` | `sadb_alg_reserved` | `-` |

### `struct sadb_spirange`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_spirange_len` | `-` |
| `__u16` | `sadb_spirange_exttype` | `-` |
| `__u32` | `sadb_spirange_min` | `-` |
| `__u32` | `sadb_spirange_max` | `-` |
| `__u32` | `sadb_spirange_reserved` | `-` |

### `struct sadb_x_kmprivate`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_kmprivate_len` | `-` |
| `__u16` | `sadb_x_kmprivate_exttype` | `-` |
| `__u32` | `sadb_x_kmprivate_reserved` | `-` |

### `struct sadb_x_sa2`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_sa2_len` | `-` |
| `__u16` | `sadb_x_sa2_exttype` | `-` |
| `__u8` | `sadb_x_sa2_mode` | `-` |
| `__u8` | `sadb_x_sa2_reserved1` | `-` |
| `__u16` | `sadb_x_sa2_reserved2` | `-` |
| `__u32` | `sadb_x_sa2_sequence` | `-` |
| `__u32` | `sadb_x_sa2_reqid` | `-` |

### `struct sadb_x_policy`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_policy_len` | `-` |
| `__u16` | `sadb_x_policy_exttype` | `-` |
| `__u16` | `sadb_x_policy_type` | `-` |
| `__u8` | `sadb_x_policy_dir` | `-` |
| `__u8` | `sadb_x_policy_reserved` | `-` |
| `__u32` | `sadb_x_policy_id` | `-` |
| `__u32` | `sadb_x_policy_priority` | `-` |

### `struct sadb_x_ipsecrequest`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_ipsecrequest_len` | `-` |
| `__u16` | `sadb_x_ipsecrequest_proto` | `-` |
| `__u8` | `sadb_x_ipsecrequest_mode` | `-` |
| `__u8` | `sadb_x_ipsecrequest_level` | `-` |
| `__u16` | `sadb_x_ipsecrequest_reserved1` | `-` |
| `__u32` | `sadb_x_ipsecrequest_reqid` | `-` |
| `__u32` | `sadb_x_ipsecrequest_reserved2` | `-` |

### `struct sadb_x_nat_t_type`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_nat_t_type_len` | `-` |
| `__u16` | `sadb_x_nat_t_type_exttype` | `-` |
| `__u8` | `sadb_x_nat_t_type_type` | `-` |
| `__u8` | `sadb_x_nat_t_type_reserved` | `3` |

### `struct sadb_x_nat_t_port`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_nat_t_port_len` | `-` |
| `__u16` | `sadb_x_nat_t_port_exttype` | `-` |
| `__be16` | `sadb_x_nat_t_port_port` | `-` |
| `__u16` | `sadb_x_nat_t_port_reserved` | `-` |

### `struct sadb_x_sec_ctx`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_sec_len` | `-` |
| `__u16` | `sadb_x_sec_exttype` | `-` |
| `__u8` | `sadb_x_ctx_alg` | `-` |
| `__u8` | `sadb_x_ctx_doi` | `-` |
| `__u16` | `sadb_x_ctx_len` | `-` |

### `struct sadb_x_kmaddress`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_kmaddress_len` | `-` |
| `__u16` | `sadb_x_kmaddress_exttype` | `-` |
| `__u32` | `sadb_x_kmaddress_reserved` | `-` |

### `struct sadb_x_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sadb_x_filter_len` | `-` |
| `__u16` | `sadb_x_filter_exttype` | `-` |
| `__u32` | `sadb_x_filter_saddr` | `4` |
| `__u32` | `sadb_x_filter_daddr` | `4` |
| `__u16` | `sadb_x_filter_family` | `-` |
| `__u8` | `sadb_x_filter_splen` | `-` |
| `__u8` | `sadb_x_filter_dplen` | `-` |