# usbdevice_fs.h

**Source:** `usbdevice_fs.h`


## Includes

- `linux/types.h`
- `linux/magic.h`

## Defines (60 total)


### UNCATEGORIZED (26)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_MAXDRIVERNAME` | `255` |  |
| `USBDEVFS_CONTROL` | `_IOWR('U', 0, struct usbdevfs_ctrltransfer)` |  |
| `USBDEVFS_CONTROL32` | `_IOWR('U', 0, struct usbdevfs_ctrltransfer32)` |  |
| `USBDEVFS_BULK` | `_IOWR('U', 2, struct usbdevfs_bulktransfer)` |  |
| `USBDEVFS_BULK32` | `_IOWR('U', 2, struct usbdevfs_bulktransfer32)` |  |
| `USBDEVFS_RESETEP` | `_IOR('U', 3, unsigned int)` |  |
| `USBDEVFS_SETINTERFACE` | `_IOR('U', 4, struct usbdevfs_setinterface)` |  |
| `USBDEVFS_SETCONFIGURATION` | `_IOR('U', 5, unsigned int)` |  |
| `USBDEVFS_GETDRIVER` | `_IOW('U', 8, struct usbdevfs_getdriver)` |  |
| `USBDEVFS_SUBMITURB` | `_IOR('U', 10, struct usbdevfs_urb)` |  |
| `USBDEVFS_SUBMITURB32` | `_IOR('U', 10, struct usbdevfs_urb32)` |  |
| `USBDEVFS_DISCARDURB` | `_IO('U', 11)` |  |
| `USBDEVFS_REAPURB` | `_IOW('U', 12, void *)` |  |
| `USBDEVFS_REAPURB32` | `_IOW('U', 12, __u32)` |  |
| `USBDEVFS_REAPURBNDELAY` | `_IOW('U', 13, void *)` |  |
| `USBDEVFS_REAPURBNDELAY32` | `_IOW('U', 13, __u32)` |  |
| `USBDEVFS_DISCSIGNAL` | `_IOR('U', 14, struct usbdevfs_disconnectsignal)` |  |
| `USBDEVFS_DISCSIGNAL32` | `_IOR('U', 14, struct usbdevfs_disconnectsignal32)` |  |
| `USBDEVFS_CLAIMINTERFACE` | `_IOR('U', 15, unsigned int)` |  |
| `USBDEVFS_RELEASEINTERFACE` | `_IOR('U', 16, unsigned int)` |  |
| `USBDEVFS_CONNECTINFO` | `_IOW('U', 17, struct usbdevfs_connectinfo)` |  |
| `USBDEVFS_IOCTL` | `_IOWR('U', 18, struct usbdevfs_ioctl)` |  |
| `USBDEVFS_IOCTL32` | `_IOWR('U', 18, struct usbdevfs_ioctl32)` |  |
| `USBDEVFS_RESET` | `_IO('U', 20)` |  |
| `USBDEVFS_DISCONNECT` | `_IO('U', 22)` |  |
| `USBDEVFS_CONNECT` | `_IO('U', 23)` |  |

### USBDEVFS_ALLOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_ALLOC_STREAMS` | `_IOR('U', 28, struct usbdevfs_streams)` |  |

### USBDEVFS_ALLOW (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_ALLOW_SUSPEND` | `_IO('U', 34)` |  |

### USBDEVFS_CAP (9)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_CAP_ZERO_PACKET` | `0x01` |  |
| `USBDEVFS_CAP_BULK_CONTINUATION` | `0x02` |  |
| `USBDEVFS_CAP_NO_PACKET_SIZE_LIM` | `0x04` |  |
| `USBDEVFS_CAP_BULK_SCATTER_GATHER` | `0x08` |  |
| `USBDEVFS_CAP_REAP_AFTER_DISCONNECT` | `0x10` |  |
| `USBDEVFS_CAP_MMAP` | `0x20` |  |
| `USBDEVFS_CAP_DROP_PRIVILEGES` | `0x40` |  |
| `USBDEVFS_CAP_CONNINFO_EX` | `0x80` |  |
| `USBDEVFS_CAP_SUSPEND` | `0x100` |  |

### USBDEVFS_CLAIM (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_CLAIM_PORT` | `_IOR('U', 24, unsigned int)` |  |

### USBDEVFS_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_CLEAR_HALT` | `_IOR('U', 21, unsigned int)` |  |

