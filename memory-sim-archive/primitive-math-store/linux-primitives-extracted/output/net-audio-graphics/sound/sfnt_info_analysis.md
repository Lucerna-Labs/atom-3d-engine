# sfnt_info.h

**Source:** `sfnt_info.h`


## Includes

- `sound/asound.h`

## Defines (39 total)


### SNDRV_EMUX (8)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_EMUX_HWDEP_NAME` | `"Emux WaveTable"` |  |
| `SNDRV_EMUX_VERSION` | `((1 << 16) \| (0 << 8) \| 0)` | 1.0.0 |
| `SNDRV_EMUX_IOCTL_VERSION` | `_IOR('H', 0x80, unsigned int)` |  |
| `SNDRV_EMUX_IOCTL_LOAD_PATCH` | `_IOWR('H', 0x81, struct soundfont_patch_info)` |  |
| `SNDRV_EMUX_IOCTL_RESET_SAMPLES` | `_IO('H', 0x82)` |  |
| `SNDRV_EMUX_IOCTL_REMOVE_LAST_SAMPLES` | `_IO('H', 0x83)` |  |
| `SNDRV_EMUX_IOCTL_MEM_AVAIL` | `_IOW('H', 0x84, int)` |  |
| `SNDRV_EMUX_IOCTL_MISC_MODE` | `_IOWR('H', 0x84, struct snd_emux_misc_mode)` |  |

### SNDRV_OSS (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_OSS_SOUNDFONT_PATCH` | `SNDRV_OSS_PATCHKEY(0x07)` |  |

### SNDRV_SFNT (30)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_SFNT_LOAD_INFO` | `0` | awe_voice_rec |
| `SNDRV_SFNT_LOAD_DATA` | `1` | awe_sample_info |
| `SNDRV_SFNT_OPEN_PATCH` | `2` | awe_open_parm |
| `SNDRV_SFNT_CLOSE_PATCH` | `3` | none |
| `SNDRV_SFNT_REPLACE_DATA` | `5` | awe_sample_info (optarg=#channels) |
| `SNDRV_SFNT_MAP_PRESET` | `6` | awe_voice_map |
| `SNDRV_SFNT_PROBE_DATA` | `8` | optarg=sample |
| `SNDRV_SFNT_REMOVE_INFO` | `9` | optarg=(bank<<8)|instr |
| `SNDRV_SFNT_PATCH_NAME_LEN` | `32` |  |
| `SNDRV_SFNT_PAT_TYPE_MISC` | `0` |  |
| `SNDRV_SFNT_PAT_TYPE_GUS` | `6` |  |
| `SNDRV_SFNT_PAT_TYPE_MAP` | `7` |  |
| `SNDRV_SFNT_PAT_LOCKED` | `0x100` | lock the samples |
| `SNDRV_SFNT_PAT_SHARED` | `0x200` | sample is shared |
| `SNDRV_SFNT_MODE_ROMSOUND` | `0x8000` |  |
| `SNDRV_SFNT_MODE_STEREO` | `1` |  |
| `SNDRV_SFNT_MODE_LOOPING` | `2` |  |
| `SNDRV_SFNT_MODE_NORELEASE` | `4` | obsolete |
| `SNDRV_SFNT_MODE_INIT_PARM` | `8` |  |
| `SNDRV_SFNT_WR_APPEND` | `0` | append anyway |
| `SNDRV_SFNT_WR_EXCLUSIVE` | `1` | skip if already exists |
| `SNDRV_SFNT_WR_REPLACE` | `2` | replace if already exists |
| `SNDRV_SFNT_SAMPLE_8BITS` | `1` | wave data is 8bits |
| `SNDRV_SFNT_SAMPLE_UNSIGNED` | `2` | wave data is unsigned |
| `SNDRV_SFNT_SAMPLE_NO_BLANK` | `4` | no blank loop is attached |
| `SNDRV_SFNT_SAMPLE_SINGLESHOT` | `8` | single-shot w/o loop |
| `SNDRV_SFNT_SAMPLE_BIDIR_LOOP` | `16` | bidirectional looping |
| `SNDRV_SFNT_SAMPLE_STEREO_LEFT` | `32` | stereo left sound |
| `SNDRV_SFNT_SAMPLE_STEREO_RIGHT` | `64` | stereo right sound |
| `SNDRV_SFNT_SAMPLE_REVERSE_LOOP` | `128` | reverse looping |

## Structs (8)


### `struct soundfont_patch_info`

| Type | Field | Array |
|------|-------|-------|
| `short` | `device_no` | `-` |
| `short` | `optarg` | `-` |
| `int` | `len` | `-` |
| `short` | `type` | `-` |
| `short` | `reserved` | `-` |

### `struct soundfont_open_parm`

| Type | Field | Array |
|------|-------|-------|
| `short` | `reserved` | `-` |
| `char` | `name` | `SNDRV_SFNT_PATCH_NAME_LEN` |

### `struct soundfont_voice_parm`

| Type | Field | Array |
|------|-------|-------|

### `struct soundfont_voice_info`

| Type | Field | Array |
|------|-------|-------|
| `short` | `rate_offset` | `-` |
| `short` | `root` | `-` |
| `short` | `tune` | `-` |
| `short` | `exclusiveClass` | `-` |
| `short` | `scaleTuning` | `-` |

### `struct soundfont_voice_rec_hdr`

| Type | Field | Array |
|------|-------|-------|
| `char` | `nvoices` | `-` |
| `char` | `write_mode` | `-` |

### `struct soundfont_sample_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `size` | `-` |
| `short` | `dummy` | `-` |

### `struct soundfont_voice_map`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_emux_misc_mode`

| Type | Field | Array |
|------|-------|-------|
| `int` | `port` | `-` |
| `int` | `mode` | `-` |
| `int` | `value` | `-` |
| `int` | `value2` | `-` |