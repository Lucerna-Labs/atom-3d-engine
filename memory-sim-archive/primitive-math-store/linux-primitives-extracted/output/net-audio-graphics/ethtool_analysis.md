# ethtool.h

**Source:** `ethtool.h`


## Includes

- `linux/const.h`
- `linux/typelimits.h`
- `linux/types.h`
- `linux/if_ether.h`

## Defines (278 total)


### DOWNSHIFT_DEV (2)

| Name | Value | Comment |
|------|-------|---------|
| `DOWNSHIFT_DEV_DEFAULT_COUNT` | `0xff` |  |
| `DOWNSHIFT_DEV_DISABLE` | `0` |  |

### ETHTOOL_BUSINFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_BUSINFO_LEN` | `32` |  |

### ETHTOOL_EROMVERS (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_EROMVERS_LEN` | `32` |  |

### ETHTOOL_F (3)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_F_UNSUPPORTED` | `(1 << ETHTOOL_F_UNSUPPORTED__BIT)` |  |
| `ETHTOOL_F_WISH` | `(1 << ETHTOOL_F_WISH__BIT)` |  |
| `ETHTOOL_F_COMPAT` | `(1 << ETHTOOL_F_COMPAT__BIT)` |  |

### ETHTOOL_FEC (6)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_FEC_NONE` | `(1 << ETHTOOL_FEC_NONE_BIT)` |  |
| `ETHTOOL_FEC_AUTO` | `(1 << ETHTOOL_FEC_AUTO_BIT)` |  |
| `ETHTOOL_FEC_OFF` | `(1 << ETHTOOL_FEC_OFF_BIT)` |  |
| `ETHTOOL_FEC_RS` | `(1 << ETHTOOL_FEC_RS_BIT)` |  |
| `ETHTOOL_FEC_BASER` | `(1 << ETHTOOL_FEC_BASER_BIT)` |  |
| `ETHTOOL_FEC_LLRS` | `(1 << ETHTOOL_FEC_LLRS_BIT)` |  |

### ETHTOOL_FLASH (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_FLASH_MAX_FILENAME` | `128` |  |

### ETHTOOL_FWVERS (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_FWVERS_LEN` | `32` |  |

### ETHTOOL_GET (3)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_GET_DUMP_FLAG` | `0x0000003f` | Get dump settings |
| `ETHTOOL_GET_DUMP_DATA` | `0x00000040` | Get dump data |
| `ETHTOOL_GET_TS_INFO` | `0x00000041` | Get time stamping and PHC info |

### ETHTOOL_GSSET (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_GSSET_INFO` | `0x00000037` | Get string set info |

### ETHTOOL_NWAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_NWAY_RST` | `0x00000009` | Restart autonegotiation. |

### ETHTOOL_PHY (7)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_PHY_FAST_LINK_DOWN_ON` | `0` |  |
| `ETHTOOL_PHY_FAST_LINK_DOWN_OFF` | `0xff` |  |
| `ETHTOOL_PHY_EDPD_DFLT_TX_MSECS` | `0xffff` |  |
| `ETHTOOL_PHY_EDPD_NO_TX` | `0xfffe` |  |
| `ETHTOOL_PHY_EDPD_DISABLE` | `0` |  |
| `ETHTOOL_PHY_GTUNABLE` | `0x0000004e` | Get PHY tunable configuration |
| `ETHTOOL_PHY_STUNABLE` | `0x0000004f` | Set PHY tunable configuration |

### ETHTOOL_PHYS (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_PHYS_ID` | `0x0000001c` | identify the NIC |

### ETHTOOL_RX (3)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_RX_FLOW_SPEC_RING` | `0x00000000FFFFFFFFLL` |  |
| `ETHTOOL_RX_FLOW_SPEC_RING_VF` | `0x000000FF00000000LL` |  |
| `ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF` | `32` |  |

### ETHTOOL_RXNTUPLE (2)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_RXNTUPLE_ACTION_DROP` | `(-1)` | drop packet |
| `ETHTOOL_RXNTUPLE_ACTION_CLEAR` | `(-2)` | clear filter |

### ETHTOOL_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHTOOL_SET_DUMP` | `0x0000003e` | Set dump settings |

