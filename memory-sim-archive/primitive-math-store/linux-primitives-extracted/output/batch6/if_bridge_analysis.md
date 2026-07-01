# if_bridge.h

**Source:** `if_bridge.h`


## Includes

- `linux/types.h`
- `linux/if_ether.h`
- `linux/in6.h`

## Defines (97 total)


### BRCTL_ADD (2)

| Name | Value | Comment |
|------|-------|---------|
| `BRCTL_ADD_BRIDGE` | `2` |  |
| `BRCTL_ADD_IF` | `4` |  |

### BRCTL_DEL (2)

| Name | Value | Comment |
|------|-------|---------|
| `BRCTL_DEL_BRIDGE` | `3` |  |
| `BRCTL_DEL_IF` | `5` |  |

### BRCTL_GET (6)

| Name | Value | Comment |
|------|-------|---------|
| `BRCTL_GET_VERSION` | `0` |  |
| `BRCTL_GET_BRIDGES` | `1` |  |
| `BRCTL_GET_BRIDGE_INFO` | `6` |  |
| `BRCTL_GET_PORT_LIST` | `7` |  |
| `BRCTL_GET_PORT_INFO` | `13` |  |
| `BRCTL_GET_FDB_ENTRIES` | `18` |  |

### BRCTL_SET (9)

| Name | Value | Comment |
|------|-------|---------|
| `BRCTL_SET_BRIDGE_FORWARD_DELAY` | `8` |  |
| `BRCTL_SET_BRIDGE_HELLO_TIME` | `9` |  |
| `BRCTL_SET_BRIDGE_MAX_AGE` | `10` |  |
| `BRCTL_SET_AGEING_TIME` | `11` |  |
| `BRCTL_SET_GC_INTERVAL` | `12` |  |
| `BRCTL_SET_BRIDGE_STP_STATE` | `14` |  |
| `BRCTL_SET_BRIDGE_PRIORITY` | `15` |  |
| `BRCTL_SET_PORT_PRIORITY` | `16` |  |
| `BRCTL_SET_PATH_COST` | `17` |  |

### BRIDGE_FLAGS (2)

| Name | Value | Comment |
|------|-------|---------|
| `BRIDGE_FLAGS_MASTER` | `1` | Bridge command to/from master |
| `BRIDGE_FLAGS_SELF` | `2` | Bridge command to/from lowerdev |

### BRIDGE_MODE (3)

| Name | Value | Comment |
|------|-------|---------|
| `BRIDGE_MODE_VEB` | `0` | Default loopback mode |
| `BRIDGE_MODE_VEPA` | `1` | 802.1Qbg defined VEPA mode |
| `BRIDGE_MODE_UNDEF` | `0xFFFF` | mode undefined |

### BRIDGE_QUERIER (1)

| Name | Value | Comment |
|------|-------|---------|
| `BRIDGE_QUERIER_MAX` | `(__BRIDGE_QUERIER_MAX - 1)` |  |

### BRIDGE_VLAN (7)

| Name | Value | Comment |
|------|-------|---------|
| `BRIDGE_VLAN_INFO_MASTER` | `(1<<0)` | Operate on Bridge device as well |
| `BRIDGE_VLAN_INFO_PVID` | `(1<<1)` | VLAN is PVID, ingress untagged |
| `BRIDGE_VLAN_INFO_UNTAGGED` | `(1<<2)` | VLAN egresses untagged |
| `BRIDGE_VLAN_INFO_RANGE_BEGIN` | `(1<<3)` | VLAN is start of vlan range |
| `BRIDGE_VLAN_INFO_RANGE_END` | `(1<<4)` | VLAN is end of vlan range |
| `BRIDGE_VLAN_INFO_BRENTRY` | `(1<<5)` | Global bridge VLAN entry |
| `BRIDGE_VLAN_INFO_ONLY_OPTS` | `(1<<6)` | Skip create/delete/flags |

### BRIDGE_VLANDB (8)

