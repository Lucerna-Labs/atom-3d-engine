# if_link.h

**Source:** `if_link.h`


## Includes

- `linux/types.h`
- `linux/netlink.h`

## Defines (79 total)


### BR_STP (1)

| Name | Value | Comment |
|------|-------|---------|
| `BR_STP_MODE_MAX` | `(__BR_STP_MODE_MAX - 1)` |  |

### IFLA_BAREUDP (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_BAREUDP_MAX` | `(__IFLA_BAREUDP_MAX - 1)` |  |

### IFLA_BOND (3)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_BOND_MAX` | `(__IFLA_BOND_MAX - 1)` |  |
| `IFLA_BOND_AD_INFO_MAX` | `(__IFLA_BOND_AD_INFO_MAX - 1)` |  |
| `IFLA_BOND_SLAVE_MAX` | `(__IFLA_BOND_SLAVE_MAX - 1)` |  |

### IFLA_BR (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_BR_MAX` | `(__IFLA_BR_MAX - 1)` |  |

### IFLA_BRPORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_BRPORT_MAX` | `(__IFLA_BRPORT_MAX - 1)` |  |

### IFLA_DSA (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_DSA_MAX` | `(__IFLA_DSA_MAX - 1)` |  |

### IFLA_GENEVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_GENEVE_MAX` | `(__IFLA_GENEVE_MAX - 1)` |  |

### IFLA_GTP (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_GTP_MAX` | `(__IFLA_GTP_MAX - 1)` |  |

### IFLA_HSR (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_HSR_MAX` | `(__IFLA_HSR_MAX - 1)` |  |

### IFLA_INET (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_INET_MAX` | `(__IFLA_INET_MAX - 1)` |  |

### IFLA_INFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_INFO_MAX` | `(__IFLA_INFO_MAX - 1)` |  |

### IFLA_IPOIB (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_IPOIB_MAX` | `(__IFLA_IPOIB_MAX - 1)` |  |

### IFLA_IPVLAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_IPVLAN_MAX` | `(__IFLA_IPVLAN_MAX - 1)` |  |

### IFLA_MACSEC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_MACSEC_MAX` | `(__IFLA_MACSEC_MAX - 1)` |  |

### IFLA_MACVLAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_MACVLAN_MAX` | `(__IFLA_MACVLAN_MAX - 1)` |  |

### IFLA_MCTP (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_MCTP_MAX` | `(__IFLA_MCTP_MAX - 1)` |  |

### IFLA_NETKIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_NETKIT_MAX` | `(__IFLA_NETKIT_MAX - 1)` |  |

### IFLA_OFFLOAD (2)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_OFFLOAD_XSTATS_MAX` | `(__IFLA_OFFLOAD_XSTATS_MAX - 1)` |  |
| `IFLA_OFFLOAD_XSTATS_HW_S_INFO_MAX` | `` |  |

### IFLA_OVPN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_OVPN_MAX` | `(__IFLA_OVPN_MAX - 1)` |  |

### IFLA_PORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_PORT_MAX` | `(__IFLA_PORT_MAX - 1)` |  |

### IFLA_PPP (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_PPP_MAX` | `(__IFLA_PPP_MAX - 1)` |  |

### IFLA_RMNET (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_RMNET_MAX` | `(__IFLA_RMNET_MAX - 1)` |  |

### IFLA_STATS (2)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_STATS_MAX` | `(__IFLA_STATS_MAX - 1)` |  |
| `IFLA_STATS_GETSET_MAX` | `(__IFLA_STATS_GETSET_MAX - 1)` |  |

### IFLA_TUN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_TUN_MAX` | `(__IFLA_TUN_MAX - 1)` |  |

### IFLA_VF (5)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_VF_INFO_MAX` | `(__IFLA_VF_INFO_MAX - 1)` |  |
| `IFLA_VF_MAX` | `(__IFLA_VF_MAX - 1)` |  |
| `IFLA_VF_VLAN_INFO_MAX` | `(__IFLA_VF_VLAN_INFO_MAX - 1)` |  |
| `IFLA_VF_STATS_MAX` | `(__IFLA_VF_STATS_MAX - 1)` |  |
| `IFLA_VF_PORT_MAX` | `(__IFLA_VF_PORT_MAX - 1)` |  |

### IFLA_VLAN (2)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_VLAN_MAX` | `(__IFLA_VLAN_MAX - 1)` |  |
| `IFLA_VLAN_QOS_MAX` | `(__IFLA_VLAN_QOS_MAX - 1)` |  |

### IFLA_VRF (2)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_VRF_MAX` | `(__IFLA_VRF_MAX - 1)` |  |
| `IFLA_VRF_PORT_MAX` | `(__IFLA_VRF_PORT_MAX - 1)` |  |

### IFLA_VXLAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_VXLAN_MAX` | `(__IFLA_VXLAN_MAX - 1)` |  |

### IFLA_XDP (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_XDP_MAX` | `(__IFLA_XDP_MAX - 1)` |  |

### IFLA_XFRM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_XFRM_MAX` | `(__IFLA_XFRM_MAX - 1)` |  |