### ETH_FW (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_FW_DUMP_DISABLE` | `0` |  |

### ETH_GSTRING (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_GSTRING_LEN` | `32` |  |

### ETH_MDIO (2)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_MDIO_SUPPORTS_C22` | `1` |  |
| `ETH_MDIO_SUPPORTS_C45` | `2` |  |

### ETH_MODULE (10)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_MODULE_SFF_8079` | `0x1` |  |
| `ETH_MODULE_SFF_8079_LEN` | `256` |  |
| `ETH_MODULE_SFF_8472` | `0x2` |  |
| `ETH_MODULE_SFF_8472_LEN` | `512` |  |
| `ETH_MODULE_SFF_8636` | `0x3` |  |
| `ETH_MODULE_SFF_8636_LEN` | `256` |  |
| `ETH_MODULE_SFF_8436` | `0x4` |  |
| `ETH_MODULE_SFF_8436_LEN` | `256` |  |
| `ETH_MODULE_SFF_8636_MAX_LEN` | `640` |  |
| `ETH_MODULE_SFF_8436_MAX_LEN` | `640` |  |

### ETH_RESET (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_RESET_SHARED_SHIFT` | `16` |  |

### ETH_RX (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_RX_NFC_IP4` | `1` |  |

### ETH_RXFH (2)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_RXFH_CONTEXT_ALLOC` | `0xffffffff` |  |
| `ETH_RXFH_INDIR_NO_CHANGE` | `0xffffffff` |  |

### ETH_TP (4)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_TP_MDI_INVALID` | `0x00` | status: unknown; control: unsupported |
| `ETH_TP_MDI` | `0x01` | status: MDI;     control: force MDI |
| `ETH_TP_MDI_X` | `0x02` | status: MDI-X;   control: force MDI-X |
| `ETH_TP_MDI_AUTO` | `0x03` | control: auto-select |

### FLOW_MAC (1)

| Name | Value | Comment |
|------|-------|---------|
| `FLOW_MAC_EXT` | `0x40000000` |  |

### MASTER_SLAVE (11)

| Name | Value | Comment |
|------|-------|---------|
| `MASTER_SLAVE_CFG_UNSUPPORTED` | `0` |  |
| `MASTER_SLAVE_CFG_UNKNOWN` | `1` |  |
| `MASTER_SLAVE_CFG_MASTER_PREFERRED` | `2` |  |
| `MASTER_SLAVE_CFG_SLAVE_PREFERRED` | `3` |  |
| `MASTER_SLAVE_CFG_MASTER_FORCE` | `4` |  |
| `MASTER_SLAVE_CFG_SLAVE_FORCE` | `5` |  |
| `MASTER_SLAVE_STATE_UNSUPPORTED` | `0` |  |
| `MASTER_SLAVE_STATE_UNKNOWN` | `1` |  |
| `MASTER_SLAVE_STATE_MASTER` | `2` |  |
| `MASTER_SLAVE_STATE_SLAVE` | `3` |  |
| `MASTER_SLAVE_STATE_ERR` | `4` |  |

### MAX_NUM (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_NUM_QUEUE` | `4096` |  |

### PFC_STORM (2)

| Name | Value | Comment |
|------|-------|---------|
| `PFC_STORM_PREVENTION_AUTO` | `0xffff` |  |
| `PFC_STORM_PREVENTION_DISABLE` | `0` |  |

### RATE_MATCH (4)

| Name | Value | Comment |
|------|-------|---------|
| `RATE_MATCH_NONE` | `0` |  |
| `RATE_MATCH_PAUSE` | `1` |  |
| `RATE_MATCH_CRS` | `2` |  |
| `RATE_MATCH_OPEN_LOOP` | `3` |  |

### RXH_GTP (1)

| Name | Value | Comment |
|------|-------|---------|
| `RXH_GTP_TEID` | `(1 << 8)` | teid in case of GTP |

### RXH_IP (2)

| Name | Value | Comment |
|------|-------|---------|
| `RXH_IP_SRC` | `(1 << 4)` |  |
| `RXH_IP_DST` | `(1 << 5)` |  |

### RXH_XFRM (3)

