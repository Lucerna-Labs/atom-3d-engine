# wireless.h

**Source:** `wireless.h`


## Includes

- `linux/types.h`
- `linux/socket.h`
- `linux/if.h`

## Defines (247 total)


### IW_AUTH (34)

| Name | Value | Comment |
|------|-------|---------|
| `IW_AUTH_INDEX` | `0x0FFF` |  |
| `IW_AUTH_FLAGS` | `0xF000` |  |
| `IW_AUTH_WPA_VERSION` | `0` |  |
| `IW_AUTH_CIPHER_PAIRWISE` | `1` |  |
| `IW_AUTH_CIPHER_GROUP` | `2` |  |
| `IW_AUTH_KEY_MGMT` | `3` |  |
| `IW_AUTH_TKIP_COUNTERMEASURES` | `4` |  |
| `IW_AUTH_DROP_UNENCRYPTED` | `5` |  |
| `IW_AUTH_80211_AUTH_ALG` | `6` |  |
| `IW_AUTH_WPA_ENABLED` | `7` |  |
| `IW_AUTH_RX_UNENCRYPTED_EAPOL` | `8` |  |
| `IW_AUTH_ROAMING_CONTROL` | `9` |  |
| `IW_AUTH_PRIVACY_INVOKED` | `10` |  |
| `IW_AUTH_CIPHER_GROUP_MGMT` | `11` |  |
| `IW_AUTH_MFP` | `12` |  |
| `IW_AUTH_WPA_VERSION_DISABLED` | `0x00000001` |  |
| `IW_AUTH_WPA_VERSION_WPA` | `0x00000002` |  |
| `IW_AUTH_WPA_VERSION_WPA2` | `0x00000004` |  |
| `IW_AUTH_CIPHER_NONE` | `0x00000001` |  |
| `IW_AUTH_CIPHER_WEP40` | `0x00000002` |  |
| `IW_AUTH_CIPHER_TKIP` | `0x00000004` |  |
| `IW_AUTH_CIPHER_CCMP` | `0x00000008` |  |
| `IW_AUTH_CIPHER_WEP104` | `0x00000010` |  |
| `IW_AUTH_CIPHER_AES_CMAC` | `0x00000020` |  |
| `IW_AUTH_KEY_MGMT_802_1X` | `1` |  |
| `IW_AUTH_KEY_MGMT_PSK` | `2` |  |
| `IW_AUTH_ALG_OPEN_SYSTEM` | `0x00000001` |  |
| `IW_AUTH_ALG_SHARED_KEY` | `0x00000002` |  |
| `IW_AUTH_ALG_LEAP` | `0x00000004` |  |
| `IW_AUTH_ROAMING_ENABLE` | `0` | driver/firmware based roaming |
| `IW_AUTH_ROAMING_DISABLE` | `1	/* user space program used for roaming` |  |
| `IW_AUTH_MFP_DISABLED` | `0` | MFP disabled |
| `IW_AUTH_MFP_OPTIONAL` | `1` | MFP optional |
| `IW_AUTH_MFP_REQUIRED` | `2` | MFP required |

### IW_CUSTOM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IW_CUSTOM_MAX` | `256` | In bytes |

### IW_ENC (5)

| Name | Value | Comment |
|------|-------|---------|
| `IW_ENC_CAPA_WPA` | `0x00000001` |  |
| `IW_ENC_CAPA_WPA2` | `0x00000002` |  |
| `IW_ENC_CAPA_CIPHER_TKIP` | `0x00000004` |  |
| `IW_ENC_CAPA_CIPHER_CCMP` | `0x00000008` |  |
| `IW_ENC_CAPA_4WAY_HANDSHAKE` | `0x00000010` |  |

### IW_ENCODE (20)

