# net.h

**Source:** `net.h`


## Includes

- `linux/types.h`

## Defines (7 total)


### DVB_NET (2)

| Name | Value | Comment |
|------|-------|---------|
| `DVB_NET_FEEDTYPE_MPE` | `0` | multi protocol encapsulation |
| `DVB_NET_FEEDTYPE_ULE` | `1` | ultra lightweight encapsulation |

### NET_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `NET_ADD_IF` | `_IOWR('o', 52, struct dvb_net_if)` |  |

### NET_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `NET_GET_IF` | `_IOWR('o', 54, struct dvb_net_if)` |  |

### NET_REMOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `NET_REMOVE_IF` | `_IO('o', 53)` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `__NET_ADD_IF_OLD` | `_IOWR('o', 52, struct __dvb_net_if_old)` |  |
| `__NET_GET_IF_OLD` | `_IOWR('o', 54, struct __dvb_net_if_old)` |  |

## Structs (2)


### `struct dvb_net_if`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `pid` | `-` |
| `__u16` | `if_num` | `-` |
| `__u8` | `feedtype` | `-` |

### `struct __dvb_net_if_old`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `pid` | `-` |
| `__u16` | `if_num` | `-` |