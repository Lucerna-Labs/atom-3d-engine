# nf_conntrack_tcp.h

**Source:** `nf_conntrack_tcp.h`


## Includes

- `linux/types.h`

## Defines (9 total)


### IP_CT (8)

| Name | Value | Comment |
|------|-------|---------|
| `IP_CT_TCP_FLAG_WINDOW_SCALE` | `0x01` |  |
| `IP_CT_TCP_FLAG_SACK_PERM` | `0x02` |  |
| `IP_CT_TCP_FLAG_CLOSE_INIT` | `0x04` |  |
| `IP_CT_TCP_FLAG_BE_LIBERAL` | `0x08` |  |
| `IP_CT_TCP_FLAG_DATA_UNACKNOWLEDGED` | `0x10` |  |
| `IP_CT_TCP_FLAG_MAXACK_SET` | `0x20` |  |
| `IP_CT_EXP_CHALLENGE_ACK` | `0x40` |  |
| `IP_CT_TCP_SIMULTANEOUS_OPEN` | `0x80` |  |

### TCP_CONNTRACK (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_CONNTRACK_SYN_SENT2` | `TCP_CONNTRACK_LISTEN` |  |

## Structs (1)


### `struct nf_ct_tcp_flags`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `mask` | `-` |