| Name | Value | Comment |
|------|-------|---------|
| `BRIDGE_VLANDB_DUMP_MAX` | `(__BRIDGE_VLANDB_DUMP_MAX - 1)` |  |
| `BRIDGE_VLANDB_DUMPF_STATS` | `(1 << 0)` | Include stats in the dump |
| `BRIDGE_VLANDB_DUMPF_GLOBAL` | `(1 << 1)` | Dump global vlan options only |
| `BRIDGE_VLANDB_MAX` | `(__BRIDGE_VLANDB_MAX - 1)` |  |
| `BRIDGE_VLANDB_ENTRY_MAX` | `(__BRIDGE_VLANDB_ENTRY_MAX - 1)` |  |
| `BRIDGE_VLANDB_TINFO_MAX` | `(__BRIDGE_VLANDB_TINFO_MAX - 1)` |  |
| `BRIDGE_VLANDB_STATS_MAX` | `(__BRIDGE_VLANDB_STATS_MAX - 1)` |  |
| `BRIDGE_VLANDB_GOPTS_MAX` | `(__BRIDGE_VLANDB_GOPTS_MAX - 1)` |  |

### BRIDGE_XSTATS (1)

| Name | Value | Comment |
|------|-------|---------|
| `BRIDGE_XSTATS_MAX` | `(__BRIDGE_XSTATS_MAX - 1)` |  |

### BR_STATE (5)

| Name | Value | Comment |
|------|-------|---------|
| `BR_STATE_DISABLED` | `0` |  |
| `BR_STATE_LISTENING` | `1` |  |
| `BR_STATE_LEARNING` | `2` |  |
| `BR_STATE_FORWARDING` | `3` |  |
| `BR_STATE_BLOCKING` | `4` |  |

### IFLA_BRIDGE (25)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_BRIDGE_MAX` | `(__IFLA_BRIDGE_MAX - 1)` |  |
| `IFLA_BRIDGE_VLAN_TUNNEL_MAX` | `(__IFLA_BRIDGE_VLAN_TUNNEL_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_MAX` | `(__IFLA_BRIDGE_MRP_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_INSTANCE_MAX` | `(__IFLA_BRIDGE_MRP_INSTANCE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_PORT_STATE_MAX` | `(__IFLA_BRIDGE_MRP_PORT_STATE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_PORT_ROLE_MAX` | `(__IFLA_BRIDGE_MRP_PORT_ROLE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_RING_STATE_MAX` | `(__IFLA_BRIDGE_MRP_RING_STATE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_RING_ROLE_MAX` | `(__IFLA_BRIDGE_MRP_RING_ROLE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_START_TEST_MAX` | `(__IFLA_BRIDGE_MRP_START_TEST_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_INFO_MAX` | `(__IFLA_BRIDGE_MRP_INFO_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_IN_STATE_MAX` | `(__IFLA_BRIDGE_MRP_IN_STATE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_IN_ROLE_MAX` | `(__IFLA_BRIDGE_MRP_IN_ROLE_MAX - 1)` |  |
| `IFLA_BRIDGE_MRP_START_IN_TEST_MAX` | `(__IFLA_BRIDGE_MRP_START_IN_TEST_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_MAX` | `(__IFLA_BRIDGE_CFM_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_MEP_CREATE_MAX` | `(__IFLA_BRIDGE_CFM_MEP_CREATE_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_MEP_DELETE_MAX` | `(__IFLA_BRIDGE_CFM_MEP_DELETE_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_MEP_CONFIG_MAX` | `(__IFLA_BRIDGE_CFM_MEP_CONFIG_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_CC_CONFIG_MAX` | `(__IFLA_BRIDGE_CFM_CC_CONFIG_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_CC_PEER_MEP_MAX` | `(__IFLA_BRIDGE_CFM_CC_PEER_MEP_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_CC_RDI_MAX` | `(__IFLA_BRIDGE_CFM_CC_RDI_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_CC_CCM_TX_MAX` | `(__IFLA_BRIDGE_CFM_CC_CCM_TX_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_MEP_STATUS_MAX` | `(__IFLA_BRIDGE_CFM_MEP_STATUS_MAX - 1)` |  |
| `IFLA_BRIDGE_CFM_CC_PEER_STATUS_MAX` | `(__IFLA_BRIDGE_CFM_CC_PEER_STATUS_MAX - 1)` |  |
| `IFLA_BRIDGE_MST_MAX` | `(__IFLA_BRIDGE_MST_MAX - 1)` |  |
| `IFLA_BRIDGE_MST_ENTRY_MAX` | `(__IFLA_BRIDGE_MST_ENTRY_MAX - 1)` |  |

