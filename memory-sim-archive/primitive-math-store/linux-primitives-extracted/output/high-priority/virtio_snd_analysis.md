# virtio_snd.h

**Source:** `virtio_snd.h`


## Includes

- `linux/virtio_types.h`

## Defines (1 total)


### VIRTIO_SND (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_SND_CHMAP_MAX_SIZE` | `18` |  |

## Structs (24)


### `struct virtio_snd_config`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `jacks` | `-` |
| `__le32` | `streams` | `-` |
| `__le32` | `chmaps` | `-` |
| `__le32` | `controls` | `-` |

### `struct virtio_snd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `code` | `-` |

### `struct virtio_snd_event`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `data` | `-` |

### `struct virtio_snd_query_info`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `start_id` | `-` |
| `__le32` | `count` | `-` |
| `__le32` | `size` | `-` |

### `struct virtio_snd_info`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `hda_fn_nid` | `-` |

### `struct virtio_snd_jack_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `jack_id` | `-` |

### `struct virtio_snd_jack_info`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `features` | `-` |
| `__le32` | `hda_reg_defconf` | `-` |
| `__le32` | `hda_reg_caps` | `-` |
| `__u8` | `connected` | `-` |
| `__u8` | `padding` | `7` |

### `struct virtio_snd_jack_remap`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `association` | `-` |
| `__le32` | `sequence` | `-` |

### `struct virtio_snd_pcm_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `stream_id` | `-` |

### `struct virtio_snd_pcm_info`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `features` | `-` |
| `__le64` | `formats` | `-` |
| `__le64` | `rates` | `-` |
| `__u8` | `direction` | `-` |
| `__u8` | `channels_min` | `-` |
| `__u8` | `channels_max` | `-` |
| `__u8` | `padding` | `5` |

### `struct virtio_snd_pcm_set_params`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `buffer_bytes` | `-` |
| `__le32` | `period_bytes` | `-` |
| `__le32` | `features` | `-` |
| `__u8` | `channels` | `-` |
| `__u8` | `format` | `-` |
| `__u8` | `rate` | `-` |
| `__u8` | `padding` | `-` |

### `struct virtio_snd_pcm_xfer`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `stream_id` | `-` |

### `struct virtio_snd_pcm_status`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `status` | `-` |
| `__le32` | `latency_bytes` | `-` |

### `struct virtio_snd_chmap_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `chmap_id` | `-` |

### `struct virtio_snd_chmap_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `direction` | `-` |
| `__u8` | `channels` | `-` |
| `__u8` | `positions` | `VIRTIO_SND_CHMAP_MAX_SIZE` |

### `struct virtio_snd_ctl_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `control_id` | `-` |

### `struct virtio_snd_ctl_info`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `role` | `-` |
| `__le32` | `type` | `-` |
| `__le32` | `access` | `-` |
| `__le32` | `count` | `-` |
| `__le32` | `index` | `-` |
| `__u8` | `name` | `44` |
| `__le32` | `min` | `-` |
| `__le32` | `max` | `-` |
| `__le32` | `step` | `-` |
| `__le64` | `min` | `-` |
| `__le64` | `max` | `-` |
| `__le64` | `step` | `-` |
| `__le32` | `items` | `-` |

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `min` | `-` |
| `__le32` | `max` | `-` |
| `__le32` | `step` | `-` |

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `min` | `-` |
| `__le64` | `max` | `-` |
| `__le64` | `step` | `-` |

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `items` | `-` |

### `struct virtio_snd_ctl_enum_item`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `item` | `64` |

### `struct virtio_snd_ctl_iec958`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `status` | `24` |
| `__u8` | `subcode` | `147` |
| `__u8` | `pad` | `-` |
| `__u8` | `dig_subframe` | `4` |

### `struct virtio_snd_ctl_value`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `integer` | `128` |
| `__le64` | `integer64` | `64` |
| `__le32` | `enumerated` | `128` |
| `__u8` | `bytes` | `512` |

### `struct virtio_snd_ctl_event`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `control_id` | `-` |
| `__le16` | `mask` | `-` |