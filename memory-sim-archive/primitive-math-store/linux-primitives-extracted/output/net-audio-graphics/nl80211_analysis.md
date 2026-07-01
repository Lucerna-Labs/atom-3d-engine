# nl80211.h

**Source:** `nl80211.h`


## Includes

- `linux/types.h`

## Defines (124 total)


### UNCATEGORIZED (124)

| Name | Value | Comment |
|------|-------|---------|
| `NL80211_GENL_NAME` | `"nl80211"` |  |
| `NL80211_MULTICAST_GROUP_CONFIG` | `"config"` |  |
| `NL80211_MULTICAST_GROUP_SCAN` | `"scan"` |  |
| `NL80211_MULTICAST_GROUP_REG` | `"regulatory"` |  |
| `NL80211_MULTICAST_GROUP_MLME` | `"mlme"` |  |
| `NL80211_MULTICAST_GROUP_VENDOR` | `"vendor"` |  |
| `NL80211_MULTICAST_GROUP_NAN` | `"nan"` |  |
| `NL80211_MULTICAST_GROUP_TESTMODE` | `"testmode"` |  |
| `NL80211_EDMG_BW_CONFIG_MIN` | `4` |  |
| `NL80211_EDMG_BW_CONFIG_MAX` | `15` |  |
| `NL80211_EDMG_CHANNELS_MIN` | `1` |  |
| `NL80211_EDMG_CHANNELS_MAX` | `0x3c` | 0b00111100 |
| `NL80211_CMD_SET_BSS` | `NL80211_CMD_SET_BSS` |  |
| `NL80211_CMD_SET_MGMT_EXTRA_IE` | `NL80211_CMD_SET_MGMT_EXTRA_IE` |  |
| `NL80211_CMD_REG_CHANGE` | `NL80211_CMD_REG_CHANGE` |  |
| `NL80211_CMD_AUTHENTICATE` | `NL80211_CMD_AUTHENTICATE` |  |
| `NL80211_CMD_ASSOCIATE` | `NL80211_CMD_ASSOCIATE` |  |
| `NL80211_CMD_DEAUTHENTICATE` | `NL80211_CMD_DEAUTHENTICATE` |  |
| `NL80211_CMD_DISASSOCIATE` | `NL80211_CMD_DISASSOCIATE` |  |
| `NL80211_CMD_REG_BEACON_HINT` | `NL80211_CMD_REG_BEACON_HINT` |  |
| `NL80211_ATTR_FEATURE_FLAGS` | `NL80211_ATTR_FEATURE_FLAGS` |  |
| `NL80211_CMD_GET_MESH_PARAMS` | `NL80211_CMD_GET_MESH_CONFIG` |  |
| `NL80211_CMD_SET_MESH_PARAMS` | `NL80211_CMD_SET_MESH_CONFIG` |  |
| `NL80211_MESH_SETUP_VENDOR_PATH_SEL_IE` | `NL80211_MESH_SETUP_IE` |  |
| `NL80211_ATTR_SCAN_GENERATION` | `NL80211_ATTR_GENERATION` |  |
| `NL80211_ATTR_MESH_PARAMS` | `NL80211_ATTR_MESH_CONFIG` |  |
| `NL80211_ATTR_IFACE_SOCKET_OWNER` | `NL80211_ATTR_SOCKET_OWNER` |  |
| `NL80211_ATTR_SAE_DATA` | `NL80211_ATTR_AUTH_DATA` |  |
| `NL80211_ATTR_CSA_C_OFF_BEACON` | `NL80211_ATTR_CNTDWN_OFFS_BEACON` |  |
| `NL80211_ATTR_CSA_C_OFF_PRESP` | `NL80211_ATTR_CNTDWN_OFFS_PRESP` |  |
| `NL80211_CMD_CONNECT` | `NL80211_CMD_CONNECT` |  |
| `NL80211_ATTR_HT_CAPABILITY` | `NL80211_ATTR_HT_CAPABILITY` |  |
| `NL80211_ATTR_BSS_BASIC_RATES` | `NL80211_ATTR_BSS_BASIC_RATES` |  |
| `NL80211_ATTR_WIPHY_TXQ_PARAMS` | `NL80211_ATTR_WIPHY_TXQ_PARAMS` |  |
| `NL80211_ATTR_WIPHY_FREQ` | `NL80211_ATTR_WIPHY_FREQ` |  |
| `NL80211_ATTR_WIPHY_CHANNEL_TYPE` | `NL80211_ATTR_WIPHY_CHANNEL_TYPE` |  |
| `NL80211_ATTR_MGMT_SUBTYPE` | `NL80211_ATTR_MGMT_SUBTYPE` |  |
| `NL80211_ATTR_IE` | `NL80211_ATTR_IE` |  |
| `NL80211_ATTR_REG_INITIATOR` | `NL80211_ATTR_REG_INITIATOR` |  |
| `NL80211_ATTR_REG_TYPE` | `NL80211_ATTR_REG_TYPE` |  |
| `NL80211_ATTR_FRAME` | `NL80211_ATTR_FRAME` |  |
| `NL80211_ATTR_SSID` | `NL80211_ATTR_SSID` |  |
| `NL80211_ATTR_AUTH_TYPE` | `NL80211_ATTR_AUTH_TYPE` |  |
| `NL80211_ATTR_REASON_CODE` | `NL80211_ATTR_REASON_CODE` |  |
| `NL80211_ATTR_CIPHER_SUITES_PAIRWISE` | `NL80211_ATTR_CIPHER_SUITES_PAIRWISE` |  |
| `NL80211_ATTR_CIPHER_SUITE_GROUP` | `NL80211_ATTR_CIPHER_SUITE_GROUP` |  |
| `NL80211_ATTR_WPA_VERSIONS` | `NL80211_ATTR_WPA_VERSIONS` |  |
| `NL80211_ATTR_AKM_SUITES` | `NL80211_ATTR_AKM_SUITES` |  |
| `NL80211_ATTR_KEY` | `NL80211_ATTR_KEY` |  |
| `NL80211_ATTR_KEYS` | `NL80211_ATTR_KEYS` |  |

*...and 74 more*

## Structs (11)


### `struct nl80211_sta_flag_update`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mask` | `-` |
| `__u32` | `set` | `-` |

### `struct nl80211_txrate_vht`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `mcs` | `NL80211_VHT_NSS_MAX` |

### `struct nl80211_txrate_he`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `mcs` | `NL80211_HE_NSS_MAX` |

### `struct nl80211_txrate_eht`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `mcs` | `NL80211_EHT_NSS_MAX` |

### `struct nl80211_pattern_support`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_patterns` | `-` |
| `__u32` | `min_pattern_len` | `-` |
| `__u32` | `max_pattern_len` | `-` |
| `__u32` | `max_pkt_offset` | `-` |

### `struct nl80211_wowlan_tcp_data_seq`

| Type | Field | Array |
|------|-------|-------|

### `struct nl80211_wowlan_tcp_data_token`

| Type | Field | Array |
|------|-------|-------|

### `struct nl80211_wowlan_tcp_data_token_feature`

| Type | Field | Array |
|------|-------|-------|

### `struct nl80211_coalesce_rule_support`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_rules` | `-` |
| `__u32` | `max_delay` | `-` |

### `struct nl80211_vendor_cmd_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vendor_id` | `-` |
| `__u32` | `subcmd` | `-` |

### `struct nl80211_bss_select_rssi_adjust`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `band` | `-` |
| `__s8` | `delta` | `-` |