| Name | Value | Comment |
|------|-------|---------|
| `IW_ENCODE_INDEX` | `0x00FF` | Token index (if needed) |
| `IW_ENCODE_FLAGS` | `0xFF00` | Flags defined below |
| `IW_ENCODE_MODE` | `0xF000` | Modes defined below |
| `IW_ENCODE_DISABLED` | `0x8000` | Encoding disabled |
| `IW_ENCODE_ENABLED` | `0x0000` | Encoding enabled |
| `IW_ENCODE_RESTRICTED` | `0x4000` | Refuse non-encoded packets |
| `IW_ENCODE_OPEN` | `0x2000` | Accept non-encoded packets |
| `IW_ENCODE_NOKEY` | `0x0800` | Key is write only, so not present |
| `IW_ENCODE_TEMP` | `0x0400` | Temporary key |
| `IW_ENCODE_SEQ_MAX_SIZE` | `8` |  |
| `IW_ENCODE_ALG_NONE` | `0` |  |
| `IW_ENCODE_ALG_WEP` | `1` |  |
| `IW_ENCODE_ALG_TKIP` | `2` |  |
| `IW_ENCODE_ALG_CCMP` | `3` |  |
| `IW_ENCODE_ALG_PMK` | `4` |  |
| `IW_ENCODE_ALG_AES_CMAC` | `5` |  |
| `IW_ENCODE_EXT_TX_SEQ_VALID` | `0x00000001` |  |
| `IW_ENCODE_EXT_RX_SEQ_VALID` | `0x00000002` |  |
| `IW_ENCODE_EXT_GROUP_KEY` | `0x00000004` |  |
| `IW_ENCODE_EXT_SET_TX_KEY` | `0x00000008` |  |

### IW_ENCODING (1)

| Name | Value | Comment |
|------|-------|---------|
| `IW_ENCODING_TOKEN_MAX` | `64` | 512 bits (for now) |

### IW_ESSID (1)

| Name | Value | Comment |
|------|-------|---------|
| `IW_ESSID_MAX_SIZE` | `32` |  |

### IW_EV (17)

| Name | Value | Comment |
|------|-------|---------|
| `IW_EV_LCP_LEN` | `(sizeof(struct iw_event) - sizeof(union iwreq_data))` |  |
| `IW_EV_CHAR_LEN` | `(IW_EV_LCP_LEN + IFNAMSIZ)` |  |
| `IW_EV_UINT_LEN` | `(IW_EV_LCP_LEN + sizeof(__u32))` |  |
| `IW_EV_FREQ_LEN` | `(IW_EV_LCP_LEN + sizeof(struct iw_freq))` |  |
| `IW_EV_PARAM_LEN` | `(IW_EV_LCP_LEN + sizeof(struct iw_param))` |  |
| `IW_EV_ADDR_LEN` | `(IW_EV_LCP_LEN + sizeof(struct sockaddr))` |  |
| `IW_EV_QUAL_LEN` | `(IW_EV_LCP_LEN + sizeof(struct iw_quality))` |  |
| `IW_EV_POINT_OFF` | `offsetof(struct iw_point, length)` |  |
| `IW_EV_POINT_LEN` | `(IW_EV_LCP_LEN + sizeof(struct iw_point) - ` |  |
| `IW_EV_LCP_PK_LEN` | `(4)` |  |
| `IW_EV_CHAR_PK_LEN` | `(IW_EV_LCP_PK_LEN + IFNAMSIZ)` |  |
| `IW_EV_UINT_PK_LEN` | `(IW_EV_LCP_PK_LEN + sizeof(__u32))` |  |
| `IW_EV_FREQ_PK_LEN` | `(IW_EV_LCP_PK_LEN + sizeof(struct iw_freq))` |  |
| `IW_EV_PARAM_PK_LEN` | `(IW_EV_LCP_PK_LEN + sizeof(struct iw_param))` |  |
| `IW_EV_ADDR_PK_LEN` | `(IW_EV_LCP_PK_LEN + sizeof(struct sockaddr))` |  |
| `IW_EV_QUAL_PK_LEN` | `(IW_EV_LCP_PK_LEN + sizeof(struct iw_quality))` |  |
| `IW_EV_POINT_PK_LEN` | `(IW_EV_LCP_PK_LEN + 4)` |  |

### IW_EVENT (2)

| Name | Value | Comment |
|------|-------|---------|
| `IW_EVENT_CAPA_K_0` | `(IW_EVENT_CAPA_MASK(0x8B04) \| ` |  |
| `IW_EVENT_CAPA_K_1` | `(IW_EVENT_CAPA_MASK(0x8B2A))` |  |

### IW_FREQ (2)

| Name | Value | Comment |
|------|-------|---------|
| `IW_FREQ_AUTO` | `0x00` | Let the driver decides |
| `IW_FREQ_FIXED` | `0x01` | Force a specific value |

### IW_GENERIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IW_GENERIC_IE_MAX` | `1024` |  |

### IW_MAX (6)

