# rtnetlink.h

**Source:** `rtnetlink.h`


## Includes

- `linux/types.h`
- `linux/netlink.h`
- `linux/if_link.h`
- `linux/if_addr.h`
- `linux/neighbour.h`

## Defines (227 total)


### RTAX_CC (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTAX_CC_ALGO` | `RTAX_CC_ALGO` |  |

### RTAX_FASTOPEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTAX_FASTOPEN_NO_COOKIE` | `RTAX_FASTOPEN_NO_COOKIE` |  |

### RTAX_FEATURE (6)

| Name | Value | Comment |
|------|-------|---------|
| `RTAX_FEATURE_ECN` | `(1 << 0)` |  |
| `RTAX_FEATURE_SACK` | `(1 << 1)` | unused |
| `RTAX_FEATURE_TIMESTAMP` | `(1 << 2)` | unused |
| `RTAX_FEATURE_ALLFRAG` | `(1 << 3)` | unused |
| `RTAX_FEATURE_TCP_USEC_TS` | `(1 << 4)` |  |
| `RTAX_FEATURE_MASK` | `(RTAX_FEATURE_ECN \|		` |  |

### RTAX_RTO (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTAX_RTO_MIN` | `RTAX_RTO_MIN` |  |

### RTEXT_FILTER (8)

| Name | Value | Comment |
|------|-------|---------|
| `RTEXT_FILTER_VF` | `(1 << 0)` |  |
| `RTEXT_FILTER_BRVLAN` | `(1 << 1)` |  |
| `RTEXT_FILTER_BRVLAN_COMPRESSED` | `(1 << 2)` |  |
| `RTEXT_FILTER_SKIP_STATS` | `(1 << 3)` |  |
| `RTEXT_FILTER_MRP` | `(1 << 4)` |  |
| `RTEXT_FILTER_CFM_CONFIG` | `(1 << 5)` |  |
| `RTEXT_FILTER_CFM_STATUS` | `(1 << 6)` |  |
| `RTEXT_FILTER_MST` | `(1 << 7)` |  |

### RTM_F (9)

| Name | Value | Comment |
|------|-------|---------|
| `RTM_F_NOTIFY` | `0x100` | Notify user of route change |
| `RTM_F_CLONED` | `0x200` | This route is cloned |
| `RTM_F_EQUALIZE` | `0x400` | Multipath equalizer: NI |
| `RTM_F_PREFIX` | `0x800` | Prefix addresses |
| `RTM_F_LOOKUP_TABLE` | `0x1000` | set rtm_table to FIB lookup result |
| `RTM_F_FIB_MATCH` | `0x2000` | return full fib lookup match |
| `RTM_F_OFFLOAD` | `0x4000` | route is offloaded |
| `RTM_F_TRAP` | `0x8000` | route is trapping packets |
| `RTM_F_OFFLOAD_FAILED` | `0x20000000 /* route offload failed, this value` |  |

### RTM_NR (2)

| Name | Value | Comment |
|------|-------|---------|
| `RTM_NR_MSGTYPES` | `(RTM_MAX + 1 - RTM_BASE)` |  |
| `RTM_NR_FAMILIES` | `(RTM_NR_MSGTYPES >> 2)` |  |

### RTNETLINK_HAVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTNETLINK_HAVE_PEERINFO` | `1` |  |

### RTNH_COMPARE (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTNH_COMPARE_MASK` | `(RTNH_F_DEAD \| RTNH_F_LINKDOWN \| ` |  |

### RTNH_F (7)

| Name | Value | Comment |
|------|-------|---------|
| `RTNH_F_DEAD` | `1` | Nexthop is dead (used by multipath) |
| `RTNH_F_PERVASIVE` | `2` | Do recursive gateway lookup |
| `RTNH_F_ONLINK` | `4` | Gateway is forced on link |
| `RTNH_F_OFFLOAD` | `8` | Nexthop is offloaded |
| `RTNH_F_LINKDOWN` | `16` | carrier-down on nexthop |
| `RTNH_F_UNRESOLVED` | `32` | The entry is unresolved (ipmr) |
| `RTNH_F_TRAP` | `64` | Nexthop is trapping packets |

### RTNLGRP_MCTP (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTNLGRP_MCTP_IFADDR` | `RTNLGRP_MCTP_IFADDR` |  |

