# video.h

**Source:** `video.h`


## Includes

- `linux/types.h`
- `time.h`

## Defines (45 total)


### UNCATEGORIZED (7)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_STOP` | `_IO('o', 21)` |  |
| `VIDEO_PLAY` | `_IO('o', 22)` |  |
| `VIDEO_FREEZE` | `_IO('o', 23)` |  |
| `VIDEO_CONTINUE` | `_IO('o', 24)` |  |
| `VIDEO_STILLPICTURE` | `_IOW('o', 30, struct video_still_picture)` |  |
| `VIDEO_SLOWMOTION` | `_IO('o', 32)` |  |
| `VIDEO_COMMAND` | `_IOWR('o', 59, struct video_command)` |  |

### VIDEO_CAP (7)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_CAP_MPEG1` | `1` |  |
| `VIDEO_CAP_MPEG2` | `2` |  |
| `VIDEO_CAP_SYS` | `4` |  |
| `VIDEO_CAP_PROG` | `8` |  |
| `VIDEO_CAP_SPU` | `16` |  |
| `VIDEO_CAP_NAVI` | `32` |  |
| `VIDEO_CAP_CSS` | `64` |  |

### VIDEO_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_CLEAR_BUFFER` | `_IO('o',  34)` |  |

### VIDEO_CMD (7)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_CMD_PLAY` | `(0)` |  |
| `VIDEO_CMD_STOP` | `(1)` |  |
| `VIDEO_CMD_FREEZE` | `(2)` |  |
| `VIDEO_CMD_CONTINUE` | `(3)` |  |
| `VIDEO_CMD_FREEZE_TO_BLACK` | `(1 << 0)` |  |
| `VIDEO_CMD_STOP_TO_BLACK` | `(1 << 0)` |  |
| `VIDEO_CMD_STOP_IMMEDIATELY` | `(1 << 1)` |  |

### VIDEO_EVENT (4)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_EVENT_SIZE_CHANGED` | `1` |  |
| `VIDEO_EVENT_FRAME_RATE_CHANGED` | `2` |  |
| `VIDEO_EVENT_DECODER_STOPPED` | `3` |  |
| `VIDEO_EVENT_VSYNC` | `4` |  |

### VIDEO_FAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_FAST_FORWARD` | `_IO('o', 31)` |  |

### VIDEO_GET (6)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_GET_STATUS` | `_IOR('o', 27, struct video_status)` |  |
| `VIDEO_GET_EVENT` | `_IOR('o', 28, struct video_event)` |  |
| `VIDEO_GET_CAPABILITIES` | `_IOR('o', 33, unsigned int)` |  |
| `VIDEO_GET_SIZE` | `_IOR('o', 55, video_size_t)` |  |
| `VIDEO_GET_PTS` | `_IOR('o', 57, __u64)` |  |
| `VIDEO_GET_FRAME_COUNT` | `_IOR('o', 58, __u64)` |  |

### VIDEO_PLAY (2)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_PLAY_FMT_NONE` | `(0)` |  |
| `VIDEO_PLAY_FMT_GOP` | `(1)` |  |

### VIDEO_SELECT (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_SELECT_SOURCE` | `_IO('o', 25)` |  |

### VIDEO_SET (4)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_SET_BLANK` | `_IO('o', 26)` |  |
| `VIDEO_SET_DISPLAY_FORMAT` | `_IO('o', 29)` |  |
| `VIDEO_SET_STREAMTYPE` | `_IO('o', 36)` |  |
| `VIDEO_SET_FORMAT` | `_IO('o', 37)` |  |

### VIDEO_TRY (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_TRY_COMMAND` | `_IOWR('o', 60, struct video_command)` |  |

### VIDEO_VSYNC (4)

| Name | Value | Comment |
|------|-------|---------|
| `VIDEO_VSYNC_FIELD_UNKNOWN` | `(0)` |  |
| `VIDEO_VSYNC_FIELD_ODD` | `(1)` |  |
| `VIDEO_VSYNC_FIELD_EVEN` | `(2)` |  |
| `VIDEO_VSYNC_FIELD_PROGRESSIVE` | `(3)` |  |

## Structs (8)


### `struct anonymous_0`

| Type | Field | Array |
|------|-------|-------|
| `int` | `w` | `-` |
| `int` | `h` | `-` |
| `video_format_t` | `aspect_ratio` | `-` |

### `struct video_command`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `pts` | `-` |
| `__s32` | `speed` | `-` |
| `__u32` | `format` | `-` |
| `__u32` | `data` | `16` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `pts` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `speed` | `-` |
| `__u32` | `format` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `data` | `16` |

### `struct video_event`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `type` | `-` |
| `long` | `timestamp` | `-` |
| `video_size_t` | `size` | `-` |

### `struct video_status`

| Type | Field | Array |
|------|-------|-------|
| `int` | `video_blank` | `-` |
| `video_play_state_t` | `play_state` | `-` |
| `video_stream_source_t` | `stream_source` | `-` |
| `video_format_t` | `video_format` | `-` |
| `video_displayformat_t` | `display_format` | `-` |

### `struct video_still_picture`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `size` | `-` |

## Enums


### `anonymous_enum_0`

- `VIDEO_FORMAT_4_3` 
- `Select` 
- `4` 
- `3` 
- `format` 
- `VIDEO_FORMAT_16_9` 
- `Select` 
- `16` 
- `9` 
- `format` 
- `VIDEO_FORMAT_221_1` 
- `2` 
- `21` 
- `1` 

### `anonymous_enum_1`

- `VIDEO_PAN_SCAN` 
- `use` 
- `pan` 
- `and` 
- `scan` 
- `format` 
- `VIDEO_LETTER_BOX` 
- `use` 
- `letterbox` 
- `format` 
- `VIDEO_CENTER_CUT_OUT` 
- `use` 
- `center` 
- `cut` 
- `out` 
- `format` 

### `anonymous_enum_2`

- `VIDEO_SOURCE_DEMUX` 
- `Select` 
- `the` 
- `demux` 
- `as` 
- `the` 
- `main` 
- `source` 
- `VIDEO_SOURCE_MEMORY` 
- `If` 
- `this` 
- `source` 
- `is` 
- `selected` 
- `the` 
- `stream` 
- `comes` 
- `from` 
- `the` 
- `user` 

*...and 5 more*

### `anonymous_enum_3`

- `VIDEO_STOPPED` 
- `Video` 
- `is` 
- `stopped` 
- `VIDEO_PLAYING` 
- `Video` 
- `is` 
- `currently` 
- `playing` 
- `VIDEO_FREEZED` 
- `Video` 
- `is` 
- `freezed` 