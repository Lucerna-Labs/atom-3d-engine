# dcbnl.h

**Source:** `dcbnl.h`


## Includes

- `linux/types.h`

## Defines (32 total)


### CEE_DCBX (2)

| Name | Value | Comment |
|------|-------|---------|
| `CEE_DCBX_MAX_PGS` | `8` |  |
| `CEE_DCBX_MAX_PRIO` | `8` |  |

### DCBX_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `DCBX_MAX_BUFFERS` | `8` |  |

### DCB_APP (3)

| Name | Value | Comment |
|------|-------|---------|
| `DCB_APP_SEL_PCP` | `255` |  |
| `DCB_APP_IDTYPE_ETHTYPE` | `0x00` |  |
| `DCB_APP_IDTYPE_PORTNUM` | `0x01` |  |

### DCB_ATTR (5)

| Name | Value | Comment |
|------|-------|---------|
| `DCB_ATTR_IEEE_MAX` | `(__DCB_ATTR_IEEE_MAX - 1)` |  |
| `DCB_ATTR_IEEE_APP_MAX` | `(__DCB_ATTR_IEEE_APP_MAX - 1)` |  |
| `DCB_ATTR_CEE_MAX` | `(__DCB_ATTR_CEE_MAX - 1)` |  |
| `DCB_ATTR_CEE_PEER_APP_MAX` | `(__DCB_ATTR_CEE_PEER_APP_MAX - 1)` |  |
| `DCB_ATTR_CEE_APP_MAX` | `(__DCB_ATTR_CEE_APP_MAX - 1)` |  |

### DCB_CAP (5)

| Name | Value | Comment |
|------|-------|---------|
| `DCB_CAP_DCBX_HOST` | `0x01` |  |
| `DCB_CAP_DCBX_LLD_MANAGED` | `0x02` |  |
| `DCB_CAP_DCBX_VER_CEE` | `0x04` |  |
| `DCB_CAP_DCBX_VER_IEEE` | `0x08` |  |
| `DCB_CAP_DCBX_STATIC` | `0x10` |  |

### DCB_FEATCFG (4)

| Name | Value | Comment |
|------|-------|---------|
| `DCB_FEATCFG_ERROR` | `0x01` | error in feature resolution |
| `DCB_FEATCFG_ENABLE` | `0x02` | enable feature |
| `DCB_FEATCFG_WILLING` | `0x04` | feature is willing |
| `DCB_FEATCFG_ADVERTISE` | `0x08` | advertise feature |

### UNCATEGORIZED (12)

| Name | Value | Comment |
|------|-------|---------|
| `IEEE_8021QAZ_MAX_TCS` | `8` |  |
| `IEEE_8021QAZ_TSA_STRICT` | `0` |  |
| `IEEE_8021QAZ_TSA_CB_SHAPER` | `1` |  |
| `IEEE_8021QAZ_TSA_ETS` | `2` |  |
| `IEEE_8021QAZ_TSA_VENDOR` | `255` |  |
| `IEEE_8021Q_MAX_PRIORITIES` | `8` |  |
| `IEEE_8021QAZ_APP_SEL_ETHERTYPE` | `1` |  |
| `IEEE_8021QAZ_APP_SEL_STREAM` | `2` |  |
| `IEEE_8021QAZ_APP_SEL_DGRAM` | `3` |  |
| `IEEE_8021QAZ_APP_SEL_ANY` | `4` |  |
| `IEEE_8021QAZ_APP_SEL_DSCP` | `5` |  |
| `IEEE_8021QAZ_APP_SEL_MAX` | `255` |  |

## Structs (11)


### `struct ieee_ets`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `willing` | `-` |
| `__u8` | `ets_cap` | `-` |
| `__u8` | `cbs` | `-` |
| `__u8` | `tc_tx_bw` | `IEEE_8021QAZ_MAX_TCS` |
| `__u8` | `tc_rx_bw` | `IEEE_8021QAZ_MAX_TCS` |
| `__u8` | `tc_tsa` | `IEEE_8021QAZ_MAX_TCS` |
| `__u8` | `prio_tc` | `IEEE_8021QAZ_MAX_TCS` |
| `__u8` | `tc_reco_bw` | `IEEE_8021QAZ_MAX_TCS` |
| `__u8` | `tc_reco_tsa` | `IEEE_8021QAZ_MAX_TCS` |
| `__u8` | `reco_prio_tc` | `IEEE_8021QAZ_MAX_TCS` |

### `struct ieee_maxrate`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `tc_maxrate` | `IEEE_8021QAZ_MAX_TCS` |

### `struct ieee_qcn`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `rpg_enable` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rppp_max_rps` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_time_reset` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_byte_reset` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_threshold` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_max_rate` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_ai_rate` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_hai_rate` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_gd` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_min_dec_fac` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rpg_min_rate` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `cndd_state_machine` | `IEEE_8021QAZ_MAX_TCS` |

### `struct ieee_qcn_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rppp_rp_centiseconds` | `IEEE_8021QAZ_MAX_TCS` |
| `__u32` | `rppp_created_rps` | `IEEE_8021QAZ_MAX_TCS` |

### `struct ieee_pfc`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `pfc_cap` | `-` |
| `__u8` | `pfc_en` | `-` |
| `__u8` | `mbc` | `-` |
| `__u16` | `delay` | `-` |
| `__u64` | `requests` | `IEEE_8021QAZ_MAX_TCS` |
| `__u64` | `indications` | `IEEE_8021QAZ_MAX_TCS` |

### `struct dcbnl_buffer`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `prio2buffer` | `IEEE_8021Q_MAX_PRIORITIES` |
| `__u32` | `buffer_size` | `DCBX_MAX_BUFFERS` |
| `__u32` | `total_size` | `-` |

### `struct cee_pg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `willing` | `-` |
| `__u8` | `error` | `-` |
| `__u8` | `pg_en` | `-` |
| `__u8` | `tcs_supported` | `-` |
| `__u8` | `pg_bw` | `CEE_DCBX_MAX_PGS` |
| `__u8` | `prio_pg` | `CEE_DCBX_MAX_PGS` |

### `struct cee_pfc`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `willing` | `-` |
| `__u8` | `error` | `-` |
| `__u8` | `pfc_en` | `-` |
| `__u8` | `tcs_supported` | `-` |

### `struct dcb_app`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `selector` | `-` |
| `__u8` | `priority` | `-` |
| `__u16` | `protocol` | `-` |

### `struct dcb_peer_app_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `willing` | `-` |
| `__u8` | `error` | `-` |

### `struct dcbmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `dcb_family` | `-` |
| `__u8` | `cmd` | `-` |
| `__u16` | `dcb_pad` | `-` |