| Name | Value | Comment |
|------|-------|---------|
| `IW_MAX_FREQUENCIES` | `32` |  |
| `IW_MAX_BITRATES` | `32` |  |
| `IW_MAX_TXPOWER` | `8` |  |
| `IW_MAX_SPY` | `8` |  |
| `IW_MAX_AP` | `64` |  |
| `IW_MAX_ENCODING_SIZES` | `8` |  |

### IW_MICFAILURE (5)

| Name | Value | Comment |
|------|-------|---------|
| `IW_MICFAILURE_KEY_ID` | `0x00000003` | Key ID 0..3 |
| `IW_MICFAILURE_GROUP` | `0x00000004` |  |
| `IW_MICFAILURE_PAIRWISE` | `0x00000008` |  |
| `IW_MICFAILURE_STAKEY` | `0x00000010` |  |
| `IW_MICFAILURE_COUNT` | `0x00000060 /* 1 or 2 (0 = count not supported)` |  |

### IW_MLME (4)

| Name | Value | Comment |
|------|-------|---------|
| `IW_MLME_DEAUTH` | `0` |  |
| `IW_MLME_DISASSOC` | `1` |  |
| `IW_MLME_AUTH` | `2` |  |
| `IW_MLME_ASSOC` | `3` |  |

### IW_MODE (8)

| Name | Value | Comment |
|------|-------|---------|
| `IW_MODE_AUTO` | `0` | Let the driver decides |
| `IW_MODE_ADHOC` | `1` | Single cell network |
| `IW_MODE_INFRA` | `2` | Multi cell network, roaming, ... |
| `IW_MODE_MASTER` | `3` | Synchronisation master or Access Point |
| `IW_MODE_REPEAT` | `4` | Wireless Repeater (forwarder) |
| `IW_MODE_SECOND` | `5` | Secondary master/repeater (backup) |
| `IW_MODE_MONITOR` | `6` | Passive monitor (listen only) |
| `IW_MODE_MESH` | `7` | Mesh (IEEE 802.11s) network |

### IW_PMKID (2)

| Name | Value | Comment |
|------|-------|---------|
| `IW_PMKID_LEN` | `16` |  |
| `IW_PMKID_CAND_PREAUTH` | `0x00000001` | RNS pre-authentication enabled |

### IW_PMKSA (3)

| Name | Value | Comment |
|------|-------|---------|
| `IW_PMKSA_ADD` | `1` |  |
| `IW_PMKSA_REMOVE` | `2` |  |
| `IW_PMKSA_FLUSH` | `3` |  |

### IW_POWER (14)

| Name | Value | Comment |
|------|-------|---------|
| `IW_POWER_ON` | `0x0000` | No details... |
| `IW_POWER_TYPE` | `0xF000` | Type of parameter |
| `IW_POWER_PERIOD` | `0x1000` | Value is a period/duration of |
| `IW_POWER_TIMEOUT` | `0x2000` | Value is a timeout (to go asleep) |
| `IW_POWER_MODE` | `0x0F00` | Power Management mode |
| `IW_POWER_UNICAST_R` | `0x0100` | Receive only unicast messages |
| `IW_POWER_MULTICAST_R` | `0x0200` | Receive only multicast messages |
| `IW_POWER_ALL_R` | `0x0300` | Receive all messages though PM |
| `IW_POWER_FORCE_S` | `0x0400` | Force PM procedure for sending unicast |
| `IW_POWER_REPEATER` | `0x0800` | Repeat broadcast messages in PM period |
| `IW_POWER_MODIFIER` | `0x000F` | Modify a parameter |
| `IW_POWER_MIN` | `0x0001` | Value is a minimum |
| `IW_POWER_MAX` | `0x0002` | Value is a maximum |
| `IW_POWER_RELATIVE` | `0x0004` | Value is not in seconds/ms/us |

### IW_PRIV (9)

| Name | Value | Comment |
|------|-------|---------|
| `IW_PRIV_TYPE_MASK` | `0x7000` | Type of arguments |
| `IW_PRIV_TYPE_NONE` | `0x0000` |  |
| `IW_PRIV_TYPE_BYTE` | `0x1000` | Char as number |
| `IW_PRIV_TYPE_CHAR` | `0x2000` | Char as character |
| `IW_PRIV_TYPE_INT` | `0x4000` | 32 bits int |
| `IW_PRIV_TYPE_FLOAT` | `0x5000` | struct iw_freq |
| `IW_PRIV_TYPE_ADDR` | `0x6000` | struct sockaddr |
| `IW_PRIV_SIZE_FIXED` | `0x0800` | Variable or fixed number of args |
| `IW_PRIV_SIZE_MASK` | `0x07FF` | Max number of those args |

