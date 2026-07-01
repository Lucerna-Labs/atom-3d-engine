# netdevice.h

**Source:** `netdevice.h`


## Includes

- `linux/if.h`
- `linux/if_ether.h`
- `linux/if_packet.h`
- `linux/if_link.h`

## Defines (11 total)


### INIT_NETDEV (1)

| Name | Value | Comment |
|------|-------|---------|
| `INIT_NETDEV_GROUP` | `0` |  |

### MAX_ADDR (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_ADDR_LEN` | `32` | Largest hardware address length |

### NET_ADDR (4)

| Name | Value | Comment |
|------|-------|---------|
| `NET_ADDR_PERM` | `0` | address is permanent (default) |
| `NET_ADDR_RANDOM` | `1` | address is generated randomly |
| `NET_ADDR_STOLEN` | `2` | address is stolen from other device |
| `NET_ADDR_SET` | `3	/* address is set using` |  |

### NET_NAME (5)

| Name | Value | Comment |
|------|-------|---------|
| `NET_NAME_UNKNOWN` | `0` | unknown origin (not exposed to userspace) |
| `NET_NAME_ENUM` | `1` | enumerated by kernel |
| `NET_NAME_PREDICTABLE` | `2` | predictably named by the kernel |
| `NET_NAME_USER` | `3` | provided by user-space |
| `NET_NAME_RENAMED` | `4` | renamed by user-space |