### IPVLAN_F (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPVLAN_F_PRIVATE` | `0x01` |  |
| `IPVLAN_F_VEPA` | `0x02` |  |

### LINK_XSTATS (1)

| Name | Value | Comment |
|------|-------|---------|
| `LINK_XSTATS_TYPE_MAX` | `(__LINK_XSTATS_TYPE_MAX - 1)` |  |

### MACVLAN_FLAG (2)

| Name | Value | Comment |
|------|-------|---------|
| `MACVLAN_FLAG_NOPROMISC` | `1` |  |
| `MACVLAN_FLAG_NODST` | `2` | skip dst macvlan if matching src macvlan |

### MAX_VLAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_VLAN_LIST_LEN` | `1` |  |

### PORT_PROFILE (1)

| Name | Value | Comment |
|------|-------|---------|
| `PORT_PROFILE_MAX` | `40` |  |

### PORT_SELF (1)

| Name | Value | Comment |
|------|-------|---------|
| `PORT_SELF_VF` | `-1` |  |

### PORT_UUID (1)

| Name | Value | Comment |
|------|-------|---------|
| `PORT_UUID_MAX` | `16` |  |

### RMNET_FLAGS (6)

| Name | Value | Comment |
|------|-------|---------|
| `RMNET_FLAGS_INGRESS_DEAGGREGATION` | `(1U << 0)` |  |
| `RMNET_FLAGS_INGRESS_MAP_COMMANDS` | `(1U << 1)` |  |
| `RMNET_FLAGS_INGRESS_MAP_CKSUMV4` | `(1U << 2)` |  |
| `RMNET_FLAGS_EGRESS_MAP_CKSUMV4` | `(1U << 3)` |  |
| `RMNET_FLAGS_INGRESS_MAP_CKSUMV5` | `(1U << 4)` |  |
| `RMNET_FLAGS_EGRESS_MAP_CKSUMV5` | `(1U << 5)` |  |

### TUNNEL_MSG (2)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_MSG_FLAG_STATS` | `0x01` |  |
| `TUNNEL_MSG_VALID_USER_FLAGS` | `TUNNEL_MSG_FLAG_STATS` |  |

### UNCATEGORIZED (12)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_COST` | `IFLA_COST` |  |
| `IFLA_PRIORITY` | `IFLA_PRIORITY` |  |
| `IFLA_MASTER` | `IFLA_MASTER` |  |
| `IFLA_WIRELESS` | `IFLA_WIRELESS` |  |
| `IFLA_PROTINFO` | `IFLA_PROTINFO` |  |
| `IFLA_TXQLEN` | `IFLA_TXQLEN` |  |
| `IFLA_MAP` | `IFLA_MAP` |  |
| `IFLA_WEIGHT` | `IFLA_WEIGHT` |  |
| `IFLA_LINKINFO` | `IFLA_LINKINFO` |  |
| `IFLA_PROMISCUITY` | `IFLA_PROMISCUITY` |  |
| `IFLA_MAX` | `(__IFLA_MAX - 1)` |  |
| `IFLA_INET6_MAX` | `(__IFLA_INET6_MAX - 1)` |  |

### VNIFILTER_ENTRY (1)

| Name | Value | Comment |
|------|-------|---------|
| `VNIFILTER_ENTRY_STATS_MAX` | `(__VNIFILTER_ENTRY_STATS_MAX - 1)` |  |

### VXLAN_VNIFILTER (2)

| Name | Value | Comment |
|------|-------|---------|
| `VXLAN_VNIFILTER_ENTRY_MAX` | `(__VXLAN_VNIFILTER_ENTRY_MAX - 1)` |  |
| `VXLAN_VNIFILTER_MAX` | `(__VXLAN_VNIFILTER_MAX - 1)` |  |

### XDP_FLAGS (7)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_FLAGS_UPDATE_IF_NOEXIST` | `(1U << 0)` |  |
| `XDP_FLAGS_SKB_MODE` | `(1U << 1)` |  |
| `XDP_FLAGS_DRV_MODE` | `(1U << 2)` |  |
| `XDP_FLAGS_HW_MODE` | `(1U << 3)` |  |
| `XDP_FLAGS_REPLACE` | `(1U << 4)` |  |
| `XDP_FLAGS_MODES` | `(XDP_FLAGS_SKB_MODE \| ` |  |
| `XDP_FLAGS_MASK` | `(XDP_FLAGS_UPDATE_IF_NOEXIST \| ` |  |

## Structs (25)


