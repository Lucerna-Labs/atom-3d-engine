# vm_sockets_diag.h

**Source:** `vm_sockets_diag.h`


## Includes

- `linux/types.h`

## Structs (2)


### `struct vsock_diag_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `vdiag_states` | `-` |
| `__u32` | `vdiag_ino` | `-` |
| `__u32` | `vdiag_show` | `-` |
| `__u32` | `vdiag_cookie` | `2` |

### `struct vsock_diag_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `vdiag_family` | `-` |
| `__u8` | `vdiag_type` | `-` |
| `__u8` | `vdiag_state` | `-` |
| `__u8` | `vdiag_shutdown` | `-` |
| `__u32` | `vdiag_src_cid` | `-` |
| `__u32` | `vdiag_src_port` | `-` |
| `__u32` | `vdiag_dst_cid` | `-` |
| `__u32` | `vdiag_dst_port` | `-` |
| `__u32` | `vdiag_ino` | `-` |
| `__u32` | `vdiag_cookie` | `2` |