| Name | Value | Comment |
|------|-------|---------|
| `RXH_XFRM_SYM_XOR` | `(1 << 0)` |  |
| `RXH_XFRM_SYM_OR_XOR` | `(1 << 1)` |  |
| `RXH_XFRM_NO_CHANGE` | `0xff` |  |

### RX_CLS (6)

| Name | Value | Comment |
|------|-------|---------|
| `RX_CLS_FLOW_DISC` | `0xffffffffffffffffULL` |  |
| `RX_CLS_FLOW_WAKE` | `0xfffffffffffffffeULL` |  |
| `RX_CLS_LOC_SPECIAL` | `0x80000000` | flag |
| `RX_CLS_LOC_ANY` | `0xffffffff` |  |
| `RX_CLS_LOC_FIRST` | `0xfffffffe` |  |
| `RX_CLS_LOC_LAST` | `0xfffffffd` |  |

### SPARC_ETH (2)

| Name | Value | Comment |
|------|-------|---------|
| `SPARC_ETH_GSET` | `ETHTOOL_GSET` |  |
| `SPARC_ETH_SSET` | `ETHTOOL_SSET` |  |

### UNCATEGORIZED (188)

| Name | Value | Comment |
|------|-------|---------|
| `SOPASS_MAX` | `6` |  |
| `ETHTOOL_GSET` | `0x00000001 /* DEPRECATED, Get settings.` |  |
| `ETHTOOL_SSET` | `0x00000002 /* DEPRECATED, Set settings.` |  |
| `ETHTOOL_GDRVINFO` | `0x00000003` | Get driver info. |
| `ETHTOOL_GREGS` | `0x00000004` | Get NIC registers. |
| `ETHTOOL_GWOL` | `0x00000005` | Get wake-on-lan options. |
| `ETHTOOL_SWOL` | `0x00000006` | Set wake-on-lan options. |
| `ETHTOOL_GMSGLVL` | `0x00000007` | Get driver message level |
| `ETHTOOL_SMSGLVL` | `0x00000008` | Set driver msg level. |
| `ETHTOOL_GLINK` | `0x0000000a` |  |
| `ETHTOOL_GEEPROM` | `0x0000000b` | Get EEPROM data |
| `ETHTOOL_SEEPROM` | `0x0000000c` | Set EEPROM data. |
| `ETHTOOL_GCOALESCE` | `0x0000000e` | Get coalesce config |
| `ETHTOOL_SCOALESCE` | `0x0000000f` | Set coalesce config. |
| `ETHTOOL_GRINGPARAM` | `0x00000010` | Get ring parameters |
| `ETHTOOL_SRINGPARAM` | `0x00000011` | Set ring parameters. |
| `ETHTOOL_GPAUSEPARAM` | `0x00000012` | Get pause parameters |
| `ETHTOOL_SPAUSEPARAM` | `0x00000013` | Set pause parameters. |
| `ETHTOOL_GRXCSUM` | `0x00000014` | Get RX hw csum enable (ethtool_value) |
| `ETHTOOL_SRXCSUM` | `0x00000015` | Set RX hw csum enable (ethtool_value) |
| `ETHTOOL_GTXCSUM` | `0x00000016` | Get TX hw csum enable (ethtool_value) |
| `ETHTOOL_STXCSUM` | `0x00000017` | Set TX hw csum enable (ethtool_value) |
| `ETHTOOL_GSG` | `0x00000018 /* Get scatter-gather enable` |  |
| `ETHTOOL_SSG` | `0x00000019 /* Set scatter-gather enable` |  |
| `ETHTOOL_TEST` | `0x0000001a` | execute NIC self-test. |
| `ETHTOOL_GSTRINGS` | `0x0000001b` | get specified string set |
| `ETHTOOL_GSTATS` | `0x0000001d` | get NIC-specific statistics |
| `ETHTOOL_GTSO` | `0x0000001e` | Get TSO enable (ethtool_value) |
| `ETHTOOL_STSO` | `0x0000001f` | Set TSO enable (ethtool_value) |
| `ETHTOOL_GPERMADDR` | `0x00000020` | Get permanent hardware address |
| `ETHTOOL_GUFO` | `0x00000021` | Get UFO enable (ethtool_value) |
| `ETHTOOL_SUFO` | `0x00000022` | Set UFO enable (ethtool_value) |
| `ETHTOOL_GGSO` | `0x00000023` | Get GSO enable (ethtool_value) |
| `ETHTOOL_SGSO` | `0x00000024` | Set GSO enable (ethtool_value) |
| `ETHTOOL_GFLAGS` | `0x00000025` | Get flags bitmap(ethtool_value) |
| `ETHTOOL_SFLAGS` | `0x00000026` | Set flags bitmap(ethtool_value) |
| `ETHTOOL_GPFLAGS` | `0x00000027` | Get driver-private flags bitmap |
| `ETHTOOL_SPFLAGS` | `0x00000028` | Set driver-private flags bitmap |
| `ETHTOOL_GRXFH` | `0x00000029` | Get RX flow hash configuration |
| `ETHTOOL_SRXFH` | `0x0000002a` | Set RX flow hash configuration |
| `ETHTOOL_GGRO` | `0x0000002b` | Get GRO enable (ethtool_value) |
| `ETHTOOL_SGRO` | `0x0000002c` | Set GRO enable (ethtool_value) |
| `ETHTOOL_GRXRINGS` | `0x0000002d` | Get RX rings available for LB |
| `ETHTOOL_GRXCLSRLCNT` | `0x0000002e` | Get RX class rule count |
| `ETHTOOL_GRXCLSRULE` | `0x0000002f` | Get RX classification rule |
| `ETHTOOL_GRXCLSRLALL` | `0x00000030` | Get all RX classification rule |
| `ETHTOOL_SRXCLSRLDEL` | `0x00000031` | Delete RX classification rule |
| `ETHTOOL_SRXCLSRLINS` | `0x00000032` | Insert RX classification rule |
| `ETHTOOL_FLASHDEV` | `0x00000033` | Flash firmware to device |
| `ETHTOOL_RESET` | `0x00000034` | Reset hardware |