### IW_QUAL (10)

| Name | Value | Comment |
|------|-------|---------|
| `IW_QUAL_QUAL_UPDATED` | `0x01` | Value was updated since last read |
| `IW_QUAL_LEVEL_UPDATED` | `0x02` |  |
| `IW_QUAL_NOISE_UPDATED` | `0x04` |  |
| `IW_QUAL_ALL_UPDATED` | `0x07` |  |
| `IW_QUAL_DBM` | `0x08` | Level + Noise are dBm |
| `IW_QUAL_QUAL_INVALID` | `0x10` | Driver doesn't provide value |
| `IW_QUAL_LEVEL_INVALID` | `0x20` |  |
| `IW_QUAL_NOISE_INVALID` | `0x40` |  |
| `IW_QUAL_RCPI` | `0x80` | Level + Noise are 802.11k RCPI |
| `IW_QUAL_ALL_INVALID` | `0x70` |  |

### IW_RETRY (10)

| Name | Value | Comment |
|------|-------|---------|
| `IW_RETRY_ON` | `0x0000` | No details... |
| `IW_RETRY_TYPE` | `0xF000` | Type of parameter |
| `IW_RETRY_LIMIT` | `0x1000` | Maximum number of retries |
| `IW_RETRY_LIFETIME` | `0x2000` | Maximum duration of retries in us |
| `IW_RETRY_MODIFIER` | `0x00FF` | Modify a parameter |
| `IW_RETRY_MIN` | `0x0001` | Value is a minimum |
| `IW_RETRY_MAX` | `0x0002` | Value is a maximum |
| `IW_RETRY_RELATIVE` | `0x0004` | Value is not in seconds/ms/us |
| `IW_RETRY_SHORT` | `0x0010` | Value is for short packets |
| `IW_RETRY_LONG` | `0x0020` | Value is for long packets |

### IW_SCAN (20)

| Name | Value | Comment |
|------|-------|---------|
| `IW_SCAN_DEFAULT` | `0x0000` | Default scan of the driver |
| `IW_SCAN_ALL_ESSID` | `0x0001` | Scan all ESSIDs |
| `IW_SCAN_THIS_ESSID` | `0x0002` | Scan only this ESSID |
| `IW_SCAN_ALL_FREQ` | `0x0004` | Scan all Frequencies |
| `IW_SCAN_THIS_FREQ` | `0x0008` | Scan only this Frequency |
| `IW_SCAN_ALL_MODE` | `0x0010` | Scan all Modes |
| `IW_SCAN_THIS_MODE` | `0x0020` | Scan only this Mode |
| `IW_SCAN_ALL_RATE` | `0x0040` | Scan all Bit-Rates |
| `IW_SCAN_THIS_RATE` | `0x0080` | Scan only this Bit-Rate |
| `IW_SCAN_TYPE_ACTIVE` | `0` |  |
| `IW_SCAN_TYPE_PASSIVE` | `1` |  |
| `IW_SCAN_MAX_DATA` | `4096` | In bytes |
| `IW_SCAN_CAPA_NONE` | `0x00` |  |
| `IW_SCAN_CAPA_ESSID` | `0x01` |  |
| `IW_SCAN_CAPA_BSSID` | `0x02` |  |
| `IW_SCAN_CAPA_CHANNEL` | `0x04` |  |
| `IW_SCAN_CAPA_MODE` | `0x08` |  |
| `IW_SCAN_CAPA_RATE` | `0x10` |  |
| `IW_SCAN_CAPA_TYPE` | `0x20` |  |
| `IW_SCAN_CAPA_TIME` | `0x40` |  |

### IW_TXPOW (5)

| Name | Value | Comment |
|------|-------|---------|
| `IW_TXPOW_TYPE` | `0x00FF` | Type of value |
| `IW_TXPOW_DBM` | `0x0000` | Value is in dBm |
| `IW_TXPOW_MWATT` | `0x0001` | Value is in mW |
| `IW_TXPOW_RELATIVE` | `0x0002` | Value is in arbitrary units |
| `IW_TXPOW_RANGE` | `0x1000` | Range of value between min/max |

### UNCATEGORIZED (67)