### `struct rtnl_link_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rx_packets` | `-` |
| `__u32` | `tx_packets` | `-` |
| `__u32` | `rx_bytes` | `-` |
| `__u32` | `tx_bytes` | `-` |
| `__u32` | `rx_errors` | `-` |
| `__u32` | `tx_errors` | `-` |
| `__u32` | `rx_dropped` | `-` |
| `__u32` | `tx_dropped` | `-` |
| `__u32` | `multicast` | `-` |
| `__u32` | `collisions` | `-` |
| `__u32` | `rx_length_errors` | `-` |
| `__u32` | `rx_over_errors` | `-` |
| `__u32` | `rx_crc_errors` | `-` |
| `__u32` | `rx_frame_errors` | `-` |
| `__u32` | `rx_fifo_errors` | `-` |
| `__u32` | `rx_missed_errors` | `-` |
| `__u32` | `tx_aborted_errors` | `-` |
| `__u32` | `tx_carrier_errors` | `-` |
| `__u32` | `tx_fifo_errors` | `-` |
| `__u32` | `tx_heartbeat_errors` | `-` |
| `__u32` | `tx_window_errors` | `-` |
| `__u32` | `rx_compressed` | `-` |
| `__u32` | `tx_compressed` | `-` |
| `__u32` | `rx_nohandler` | `-` |

### `struct rtnl_link_stats64`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rx_packets` | `-` |
| `__u64` | `tx_packets` | `-` |
| `__u64` | `rx_bytes` | `-` |
| `__u64` | `tx_bytes` | `-` |
| `__u64` | `rx_errors` | `-` |
| `__u64` | `tx_errors` | `-` |
| `__u64` | `rx_dropped` | `-` |
| `__u64` | `tx_dropped` | `-` |
| `__u64` | `multicast` | `-` |
| `__u64` | `collisions` | `-` |
| `__u64` | `rx_length_errors` | `-` |
| `__u64` | `rx_over_errors` | `-` |
| `__u64` | `rx_crc_errors` | `-` |
| `__u64` | `rx_frame_errors` | `-` |
| `__u64` | `rx_fifo_errors` | `-` |
| `__u64` | `rx_missed_errors` | `-` |
| `__u64` | `tx_aborted_errors` | `-` |
| `__u64` | `tx_carrier_errors` | `-` |
| `__u64` | `tx_fifo_errors` | `-` |
| `__u64` | `tx_heartbeat_errors` | `-` |
| `__u64` | `tx_window_errors` | `-` |
| `__u64` | `rx_compressed` | `-` |
| `__u64` | `tx_compressed` | `-` |
| `__u64` | `rx_nohandler` | `-` |
| `__u64` | `rx_otherhost_dropped` | `-` |

### `struct rtnl_hw_stats64`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rx_packets` | `-` |
| `__u64` | `tx_packets` | `-` |
| `__u64` | `rx_bytes` | `-` |
| `__u64` | `tx_bytes` | `-` |
| `__u64` | `rx_errors` | `-` |
| `__u64` | `tx_errors` | `-` |
| `__u64` | `rx_dropped` | `-` |
| `__u64` | `tx_dropped` | `-` |
| `__u64` | `multicast` | `-` |

### `struct rtnl_link_ifmap`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mem_start` | `-` |
| `__u64` | `mem_end` | `-` |
| `__u64` | `base_addr` | `-` |
| `__u16` | `irq` | `-` |
| `__u8` | `dma` | `-` |
| `__u8` | `port` | `-` |

### `struct ifla_bridge_id`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `prio` | `2` |
| `__u8` | `addr` | `6` |

### `struct ifla_cacheinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_reasm_len` | `-` |
| `__u32` | `tstamp` | `-` |
| `__u32` | `reachable_time` | `-` |
| `__u32` | `retrans_time` | `-` |

### `struct ifla_vlan_flags`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `mask` | `-` |

### `struct ifla_vlan_qos_mapping`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `from` | `-` |
| `__u32` | `to` | `-` |

### `struct tunnel_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `family` | `-` |
| `__u8` | `flags` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `ifindex` | `-` |

### `struct ifla_vxlan_port_range`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `low` | `-` |
| `__be16` | `high` | `-` |

### `struct ifla_geneve_port_range`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `low` | `-` |
| `__be16` | `high` | `-` |

### `struct ifla_vf_mac`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u8` | `mac` | `32` |

### `struct ifla_vf_broadcast`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `broadcast` | `32` |

### `struct ifla_vf_vlan`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `vlan` | `-` |
| `__u32` | `qos` | `-` |

### `struct ifla_vf_vlan_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `vlan` | `-` |
| `__u32` | `qos` | `-` |
| `__be16` | `vlan_proto` | `-` |

### `struct ifla_vf_tx_rate`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `rate` | `-` |

### `struct ifla_vf_rate`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `min_tx_rate` | `-` |
| `__u32` | `max_tx_rate` | `-` |

### `struct ifla_vf_spoofchk`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `setting` | `-` |

### `struct ifla_vf_guid`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u64` | `guid` | `-` |

### `struct ifla_vf_link_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `link_state` | `-` |

### `struct ifla_vf_rss_query_en`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `setting` | `-` |

### `struct ifla_vf_trust`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vf` | `-` |
| `__u32` | `setting` | `-` |

### `struct ifla_port_vsi`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `vsi_mgr_id` | `-` |
| `__u8` | `vsi_type_id` | `3` |
| `__u8` | `vsi_type_version` | `-` |
| `__u8` | `pad` | `3` |

### `struct if_stats_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `family` | `-` |
| `__u8` | `pad1` | `-` |
| `__u16` | `pad2` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `filter_mask` | `-` |

### `struct ifla_rmnet_flags`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `mask` | `-` |