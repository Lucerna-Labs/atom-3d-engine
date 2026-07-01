# if.h

**Source:** `if.h`


## Includes

- `linux/libc-compat.h`
- `linux/types.h`
- `linux/socket.h`
- `linux/compiler.h`
- `sys/socket.h`
- `linux/hdlc/ioctl.h`

## Defines (65 total)


### IFF_LOWER (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_LOWER_UP` | `IFF_LOWER_UP` |  |

### IF_GET (2)

| Name | Value | Comment |
|------|-------|---------|
| `IF_GET_IFACE` | `0x0001` | for querying only |
| `IF_GET_PROTO` | `0x0002` |  |

### IF_IFACE (7)

| Name | Value | Comment |
|------|-------|---------|
| `IF_IFACE_V35` | `0x1000` | V.35 serial interface |
| `IF_IFACE_V24` | `0x1001` | V.24 serial interface |
| `IF_IFACE_X21` | `0x1002` | X.21 serial interface |
| `IF_IFACE_T1` | `0x1003` | T1 telco serial interface |
| `IF_IFACE_E1` | `0x1004` | E1 telco serial interface |
| `IF_IFACE_SYNC_SERIAL` | `0x1005` | can't be set by software |
| `IF_IFACE_X21D` | `0x1006` | X.21 Dual Clocking (FarSite) |

### IF_PROTO (13)

| Name | Value | Comment |
|------|-------|---------|
| `IF_PROTO_HDLC` | `0x2000` | raw HDLC protocol |
| `IF_PROTO_PPP` | `0x2001` | PPP protocol |
| `IF_PROTO_CISCO` | `0x2002` | Cisco HDLC protocol |
| `IF_PROTO_FR` | `0x2003` | Frame Relay protocol |
| `IF_PROTO_FR_ADD_PVC` | `0x2004` | Create FR PVC |
| `IF_PROTO_FR_DEL_PVC` | `0x2005` | Delete FR PVC |
| `IF_PROTO_X25` | `0x2006` | X.25 |
| `IF_PROTO_HDLC_ETH` | `0x2007` | raw HDLC, Ethernet emulation |
| `IF_PROTO_FR_ADD_ETH_PVC` | `0x2008` | Create FR Ethernet-bridged PVC |
| `IF_PROTO_FR_DEL_ETH_PVC` | `0x2009` | Delete FR Ethernet-bridged PVC |
| `IF_PROTO_FR_PVC` | `0x200A` | for reading PVC status |
| `IF_PROTO_FR_ETH_PVC` | `0x200B` |  |
| `IF_PROTO_RAW` | `0x200C` | RAW Socket |

### UNCATEGORIZED (42)

| Name | Value | Comment |
|------|-------|---------|
| `IFNAMSIZ` | `16` |  |
| `IFALIASZ` | `256` |  |
| `ALTIFNAMSIZ` | `128` |  |
| `IFF_UP` | `IFF_UP` |  |
| `IFF_BROADCAST` | `IFF_BROADCAST` |  |
| `IFF_DEBUG` | `IFF_DEBUG` |  |
| `IFF_LOOPBACK` | `IFF_LOOPBACK` |  |
| `IFF_POINTOPOINT` | `IFF_POINTOPOINT` |  |
| `IFF_NOTRAILERS` | `IFF_NOTRAILERS` |  |
| `IFF_RUNNING` | `IFF_RUNNING` |  |
| `IFF_NOARP` | `IFF_NOARP` |  |
| `IFF_PROMISC` | `IFF_PROMISC` |  |
| `IFF_ALLMULTI` | `IFF_ALLMULTI` |  |
| `IFF_MASTER` | `IFF_MASTER` |  |
| `IFF_SLAVE` | `IFF_SLAVE` |  |
| `IFF_MULTICAST` | `IFF_MULTICAST` |  |
| `IFF_PORTSEL` | `IFF_PORTSEL` |  |
| `IFF_AUTOMEDIA` | `IFF_AUTOMEDIA` |  |
| `IFF_DYNAMIC` | `IFF_DYNAMIC` |  |
| `IFF_DORMANT` | `IFF_DORMANT` |  |
| `IFF_ECHO` | `IFF_ECHO` |  |
| `IFF_VOLATILE` | `(IFF_LOOPBACK\|IFF_POINTOPOINT\|IFF_BROADCAST\|IFF_ECHO\|` |  |
| `IFHWADDRLEN` | `6` |  |
| `ifr_name` | `ifr_ifrn.ifrn_name` | interface name |
| `ifr_hwaddr` | `ifr_ifru.ifru_hwaddr` | MAC address |
| `ifr_addr` | `ifr_ifru.ifru_addr` | address |
| `ifr_dstaddr` | `ifr_ifru.ifru_dstaddr` | other end of p-p lnk |
| `ifr_broadaddr` | `ifr_ifru.ifru_broadaddr` | broadcast address |
| `ifr_netmask` | `ifr_ifru.ifru_netmask` | interface net mask |
| `ifr_flags` | `ifr_ifru.ifru_flags` | flags |
| `ifr_metric` | `ifr_ifru.ifru_ivalue` | metric |
| `ifr_mtu` | `ifr_ifru.ifru_mtu` | mtu |
| `ifr_map` | `ifr_ifru.ifru_map` | device map |
| `ifr_slave` | `ifr_ifru.ifru_slave` | slave device |
| `ifr_data` | `ifr_ifru.ifru_data` | for use by interface |
| `ifr_ifindex` | `ifr_ifru.ifru_ivalue` | interface index |
| `ifr_bandwidth` | `ifr_ifru.ifru_ivalue` | link bandwidth |
| `ifr_qlen` | `ifr_ifru.ifru_ivalue` | Queue length |
| `ifr_newname` | `ifr_ifru.ifru_newname` | New name |
| `ifr_settings` | `ifr_ifru.ifru_settings` | Device/proto settings |
| `ifc_buf` | `ifc_ifcu.ifcu_buf` | buffer address |
| `ifc_req` | `ifc_ifcu.ifcu_req` | array of structures |

## Structs (4)


### `struct ifmap`

| Type | Field | Array |
|------|-------|-------|

### `struct if_settings`

| Type | Field | Array |
|------|-------|-------|

### `struct ifreq`

| Type | Field | Array |
|------|-------|-------|
| `char` | `ifrn_name` | `IFNAMSIZ` |
| `short` | `ifru_flags` | `-` |
| `int` | `ifru_ivalue` | `-` |
| `int` | `ifru_mtu` | `-` |
| `char` | `ifru_slave` | `IFNAMSIZ` |
| `char` | `ifru_newname` | `IFNAMSIZ` |

### `struct ifconf`

| Type | Field | Array |
|------|-------|-------|
| `int` | `ifc_len` | `-` |