*...and 138 more*

### WOL_MODE (1)

| Name | Value | Comment |
|------|-------|---------|
| `WOL_MODE_COUNT` | `8` |  |

## Structs (41)


### `struct ethtool_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `supported` | `-` |
| `__u32` | `advertising` | `-` |
| `__u16` | `speed` | `-` |
| `__u8` | `duplex` | `-` |
| `__u8` | `port` | `-` |
| `__u8` | `phy_address` | `-` |
| `__u8` | `transceiver` | `-` |
| `__u8` | `autoneg` | `-` |
| `__u8` | `mdio_support` | `-` |
| `__u32` | `maxtxpkt` | `-` |
| `__u32` | `maxrxpkt` | `-` |
| `__u16` | `speed_hi` | `-` |
| `__u8` | `eth_tp_mdix` | `-` |
| `__u8` | `eth_tp_mdix_ctrl` | `-` |
| `__u32` | `lp_advertising` | `-` |
| `__u32` | `reserved` | `2` |

### `struct ethtool_drvinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `char` | `driver` | `32` |
| `char` | `version` | `32` |
| `char` | `fw_version` | `ETHTOOL_FWVERS_LEN` |
| `char` | `bus_info` | `ETHTOOL_BUSINFO_LEN` |
| `char` | `erom_version` | `ETHTOOL_EROMVERS_LEN` |
| `char` | `reserved2` | `12` |
| `__u32` | `n_priv_flags` | `-` |
| `__u32` | `n_stats` | `-` |
| `__u32` | `testinfo_len` | `-` |
| `__u32` | `eedump_len` | `-` |
| `__u32` | `regdump_len` | `-` |

### `struct ethtool_wolinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `supported` | `-` |
| `__u32` | `wolopts` | `-` |
| `__u8` | `sopass` | `SOPASS_MAX` |

### `struct ethtool_value`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `data` | `-` |

### `struct ethtool_tunable`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `type_id` | `-` |
| `__u32` | `len` | `-` |

### `struct ethtool_regs`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `len` | `-` |

### `struct ethtool_eeprom`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `magic` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `len` | `-` |

