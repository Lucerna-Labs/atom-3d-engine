# ip_vs.h

**Source:** `ip_vs.h`


## Includes

- `linux/types.h`

## Defines (77 total)


### IPVS_CMD (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_CMD_MAX` | `(__IPVS_CMD_MAX - 1)` |  |
| `IPVS_CMD_ATTR_MAX` | `(__IPVS_CMD_ATTR_MAX - 1)` |  |

### IPVS_DAEMON (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_DAEMON_ATTR_MAX` | `(__IPVS_DAEMON_ATTR_MAX - 1)` |  |

### IPVS_DEST (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_DEST_ATTR_MAX` | `(__IPVS_DEST_ATTR_MAX - 1)` |  |

### IPVS_GENL (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_GENL_NAME` | `"IPVS"` |  |
| `IPVS_GENL_VERSION` | `0x1` |  |

### IPVS_INFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_INFO_ATTR_MAX` | `(__IPVS_INFO_ATTR_MAX - 1)` |  |

### IPVS_STATS (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_STATS_ATTR_MAX` | `(__IPVS_STATS_ATTR_MAX - 1)` |  |

### IPVS_SVC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPVS_SVC_ATTR_MAX` | `(__IPVS_SVC_ATTR_MAX - 1)` |  |

### IP_VS (68)

| Name | Value | Comment |
|------|-------|---------|
| `IP_VS_VERSION_CODE` | `0x010201` |  |
| `IP_VS_SVC_F_PERSISTENT` | `0x0001` | persistent port |
| `IP_VS_SVC_F_HASHED` | `0x0002` | hashed entry |
| `IP_VS_SVC_F_ONEPACKET` | `0x0004` | one-packet scheduling |
| `IP_VS_SVC_F_SCHED1` | `0x0008` | scheduler flag 1 |
| `IP_VS_SVC_F_SCHED2` | `0x0010` | scheduler flag 2 |
| `IP_VS_SVC_F_SCHED3` | `0x0020` | scheduler flag 3 |
| `IP_VS_SVC_F_SCHED_SH_FALLBACK` | `IP_VS_SVC_F_SCHED1` | SH fallback |
| `IP_VS_SVC_F_SCHED_SH_PORT` | `IP_VS_SVC_F_SCHED2` | SH use port |
| `IP_VS_DEST_F_AVAILABLE` | `0x0001` | server is available |
| `IP_VS_DEST_F_OVERLOAD` | `0x0002` | server is overloaded |
| `IP_VS_STATE_NONE` | `0x0000` | daemon is stopped |
| `IP_VS_STATE_MASTER` | `0x0001` | started as master |
| `IP_VS_STATE_BACKUP` | `0x0002` | started as backup |
| `IP_VS_BASE_CTL` | `(64+1024+64)` | base |
| `IP_VS_SO_SET_NONE` | `IP_VS_BASE_CTL` | just peek |
| `IP_VS_SO_SET_INSERT` | `(IP_VS_BASE_CTL+1)` |  |
| `IP_VS_SO_SET_ADD` | `(IP_VS_BASE_CTL+2)` |  |
| `IP_VS_SO_SET_EDIT` | `(IP_VS_BASE_CTL+3)` |  |
| `IP_VS_SO_SET_DEL` | `(IP_VS_BASE_CTL+4)` |  |
| `IP_VS_SO_SET_FLUSH` | `(IP_VS_BASE_CTL+5)` |  |
| `IP_VS_SO_SET_LIST` | `(IP_VS_BASE_CTL+6)` |  |
| `IP_VS_SO_SET_ADDDEST` | `(IP_VS_BASE_CTL+7)` |  |
| `IP_VS_SO_SET_DELDEST` | `(IP_VS_BASE_CTL+8)` |  |
| `IP_VS_SO_SET_EDITDEST` | `(IP_VS_BASE_CTL+9)` |  |
| `IP_VS_SO_SET_TIMEOUT` | `(IP_VS_BASE_CTL+10)` |  |
| `IP_VS_SO_SET_STARTDAEMON` | `(IP_VS_BASE_CTL+11)` |  |
| `IP_VS_SO_SET_STOPDAEMON` | `(IP_VS_BASE_CTL+12)` |  |
| `IP_VS_SO_SET_RESTORE` | `(IP_VS_BASE_CTL+13)` |  |
| `IP_VS_SO_SET_SAVE` | `(IP_VS_BASE_CTL+14)` |  |
| `IP_VS_SO_SET_ZERO` | `(IP_VS_BASE_CTL+15)` |  |
| `IP_VS_SO_SET_MAX` | `IP_VS_SO_SET_ZERO` |  |
| `IP_VS_SO_GET_VERSION` | `IP_VS_BASE_CTL` |  |
| `IP_VS_SO_GET_INFO` | `(IP_VS_BASE_CTL+1)` |  |
| `IP_VS_SO_GET_SERVICES` | `(IP_VS_BASE_CTL+2)` |  |
| `IP_VS_SO_GET_SERVICE` | `(IP_VS_BASE_CTL+3)` |  |
| `IP_VS_SO_GET_DESTS` | `(IP_VS_BASE_CTL+4)` |  |
| `IP_VS_SO_GET_DEST` | `(IP_VS_BASE_CTL+5)` | not used now |
| `IP_VS_SO_GET_TIMEOUT` | `(IP_VS_BASE_CTL+6)` |  |
| `IP_VS_SO_GET_DAEMON` | `(IP_VS_BASE_CTL+7)` |  |
| `IP_VS_SO_GET_MAX` | `IP_VS_SO_GET_DAEMON` |  |
| `IP_VS_CONN_F_FWD_MASK` | `0x0007` | mask for the fwd methods |
| `IP_VS_CONN_F_MASQ` | `0x0000` | masquerading/NAT |
| `IP_VS_CONN_F_LOCALNODE` | `0x0001` | local node |
| `IP_VS_CONN_F_TUNNEL` | `0x0002` | tunneling |
| `IP_VS_CONN_F_DROUTE` | `0x0003` | direct routing |
| `IP_VS_CONN_F_BYPASS` | `0x0004` | cache bypass |
| `IP_VS_CONN_F_SYNC` | `0x0020` | entry created by sync |
| `IP_VS_CONN_F_HASHED` | `0x0040` | hashed entry |
| `IP_VS_CONN_F_NOOUTPUT` | `0x0080` | no output packets |

