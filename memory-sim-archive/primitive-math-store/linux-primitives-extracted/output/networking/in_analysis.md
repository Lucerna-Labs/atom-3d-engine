# in.h

**Source:** `in.h`


## Includes

- `linux/types.h`
- `linux/stddef.h`
- `linux/libc-compat.h`
- `linux/socket.h`
- `asm/byteorder.h`

## Defines (116 total)


### INADDR_ALLHOSTS (1)

| Name | Value | Comment |
|------|-------|---------|
| `INADDR_ALLHOSTS_GROUP` | `0xe0000001U` | 224.0.0.1 |

### INADDR_ALLRTRS (1)

| Name | Value | Comment |
|------|-------|---------|
| `INADDR_ALLRTRS_GROUP` | `0xe0000002U` | 224.0.0.2 |

### INADDR_ALLSNOOPERS (1)

| Name | Value | Comment |
|------|-------|---------|
| `INADDR_ALLSNOOPERS_GROUP` | `0xe000006aU` | 224.0.0.106 |

### INADDR_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `INADDR_MAX_LOCAL_GROUP` | `0xe00000ffU` | 224.0.0.255 |

### INADDR_UNSPEC (1)

| Name | Value | Comment |
|------|-------|---------|
| `INADDR_UNSPEC_GROUP` | `0xe0000000U` | 224.0.0.0 |

### IN_CLASSA (4)

| Name | Value | Comment |
|------|-------|---------|
| `IN_CLASSA_NET` | `0xff000000` |  |
| `IN_CLASSA_NSHIFT` | `24` |  |
| `IN_CLASSA_HOST` | `(0xffffffff & ~IN_CLASSA_NET)` |  |
| `IN_CLASSA_MAX` | `128` |  |

### IN_CLASSB (4)

| Name | Value | Comment |
|------|-------|---------|
| `IN_CLASSB_NET` | `0xffff0000` |  |
| `IN_CLASSB_NSHIFT` | `16` |  |
| `IN_CLASSB_HOST` | `(0xffffffff & ~IN_CLASSB_NET)` |  |
| `IN_CLASSB_MAX` | `65536` |  |

### IN_CLASSC (3)

| Name | Value | Comment |
|------|-------|---------|
| `IN_CLASSC_NET` | `0xffffff00` |  |
| `IN_CLASSC_NSHIFT` | `8` |  |
| `IN_CLASSC_HOST` | `(0xffffffff & ~IN_CLASSC_NET)` |  |

### IN_CLASSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `IN_CLASSE_NET` | `0xffffffff` |  |
| `IN_CLASSE_NSHIFT` | `0` |  |

### IN_MULTICAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `IN_MULTICAST_NET` | `0xe0000000` |  |

### IP_ADD (2)

| Name | Value | Comment |
|------|-------|---------|
| `IP_ADD_MEMBERSHIP` | `35` |  |
| `IP_ADD_SOURCE_MEMBERSHIP` | `39` |  |

### IP_BIND (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_BIND_ADDRESS_NO_PORT` | `24` |  |

### IP_BLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_BLOCK_SOURCE` | `38` |  |

### IP_DEFAULT (2)

| Name | Value | Comment |
|------|-------|---------|
| `IP_DEFAULT_MULTICAST_TTL` | `1` |  |
| `IP_DEFAULT_MULTICAST_LOOP` | `1` |  |

### IP_DROP (2)

| Name | Value | Comment |
|------|-------|---------|
| `IP_DROP_MEMBERSHIP` | `36` |  |
| `IP_DROP_SOURCE_MEMBERSHIP` | `40` |  |

### IP_IPSEC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_IPSEC_POLICY` | `16` |  |

### IP_LOCAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_LOCAL_PORT_RANGE` | `51` |  |

### IP_MTU (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_MTU_DISCOVER` | `10` |  |

### IP_MULTICAST (4)

| Name | Value | Comment |
|------|-------|---------|
| `IP_MULTICAST_IF` | `32` |  |
| `IP_MULTICAST_TTL` | `33` |  |
| `IP_MULTICAST_LOOP` | `34` |  |
| `IP_MULTICAST_ALL` | `49` |  |

### IP_PMTUDISC (6)

| Name | Value | Comment |
|------|-------|---------|
| `IP_PMTUDISC_DONT` | `0` | Never send DF frames |
| `IP_PMTUDISC_WANT` | `1` | Use per route hints |
| `IP_PMTUDISC_DO` | `2` | Always DF |
| `IP_PMTUDISC_PROBE` | `3` | Ignore dst pmtu |
| `IP_PMTUDISC_INTERFACE` | `4` |  |
| `IP_PMTUDISC_OMIT` | `5` |  |

### IP_RECVERR (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_RECVERR_RFC4884` | `26` |  |

### IP_ROUTER (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_ROUTER_ALERT` | `5` |  |

### IP_UNBLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_UNBLOCK_SOURCE` | `37` |  |

### IP_UNICAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_UNICAST_IF` | `50` |  |

### IP_XFRM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IP_XFRM_POLICY` | `17` |  |

### MCAST_BLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `MCAST_BLOCK_SOURCE` | `43` |  |

### MCAST_JOIN (2)

| Name | Value | Comment |
|------|-------|---------|
| `MCAST_JOIN_GROUP` | `42` |  |
| `MCAST_JOIN_SOURCE_GROUP` | `46` |  |

### MCAST_LEAVE (2)

| Name | Value | Comment |
|------|-------|---------|
| `MCAST_LEAVE_GROUP` | `45` |  |
| `MCAST_LEAVE_SOURCE_GROUP` | `47` |  |

