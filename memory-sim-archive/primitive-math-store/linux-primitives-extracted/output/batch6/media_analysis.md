# media.h

**Source:** `media.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (98 total)


### MEDIA_API (1)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_API_VERSION` | `((0U << 16) \| (1U << 8) \| 0U)` |  |

### MEDIA_ENT (60)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_ENT_F_BASE` | `0x00000000` |  |
| `MEDIA_ENT_F_OLD_BASE` | `0x00010000` |  |
| `MEDIA_ENT_F_OLD_SUBDEV_BASE` | `0x00020000` |  |
| `MEDIA_ENT_F_UNKNOWN` | `MEDIA_ENT_F_BASE` |  |
| `MEDIA_ENT_F_V4L2_SUBDEV_UNKNOWN` | `MEDIA_ENT_F_OLD_SUBDEV_BASE` |  |
| `MEDIA_ENT_F_DTV_DEMOD` | `(MEDIA_ENT_F_BASE + 0x00001)` |  |
| `MEDIA_ENT_F_TS_DEMUX` | `(MEDIA_ENT_F_BASE + 0x00002)` |  |
| `MEDIA_ENT_F_DTV_CA` | `(MEDIA_ENT_F_BASE + 0x00003)` |  |
| `MEDIA_ENT_F_DTV_NET_DECAP` | `(MEDIA_ENT_F_BASE + 0x00004)` |  |
| `MEDIA_ENT_F_IO_V4L` | `(MEDIA_ENT_F_OLD_BASE + 1)` |  |
| `MEDIA_ENT_F_IO_DTV` | `(MEDIA_ENT_F_BASE + 0x01001)` |  |
| `MEDIA_ENT_F_IO_VBI` | `(MEDIA_ENT_F_BASE + 0x01002)` |  |
| `MEDIA_ENT_F_IO_SWRADIO` | `(MEDIA_ENT_F_BASE + 0x01003)` |  |
| `MEDIA_ENT_F_CAM_SENSOR` | `(MEDIA_ENT_F_OLD_SUBDEV_BASE + 1)` |  |
| `MEDIA_ENT_F_FLASH` | `(MEDIA_ENT_F_OLD_SUBDEV_BASE + 2)` |  |
| `MEDIA_ENT_F_LENS` | `(MEDIA_ENT_F_OLD_SUBDEV_BASE + 3)` |  |
| `MEDIA_ENT_F_TUNER` | `(MEDIA_ENT_F_OLD_SUBDEV_BASE + 5)` |  |
| `MEDIA_ENT_F_IF_VID_DECODER` | `(MEDIA_ENT_F_BASE + 0x02001)` |  |
| `MEDIA_ENT_F_IF_AUD_DECODER` | `(MEDIA_ENT_F_BASE + 0x02002)` |  |
| `MEDIA_ENT_F_AUDIO_CAPTURE` | `(MEDIA_ENT_F_BASE + 0x03001)` |  |
| `MEDIA_ENT_F_AUDIO_PLAYBACK` | `(MEDIA_ENT_F_BASE + 0x03002)` |  |
| `MEDIA_ENT_F_AUDIO_MIXER` | `(MEDIA_ENT_F_BASE + 0x03003)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_COMPOSER` | `(MEDIA_ENT_F_BASE + 0x4001)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER` | `(MEDIA_ENT_F_BASE + 0x4002)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV` | `(MEDIA_ENT_F_BASE + 0x4003)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_LUT` | `(MEDIA_ENT_F_BASE + 0x4004)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_SCALER` | `(MEDIA_ENT_F_BASE + 0x4005)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_STATISTICS` | `(MEDIA_ENT_F_BASE + 0x4006)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_ENCODER` | `(MEDIA_ENT_F_BASE + 0x4007)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_DECODER` | `(MEDIA_ENT_F_BASE + 0x4008)` |  |
| `MEDIA_ENT_F_PROC_VIDEO_ISP` | `(MEDIA_ENT_F_BASE + 0x4009)` |  |
| `MEDIA_ENT_F_VID_MUX` | `(MEDIA_ENT_F_BASE + 0x5001)` |  |
| `MEDIA_ENT_F_VID_IF_BRIDGE` | `(MEDIA_ENT_F_BASE + 0x5002)` |  |
| `MEDIA_ENT_F_ATV_DECODER` | `(MEDIA_ENT_F_OLD_SUBDEV_BASE + 4)` |  |
| `MEDIA_ENT_F_DV_DECODER` | `(MEDIA_ENT_F_BASE + 0x6001)` |  |
| `MEDIA_ENT_F_DV_ENCODER` | `(MEDIA_ENT_F_BASE + 0x6002)` |  |
| `MEDIA_ENT_FL_DEFAULT` | `(1U << 0)` |  |
| `MEDIA_ENT_FL_CONNECTOR` | `(1U << 1)` |  |
| `MEDIA_ENT_ID_FLAG_NEXT` | `(1U << 31)` |  |
| `MEDIA_ENT_F_CONN_RF` | `(MEDIA_ENT_F_BASE + 0x30001)` |  |
| `MEDIA_ENT_F_CONN_SVIDEO` | `(MEDIA_ENT_F_BASE + 0x30002)` |  |
| `MEDIA_ENT_F_CONN_COMPOSITE` | `(MEDIA_ENT_F_BASE + 0x30003)` |  |
| `MEDIA_ENT_TYPE_SHIFT` | `16` |  |
| `MEDIA_ENT_TYPE_MASK` | `0x00ff0000` |  |
| `MEDIA_ENT_SUBTYPE_MASK` | `0x0000ffff` |  |
| `MEDIA_ENT_T_DEVNODE_UNKNOWN` | `(MEDIA_ENT_F_OLD_BASE \| ` |  |
| `MEDIA_ENT_T_DEVNODE` | `MEDIA_ENT_F_OLD_BASE` |  |
| `MEDIA_ENT_T_DEVNODE_V4L` | `MEDIA_ENT_F_IO_V4L` |  |
| `MEDIA_ENT_T_DEVNODE_FB` | `(MEDIA_ENT_F_OLD_BASE + 2)` |  |
| `MEDIA_ENT_T_DEVNODE_ALSA` | `(MEDIA_ENT_F_OLD_BASE + 3)` |  |