### `struct ethtool_eee`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `supported` | `-` |
| `__u32` | `advertised` | `-` |
| `__u32` | `lp_advertised` | `-` |
| `__u32` | `eee_active` | `-` |
| `__u32` | `eee_enabled` | `-` |
| `__u32` | `tx_lpi_enabled` | `-` |
| `__u32` | `tx_lpi_timer` | `-` |
| `__u32` | `reserved` | `2` |

### `struct ethtool_modinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `eeprom_len` | `-` |
| `__u32` | `reserved` | `8` |

### `struct ethtool_coalesce`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `rx_coalesce_usecs` | `-` |
| `__u32` | `rx_max_coalesced_frames` | `-` |
| `__u32` | `rx_coalesce_usecs_irq` | `-` |
| `__u32` | `rx_max_coalesced_frames_irq` | `-` |
| `__u32` | `tx_coalesce_usecs` | `-` |
| `__u32` | `tx_max_coalesced_frames` | `-` |
| `__u32` | `tx_coalesce_usecs_irq` | `-` |
| `__u32` | `tx_max_coalesced_frames_irq` | `-` |
| `__u32` | `stats_block_coalesce_usecs` | `-` |
| `__u32` | `use_adaptive_rx_coalesce` | `-` |
| `__u32` | `use_adaptive_tx_coalesce` | `-` |
| `__u32` | `pkt_rate_low` | `-` |
| `__u32` | `rx_coalesce_usecs_low` | `-` |
| `__u32` | `rx_max_coalesced_frames_low` | `-` |
| `__u32` | `tx_coalesce_usecs_low` | `-` |
| `__u32` | `tx_max_coalesced_frames_low` | `-` |
| `__u32` | `pkt_rate_high` | `-` |
| `__u32` | `rx_coalesce_usecs_high` | `-` |
| `__u32` | `rx_max_coalesced_frames_high` | `-` |
| `__u32` | `tx_coalesce_usecs_high` | `-` |
| `__u32` | `tx_max_coalesced_frames_high` | `-` |
| `__u32` | `rate_sample_interval` | `-` |

### `struct ethtool_ringparam`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `rx_max_pending` | `-` |
| `__u32` | `rx_mini_max_pending` | `-` |
| `__u32` | `rx_jumbo_max_pending` | `-` |
| `__u32` | `tx_max_pending` | `-` |
| `__u32` | `rx_pending` | `-` |
| `__u32` | `rx_mini_pending` | `-` |
| `__u32` | `rx_jumbo_pending` | `-` |
| `__u32` | `tx_pending` | `-` |

### `struct ethtool_channels`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `max_rx` | `-` |
| `__u32` | `max_tx` | `-` |
| `__u32` | `max_other` | `-` |
| `__u32` | `max_combined` | `-` |
| `__u32` | `rx_count` | `-` |
| `__u32` | `tx_count` | `-` |
| `__u32` | `other_count` | `-` |
| `__u32` | `combined_count` | `-` |

### `struct ethtool_pauseparam`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `autoneg` | `-` |
| `__u32` | `rx_pause` | `-` |
| `__u32` | `tx_pause` | `-` |

### `struct ethtool_gstrings`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `string_set` | `-` |
| `__u32` | `len` | `-` |

### `struct ethtool_sset_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `reserved` | `-` |
| `__u64` | `sset_mask` | `-` |

### `struct ethtool_test`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `len` | `-` |

### `struct ethtool_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `n_stats` | `-` |

### `struct ethtool_perm_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `size` | `-` |

### `struct ethtool_tcpip4_spec`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip4src` | `-` |
| `__be32` | `ip4dst` | `-` |
| `__be16` | `psrc` | `-` |
| `__be16` | `pdst` | `-` |
| `__u8` | `tos` | `-` |

### `struct ethtool_ah_espip4_spec`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip4src` | `-` |
| `__be32` | `ip4dst` | `-` |
| `__be32` | `spi` | `-` |
| `__u8` | `tos` | `-` |

### `struct ethtool_usrip4_spec`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip4src` | `-` |
| `__be32` | `ip4dst` | `-` |
| `__be32` | `l4_4_bytes` | `-` |
| `__u8` | `tos` | `-` |
| `__u8` | `ip_ver` | `-` |
| `__u8` | `proto` | `-` |

