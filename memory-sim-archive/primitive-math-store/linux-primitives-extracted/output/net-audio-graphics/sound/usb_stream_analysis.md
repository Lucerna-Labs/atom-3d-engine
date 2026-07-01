# usb_stream.h

**Source:** `usb_stream.h`


## Defines (2 total)


### SNDRV_USB (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_USB_STREAM_IOCTL_SET_PARAMS` | `` |  |

### USB_STREAM (1)

| Name | Value | Comment |
|------|-------|---------|
| `USB_STREAM_INTERFACE_VERSION` | `2` |  |

## Structs (3)


### `struct usb_stream_packet`

| Type | Field | Array |
|------|-------|-------|
| `unsigned` | `offset` | `-` |
| `unsigned` | `length` | `-` |

### `struct usb_stream_config`

| Type | Field | Array |
|------|-------|-------|
| `unsigned` | `version` | `-` |
| `unsigned` | `sample_rate` | `-` |
| `unsigned` | `period_frames` | `-` |
| `unsigned` | `frame_size` | `-` |

### `struct usb_stream`

| Type | Field | Array |
|------|-------|-------|
| `unsigned` | `read_size` | `-` |
| `unsigned` | `write_size` | `-` |
| `int` | `period_size` | `-` |
| `unsigned` | `state` | `-` |
| `int` | `idle_insize` | `-` |
| `int` | `idle_outsize` | `-` |
| `int` | `sync_packet` | `-` |
| `unsigned` | `insize_done` | `-` |
| `unsigned` | `periods_done` | `-` |
| `unsigned` | `periods_polled` | `-` |
| `unsigned` | `inpackets` | `-` |
| `unsigned` | `inpacket_head` | `-` |
| `unsigned` | `inpacket_split` | `-` |
| `unsigned` | `inpacket_split_at` | `-` |
| `unsigned` | `next_inpacket_split` | `-` |
| `unsigned` | `next_inpacket_split_at` | `-` |