*...and 10 more*

### MEDIA_INTF (22)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_INTF_T_DVB_BASE` | `0x00000100` |  |
| `MEDIA_INTF_T_V4L_BASE` | `0x00000200` |  |
| `MEDIA_INTF_T_DVB_FE` | `(MEDIA_INTF_T_DVB_BASE)` |  |
| `MEDIA_INTF_T_DVB_DEMUX` | `(MEDIA_INTF_T_DVB_BASE + 1)` |  |
| `MEDIA_INTF_T_DVB_DVR` | `(MEDIA_INTF_T_DVB_BASE + 2)` |  |
| `MEDIA_INTF_T_DVB_CA` | `(MEDIA_INTF_T_DVB_BASE + 3)` |  |
| `MEDIA_INTF_T_DVB_NET` | `(MEDIA_INTF_T_DVB_BASE + 4)` |  |
| `MEDIA_INTF_T_V4L_VIDEO` | `(MEDIA_INTF_T_V4L_BASE)` |  |
| `MEDIA_INTF_T_V4L_VBI` | `(MEDIA_INTF_T_V4L_BASE + 1)` |  |
| `MEDIA_INTF_T_V4L_RADIO` | `(MEDIA_INTF_T_V4L_BASE + 2)` |  |
| `MEDIA_INTF_T_V4L_SUBDEV` | `(MEDIA_INTF_T_V4L_BASE + 3)` |  |
| `MEDIA_INTF_T_V4L_SWRADIO` | `(MEDIA_INTF_T_V4L_BASE + 4)` |  |
| `MEDIA_INTF_T_V4L_TOUCH` | `(MEDIA_INTF_T_V4L_BASE + 5)` |  |
| `MEDIA_INTF_T_ALSA_BASE` | `0x00000300` |  |
| `MEDIA_INTF_T_ALSA_PCM_CAPTURE` | `(MEDIA_INTF_T_ALSA_BASE)` |  |
| `MEDIA_INTF_T_ALSA_PCM_PLAYBACK` | `(MEDIA_INTF_T_ALSA_BASE + 1)` |  |
| `MEDIA_INTF_T_ALSA_CONTROL` | `(MEDIA_INTF_T_ALSA_BASE + 2)` |  |
| `MEDIA_INTF_T_ALSA_COMPRESS` | `(MEDIA_INTF_T_ALSA_BASE + 3)` |  |
| `MEDIA_INTF_T_ALSA_RAWMIDI` | `(MEDIA_INTF_T_ALSA_BASE + 4)` |  |
| `MEDIA_INTF_T_ALSA_HWDEP` | `(MEDIA_INTF_T_ALSA_BASE + 5)` |  |
| `MEDIA_INTF_T_ALSA_SEQUENCER` | `(MEDIA_INTF_T_ALSA_BASE + 6)` |  |
| `MEDIA_INTF_T_ALSA_TIMER` | `(MEDIA_INTF_T_ALSA_BASE + 7)` |  |

