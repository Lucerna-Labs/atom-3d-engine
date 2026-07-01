# dma-buf.h

**Source:** `dma-buf.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (14 total)


### DMA_BUF (14)

| Name | Value | Comment |
|------|-------|---------|
| `DMA_BUF_SYNC_READ` | `(1 << 0)` |  |
| `DMA_BUF_SYNC_WRITE` | `(2 << 0)` |  |
| `DMA_BUF_SYNC_RW` | `(DMA_BUF_SYNC_READ \| DMA_BUF_SYNC_WRITE)` |  |
| `DMA_BUF_SYNC_START` | `(0 << 2)` |  |
| `DMA_BUF_SYNC_END` | `(1 << 2)` |  |
| `DMA_BUF_SYNC_VALID_FLAGS_MASK` | `` |  |
| `DMA_BUF_NAME_LEN` | `32` |  |
| `DMA_BUF_BASE` | `'b'` |  |
| `DMA_BUF_IOCTL_SYNC` | `_IOW(DMA_BUF_BASE, 0, struct dma_buf_sync)` |  |
| `DMA_BUF_SET_NAME` | `_IOW(DMA_BUF_BASE, 1, const char *)` |  |
| `DMA_BUF_SET_NAME_A` | `_IOW(DMA_BUF_BASE, 1, __u32)` |  |
| `DMA_BUF_SET_NAME_B` | `_IOW(DMA_BUF_BASE, 1, __u64)` |  |
| `DMA_BUF_IOCTL_EXPORT_SYNC_FILE` | `_IOWR(DMA_BUF_BASE, 2, struct dma_buf_export_sync_file)` |  |
| `DMA_BUF_IOCTL_IMPORT_SYNC_FILE` | `_IOW(DMA_BUF_BASE, 3, struct dma_buf_import_sync_file)` |  |

## Structs (3)


### `struct dma_buf_sync`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |

### `struct dma_buf_export_sync_file`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__s32` | `fd` | `-` |

### `struct dma_buf_import_sync_file`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__s32` | `fd` | `-` |