| Name | Value | Comment |
|------|-------|---------|
| `WIRELESS_EXT` | `22` |  |
| `SIOCSIWCOMMIT` | `0x8B00` | Commit pending changes to driver |
| `SIOCGIWNAME` | `0x8B01` | get name == wireless protocol |
| `SIOCSIWNWID` | `0x8B02` | set network id (pre-802.11) |
| `SIOCGIWNWID` | `0x8B03` | get network id (the cell) |
| `SIOCSIWFREQ` | `0x8B04` | set channel/frequency (Hz) |
| `SIOCGIWFREQ` | `0x8B05` | get channel/frequency (Hz) |
| `SIOCSIWMODE` | `0x8B06` | set operation mode |
| `SIOCGIWMODE` | `0x8B07` | get operation mode |
| `SIOCSIWSENS` | `0x8B08` | set sensitivity (dBm) |
| `SIOCGIWSENS` | `0x8B09` | get sensitivity (dBm) |
| `SIOCSIWRANGE` | `0x8B0A` | Unused |
| `SIOCGIWRANGE` | `0x8B0B` | Get range of parameters |
| `SIOCSIWPRIV` | `0x8B0C` | Unused |
| `SIOCGIWPRIV` | `0x8B0D` | get private ioctl interface info |
| `SIOCSIWSTATS` | `0x8B0E` | Unused |
| `SIOCGIWSTATS` | `0x8B0F` | Get /proc/net/wireless stats |
| `SIOCSIWSPY` | `0x8B10` | set spy addresses |
| `SIOCGIWSPY` | `0x8B11` | get spy info (quality of link) |
| `SIOCSIWTHRSPY` | `0x8B12` | set spy threshold (spy event) |
| `SIOCGIWTHRSPY` | `0x8B13` | get spy threshold |
| `SIOCSIWAP` | `0x8B14` | set access point MAC addresses |
| `SIOCGIWAP` | `0x8B15` | get access point MAC addresses |
| `SIOCGIWAPLIST` | `0x8B17` | Deprecated in favor of scanning |
| `SIOCSIWSCAN` | `0x8B18` | trigger scanning (list cells) |
| `SIOCGIWSCAN` | `0x8B19` | get scanning results |
| `SIOCSIWESSID` | `0x8B1A` | set ESSID (network name) |
| `SIOCGIWESSID` | `0x8B1B` | get ESSID |
| `SIOCSIWNICKN` | `0x8B1C` | set node name/nickname |
| `SIOCGIWNICKN` | `0x8B1D` | get node name/nickname |
| `SIOCSIWRATE` | `0x8B20` | set default bit rate (bps) |
| `SIOCGIWRATE` | `0x8B21` | get default bit rate (bps) |
| `SIOCSIWRTS` | `0x8B22` | set RTS/CTS threshold (bytes) |
| `SIOCGIWRTS` | `0x8B23` | get RTS/CTS threshold (bytes) |
| `SIOCSIWFRAG` | `0x8B24` | set fragmentation thr (bytes) |
| `SIOCGIWFRAG` | `0x8B25` | get fragmentation thr (bytes) |
| `SIOCSIWTXPOW` | `0x8B26` | set transmit power (dBm) |
| `SIOCGIWTXPOW` | `0x8B27` | get transmit power (dBm) |
| `SIOCSIWRETRY` | `0x8B28` | set retry limits and lifetime |
| `SIOCGIWRETRY` | `0x8B29` | get retry limits and lifetime |
| `SIOCSIWENCODE` | `0x8B2A` | set encoding token & mode |
| `SIOCGIWENCODE` | `0x8B2B` | get encoding token & mode |
| `SIOCSIWPOWER` | `0x8B2C` | set Power Management settings |
| `SIOCGIWPOWER` | `0x8B2D` | get Power Management settings |
| `SIOCSIWGENIE` | `0x8B30` | set generic IE |
| `SIOCGIWGENIE` | `0x8B31` | get generic IE |
| `SIOCSIWMLME` | `0x8B16		/* request MLME operation; uses` |  |
| `SIOCSIWAUTH` | `0x8B32` | set authentication mode params |
| `SIOCGIWAUTH` | `0x8B33` | get authentication mode params |
| `SIOCSIWENCODEEXT` | `0x8B34` | set encoding token & mode |

*...and 17 more*

## Structs (18)


### `struct iw_param`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `value` | `-` |
| `__u8` | `fixed` | `-` |
| `__u8` | `disabled` | `-` |
| `__u16` | `flags` | `-` |

### `struct iw_point`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `length` | `-` |
| `__u16` | `flags` | `-` |