### RTNLGRP_MPLS (2)

| Name | Value | Comment |
|------|-------|---------|
| `RTNLGRP_MPLS_ROUTE` | `RTNLGRP_MPLS_ROUTE` |  |
| `RTNLGRP_MPLS_NETCONF` | `RTNLGRP_MPLS_NETCONF` |  |

### RTNLGRP_ND (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTNLGRP_ND_USEROPT` | `RTNLGRP_ND_USEROPT` |  |

### RTNLGRP_PHONET (2)

| Name | Value | Comment |
|------|-------|---------|
| `RTNLGRP_PHONET_IFADDR` | `RTNLGRP_PHONET_IFADDR` |  |
| `RTNLGRP_PHONET_ROUTE` | `RTNLGRP_PHONET_ROUTE` |  |

### RTNL_FAMILY (3)

| Name | Value | Comment |
|------|-------|---------|
| `RTNL_FAMILY_IPMR` | `128` |  |
| `RTNL_FAMILY_IP6MR` | `129` |  |
| `RTNL_FAMILY_MAX` | `129` |  |

### TCA_ACT (3)

| Name | Value | Comment |
|------|-------|---------|
| `TCA_ACT_TAB` | `TCA_ROOT_TAB` |  |
| `TCA_ACT_FLAG_LARGE_DUMP_ON` | `TCA_FLAG_LARGE_DUMP_ON` |  |
| `TCA_ACT_FLAG_TERSE_DUMP` | `(1 << 1)` |  |

### TCA_DUMP (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCA_DUMP_FLAGS_TERSE` | `(1 << 0) /* Means that in dump user gets only basic` |  |

### TCA_FLAG (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCA_FLAG_LARGE_DUMP_ON` | `(1 << 0)` |  |

### TCA_ROOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCA_ROOT_MAX` | `(__TCA_ROOT_MAX - 1)` |  |

### TCM_IFINDEX (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCM_IFINDEX_MAGIC_BLOCK` | `(0xFFFFFFFFU)` |  |

### UNCATEGORIZED (174)

| Name | Value | Comment |
|------|-------|---------|
| `RTM_BASE` | `RTM_BASE` |  |
| `RTM_NEWLINK` | `RTM_NEWLINK` |  |
| `RTM_DELLINK` | `RTM_DELLINK` |  |
| `RTM_GETLINK` | `RTM_GETLINK` |  |
| `RTM_SETLINK` | `RTM_SETLINK` |  |
| `RTM_NEWADDR` | `RTM_NEWADDR` |  |
| `RTM_DELADDR` | `RTM_DELADDR` |  |
| `RTM_GETADDR` | `RTM_GETADDR` |  |
| `RTM_NEWROUTE` | `RTM_NEWROUTE` |  |
| `RTM_DELROUTE` | `RTM_DELROUTE` |  |
| `RTM_GETROUTE` | `RTM_GETROUTE` |  |
| `RTM_NEWNEIGH` | `RTM_NEWNEIGH` |  |
| `RTM_DELNEIGH` | `RTM_DELNEIGH` |  |
| `RTM_GETNEIGH` | `RTM_GETNEIGH` |  |
| `RTM_NEWRULE` | `RTM_NEWRULE` |  |
| `RTM_DELRULE` | `RTM_DELRULE` |  |
| `RTM_GETRULE` | `RTM_GETRULE` |  |
| `RTM_NEWQDISC` | `RTM_NEWQDISC` |  |
| `RTM_DELQDISC` | `RTM_DELQDISC` |  |
| `RTM_GETQDISC` | `RTM_GETQDISC` |  |
| `RTM_NEWTCLASS` | `RTM_NEWTCLASS` |  |
| `RTM_DELTCLASS` | `RTM_DELTCLASS` |  |
| `RTM_GETTCLASS` | `RTM_GETTCLASS` |  |
| `RTM_NEWTFILTER` | `RTM_NEWTFILTER` |  |
| `RTM_DELTFILTER` | `RTM_DELTFILTER` |  |
| `RTM_GETTFILTER` | `RTM_GETTFILTER` |  |
| `RTM_NEWACTION` | `RTM_NEWACTION` |  |
| `RTM_DELACTION` | `RTM_DELACTION` |  |
| `RTM_GETACTION` | `RTM_GETACTION` |  |
| `RTM_NEWPREFIX` | `RTM_NEWPREFIX` |  |
| `RTM_NEWMULTICAST` | `RTM_NEWMULTICAST` |  |
| `RTM_DELMULTICAST` | `RTM_DELMULTICAST` |  |
| `RTM_GETMULTICAST` | `RTM_GETMULTICAST` |  |
| `RTM_NEWANYCAST` | `RTM_NEWANYCAST` |  |
| `RTM_DELANYCAST` | `RTM_DELANYCAST` |  |
| `RTM_GETANYCAST` | `RTM_GETANYCAST` |  |
| `RTM_NEWNEIGHTBL` | `RTM_NEWNEIGHTBL` |  |
| `RTM_GETNEIGHTBL` | `RTM_GETNEIGHTBL` |  |
| `RTM_SETNEIGHTBL` | `RTM_SETNEIGHTBL` |  |
| `RTM_NEWNDUSEROPT` | `RTM_NEWNDUSEROPT` |  |
| `RTM_NEWADDRLABEL` | `RTM_NEWADDRLABEL` |  |
| `RTM_DELADDRLABEL` | `RTM_DELADDRLABEL` |  |
| `RTM_GETADDRLABEL` | `RTM_GETADDRLABEL` |  |
| `RTM_GETDCB` | `RTM_GETDCB` |  |
| `RTM_SETDCB` | `RTM_SETDCB` |  |
| `RTM_NEWNETCONF` | `RTM_NEWNETCONF` |  |
| `RTM_DELNETCONF` | `RTM_DELNETCONF` |  |
| `RTM_GETNETCONF` | `RTM_GETNETCONF` |  |
| `RTM_NEWMDB` | `RTM_NEWMDB` |  |
| `RTM_DELMDB` | `RTM_DELMDB` |  |

