# if_pppox.h

**Source:** `if_pppox.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`
- `linux/socket.h`
- `linux/if.h`
- `linux/if_ether.h`
- `linux/if_pppol2tp.h`
- `linux/in.h`
- `linux/in6.h`

## Defines (22 total)


### PPPOE_SES (1)

| Name | Value | Comment |
|------|-------|---------|
| `PPPOE_SES_HLEN` | `8` |  |

### PTT_AC (2)

| Name | Value | Comment |
|------|-------|---------|
| `PTT_AC_NAME` | `__cpu_to_be16(0x0102)` |  |
| `PTT_AC_COOKIE` | `__cpu_to_be16(0x0104)` |  |

### PTT_GEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `PTT_GEN_ERR` | `__cpu_to_be16(0x0203)` |  |

### PTT_HOST (1)

| Name | Value | Comment |
|------|-------|---------|
| `PTT_HOST_UNIQ` | `__cpu_to_be16(0x0103)` |  |

### PTT_RELAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `PTT_RELAY_SID` | `__cpu_to_be16(0x0110)` |  |

### PTT_SRV (2)

| Name | Value | Comment |
|------|-------|---------|
| `PTT_SRV_NAME` | `__cpu_to_be16(0x0101)` |  |
| `PTT_SRV_ERR` | `__cpu_to_be16(0x0201)` |  |

### PTT_SYS (1)

| Name | Value | Comment |
|------|-------|---------|
| `PTT_SYS_ERR` | `__cpu_to_be16(0x0202)` |  |

### PX_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `PX_MAX_PROTO` | `3` |  |

### PX_PROTO (3)

| Name | Value | Comment |
|------|-------|---------|
| `PX_PROTO_OE` | `0` | Currently just PPPoE |
| `PX_PROTO_OL2TP` | `1` | Now L2TP also |
| `PX_PROTO_PPTP` | `2` |  |

### UNCATEGORIZED (9)

| Name | Value | Comment |
|------|-------|---------|
| `AF_PPPOX` | `24` |  |
| `PF_PPPOX` | `AF_PPPOX` |  |
| `PADI_CODE` | `0x09` |  |
| `PADO_CODE` | `0x07` |  |
| `PADR_CODE` | `0x19` |  |
| `PADS_CODE` | `0x65` |  |
| `PADT_CODE` | `0xa7` |  |
| `PTT_EOL` | `__cpu_to_be16(0x0000)` |  |
| `PTT_VENDOR` | `__cpu_to_be16(0x0105)` |  |

## Structs (9)


### `struct pppoe_addr`

| Type | Field | Array |
|------|-------|-------|
| `sid_t` | `sid` | `-` |
| `char` | `dev` | `IFNAMSIZ` |

### `struct pptp_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `call_id` | `-` |

### `struct sockaddr_pppox`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sa_family` | `-` |

### `struct sockaddr_pppol2tp`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sa_family` | `-` |

### `struct sockaddr_pppol2tpin6`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sa_family` | `-` |

### `struct sockaddr_pppol2tpv3`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sa_family` | `-` |

### `struct sockaddr_pppol2tpv3in6`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sa_family` | `-` |

### `struct pppoe_tag`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `tag_type` | `-` |
| `__be16` | `tag_len` | `-` |

### `struct pppoe_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `code` | `-` |
| `__be16` | `sid` | `-` |
| `__be16` | `length` | `-` |