### MCAST_UNBLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `MCAST_UNBLOCK_SOURCE` | `44` |  |

### UNCATEGORIZED (65)

| Name | Value | Comment |
|------|-------|---------|
| `IPPROTO_IP` | `IPPROTO_IP` |  |
| `IPPROTO_ICMP` | `IPPROTO_ICMP` |  |
| `IPPROTO_IGMP` | `IPPROTO_IGMP` |  |
| `IPPROTO_IPIP` | `IPPROTO_IPIP` |  |
| `IPPROTO_TCP` | `IPPROTO_TCP` |  |
| `IPPROTO_EGP` | `IPPROTO_EGP` |  |
| `IPPROTO_PUP` | `IPPROTO_PUP` |  |
| `IPPROTO_UDP` | `IPPROTO_UDP` |  |
| `IPPROTO_IDP` | `IPPROTO_IDP` |  |
| `IPPROTO_TP` | `IPPROTO_TP` |  |
| `IPPROTO_DCCP` | `IPPROTO_DCCP` |  |
| `IPPROTO_IPV6` | `IPPROTO_IPV6` |  |
| `IPPROTO_RSVP` | `IPPROTO_RSVP` |  |
| `IPPROTO_GRE` | `IPPROTO_GRE` |  |
| `IPPROTO_ESP` | `IPPROTO_ESP` |  |
| `IPPROTO_AH` | `IPPROTO_AH` |  |
| `IPPROTO_MTP` | `IPPROTO_MTP` |  |
| `IPPROTO_BEETPH` | `IPPROTO_BEETPH` |  |
| `IPPROTO_ENCAP` | `IPPROTO_ENCAP` |  |
| `IPPROTO_PIM` | `IPPROTO_PIM` |  |
| `IPPROTO_COMP` | `IPPROTO_COMP` |  |
| `IPPROTO_L2TP` | `IPPROTO_L2TP` |  |
| `IPPROTO_SCTP` | `IPPROTO_SCTP` |  |
| `IPPROTO_UDPLITE` | `IPPROTO_UDPLITE` |  |
| `IPPROTO_MPLS` | `IPPROTO_MPLS` |  |
| `IPPROTO_ETHERNET` | `IPPROTO_ETHERNET` |  |
| `IPPROTO_AGGFRAG` | `IPPROTO_AGGFRAG` |  |
| `IPPROTO_RAW` | `IPPROTO_RAW` |  |
| `IPPROTO_SMC` | `IPPROTO_SMC` |  |
| `IPPROTO_MPTCP` | `IPPROTO_MPTCP` |  |
| `IP_TOS` | `1` |  |
| `IP_TTL` | `2` |  |
| `IP_HDRINCL` | `3` |  |
| `IP_OPTIONS` | `4` |  |
| `IP_RECVOPTS` | `6` |  |
| `IP_RETOPTS` | `7` |  |
| `IP_PKTINFO` | `8` |  |
| `IP_PKTOPTIONS` | `9` |  |
| `IP_RECVERR` | `11` |  |
| `IP_RECVTTL` | `12` |  |
| `IP_RECVTOS` | `13` |  |
| `IP_MTU` | `14` |  |
| `IP_FREEBIND` | `15` |  |
| `IP_PASSSEC` | `18` |  |
| `IP_TRANSPARENT` | `19` |  |
| `IP_RECVRETOPTS` | `IP_RETOPTS` |  |
| `IP_ORIGDSTADDR` | `20` |  |
| `IP_RECVORIGDSTADDR` | `IP_ORIGDSTADDR` |  |
| `IP_MINTTL` | `21` |  |
| `IP_NODEFRAG` | `22` |  |

*...and 15 more*

## Structs (12)


### `struct in_addr`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `s_addr` | `-` |

### `struct ip_mreq`

| Type | Field | Array |
|------|-------|-------|

### `struct ip_mreqn`

| Type | Field | Array |
|------|-------|-------|
| `int` | `imr_ifindex` | `-` |

### `struct ip_mreq_source`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `imr_multiaddr` | `-` |
| `__be32` | `imr_interface` | `-` |
| `__be32` | `imr_sourceaddr` | `-` |

### `struct ip_msfilter`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `imsf_multiaddr` | `-` |
| `__be32` | `imsf_interface` | `-` |
| `__u32` | `imsf_fmode` | `-` |
| `__u32` | `imsf_numsrc` | `-` |
| `__be32` | `imsf_slist` | `1` |

### `struct group_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gr_interface` | `-` |

### `struct group_source_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gsr_interface` | `-` |

### `struct group_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gf_interface_aux` | `-` |
| `__u32` | `gf_fmode_aux` | `-` |
| `__u32` | `gf_numsrc_aux` | `-` |
| `__u32` | `gf_interface` | `-` |
| `__u32` | `gf_fmode` | `-` |
| `__u32` | `gf_numsrc` | `-` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gf_interface_aux` | `-` |
| `__u32` | `gf_fmode_aux` | `-` |
| `__u32` | `gf_numsrc_aux` | `-` |

### `struct anonymous_9`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gf_interface` | `-` |
| `__u32` | `gf_fmode` | `-` |
| `__u32` | `gf_numsrc` | `-` |

### `struct in_pktinfo`

| Type | Field | Array |
|------|-------|-------|
| `int` | `ipi_ifindex` | `-` |

### `struct sockaddr_in`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sin_family` | `-` |
| `__be16` | `sin_port` | `-` |