*...and 18 more*

## Structs (11)


### `struct ip_vs_service_user`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `protocol` | `-` |
| `__be32` | `addr` | `-` |
| `__be16` | `port` | `-` |
| `__u32` | `fwmark` | `-` |
| `char` | `sched_name` | `IP_VS_SCHEDNAME_MAXLEN` |
| `__be32` | `netmask` | `-` |

### `struct ip_vs_dest_user`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `addr` | `-` |
| `__be16` | `port` | `-` |
| `int` | `weight` | `-` |
| `__u32` | `u_threshold` | `-` |
| `__u32` | `l_threshold` | `-` |

### `struct ip_vs_stats_user`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `conns` | `-` |
| `__u32` | `inpkts` | `-` |
| `__u32` | `outpkts` | `-` |
| `__u64` | `inbytes` | `-` |
| `__u64` | `outbytes` | `-` |
| `__u32` | `cps` | `-` |
| `__u32` | `inpps` | `-` |
| `__u32` | `outpps` | `-` |
| `__u32` | `inbps` | `-` |
| `__u32` | `outbps` | `-` |

### `struct ip_vs_getinfo`

| Type | Field | Array |
|------|-------|-------|

### `struct ip_vs_service_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `protocol` | `-` |
| `__be32` | `addr` | `-` |
| `__be16` | `port` | `-` |
| `__u32` | `fwmark` | `-` |
| `char` | `sched_name` | `IP_VS_SCHEDNAME_MAXLEN` |
| `__be32` | `netmask` | `-` |

### `struct ip_vs_dest_entry`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `addr` | `-` |
| `__be16` | `port` | `-` |
| `int` | `weight` | `-` |
| `__u32` | `u_threshold` | `-` |
| `__u32` | `l_threshold` | `-` |
| `__u32` | `activeconns` | `-` |
| `__u32` | `inactconns` | `-` |
| `__u32` | `persistconns` | `-` |

### `struct ip_vs_get_dests`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `protocol` | `-` |
| `__be32` | `addr` | `-` |
| `__be16` | `port` | `-` |
| `__u32` | `fwmark` | `-` |

### `struct ip_vs_get_services`

| Type | Field | Array |
|------|-------|-------|

### `struct ip_vs_timeout_user`

| Type | Field | Array |
|------|-------|-------|
| `int` | `tcp_timeout` | `-` |
| `int` | `tcp_fin_timeout` | `-` |
| `int` | `udp_timeout` | `-` |

### `struct ip_vs_daemon_user`

| Type | Field | Array |
|------|-------|-------|
| `int` | `state` | `-` |
| `char` | `mcast_ifn` | `IP_VS_IFNAME_MAXLEN` |
| `int` | `syncid` | `-` |

### `struct ip_vs_flags`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `mask` | `-` |