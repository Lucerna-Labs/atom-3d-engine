# ipv6.h

**Source:** `ipv6.h`


## Includes

- `linux/libc-compat.h`
- `linux/types.h`
- `linux/stddef.h`
- `linux/in6.h`
- `asm/byteorder.h`

## Defines (11 total)


### UNCATEGORIZED (11)

| Name | Value | Comment |
|------|-------|---------|
| `IPV6_MIN_MTU` | `1280` |  |
| `IPV6_SRCRT_STRICT` | `0x01` | Deprecated; will be removed |
| `IPV6_SRCRT_TYPE_0` | `0` | Deprecated; will be removed |
| `IPV6_SRCRT_TYPE_2` | `2` | IPv6 type 2 Routing Header |
| `IPV6_SRCRT_TYPE_3` | `3` | RPL Segment Routing with IPv6 |
| `IPV6_SRCRT_TYPE_4` | `4` | Segment Routing with IPv6 |
| `ipv6_destopt_hdr` | `ipv6_opt_hdr` |  |
| `ipv6_hopopt_hdr` | `ipv6_opt_hdr` |  |
| `IPV6_OPT_ROUTERALERT_MLD` | `0x0000` | MLD(RFC2710) |
| `rt0_type` | `rt_hdr.type` |  |
| `rt2_type` | `rt_hdr.type` |  |

## Structs (9)


### `struct in6_pktinfo`

| Type | Field | Array |
|------|-------|-------|
| `int` | `ipi6_ifindex` | `-` |

### `struct ip6_mtuinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ip6m_mtu` | `-` |

### `struct in6_ifreq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifr6_prefixlen` | `-` |
| `int` | `ifr6_ifindex` | `-` |

### `struct ipv6_rt_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `nexthdr` | `-` |
| `__u8` | `hdrlen` | `-` |
| `__u8` | `type` | `-` |
| `__u8` | `segments_left` | `-` |

### `struct ipv6_opt_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `nexthdr` | `-` |
| `__u8` | `hdrlen` | `-` |

### `struct rt0_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `reserved` | `-` |

### `struct rt2_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `reserved` | `-` |

### `struct ipv6_destopt_hao`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `length` | `-` |

### `struct ipv6hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flow_lbl` | `3` |
| `__be16` | `payload_len` | `-` |
| `__u8` | `nexthdr` | `-` |
| `__u8` | `hop_limit` | `-` |