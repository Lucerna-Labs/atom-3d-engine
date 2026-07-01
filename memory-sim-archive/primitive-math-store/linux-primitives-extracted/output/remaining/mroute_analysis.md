# mroute.h

**Source:** `mroute.h`


## Includes

- `linux/sockios.h`
- `linux/types.h`
- `linux/in.h`

## Defines (37 total)


### IPMRA_CREPORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMRA_CREPORT_MAX` | `(__IPMRA_CREPORT_MAX - 1)` |  |

### IPMRA_TABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMRA_TABLE_MAX` | `(__IPMRA_TABLE_MAX - 1)` |  |

### IPMRA_VIF (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMRA_VIF_MAX` | `(__IPMRA_VIF_MAX - 1)` |  |

### IPMRA_VIFA (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMRA_VIFA_MAX` | `(__IPMRA_VIFA_MAX - 1)` |  |

### MFC_ASSERT (1)

| Name | Value | Comment |
|------|-------|---------|
| `MFC_ASSERT_THRESH` | `(3*HZ)` | Maximal freq. of asserts |

### MRT_ADD (3)

| Name | Value | Comment |
|------|-------|---------|
| `MRT_ADD_VIF` | `(MRT_BASE+2)` | Add a virtual interface |
| `MRT_ADD_MFC` | `(MRT_BASE+4)` | Add a multicast forwarding entry |
| `MRT_ADD_MFC_PROXY` | `(MRT_BASE+10)` | Add a (*,*|G) mfc entry |

### MRT_DEL (3)

| Name | Value | Comment |
|------|-------|---------|
| `MRT_DEL_VIF` | `(MRT_BASE+3)` | Delete a virtual interface |
| `MRT_DEL_MFC` | `(MRT_BASE+5)` | Delete a multicast forwarding entry |
| `MRT_DEL_MFC_PROXY` | `(MRT_BASE+11)` | Del a (*,*|G) mfc entry |

### MRT_FLUSH (4)

| Name | Value | Comment |
|------|-------|---------|
| `MRT_FLUSH_MFC` | `1` | Flush multicast entries |
| `MRT_FLUSH_MFC_STATIC` | `2` | Flush static multicast entries |
| `MRT_FLUSH_VIFS` | `4` | Flush multicast vifs |
| `MRT_FLUSH_VIFS_STATIC` | `8` | Flush static multicast vifs |

### UNCATEGORIZED (21)

| Name | Value | Comment |
|------|-------|---------|
| `MRT_BASE` | `200` |  |
| `MRT_INIT` | `(MRT_BASE)` | Activate the kernel mroute code |
| `MRT_DONE` | `(MRT_BASE+1)` | Shutdown the kernel mroute |
| `MRT_VERSION` | `(MRT_BASE+6)` | Get the kernel multicast version |
| `MRT_ASSERT` | `(MRT_BASE+7)` | Activate PIM assert mode |
| `MRT_PIM` | `(MRT_BASE+8)` | enable PIM code |
| `MRT_TABLE` | `(MRT_BASE+9)` | Specify mroute table ID |
| `MRT_FLUSH` | `(MRT_BASE+12)` | Flush all mfc entries and/or vifs |
| `MRT_MAX` | `(MRT_BASE+12)` |  |
| `SIOCGETVIFCNT` | `SIOCPROTOPRIVATE` | IP protocol privates |
| `SIOCGETSGCNT` | `(SIOCPROTOPRIVATE+1)` |  |
| `SIOCGETRPF` | `(SIOCPROTOPRIVATE+2)` |  |
| `MAXVIFS` | `32` |  |
| `ALL_VIFS` | `((vifi_t)(-1))` |  |
| `VIFF_TUNNEL` | `0x1` | IPIP tunnel |
| `VIFF_SRCRT` | `0x2` | NI |
| `VIFF_REGISTER` | `0x4` | register vif |
| `IGMPMSG_NOCACHE` | `1` | Kern cache fill request to mrouted |
| `IGMPMSG_WRONGVIF` | `2` | For PIM assert processing (unused) |
| `IGMPMSG_WHOLEPKT` | `3` | For PIM Register processing |
| `IGMPMSG_WRVIFWHOLE` | `4` | For PIM Register and assert processing |

### VIFF_USE (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIFF_USE_IFINDEX` | `0x8	/* use vifc_lcl_ifindex instead of` |  |

## Structs (5)


### `struct vifctl`

| Type | Field | Array |
|------|-------|-------|
| `vifi_t` | `vifc_vifi` | `-` |
| `int` | `vifc_lcl_ifindex` | `-` |

### `struct mfcctl`

| Type | Field | Array |
|------|-------|-------|
| `vifi_t` | `mfcc_parent` | `-` |
| `int` | `mfcc_expire` | `-` |

### `struct sioc_sg_req`

| Type | Field | Array |
|------|-------|-------|

### `struct sioc_vif_req`

| Type | Field | Array |
|------|-------|-------|
| `vifi_t` | `vifi` | `-` |

### `struct igmpmsg`

| Type | Field | Array |
|------|-------|-------|