### MEDIA_IOC (6)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_IOC_DEVICE_INFO` | `_IOWR('\|', 0x00, struct media_device_info)` |  |
| `MEDIA_IOC_ENUM_ENTITIES` | `_IOWR('\|', 0x01, struct media_entity_desc)` |  |
| `MEDIA_IOC_ENUM_LINKS` | `_IOWR('\|', 0x02, struct media_links_enum)` |  |
| `MEDIA_IOC_SETUP_LINK` | `_IOWR('\|', 0x03, struct media_link_desc)` |  |
| `MEDIA_IOC_G_TOPOLOGY` | `_IOWR('\|', 0x04, struct media_v2_topology)` |  |
| `MEDIA_IOC_REQUEST_ALLOC` | `_IOR ('\|', 0x05, int)` |  |

### MEDIA_LNK (4)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_LNK_FL_ENABLED` | `(1U << 0)` |  |
| `MEDIA_LNK_FL_IMMUTABLE` | `(1U << 1)` |  |
| `MEDIA_LNK_FL_DYNAMIC` | `(1U << 2)` |  |
| `MEDIA_LNK_FL_LINK_TYPE` | `(0xf << 28)` |  |

### MEDIA_PAD (3)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_PAD_FL_SINK` | `(1U << 0)` |  |
| `MEDIA_PAD_FL_SOURCE` | `(1U << 1)` |  |
| `MEDIA_PAD_FL_MUST_CONNECT` | `(1U << 2)` |  |

### MEDIA_REQUEST (2)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_REQUEST_IOC_QUEUE` | `_IO('\|',  0x80)` |  |
| `MEDIA_REQUEST_IOC_REINIT` | `_IO('\|',  0x81)` |  |

## Structs (15)


### `struct media_device_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `driver` | `16` |
| `char` | `model` | `32` |
| `char` | `serial` | `40` |
| `char` | `bus_info` | `32` |
| `__u32` | `media_version` | `-` |
| `__u32` | `hw_revision` | `-` |
| `__u32` | `driver_version` | `-` |
| `__u32` | `reserved` | `31` |

### `struct media_entity_desc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `char` | `name` | `32` |
| `__u32` | `type` | `-` |
| `__u32` | `revision` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `group_id` | `-` |
| `__u16` | `pads` | `-` |
| `__u16` | `links` | `-` |
| `__u32` | `reserved` | `4` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u32` | `card` | `-` |
| `__u32` | `device` | `-` |
| `__u32` | `subdevice` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `int` | `dvb` | `-` |
| `__u8` | `raw` | `184` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `card` | `-` |
| `__u32` | `device` | `-` |
| `__u32` | `subdevice` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |

### `struct media_pad_desc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `entity` | `-` |
| `__u16` | `index` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `2` |

### `struct media_link_desc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `2` |

### `struct media_links_enum`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `entity` | `-` |
| `__u32` | `reserved` | `4` |

### `struct media_v2_entity`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `char` | `name` | `64` |
| `__u32` | `function` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `5` |

### `struct media_v2_intf_devnode`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |

### `struct media_v2_interface`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `intf_type` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `9` |
| `__u32` | `raw` | `16` |

### `struct media_v2_pad`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `entity_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `index` | `-` |
| `__u32` | `reserved` | `4` |

### `struct media_v2_link`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `source_id` | `-` |
| `__u32` | `sink_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `6` |

### `struct media_v2_topology`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `topology_version` | `-` |
| `__u32` | `num_entities` | `-` |
| `__u32` | `reserved1` | `-` |
| `__u64` | `ptr_entities` | `-` |
| `__u32` | `num_interfaces` | `-` |
| `__u32` | `reserved2` | `-` |
| `__u64` | `ptr_interfaces` | `-` |
| `__u32` | `num_pads` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `ptr_pads` | `-` |
| `__u32` | `num_links` | `-` |
| `__u32` | `reserved4` | `-` |
| `__u64` | `ptr_links` | `-` |