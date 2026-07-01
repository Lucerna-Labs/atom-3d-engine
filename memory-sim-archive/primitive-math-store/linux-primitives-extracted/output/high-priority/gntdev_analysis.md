# gntdev.h

**Source:** `gntdev.h`


## Includes

- `linux/types.h`

## Defines (14 total)


### GNTDEV_DMA (2)

| Name | Value | Comment |
|------|-------|---------|
| `GNTDEV_DMA_FLAG_WC` | `(1 << 0)` |  |
| `GNTDEV_DMA_FLAG_COHERENT` | `(1 << 1)` |  |

### IOCTL_GNTDEV (10)

| Name | Value | Comment |
|------|-------|---------|
| `IOCTL_GNTDEV_MAP_GRANT_REF` | `` |  |
| `IOCTL_GNTDEV_UNMAP_GRANT_REF` | `` |  |
| `IOCTL_GNTDEV_GET_OFFSET_FOR_VADDR` | `` |  |
| `IOCTL_GNTDEV_SET_MAX_GRANTS` | `` |  |
| `IOCTL_GNTDEV_SET_UNMAP_NOTIFY` | `` |  |
| `IOCTL_GNTDEV_GRANT_COPY` | `` |  |
| `IOCTL_GNTDEV_DMABUF_EXP_FROM_REFS` | `` |  |
| `IOCTL_GNTDEV_DMABUF_EXP_WAIT_RELEASED` | `` |  |
| `IOCTL_GNTDEV_DMABUF_IMP_TO_REFS` | `` |  |
| `IOCTL_GNTDEV_DMABUF_IMP_RELEASE` | `` |  |

### UNMAP_NOTIFY (2)

| Name | Value | Comment |
|------|-------|---------|
| `UNMAP_NOTIFY_CLEAR_BYTE` | `0x1` |  |
| `UNMAP_NOTIFY_SEND_EVENT` | `0x2` |  |

## Structs (13)


### `struct ioctl_gntdev_grant_ref`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `domid` | `-` |
| `__u32` | `ref` | `-` |

### `struct ioctl_gntdev_map_grant_ref`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `index` | `-` |

### `struct ioctl_gntdev_unmap_grant_ref`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `index` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `pad` | `-` |

### `struct ioctl_gntdev_get_offset_for_vaddr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `vaddr` | `-` |
| `__u64` | `offset` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `pad` | `-` |

### `struct ioctl_gntdev_set_max_grants`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |

### `struct ioctl_gntdev_unmap_notify`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `index` | `-` |
| `__u32` | `action` | `-` |
| `__u32` | `event_channel_port` | `-` |

### `struct gntdev_grant_copy_segment`

| Type | Field | Array |
|------|-------|-------|
| `grant_ref_t` | `ref` | `-` |
| `__u16` | `offset` | `-` |
| `domid_t` | `domid` | `-` |
| `__u16` | `len` | `-` |
| `__u16` | `flags` | `-` |
| `__s16` | `status` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `grant_ref_t` | `ref` | `-` |
| `__u16` | `offset` | `-` |
| `domid_t` | `domid` | `-` |

### `struct ioctl_gntdev_grant_copy`

| Type | Field | Array |
|------|-------|-------|

### `struct ioctl_gntdev_dmabuf_exp_from_refs`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `fd` | `-` |
| `__u32` | `domid` | `-` |
| `__u32` | `refs` | `1` |

### `struct ioctl_gntdev_dmabuf_exp_wait_released`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fd` | `-` |
| `__u32` | `wait_to_ms` | `-` |

### `struct ioctl_gntdev_dmabuf_imp_to_refs`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fd` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `domid` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `refs` | `1` |

### `struct ioctl_gntdev_dmabuf_imp_release`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fd` | `-` |
| `__u32` | `reserved` | `-` |