*...and 124 more*

## Structs (16)


### `struct rtattr`

| Type | Field | Array |
|------|-------|-------|

### `struct rtmsg`

| Type | Field | Array |
|------|-------|-------|
| `unsigned` | `rtm_flags` | `-` |

### `struct rtnexthop`

| Type | Field | Array |
|------|-------|-------|
| `int` | `rtnh_ifindex` | `-` |

### `struct rtvia`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `rtvia_family` | `-` |

### `struct rta_cacheinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rta_clntref` | `-` |
| `__u32` | `rta_lastuse` | `-` |
| `__s32` | `rta_expires` | `-` |
| `__u32` | `rta_error` | `-` |
| `__u32` | `rta_used` | `-` |
| `__u32` | `rta_id` | `-` |
| `__u32` | `rta_ts` | `-` |
| `__u32` | `rta_tsage` | `-` |

### `struct rta_session`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `proto` | `-` |
| `__u8` | `pad1` | `-` |
| `__u16` | `pad2` | `-` |
| `__u16` | `sport` | `-` |
| `__u16` | `dport` | `-` |
| `__u8` | `type` | `-` |
| `__u8` | `code` | `-` |
| `__u16` | `ident` | `-` |
| `__u32` | `spi` | `-` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sport` | `-` |
| `__u16` | `dport` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `code` | `-` |
| `__u16` | `ident` | `-` |

### `struct rta_mfc_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mfcs_packets` | `-` |
| `__u64` | `mfcs_bytes` | `-` |
| `__u64` | `mfcs_wrong_if` | `-` |

### `struct rtgenmsg`

| Type | Field | Array |
|------|-------|-------|

### `struct ifinfomsg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `ifi_index` | `-` |
| `unsigned` | `ifi_flags` | `-` |
| `unsigned` | `ifi_change` | `-` |

### `struct prefixmsg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `prefix_ifindex` | `-` |

### `struct prefix_cacheinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `preferred_time` | `-` |
| `__u32` | `valid_time` | `-` |

### `struct tcmsg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `tcm_ifindex` | `-` |
| `__u32` | `tcm_handle` | `-` |
| `__u32` | `tcm_parent` | `-` |
| `__u32` | `tcm_info` | `-` |

### `struct nduseroptmsg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `nduseropt_ifindex` | `-` |
| `__u8` | `nduseropt_icmp_type` | `-` |
| `__u8` | `nduseropt_icmp_code` | `-` |

### `struct tcamsg`

| Type | Field | Array |
|------|-------|-------|