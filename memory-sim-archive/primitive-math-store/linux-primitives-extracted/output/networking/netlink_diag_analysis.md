# netlink_diag.h

**Source:** `netlink_diag.h`


## Includes

- `linux/types.h`

## Defines (12 total)


### NDIAG_FLAG (6)

| Name | Value | Comment |
|------|-------|---------|
| `NDIAG_FLAG_CB_RUNNING` | `0x00000001` |  |
| `NDIAG_FLAG_PKTINFO` | `0x00000002` |  |
| `NDIAG_FLAG_BROADCAST_ERROR` | `0x00000004` |  |
| `NDIAG_FLAG_NO_ENOBUFS` | `0x00000008` |  |
| `NDIAG_FLAG_LISTEN_ALL_NSID` | `0x00000010` |  |
| `NDIAG_FLAG_CAP_ACK` | `0x00000020` |  |

### NDIAG_PROTO (1)

| Name | Value | Comment |
|------|-------|---------|
| `NDIAG_PROTO_ALL` | `((__u8) ~0)` |  |

### NDIAG_SHOW (4)

| Name | Value | Comment |
|------|-------|---------|
| `NDIAG_SHOW_MEMINFO` | `0x00000001` | show memory info of a socket |
| `NDIAG_SHOW_GROUPS` | `0x00000002` | show groups of a netlink socket |
| `NDIAG_SHOW_RING_CFG` | `0x00000004` | show ring configuration |
| `NDIAG_SHOW_FLAGS` | `0x00000008` | show flags of a netlink socket |

### NETLINK_DIAG (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_DIAG_MAX` | `(__NETLINK_DIAG_MAX - 1)` |  |

## Structs (3)


### `struct netlink_diag_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `ndiag_ino` | `-` |
| `__u32` | `ndiag_show` | `-` |
| `__u32` | `ndiag_cookie` | `2` |

### `struct netlink_diag_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `ndiag_family` | `-` |
| `__u8` | `ndiag_type` | `-` |
| `__u8` | `ndiag_protocol` | `-` |
| `__u8` | `ndiag_state` | `-` |
| `__u32` | `ndiag_portid` | `-` |
| `__u32` | `ndiag_dst_portid` | `-` |
| `__u32` | `ndiag_dst_group` | `-` |
| `__u32` | `ndiag_ino` | `-` |
| `__u32` | `ndiag_cookie` | `2` |

### `struct netlink_diag_ring`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ndr_block_size` | `-` |
| `__u32` | `ndr_block_nr` | `-` |
| `__u32` | `ndr_frame_size` | `-` |
| `__u32` | `ndr_frame_nr` | `-` |