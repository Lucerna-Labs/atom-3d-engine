# if_tun.h

**Source:** `if_tun.h`


## Includes

- `linux/types.h`
- `linux/if_ether.h`
- `linux/filter.h`

## Defines (58 total)


### IFF_ATTACH (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_ATTACH_QUEUE` | `0x0200` |  |

### IFF_DETACH (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_DETACH_QUEUE` | `0x0400` |  |

### IFF_MULTI (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_MULTI_QUEUE` | `0x0100` |  |

### IFF_NAPI (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_NAPI_FRAGS` | `0x0020` |  |

### IFF_NO (2)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_NO_CARRIER` | `0x0040` |  |
| `IFF_NO_PI` | `0x1000` |  |

### IFF_ONE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_ONE_QUEUE` | `0x2000` |  |

### IFF_TUN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_TUN_EXCL` | `0x8000` |  |

### IFF_VNET (1)

| Name | Value | Comment |
|------|-------|---------|
| `IFF_VNET_HDR` | `0x4000` |  |

### TUN_F (9)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_F_CSUM` | `0x01` | You can hand me unchecksummed packets. |
| `TUN_F_TSO4` | `0x02` | I can handle TSO for IPv4 packets |
| `TUN_F_TSO6` | `0x04` | I can handle TSO for IPv6 packets |
| `TUN_F_TSO_ECN` | `0x08` | I can handle TSO with ECN bits. |
| `TUN_F_UFO` | `0x10` | I can handle UFO packets |
| `TUN_F_USO4` | `0x20` | I can handle USO for IPv4 packets |
| `TUN_F_USO6` | `0x40` | I can handle USO for IPv6 packets |
| `TUN_F_UDP_TUNNEL_GSO` | `0x080` |  |
| `TUN_F_UDP_TUNNEL_GSO_CSUM` | `0x100` |  |

### TUN_FLT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_FLT_ALLMULTI` | `0x0001` | Accept all multicast packets |

### TUN_PKT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_PKT_STRIP` | `0x0001` |  |

### TUN_READQ (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_READQ_SIZE` | `500` |  |

### TUN_TAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_TAP_DEV` | `IFF_TAP` |  |

### TUN_TUN (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_TUN_DEV` | `IFF_TUN` |  |

### TUN_TX (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_TX_TIMESTAMP` | `1` |  |

### TUN_TYPE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TUN_TYPE_MASK` | `0x000f` |  |

### UNCATEGORIZED (33)

| Name | Value | Comment |
|------|-------|---------|
| `TUNSETNOCSUM` | `_IOW('T', 200, int)` |  |
| `TUNSETDEBUG` | `_IOW('T', 201, int)` |  |
| `TUNSETIFF` | `_IOW('T', 202, int)` |  |
| `TUNSETPERSIST` | `_IOW('T', 203, int)` |  |
| `TUNSETOWNER` | `_IOW('T', 204, int)` |  |
| `TUNSETLINK` | `_IOW('T', 205, int)` |  |
| `TUNSETGROUP` | `_IOW('T', 206, int)` |  |
| `TUNGETFEATURES` | `_IOR('T', 207, unsigned int)` |  |
| `TUNSETOFFLOAD` | `_IOW('T', 208, unsigned int)` |  |
| `TUNSETTXFILTER` | `_IOW('T', 209, unsigned int)` |  |
| `TUNGETIFF` | `_IOR('T', 210, unsigned int)` |  |
| `TUNGETSNDBUF` | `_IOR('T', 211, int)` |  |
| `TUNSETSNDBUF` | `_IOW('T', 212, int)` |  |
| `TUNATTACHFILTER` | `_IOW('T', 213, struct sock_fprog)` |  |
| `TUNDETACHFILTER` | `_IOW('T', 214, struct sock_fprog)` |  |
| `TUNGETVNETHDRSZ` | `_IOR('T', 215, int)` |  |
| `TUNSETVNETHDRSZ` | `_IOW('T', 216, int)` |  |
| `TUNSETQUEUE` | `_IOW('T', 217, int)` |  |
| `TUNSETIFINDEX` | `_IOW('T', 218, unsigned int)` |  |
| `TUNGETFILTER` | `_IOR('T', 219, struct sock_fprog)` |  |
| `TUNSETVNETLE` | `_IOW('T', 220, int)` |  |
| `TUNGETVNETLE` | `_IOR('T', 221, int)` |  |
| `TUNSETVNETBE` | `_IOW('T', 222, int)` |  |
| `TUNGETVNETBE` | `_IOR('T', 223, int)` |  |
| `TUNSETSTEERINGEBPF` | `_IOR('T', 224, int)` |  |
| `TUNSETFILTEREBPF` | `_IOR('T', 225, int)` |  |
| `TUNSETCARRIER` | `_IOW('T', 226, int)` |  |
| `TUNGETDEVNETNS` | `_IO('T', 227)` |  |
| `IFF_TUN` | `0x0001` |  |
| `IFF_TAP` | `0x0002` |  |
| `IFF_NAPI` | `0x0010` |  |
| `IFF_PERSIST` | `0x0800` |  |
| `IFF_NOFILTER` | `0x1000` |  |

## Structs (2)


### `struct tun_pi`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `flags` | `-` |
| `__be16` | `proto` | `-` |

### `struct tun_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `flags` | `-` |
| `__u16` | `count` | `-` |
| `__u8` | `addr` | `][ETH_ALEN` |