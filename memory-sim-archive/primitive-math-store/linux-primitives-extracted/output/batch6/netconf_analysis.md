# netconf.h

**Source:** `netconf.h`


## Includes

- `linux/types.h`
- `linux/netlink.h`

## Defines (4 total)


### NETCONFA_IFINDEX (2)

| Name | Value | Comment |
|------|-------|---------|
| `NETCONFA_IFINDEX_ALL` | `-1` |  |
| `NETCONFA_IFINDEX_DEFAULT` | `-2` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `NETCONFA_MAX` | `(__NETCONFA_MAX - 1)` |  |
| `NETCONFA_ALL` | `-1` |  |

## Structs (1)


### `struct netconfmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `ncm_family` | `-` |