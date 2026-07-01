# if_xdp.h

**Source:** `if_xdp.h`


## Includes

- `linux/types.h`

## Defines (30 total)


### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_COPY` | `(1 << 1)` | Force copy-mode |
| `XDP_ZEROCOPY` | `(1 << 2)` | Force zero-copy mode |
| `XDP_STATISTICS` | `7` |  |
| `XDP_OPTIONS` | `8` |  |

### XDP_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_MAX_TX_SKB_BUDGET` | `9` |  |

### XDP_MMAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_MMAP_OFFSETS` | `1` |  |

### XDP_OPTIONS (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_OPTIONS_ZEROCOPY` | `(1 << 0)` |  |

### XDP_PGOFF (2)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_PGOFF_RX_RING` | `0` |  |
| `XDP_PGOFF_TX_RING` | `0x80000000` |  |

### XDP_PKT (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_PKT_CONTD` | `(1 << 0)` |  |

### XDP_RING (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_RING_NEED_WAKEUP` | `(1 << 0)` |  |

### XDP_RX (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_RX_RING` | `2` |  |

### XDP_SHARED (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_SHARED_UMEM` | `(1 << 0)` |  |

### XDP_TX (2)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_TX_RING` | `3` |  |
| `XDP_TX_METADATA` | `(1 << 1)` |  |

### XDP_TXMD (3)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_TXMD_FLAGS_TIMESTAMP` | `(1 << 0)` |  |
| `XDP_TXMD_FLAGS_CHECKSUM` | `(1 << 1)` |  |
| `XDP_TXMD_FLAGS_LAUNCH_TIME` | `(1 << 2)` |  |

### XDP_UMEM (8)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_UMEM_UNALIGNED_CHUNK_FLAG` | `(1 << 0)` |  |
| `XDP_UMEM_TX_SW_CSUM` | `(1 << 1)` |  |
| `XDP_UMEM_TX_METADATA_LEN` | `(1 << 2)` |  |
| `XDP_UMEM_REG` | `4` |  |
| `XDP_UMEM_FILL_RING` | `5` |  |
| `XDP_UMEM_COMPLETION_RING` | `6` |  |
| `XDP_UMEM_PGOFF_FILL_RING` | `0x100000000ULL` |  |
| `XDP_UMEM_PGOFF_COMPLETION_RING` | `0x180000000ULL` |  |

### XDP_USE (2)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_USE_NEED_WAKEUP` | `(1 << 3)` |  |
| `XDP_USE_SG` | `(1 << 4)` |  |

### XSK_UNALIGNED (2)

| Name | Value | Comment |
|------|-------|---------|
| `XSK_UNALIGNED_BUF_OFFSET_SHIFT` | `48` |  |
| `XSK_UNALIGNED_BUF_ADDR_MASK` | `` |  |

## Structs (10)


### `struct sockaddr_xdp`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sxdp_family` | `-` |
| `__u16` | `sxdp_flags` | `-` |
| `__u32` | `sxdp_ifindex` | `-` |
| `__u32` | `sxdp_queue_id` | `-` |
| `__u32` | `sxdp_shared_umem_fd` | `-` |

### `struct xdp_ring_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `producer` | `-` |
| `__u64` | `consumer` | `-` |
| `__u64` | `desc` | `-` |
| `__u64` | `flags` | `-` |

### `struct xdp_mmap_offsets`

| Type | Field | Array |
|------|-------|-------|

### `struct xdp_umem_reg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `len` | `-` |
| `__u32` | `chunk_size` | `-` |
| `__u32` | `headroom` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `tx_metadata_len` | `-` |

### `struct xdp_statistics`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rx_dropped` | `-` |
| `__u64` | `rx_invalid_descs` | `-` |
| `__u64` | `tx_invalid_descs` | `-` |
| `__u64` | `rx_ring_full` | `-` |
| `__u64` | `rx_fill_ring_empty_descs` | `-` |
| `__u64` | `tx_ring_empty_descs` | `-` |

### `struct xdp_options`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |

### `struct xsk_tx_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u16` | `csum_start` | `-` |
| `__u16` | `csum_offset` | `-` |
| `__u64` | `launch_time` | `-` |
| `__u64` | `tx_timestamp` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `csum_start` | `-` |
| `__u16` | `csum_offset` | `-` |
| `__u64` | `launch_time` | `-` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `tx_timestamp` | `-` |

### `struct xdp_desc`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u32` | `len` | `-` |
| `__u32` | `options` | `-` |