# compress_offload.h

**Source:** `compress_offload.h`


## Includes

- `linux/types.h`
- `sound/asound.h`
- `sound/compress_params.h`

## Defines (28 total)


### SNDRV_COMPRESS (24)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_COMPRESS_VERSION` | `SNDRV_PROTOCOL_VERSION(0, 4, 1)` |  |
| `SNDRV_COMPRESS_IOCTL_VERSION` | `_IOR('C', 0x00, int)` |  |
| `SNDRV_COMPRESS_GET_CAPS` | `_IOWR('C', 0x10, struct snd_compr_caps)` |  |
| `SNDRV_COMPRESS_GET_CODEC_CAPS` | `_IOWR('C', 0x11,` |  |
| `SNDRV_COMPRESS_SET_PARAMS` | `_IOW('C', 0x12, struct snd_compr_params)` |  |
| `SNDRV_COMPRESS_GET_PARAMS` | `_IOR('C', 0x13, struct snd_codec)` |  |
| `SNDRV_COMPRESS_SET_METADATA` | `_IOW('C', 0x14,` |  |
| `SNDRV_COMPRESS_GET_METADATA` | `_IOWR('C', 0x15,` |  |
| `SNDRV_COMPRESS_TSTAMP` | `_IOR('C', 0x20, struct snd_compr_tstamp)` |  |
| `SNDRV_COMPRESS_AVAIL` | `_IOR('C', 0x21, struct snd_compr_avail)` |  |
| `SNDRV_COMPRESS_TSTAMP64` | `_IOR('C', 0x22, struct snd_compr_tstamp64)` |  |
| `SNDRV_COMPRESS_AVAIL64` | `_IOR('C', 0x23, struct snd_compr_avail64)` |  |
| `SNDRV_COMPRESS_PAUSE` | `_IO('C', 0x30)` |  |
| `SNDRV_COMPRESS_RESUME` | `_IO('C', 0x31)` |  |
| `SNDRV_COMPRESS_START` | `_IO('C', 0x32)` |  |
| `SNDRV_COMPRESS_STOP` | `_IO('C', 0x33)` |  |
| `SNDRV_COMPRESS_DRAIN` | `_IO('C', 0x34)` |  |
| `SNDRV_COMPRESS_NEXT_TRACK` | `_IO('C', 0x35)` |  |
| `SNDRV_COMPRESS_PARTIAL_DRAIN` | `_IO('C', 0x36)` |  |
| `SNDRV_COMPRESS_TASK_CREATE` | `_IOWR('C', 0x60, struct snd_compr_task)` |  |
| `SNDRV_COMPRESS_TASK_FREE` | `_IOW('C', 0x61, __u64)` |  |
| `SNDRV_COMPRESS_TASK_START` | `_IOWR('C', 0x62, struct snd_compr_task)` |  |
| `SNDRV_COMPRESS_TASK_STOP` | `_IOW('C', 0x63, __u64)` |  |
| `SNDRV_COMPRESS_TASK_STATUS` | `_IOWR('C', 0x68, struct snd_compr_task_status)` |  |

### SND_COMPR (3)

| Name | Value | Comment |
|------|-------|---------|
| `SND_COMPR_TRIGGER_DRAIN` | `7` | FIXME move this to pcm.h |
| `SND_COMPR_TRIGGER_NEXT_TRACK` | `8` |  |
| `SND_COMPR_TRIGGER_PARTIAL_DRAIN` | `9` |  |

### SND_COMPRESS (1)

| Name | Value | Comment |
|------|-------|---------|
| `SND_COMPRESS_TFLG_NEW_STREAM` | `(1<<0)` | mark for the new stream data |

## Structs (11)


### `struct snd_compressed_buffer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fragment_size` | `-` |
| `__u32` | `fragments` | `-` |

### `struct snd_compr_params`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `no_wake_mode` | `-` |

### `struct snd_compr_tstamp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `byte_offset` | `-` |
| `__u32` | `copied_total` | `-` |
| `__u32` | `pcm_frames` | `-` |
| `__u32` | `pcm_io_frames` | `-` |
| `__u32` | `sampling_rate` | `-` |

### `struct snd_compr_tstamp64`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `byte_offset` | `-` |
| `__u64` | `copied_total` | `-` |
| `__u64` | `pcm_frames` | `-` |
| `__u64` | `pcm_io_frames` | `-` |
| `__u32` | `sampling_rate` | `-` |

### `struct snd_compr_avail`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `avail` | `-` |

### `struct snd_compr_avail64`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `avail` | `-` |

### `struct snd_compr_caps`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_codecs` | `-` |
| `__u32` | `direction` | `-` |
| `__u32` | `min_fragment_size` | `-` |
| `__u32` | `max_fragment_size` | `-` |
| `__u32` | `min_fragments` | `-` |
| `__u32` | `max_fragments` | `-` |
| `__u32` | `codecs` | `MAX_NUM_CODECS` |
| `__u32` | `reserved` | `11` |

### `struct snd_compr_codec_caps`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `codec` | `-` |
| `__u32` | `num_descriptors` | `-` |

### `struct snd_compr_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `key` | `-` |
| `__u32` | `value` | `8` |

### `struct snd_compr_task`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `seqno` | `-` |
| `__u64` | `origin_seqno` | `-` |
| `int` | `input_fd` | `-` |
| `int` | `output_fd` | `-` |
| `__u64` | `input_size` | `-` |
| `__u32` | `flags` | `-` |
| `__u8` | `reserved` | `16` |

### `struct snd_compr_task_status`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `seqno` | `-` |
| `__u64` | `input_size` | `-` |
| `__u64` | `output_size` | `-` |
| `__u32` | `output_flags` | `-` |
| `__u8` | `state` | `-` |
| `__u8` | `reserved` | `15` |