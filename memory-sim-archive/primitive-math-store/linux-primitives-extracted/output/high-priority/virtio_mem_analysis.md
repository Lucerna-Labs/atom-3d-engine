# virtio_mem.h

**Source:** `virtio_mem.h`


## Includes

- `linux/types.h`
- `linux/virtio_types.h`
- `linux/virtio_ids.h`
- `linux/virtio_config.h`

## Defines (14 total)


### VIRTIO_MEM (14)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_MEM_F_ACPI_PXM` | `0` |  |
| `VIRTIO_MEM_F_UNPLUGGED_INACCESSIBLE` | `1` |  |
| `VIRTIO_MEM_F_PERSISTENT_SUSPEND` | `2` |  |
| `VIRTIO_MEM_REQ_PLUG` | `0` |  |
| `VIRTIO_MEM_REQ_UNPLUG` | `1` |  |
| `VIRTIO_MEM_REQ_UNPLUG_ALL` | `2` |  |
| `VIRTIO_MEM_REQ_STATE` | `3` |  |
| `VIRTIO_MEM_RESP_ACK` | `0` |  |
| `VIRTIO_MEM_RESP_NACK` | `1` |  |
| `VIRTIO_MEM_RESP_BUSY` | `2` |  |
| `VIRTIO_MEM_RESP_ERROR` | `3` |  |
| `VIRTIO_MEM_STATE_PLUGGED` | `0` |  |
| `VIRTIO_MEM_STATE_UNPLUGGED` | `1` |  |
| `VIRTIO_MEM_STATE_MIXED` | `2` |  |

## Structs (7)


### `struct virtio_mem_req_plug`

| Type | Field | Array |
|------|-------|-------|
| `__virtio64` | `addr` | `-` |
| `__virtio16` | `nb_blocks` | `-` |
| `__virtio16` | `padding` | `3` |

### `struct virtio_mem_req_unplug`

| Type | Field | Array |
|------|-------|-------|
| `__virtio64` | `addr` | `-` |
| `__virtio16` | `nb_blocks` | `-` |
| `__virtio16` | `padding` | `3` |

### `struct virtio_mem_req_state`

| Type | Field | Array |
|------|-------|-------|
| `__virtio64` | `addr` | `-` |
| `__virtio16` | `nb_blocks` | `-` |
| `__virtio16` | `padding` | `3` |

### `struct virtio_mem_req`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `type` | `-` |
| `__virtio16` | `padding` | `3` |

### `struct virtio_mem_resp_state`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `state` | `-` |

### `struct virtio_mem_resp`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `type` | `-` |
| `__virtio16` | `padding` | `3` |

### `struct virtio_mem_config`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `block_size` | `-` |
| `__le16` | `node_id` | `-` |
| `__u8` | `padding` | `6` |
| `__le64` | `addr` | `-` |
| `__le64` | `region_size` | `-` |
| `__le64` | `usable_region_size` | `-` |
| `__le64` | `plugged_size` | `-` |
| `__le64` | `requested_size` | `-` |