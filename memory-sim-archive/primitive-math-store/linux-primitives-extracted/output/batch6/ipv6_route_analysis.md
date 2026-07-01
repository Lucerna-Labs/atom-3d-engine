# ipv6_route.h

**Source:** `ipv6_route.h`


## Includes

- `linux/types.h`
- `linux/in6.h`

## Defines (20 total)


### RTF_PREF (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTF_PREF_MASK` | `0x18000000` |  |

### RTF_PREFIX (1)

| Name | Value | Comment |
|------|-------|---------|
| `RTF_PREFIX_RT` | `0x00080000` | A prefix only route - RA |

### UNCATEGORIZED (18)

| Name | Value | Comment |
|------|-------|---------|
| `RTF_DEFAULT` | `0x00010000` | default - learned via ND |
| `RTF_ALLONLINK` | `0x00020000	/* (deprecated and will be removed)` |  |
| `RTF_ADDRCONF` | `0x00040000` | addrconf route - RA |
| `RTF_ANYCAST` | `0x00100000` | Anycast |
| `RTF_NONEXTHOP` | `0x00200000` | route with no nexthop |
| `RTF_EXPIRES` | `0x00400000` |  |
| `RTF_ROUTEINFO` | `0x00800000` | route information - RA |
| `RTF_CACHE` | `0x01000000` | read-only: can not be set by user |
| `RTF_FLOW` | `0x02000000` | flow significant route |
| `RTF_POLICY` | `0x04000000` | policy route |
| `RTF_PCPU` | `0x40000000` | read-only: can not be set by user |
| `RTF_LOCAL` | `0x80000000` |  |
| `RTMSG_NEWDEVICE` | `0x11` |  |
| `RTMSG_DELDEVICE` | `0x12` |  |
| `RTMSG_NEWROUTE` | `0x21` |  |
| `RTMSG_DELROUTE` | `0x22` |  |
| `IP6_RT_PRIO_USER` | `1024` |  |
| `IP6_RT_PRIO_ADDRCONF` | `256` |  |

## Structs (1)


### `struct in6_rtmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rtmsg_type` | `-` |
| `__u16` | `rtmsg_dst_len` | `-` |
| `__u16` | `rtmsg_src_len` | `-` |
| `__u32` | `rtmsg_metric` | `-` |
| `__u32` | `rtmsg_flags` | `-` |
| `int` | `rtmsg_ifindex` | `-` |