### `struct ethtool_tcpip6_spec`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip6src` | `4` |
| `__be32` | `ip6dst` | `4` |
| `__be16` | `psrc` | `-` |
| `__be16` | `pdst` | `-` |
| `__u8` | `tclass` | `-` |

### `struct ethtool_ah_espip6_spec`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip6src` | `4` |
| `__be32` | `ip6dst` | `4` |
| `__be32` | `spi` | `-` |
| `__u8` | `tclass` | `-` |

### `struct ethtool_usrip6_spec`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip6src` | `4` |
| `__be32` | `ip6dst` | `4` |
| `__be32` | `l4_4_bytes` | `-` |
| `__u8` | `tclass` | `-` |
| `__u8` | `l4_proto` | `-` |

### `struct ethtool_flow_ext`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `padding` | `2` |
| `__be16` | `vlan_etype` | `-` |
| `__be16` | `vlan_tci` | `-` |
| `__be32` | `data` | `2` |

### `struct ethtool_rx_flow_spec`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flow_type` | `-` |
| `__u64` | `ring_cookie` | `-` |
| `__u32` | `location` | `-` |

### `struct ethtool_rxnfc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `flow_type` | `-` |
| `__u64` | `data` | `-` |
| `__u32` | `rule_cnt` | `-` |
| `__u32` | `rss_context` | `-` |

### `struct ethtool_rxfh_indir`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `size` | `-` |

### `struct ethtool_rxfh`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `rss_context` | `-` |
| `__u32` | `indir_size` | `-` |
| `__u32` | `key_size` | `-` |
| `__u8` | `hfunc` | `-` |
| `__u8` | `input_xfrm` | `-` |
| `__u8` | `rsvd8` | `2` |
| `__u32` | `rsvd32` | `-` |

### `struct ethtool_rx_ntuple_flow_spec`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flow_type` | `-` |
| `__u8` | `hdata` | `72` |
| `__u16` | `vlan_tag` | `-` |
| `__u16` | `vlan_tag_mask` | `-` |
| `__u64` | `data` | `-` |
| `__u64` | `data_mask` | `-` |
| `__s32` | `action` | `-` |

### `struct ethtool_rx_ntuple`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |

### `struct ethtool_flash`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `region` | `-` |
| `char` | `data` | `ETHTOOL_FLASH_MAX_FILENAME` |

### `struct ethtool_dump`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `flag` | `-` |
| `__u32` | `len` | `-` |

### `struct ethtool_get_features_block`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `available` | `-` |
| `__u32` | `requested` | `-` |
| `__u32` | `active` | `-` |
| `__u32` | `never_changed` | `-` |

### `struct ethtool_gfeatures`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `size` | `-` |

### `struct ethtool_set_features_block`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `valid` | `-` |
| `__u32` | `requested` | `-` |

### `struct ethtool_sfeatures`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `size` | `-` |

### `struct ethtool_ts_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `so_timestamping` | `-` |
| `__s32` | `phc_index` | `-` |
| `__u32` | `tx_types` | `-` |
| `__u32` | `tx_reserved` | `3` |
| `__u32` | `rx_filters` | `-` |
| `__u32` | `rx_reserved` | `3` |

### `struct ethtool_per_queue_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `sub_command` | `-` |
| `__u32` | `queue_mask` | `__KERNEL_DIV_ROUND_UP(MAX_NUM_QUEUE, 32)` |

### `struct ethtool_fecparam`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `active_fec` | `-` |
| `__u32` | `fec` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ethtool_link_settings`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `speed` | `-` |
| `__u8` | `duplex` | `-` |
| `__u8` | `port` | `-` |
| `__u8` | `phy_address` | `-` |
| `__u8` | `autoneg` | `-` |
| `__u8` | `mdio_support` | `-` |
| `__u8` | `eth_tp_mdix` | `-` |
| `__u8` | `eth_tp_mdix_ctrl` | `-` |
| `__s8` | `link_mode_masks_nwords` | `-` |
| `__u8` | `transceiver` | `-` |
| `__u8` | `master_slave_cfg` | `-` |
| `__u8` | `master_slave_state` | `-` |
| `__u8` | `rate_matching` | `-` |
| `__u32` | `reserved` | `7` |