### USBDEVFS_DISCONNECT (3)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_DISCONNECT_CLAIM_IF_DRIVER` | `0x01` |  |
| `USBDEVFS_DISCONNECT_CLAIM_EXCEPT_DRIVER` | `0x02` |  |
| `USBDEVFS_DISCONNECT_CLAIM` | `_IOR('U', 27, struct usbdevfs_disconnect_claim)` |  |

### USBDEVFS_DROP (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_DROP_PRIVILEGES` | `_IOW('U', 30, __u32)` |  |

### USBDEVFS_FORBID (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_FORBID_SUSPEND` | `_IO('U', 33)` |  |

### USBDEVFS_FREE (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_FREE_STREAMS` | `_IOR('U', 29, struct usbdevfs_streams)` |  |

### USBDEVFS_GET (2)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_GET_CAPABILITIES` | `_IOR('U', 26, __u32)` |  |
| `USBDEVFS_GET_SPEED` | `_IO('U', 31)` |  |

### USBDEVFS_HUB (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_HUB_PORTINFO` | `_IOR('U', 19, struct usbdevfs_hub_portinfo)` |  |

### USBDEVFS_RELEASE (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_RELEASE_PORT` | `_IOR('U', 25, unsigned int)` |  |

### USBDEVFS_URB (10)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_URB_SHORT_NOT_OK` | `0x01` |  |
| `USBDEVFS_URB_ISO_ASAP` | `0x02` |  |
| `USBDEVFS_URB_BULK_CONTINUATION` | `0x04` |  |
| `USBDEVFS_URB_NO_FSBR` | `0x20` | Not used |
| `USBDEVFS_URB_ZERO_PACKET` | `0x40` |  |
| `USBDEVFS_URB_NO_INTERRUPT` | `0x80` |  |
| `USBDEVFS_URB_TYPE_ISO` | `0` |  |
| `USBDEVFS_URB_TYPE_INTERRUPT` | `1` |  |
| `USBDEVFS_URB_TYPE_CONTROL` | `2` |  |
| `USBDEVFS_URB_TYPE_BULK` | `3` |  |

### USBDEVFS_WAIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `USBDEVFS_WAIT_FOR_RESUME` | `_IO('U', 35)` |  |

## Structs (13)


### `struct usbdevfs_ctrltransfer`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `bRequestType` | `-` |
| `__u8` | `bRequest` | `-` |
| `__u16` | `wValue` | `-` |
| `__u16` | `wIndex` | `-` |
| `__u16` | `wLength` | `-` |
| `__u32` | `timeout` | `-` |

### `struct usbdevfs_bulktransfer`

| Type | Field | Array |
|------|-------|-------|

### `struct usbdevfs_setinterface`

| Type | Field | Array |
|------|-------|-------|

### `struct usbdevfs_disconnectsignal`

| Type | Field | Array |
|------|-------|-------|

### `struct usbdevfs_getdriver`

| Type | Field | Array |
|------|-------|-------|
| `char` | `driver` | `USBDEVFS_MAXDRIVERNAME + 1` |

### `struct usbdevfs_connectinfo`

| Type | Field | Array |
|------|-------|-------|

### `struct usbdevfs_conninfo_ex`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `busnum` | `-` |
| `__u32` | `devnum` | `-` |
| `__u32` | `speed` | `-` |
| `__u8` | `num_ports` | `-` |
| `__u8` | `ports` | `7` |

### `struct usbdevfs_iso_packet_desc`

| Type | Field | Array |
|------|-------|-------|

### `struct usbdevfs_urb`

| Type | Field | Array |
|------|-------|-------|
| `int` | `status` | `-` |
| `int` | `buffer_length` | `-` |
| `int` | `actual_length` | `-` |
| `int` | `start_frame` | `-` |
| `int` | `number_of_packets` | `-` |
| `int` | `error_count` | `-` |

### `struct usbdevfs_ioctl`

| Type | Field | Array |
|------|-------|-------|
| `int` | `ifno` | `-` |
| `int` | `ioctl_code` | `-` |

### `struct usbdevfs_hub_portinfo`

| Type | Field | Array |
|------|-------|-------|
| `char` | `nports` | `-` |
| `char` | `port` | `127` |

### `struct usbdevfs_disconnect_claim`

| Type | Field | Array |
|------|-------|-------|
| `char` | `driver` | `USBDEVFS_MAXDRIVERNAME + 1` |

### `struct usbdevfs_streams`

| Type | Field | Array |
|------|-------|-------|