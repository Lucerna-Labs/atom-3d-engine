# audio.h

**Source:** `audio.h`


## Includes

- `linux/types.h`

## Defines (25 total)


### AUDIO_BILINGUAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_BILINGUAL_CHANNEL_SELECT` | `_IO('o', 20)` |  |

### AUDIO_CAP (9)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_CAP_DTS` | `1` |  |
| `AUDIO_CAP_LPCM` | `2` |  |
| `AUDIO_CAP_MP1` | `4` |  |
| `AUDIO_CAP_MP2` | `8` |  |
| `AUDIO_CAP_MP3` | `16` |  |
| `AUDIO_CAP_AAC` | `32` |  |
| `AUDIO_CAP_OGG` | `64` |  |
| `AUDIO_CAP_SDDS` | `128` |  |
| `AUDIO_CAP_AC3` | `256` |  |

### AUDIO_CHANNEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_CHANNEL_SELECT` | `_IO('o', 9)` |  |

### AUDIO_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_CLEAR_BUFFER` | `_IO('o',  12)` |  |

### AUDIO_GET (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_GET_STATUS` | `_IOR('o', 10, audio_status_t)` |  |
| `AUDIO_GET_CAPABILITIES` | `_IOR('o', 11, unsigned int)` |  |

### AUDIO_SELECT (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_SELECT_SOURCE` | `_IO('o', 5)` |  |

### AUDIO_SET (6)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_SET_MUTE` | `_IO('o', 6)` |  |
| `AUDIO_SET_AV_SYNC` | `_IO('o', 7)` |  |
| `AUDIO_SET_BYPASS_MODE` | `_IO('o', 8)` |  |
| `AUDIO_SET_ID` | `_IO('o', 13)` |  |
| `AUDIO_SET_MIXER` | `_IOW('o', 14, audio_mixer_t)` |  |
| `AUDIO_SET_STREAMTYPE` | `_IO('o', 15)` |  |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIO_STOP` | `_IO('o', 1)` |  |
| `AUDIO_PLAY` | `_IO('o', 2)` |  |
| `AUDIO_PAUSE` | `_IO('o', 3)` |  |
| `AUDIO_CONTINUE` | `_IO('o', 4)` |  |

## Structs (2)


### `struct audio_mixer`

| Type | Field | Array |
|------|-------|-------|

### `struct audio_status`

| Type | Field | Array |
|------|-------|-------|
| `int` | `AV_sync_state` | `-` |
| `int` | `mute_state` | `-` |
| `audio_play_state_t` | `play_state` | `-` |
| `audio_stream_source_t` | `stream_source` | `-` |
| `audio_channel_select_t` | `channel_select` | `-` |
| `int` | `bypass_mode` | `-` |
| `audio_mixer_t` | `mixer_state` | `-` |

## Typedefs

- `audio_mixer`
- `audio_status`

## Enums


### `anonymous_enum_0`

- `AUDIO_SOURCE_DEMUX` 
- `Select` 
- `the` 
- `demux` 
- `as` 
- `the` 
- `main` 
- `source` 
- `AUDIO_SOURCE_MEMORY` 
- `Select` 
- `internal` 
- `memory` 
- `as` 
- `the` 
- `main` 
- `source` 

### `anonymous_enum_1`

- `AUDIO_STOPPED` 
- `Device` 
- `is` 
- `stopped` 
- `AUDIO_PLAYING` 
- `Device` 
- `is` 
- `currently` 
- `playing` 
- `AUDIO_PAUSED` 
- `Device` 
- `is` 
- `paused` 

### `anonymous_enum_2`

- `AUDIO_STEREO` 
- `AUDIO_MONO_LEFT` 
- `AUDIO_MONO_RIGHT` 
- `AUDIO_MONO` 
- `AUDIO_STEREO_SWAPPED` 