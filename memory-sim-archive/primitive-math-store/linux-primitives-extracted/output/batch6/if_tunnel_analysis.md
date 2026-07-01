# if_tunnel.h

**Source:** `if_tunnel.h`


## Includes

- `linux/types.h`
- `linux/if.h`
- `linux/ip.h`
- `linux/in6.h`
- `asm/byteorder.h`

## Defines (51 total)


### GRE_PPTP (1)

| Name | Value | Comment |
|------|-------|---------|
| `GRE_PPTP_KEY_MASK` | `__cpu_to_be32(0xffff)` |  |

### GRE_PROTO (1)

| Name | Value | Comment |
|------|-------|---------|
| `GRE_PROTO_PPP` | `__cpu_to_be16(0x880b)` |  |

### GRE_VERSION (2)

| Name | Value | Comment |
|------|-------|---------|
| `GRE_VERSION_0` | `__cpu_to_be16(0x0000)` |  |
| `GRE_VERSION_1` | `__cpu_to_be16(0x0001)` |  |

### IFLA_GRE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_GRE_MAX` | `(__IFLA_GRE_MAX - 1)` |  |

### IFLA_IPTUN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_IPTUN_MAX` | `(__IFLA_IPTUN_MAX - 1)` |  |

### IFLA_VTI (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFLA_VTI_MAX` | `(__IFLA_VTI_MAX - 1)` |  |

### TUNNEL_CRIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_CRIT_OPT` | `__cpu_to_be16(0x0400)` |  |

### TUNNEL_DONT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_DONT_FRAGMENT` | `__cpu_to_be16(0x0100)` |  |

### TUNNEL_ENCAP (3)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_ENCAP_FLAG_CSUM` | `(1<<0)` |  |
| `TUNNEL_ENCAP_FLAG_CSUM6` | `(1<<1)` |  |
| `TUNNEL_ENCAP_FLAG_REMCSUM` | `(1<<2)` |  |

### TUNNEL_ERSPAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_ERSPAN_OPT` | `__cpu_to_be16(0x4000)` |  |

### TUNNEL_GENEVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_GENEVE_OPT` | `__cpu_to_be16(0x0800)` |  |

### TUNNEL_GTP (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_GTP_OPT` | `__cpu_to_be16(0x8000)` |  |

### TUNNEL_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_NO_KEY` | `__cpu_to_be16(0x80)` |  |

### TUNNEL_OPTIONS (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_OPTIONS_PRESENT` | `` |  |

### TUNNEL_VXLAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUNNEL_VXLAN_OPT` | `__cpu_to_be16(0x1000)` |  |

### UNCATEGORIZED (33)

| Name | Value | Comment |
|------|-------|---------|
| `SIOCGETTUNNEL` | `(SIOCDEVPRIVATE + 0)` |  |
| `SIOCADDTUNNEL` | `(SIOCDEVPRIVATE + 1)` |  |
| `SIOCDELTUNNEL` | `(SIOCDEVPRIVATE + 2)` |  |
| `SIOCCHGTUNNEL` | `(SIOCDEVPRIVATE + 3)` |  |
| `SIOCGETPRL` | `(SIOCDEVPRIVATE + 4)` |  |
| `SIOCADDPRL` | `(SIOCDEVPRIVATE + 5)` |  |
| `SIOCDELPRL` | `(SIOCDEVPRIVATE + 6)` |  |
| `SIOCCHGPRL` | `(SIOCDEVPRIVATE + 7)` |  |
| `SIOCGET6RD` | `(SIOCDEVPRIVATE + 8)` |  |
| `SIOCADD6RD` | `(SIOCDEVPRIVATE + 9)` |  |
| `SIOCDEL6RD` | `(SIOCDEVPRIVATE + 10)` |  |
| `SIOCCHG6RD` | `(SIOCDEVPRIVATE + 11)` |  |
| `GRE_CSUM` | `__cpu_to_be16(0x8000)` |  |
| `GRE_ROUTING` | `__cpu_to_be16(0x4000)` |  |
| `GRE_KEY` | `__cpu_to_be16(0x2000)` |  |
| `GRE_SEQ` | `__cpu_to_be16(0x1000)` |  |
| `GRE_STRICT` | `__cpu_to_be16(0x0800)` |  |
| `GRE_REC` | `__cpu_to_be16(0x0700)` |  |
| `GRE_ACK` | `__cpu_to_be16(0x0080)` |  |
| `GRE_FLAGS` | `__cpu_to_be16(0x0078)` |  |
| `GRE_VERSION` | `__cpu_to_be16(0x0007)` |  |
| `SIT_ISATAP` | `0x0001` |  |
| `PRL_DEFAULT` | `0x0001` |  |
| `VTI_ISVTI` | `((__force __be16)0x0001)` |  |
| `TUNNEL_CSUM` | `__cpu_to_be16(0x01)` |  |
| `TUNNEL_ROUTING` | `__cpu_to_be16(0x02)` |  |
| `TUNNEL_KEY` | `__cpu_to_be16(0x04)` |  |
| `TUNNEL_SEQ` | `__cpu_to_be16(0x08)` |  |
| `TUNNEL_STRICT` | `__cpu_to_be16(0x10)` |  |
| `TUNNEL_REC` | `__cpu_to_be16(0x20)` |  |
| `TUNNEL_VERSION` | `__cpu_to_be16(0x40)` |  |
| `TUNNEL_OAM` | `__cpu_to_be16(0x0200)` |  |
| `TUNNEL_NOCACHE` | `__cpu_to_be16(0x2000)` |  |

## Structs (3)


### `struct ip_tunnel_parm`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `IFNAMSIZ` |
| `int` | `link` | `-` |
| `__be16` | `i_flags` | `-` |
| `__be16` | `o_flags` | `-` |
| `__be32` | `i_key` | `-` |
| `__be32` | `o_key` | `-` |

### `struct ip_tunnel_prl`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `addr` | `-` |
| `__u16` | `flags` | `-` |
| `__u16` | `__reserved` | `-` |
| `__u32` | `datalen` | `-` |
| `__u32` | `__reserved2` | `-` |

### `struct ip_tunnel_6rd`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `relay_prefix` | `-` |
| `__u16` | `prefixlen` | `-` |
| `__u16` | `relay_prefixlen` | `-` |