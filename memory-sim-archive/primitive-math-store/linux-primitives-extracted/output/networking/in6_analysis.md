# in6.h

**Source:** `in6.h`


## Includes

- `linux/types.h`
- `linux/libc-compat.h`

## Defines (119 total)


### UNCATEGORIZED (119)

| Name | Value | Comment |
|------|-------|---------|
| `s6_addr` | `in6_u.u6_addr8` |  |
| `s6_addr16` | `in6_u.u6_addr16` |  |
| `s6_addr32` | `in6_u.u6_addr32` |  |
| `ipv6mr_acaddr` | `ipv6mr_multiaddr` |  |
| `IPV6_FL_A_GET` | `0` |  |
| `IPV6_FL_A_PUT` | `1` |  |
| `IPV6_FL_A_RENEW` | `2` |  |
| `IPV6_FL_F_CREATE` | `1` |  |
| `IPV6_FL_F_EXCL` | `2` |  |
| `IPV6_FL_F_REFLECT` | `4` |  |
| `IPV6_FL_F_REMOTE` | `8` |  |
| `IPV6_FL_S_NONE` | `0` |  |
| `IPV6_FL_S_EXCL` | `1` |  |
| `IPV6_FL_S_PROCESS` | `2` |  |
| `IPV6_FL_S_USER` | `3` |  |
| `IPV6_FL_S_ANY` | `255` |  |
| `IPV6_FLOWINFO_FLOWLABEL` | `0x000fffff` |  |
| `IPV6_FLOWINFO_PRIORITY` | `0x0ff00000` |  |
| `IPV6_PRIORITY_UNCHARACTERIZED` | `0x0000` |  |
| `IPV6_PRIORITY_FILLER` | `0x0100` |  |
| `IPV6_PRIORITY_UNATTENDED` | `0x0200` |  |
| `IPV6_PRIORITY_RESERVED1` | `0x0300` |  |
| `IPV6_PRIORITY_BULK` | `0x0400` |  |
| `IPV6_PRIORITY_RESERVED2` | `0x0500` |  |
| `IPV6_PRIORITY_INTERACTIVE` | `0x0600` |  |
| `IPV6_PRIORITY_CONTROL` | `0x0700` |  |
| `IPV6_PRIORITY_8` | `0x0800` |  |
| `IPV6_PRIORITY_9` | `0x0900` |  |
| `IPV6_PRIORITY_10` | `0x0a00` |  |
| `IPV6_PRIORITY_11` | `0x0b00` |  |
| `IPV6_PRIORITY_12` | `0x0c00` |  |
| `IPV6_PRIORITY_13` | `0x0d00` |  |
| `IPV6_PRIORITY_14` | `0x0e00` |  |
| `IPV6_PRIORITY_15` | `0x0f00` |  |
| `IPPROTO_HOPOPTS` | `0` | IPv6 hop-by-hop options |
| `IPPROTO_ROUTING` | `43` | IPv6 routing header |
| `IPPROTO_FRAGMENT` | `44` | IPv6 fragmentation header |
| `IPPROTO_ICMPV6` | `58` | ICMPv6 |
| `IPPROTO_NONE` | `59` | IPv6 no next header |
| `IPPROTO_DSTOPTS` | `60` | IPv6 destination options |
| `IPPROTO_MH` | `135` | IPv6 mobility header |
| `IPV6_TLV_PAD1` | `0` |  |
| `IPV6_TLV_PADN` | `1` |  |
| `IPV6_TLV_ROUTERALERT` | `5` |  |
| `IPV6_TLV_CALIPSO` | `7` | RFC 5570 |
| `IPV6_TLV_IOAM` | `49` | RFC 9486 |
| `IPV6_TLV_JUMBO` | `194` |  |
| `IPV6_TLV_HAO` | `201` | home address option |
| `IPV6_ADDRFORM` | `1` |  |
| `IPV6_2292PKTINFO` | `2` |  |

*...and 69 more*

## Structs (4)


### `struct in6_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `u6_addr8` | `16` |
| `__be16` | `u6_addr16` | `8` |
| `__be32` | `u6_addr32` | `4` |

### `struct sockaddr_in6`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `sin6_port` | `-` |
| `__be32` | `sin6_flowinfo` | `-` |
| `__u32` | `sin6_scope_id` | `-` |

### `struct ipv6_mreq`

| Type | Field | Array |
|------|-------|-------|
| `int` | `ipv6mr_ifindex` | `-` |

### `struct in6_flowlabel_req`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `flr_label` | `-` |
| `__u8` | `flr_action` | `-` |
| `__u8` | `flr_share` | `-` |
| `__u16` | `flr_flags` | `-` |
| `__u16` | `flr_expires` | `-` |
| `__u16` | `flr_linger` | `-` |
| `__u32` | `__flr_pad` | `-` |