### MDBA_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `MDBA_GET_ENTRY_MAX` | `(__MDBA_GET_ENTRY_MAX - 1)` |  |

### MDBA_MDB (5)

| Name | Value | Comment |
|------|-------|---------|
| `MDBA_MDB_MAX` | `(__MDBA_MDB_MAX - 1)` |  |
| `MDBA_MDB_ENTRY_MAX` | `(__MDBA_MDB_ENTRY_MAX - 1)` |  |
| `MDBA_MDB_EATTR_MAX` | `(__MDBA_MDB_EATTR_MAX - 1)` |  |
| `MDBA_MDB_SRCLIST_MAX` | `(__MDBA_MDB_SRCLIST_MAX - 1)` |  |
| `MDBA_MDB_SRCATTR_MAX` | `(__MDBA_MDB_SRCATTR_MAX - 1)` |  |

### MDBA_ROUTER (2)

| Name | Value | Comment |
|------|-------|---------|
| `MDBA_ROUTER_MAX` | `(__MDBA_ROUTER_MAX - 1)` |  |
| `MDBA_ROUTER_PATTR_MAX` | `(__MDBA_ROUTER_PATTR_MAX - 1)` |  |

### MDBA_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `MDBA_SET_ENTRY_MAX` | `(__MDBA_SET_ENTRY_MAX - 1)` |  |

### MDBE_ATTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `MDBE_ATTR_MAX` | `(__MDBE_ATTR_MAX - 1)` |  |

### MDBE_SRC (1)

| Name | Value | Comment |
|------|-------|---------|
| `MDBE_SRC_LIST_MAX` | `(__MDBE_SRC_LIST_MAX - 1)` |  |

### MDBE_SRCATTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `MDBE_SRCATTR_MAX` | `(__MDBE_SRCATTR_MAX - 1)` |  |

### MDB_FLAGS (5)

| Name | Value | Comment |
|------|-------|---------|
| `MDB_FLAGS_OFFLOAD` | `(1 << 0)` |  |
| `MDB_FLAGS_FAST_LEAVE` | `(1 << 1)` |  |
| `MDB_FLAGS_STAR_EXCL` | `(1 << 2)` |  |
| `MDB_FLAGS_BLOCKED` | `(1 << 3)` |  |
| `MDB_FLAGS_OFFLOAD_FAILED` | `(1 << 4)` |  |

### SYSFS_BRIDGE (5)

| Name | Value | Comment |
|------|-------|---------|
| `SYSFS_BRIDGE_ATTR` | `"bridge"` |  |
| `SYSFS_BRIDGE_FDB` | `"brforward"` |  |
| `SYSFS_BRIDGE_PORT_SUBDIR` | `"brif"` |  |
| `SYSFS_BRIDGE_PORT_ATTR` | `"brport"` |  |
| `SYSFS_BRIDGE_PORT_LINK` | `"bridge"` |  |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `BRCTL_VERSION` | `1` |  |
| `MDBA_MAX` | `(__MDBA_MAX - 1)` |  |
| `MDB_TEMPORARY` | `0` |  |
| `MDB_PERMANENT` | `1` |  |

## Structs (19)


### `struct __bridge_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `designated_root` | `-` |
| `__u64` | `bridge_id` | `-` |
| `__u32` | `root_path_cost` | `-` |
| `__u32` | `max_age` | `-` |
| `__u32` | `hello_time` | `-` |
| `__u32` | `forward_delay` | `-` |
| `__u32` | `bridge_max_age` | `-` |
| `__u32` | `bridge_hello_time` | `-` |
| `__u32` | `bridge_forward_delay` | `-` |
| `__u8` | `topology_change` | `-` |
| `__u8` | `topology_change_detected` | `-` |
| `__u8` | `root_port` | `-` |
| `__u8` | `stp_enabled` | `-` |
| `__u32` | `ageing_time` | `-` |
| `__u32` | `gc_interval` | `-` |
| `__u32` | `hello_timer_value` | `-` |
| `__u32` | `tcn_timer_value` | `-` |
| `__u32` | `topology_change_timer_value` | `-` |
| `__u32` | `gc_timer_value` | `-` |