### `struct iw_freq`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `m` | `-` |
| `__s16` | `e` | `-` |
| `__u8` | `i` | `-` |
| `__u8` | `flags` | `-` |

### `struct iw_quality`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `qual` | `-` |
| `__u8` | `level` | `-` |
| `__u8` | `noise` | `-` |
| `__u8` | `updated` | `-` |

### `struct iw_discarded`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nwid` | `-` |
| `__u32` | `code` | `-` |
| `__u32` | `fragment` | `-` |
| `__u32` | `retries` | `-` |
| `__u32` | `misc` | `-` |

### `struct iw_missed`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `beacon` | `-` |

### `struct iw_thrspy`

| Type | Field | Array |
|------|-------|-------|

### `struct iw_scan_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `scan_type` | `-` |
| `__u8` | `essid_len` | `-` |
| `__u8` | `num_channels` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `essid` | `IW_ESSID_MAX_SIZE` |
| `__u32` | `min_channel_time` | `-` |
| `__u32` | `max_channel_time` | `-` |

### `struct iw_encode_ext`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ext_flags` | `-` |
| `__u8` | `tx_seq` | `IW_ENCODE_SEQ_MAX_SIZE` |
| `__u8` | `rx_seq` | `IW_ENCODE_SEQ_MAX_SIZE` |
| `__u16` | `alg` | `-` |
| `__u16` | `key_len` | `-` |

### `struct iw_mlme`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `cmd` | `-` |
| `__u16` | `reason_code` | `-` |

### `struct iw_pmksa`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u8` | `pmkid` | `IW_PMKID_LEN` |

### `struct iw_michaelmicfailure`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u8` | `tsc` | `IW_ENCODE_SEQ_MAX_SIZE` |

### `struct iw_pmkid_cand`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `index` | `-` |

### `struct iw_statistics`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `status` | `-` |

### `struct iwreq`

| Type | Field | Array |
|------|-------|-------|
| `char` | `ifrn_name` | `IFNAMSIZ` |

### `struct iw_range`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `throughput` | `-` |
| `__u32` | `min_nwid` | `-` |
| `__u32` | `max_nwid` | `-` |
| `__u16` | `old_num_channels` | `-` |
| `__u8` | `old_num_frequency` | `-` |
| `__u8` | `scan_capa` | `-` |
| `__u32` | `event_capa` | `6` |
| `__s32` | `sensitivity` | `-` |
| `__u8` | `num_bitrates` | `-` |
| `__s32` | `bitrate` | `IW_MAX_BITRATES` |
| `__s32` | `min_rts` | `-` |
| `__s32` | `max_rts` | `-` |
| `__s32` | `min_frag` | `-` |
| `__s32` | `max_frag` | `-` |
| `__s32` | `min_pmp` | `-` |
| `__s32` | `max_pmp` | `-` |
| `__s32` | `min_pmt` | `-` |
| `__s32` | `max_pmt` | `-` |
| `__u16` | `pmp_flags` | `-` |
| `__u16` | `pmt_flags` | `-` |
| `__u16` | `pm_capa` | `-` |
| `__u16` | `encoding_size` | `IW_MAX_ENCODING_SIZES` |
| `__u8` | `num_encoding_sizes` | `-` |
| `__u8` | `max_encoding_tokens` | `-` |
| `__u8` | `encoding_login_index` | `-` |
| `__u16` | `txpower_capa` | `-` |
| `__u8` | `num_txpower` | `-` |
| `__s32` | `txpower` | `IW_MAX_TXPOWER` |
| `__u8` | `we_version_compiled` | `-` |
| `__u8` | `we_version_source` | `-` |
| `__u16` | `retry_capa` | `-` |
| `__u16` | `retry_flags` | `-` |
| `__u16` | `r_time_flags` | `-` |
| `__s32` | `min_retry` | `-` |
| `__s32` | `max_retry` | `-` |
| `__s32` | `min_r_time` | `-` |
| `__s32` | `max_r_time` | `-` |
| `__u16` | `num_channels` | `-` |
| `__u8` | `num_frequency` | `-` |
| `__u32` | `enc_capa` | `-` |

### `struct iw_priv_args`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u16` | `set_args` | `-` |
| `__u16` | `get_args` | `-` |
| `char` | `name` | `IFNAMSIZ` |

### `struct iw_event`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `len` | `-` |
| `__u16` | `cmd` | `-` |