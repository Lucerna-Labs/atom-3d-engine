# mroute6.h

**Source:** `mroute6.h`


## Includes

- `linux/const.h`
- `linux/types.h`
- `linux/sockios.h`
- `linux/in6.h`

## Defines (32 total)


### UNCATEGORIZED (32)

| Name | Value | Comment |
|------|-------|---------|
| `MRT6_BASE` | `200` |  |
| `MRT6_INIT` | `(MRT6_BASE)` | Activate the kernel mroute code |
| `MRT6_DONE` | `(MRT6_BASE+1)` | Shutdown the kernel mroute |
| `MRT6_ADD_MIF` | `(MRT6_BASE+2)` | Add a virtual interface |
| `MRT6_DEL_MIF` | `(MRT6_BASE+3)` | Delete a virtual interface |
| `MRT6_ADD_MFC` | `(MRT6_BASE+4)` | Add a multicast forwarding entry |
| `MRT6_DEL_MFC` | `(MRT6_BASE+5)` | Delete a multicast forwarding entry |
| `MRT6_VERSION` | `(MRT6_BASE+6)` | Get the kernel multicast version |
| `MRT6_ASSERT` | `(MRT6_BASE+7)` | Activate PIM assert mode |
| `MRT6_PIM` | `(MRT6_BASE+8)` | enable PIM code |
| `MRT6_TABLE` | `(MRT6_BASE+9)` | Specify mroute table ID |
| `MRT6_ADD_MFC_PROXY` | `(MRT6_BASE+10)` | Add a (*,*|G) mfc entry |
| `MRT6_DEL_MFC_PROXY` | `(MRT6_BASE+11)` | Del a (*,*|G) mfc entry |
| `MRT6_FLUSH` | `(MRT6_BASE+12)` | Flush all mfc entries and/or vifs |
| `MRT6_MAX` | `(MRT6_BASE+12)` |  |
| `SIOCGETMIFCNT_IN6` | `SIOCPROTOPRIVATE` | IP protocol privates |
| `SIOCGETSGCNT_IN6` | `(SIOCPROTOPRIVATE+1)` |  |
| `SIOCGETRPF` | `(SIOCPROTOPRIVATE+2)` |  |
| `MRT6_FLUSH_MFC` | `1` | Flush multicast entries |
| `MRT6_FLUSH_MFC_STATIC` | `2` | Flush static multicast entries |
| `MRT6_FLUSH_MIFS` | `4` | Flushing multicast vifs |
| `MRT6_FLUSH_MIFS_STATIC` | `8` | Flush static multicast vifs |
| `MAXMIFS` | `32` |  |
| `ALL_MIFS` | `((mifi_t)(-1))` |  |
| `IF_SETSIZE` | `256` |  |
| `NIFBITS` | `(sizeof(if_mask) * 8)` | bits per mask |
| `MIFF_REGISTER` | `0x1` | register vif |
| `MRT6MSG_NOCACHE` | `1` |  |
| `MRT6MSG_WRONGMIF` | `2` |  |
| `MRT6MSG_WHOLEPKT` | `3` | used for use level encap |
| `MRT6MSG_WRMIFWHOLE` | `4` | For PIM Register and assert processing |
| `IP6MRA_CREPORT_MAX` | `(__IP6MRA_CREPORT_MAX - 1)` |  |

## Structs (6)


### `struct if_set`

| Type | Field | Array |
|------|-------|-------|
| `if_mask` | `ifs_bits` | `__KERNEL_DIV_ROUND_UP(IF_SETSIZE, NIFBITS)` |

### `struct mif6ctl`

| Type | Field | Array |
|------|-------|-------|
| `mifi_t` | `mif6c_mifi` | `-` |
| `__u16` | `mif6c_pifi` | `-` |

### `struct mf6cctl`

| Type | Field | Array |
|------|-------|-------|
| `mifi_t` | `mf6cc_parent` | `-` |

### `struct sioc_sg_req6`

| Type | Field | Array |
|------|-------|-------|

### `struct sioc_mif_req6`

| Type | Field | Array |
|------|-------|-------|
| `mifi_t` | `mifi` | `-` |

### `struct mrt6msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `im6_mbz` | `-` |
| `__u8` | `im6_msgtype` | `-` |
| `__u16` | `im6_mif` | `-` |
| `__u32` | `im6_pad` | `-` |

## Typedefs

- `if_set`