### `struct __port_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `designated_root` | `-` |
| `__u64` | `designated_bridge` | `-` |
| `__u16` | `port_id` | `-` |
| `__u16` | `designated_port` | `-` |
| `__u32` | `path_cost` | `-` |
| `__u32` | `designated_cost` | `-` |
| `__u8` | `state` | `-` |
| `__u8` | `top_change_ack` | `-` |
| `__u8` | `config_pending` | `-` |
| `__u8` | `unused0` | `-` |
| `__u32` | `message_age_timer_value` | `-` |
| `__u32` | `forward_delay_timer_value` | `-` |
| `__u32` | `hold_timer_value` | `-` |

### `struct __fdb_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `mac_addr` | `ETH_ALEN` |
| `__u8` | `port_no` | `-` |
| `__u8` | `is_local` | `-` |
| `__u32` | `ageing_timer_value` | `-` |
| `__u8` | `port_hi` | `-` |
| `__u8` | `pad0` | `-` |
| `__u16` | `unused` | `-` |

### `struct bridge_vlan_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `flags` | `-` |
| `__u16` | `vid` | `-` |

### `struct bridge_vlan_xstats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rx_bytes` | `-` |
| `__u64` | `rx_packets` | `-` |
| `__u64` | `tx_bytes` | `-` |
| `__u64` | `tx_packets` | `-` |
| `__u16` | `vid` | `-` |
| `__u16` | `flags` | `-` |
| `__u32` | `pad2` | `-` |

### `struct br_mrp_instance`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ring_id` | `-` |
| `__u32` | `p_ifindex` | `-` |
| `__u32` | `s_ifindex` | `-` |
| `__u16` | `prio` | `-` |

### `struct br_mrp_ring_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ring_id` | `-` |
| `__u32` | `ring_state` | `-` |

### `struct br_mrp_ring_role`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ring_id` | `-` |
| `__u32` | `ring_role` | `-` |

### `struct br_mrp_start_test`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ring_id` | `-` |
| `__u32` | `interval` | `-` |
| `__u32` | `max_miss` | `-` |
| `__u32` | `period` | `-` |
| `__u32` | `monitor` | `-` |

### `struct br_mrp_in_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `in_state` | `-` |
| `__u16` | `in_id` | `-` |

### `struct br_mrp_in_role`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ring_id` | `-` |
| `__u32` | `in_role` | `-` |
| `__u32` | `i_ifindex` | `-` |
| `__u16` | `in_id` | `-` |

### `struct br_mrp_start_in_test`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `interval` | `-` |
| `__u32` | `max_miss` | `-` |
| `__u32` | `period` | `-` |
| `__u16` | `in_id` | `-` |

### `struct bridge_stp_xstats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `transition_blk` | `-` |
| `__u64` | `transition_fwd` | `-` |
| `__u64` | `rx_bpdu` | `-` |
| `__u64` | `tx_bpdu` | `-` |
| `__u64` | `rx_tcn` | `-` |
| `__u64` | `tx_tcn` | `-` |

### `struct br_vlan_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `family` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `ifindex` | `-` |

### `struct br_port_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `family` | `-` |
| `__u32` | `ifindex` | `-` |

### `struct br_mdb_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifindex` | `-` |
| `__u8` | `state` | `-` |
| `__u8` | `flags` | `-` |
| `__u16` | `vid` | `-` |
| `__be32` | `ip4` | `-` |
| `__be16` | `proto` | `-` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ip4` | `-` |
| `__be16` | `proto` | `-` |

### `struct br_mcast_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `igmp_v1queries` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_v2queries` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_v3queries` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_leaves` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_v1reports` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_v2reports` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_v3reports` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `igmp_parse_errors` | `-` |
| `__u64` | `mld_v1queries` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `mld_v2queries` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `mld_leaves` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `mld_v1reports` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `mld_v2reports` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `mld_parse_errors` | `-` |
| `__u64` | `mcast_bytes` | `BR_MCAST_DIR_SIZE` |
| `__u64` | `mcast_packets` | `BR_MCAST_DIR_SIZE` |

### `struct br_boolopt_multi`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `optval` | `-` |
| `__u32` | `optmask` | `-` |