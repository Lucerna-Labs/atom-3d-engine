# virtio_console.h

**Source:** `virtio_console.h`


## Includes

- `linux/types.h`
- `linux/virtio_types.h`
- `linux/virtio_ids.h`
- `linux/virtio_config.h`

## Defines (12 total)


### VIRTIO_CONSOLE (12)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_CONSOLE_F_SIZE` | `0` | Does host provide console size? |
| `VIRTIO_CONSOLE_F_MULTIPORT` | `1` | Does host provide multiple ports? |
| `VIRTIO_CONSOLE_F_EMERG_WRITE` | `2` | Does host support emergency write? |
| `VIRTIO_CONSOLE_BAD_ID` | `(~(__u32)0)` |  |
| `VIRTIO_CONSOLE_DEVICE_READY` | `0` |  |
| `VIRTIO_CONSOLE_PORT_ADD` | `1` |  |
| `VIRTIO_CONSOLE_PORT_REMOVE` | `2` |  |
| `VIRTIO_CONSOLE_PORT_READY` | `3` |  |
| `VIRTIO_CONSOLE_CONSOLE_PORT` | `4` |  |
| `VIRTIO_CONSOLE_RESIZE` | `5` |  |
| `VIRTIO_CONSOLE_PORT_OPEN` | `6` |  |
| `VIRTIO_CONSOLE_PORT_NAME` | `7` |  |

## Structs (2)


### `struct virtio_console_config`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `cols` | `-` |
| `__virtio16` | `rows` | `-` |
| `__virtio32` | `max_nr_ports` | `-` |
| `__virtio32` | `emerg_wr` | `-` |

### `struct virtio_console_control`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `id` | `-` |
| `__virtio16` | `event` | `-` |
| `__virtio16` | `value` | `-` |