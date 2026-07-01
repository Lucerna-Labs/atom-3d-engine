# xdp_diag.h

**Source:** `xdp_diag.h`


## Includes

- `linux/types.h`

## Defines (7 total)


### XDP_DIAG (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_DIAG_MAX` | `(__XDP_DIAG_MAX - 1)` |  |

### XDP_DU (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_DU_F_ZEROCOPY` | `(1 << 0)` |  |

### XDP_SHOW (5)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_SHOW_INFO` | `(1 << 0)` | Basic information |
| `XDP_SHOW_RING_CFG` | `(1 << 1)` |  |
| `XDP_SHOW_UMEM` | `(1 << 2)` |  |
| `XDP_SHOW_MEMINFO` | `(1 << 3)` |  |
| `XDP_SHOW_STATS` | `(1 << 4)` |  |

## Structs (6)


### `struct xdp_diag_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `xdiag_ino` | `-` |
| `__u32` | `xdiag_show` | `-` |
| `__u32` | `xdiag_cookie` | `2` |

### `struct xdp_diag_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `xdiag_family` | `-` |
| `__u8` | `xdiag_type` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `xdiag_ino` | `-` |
| `__u32` | `xdiag_cookie` | `2` |

### `struct xdp_diag_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifindex` | `-` |
| `__u32` | `queue_id` | `-` |

### `struct xdp_diag_ring`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `entries` | `-` |

### `struct xdp_diag_umem`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `num_pages` | `-` |
| `__u32` | `chunk_size` | `-` |
| `__u32` | `headroom` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `queue_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `refs` | `-` |

### `struct xdp_diag_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `n_rx_dropped` | `-` |
| `__u64` | `n_rx_invalid` | `-` |
| `__u64` | `n_rx_full` | `-` |
| `__u64` | `n_fill_ring_empty` | `-` |
| `__u64` | `n_tx_invalid` | `-` |
| `__u64` | `n_tx_ring_empty` | `-` |