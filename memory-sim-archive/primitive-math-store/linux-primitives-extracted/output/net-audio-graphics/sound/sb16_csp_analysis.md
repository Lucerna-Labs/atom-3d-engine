# sb16_csp.h

**Source:** `sb16_csp.h`


## Defines (30 total)


### SNDRV_SB (30)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_SB_CSP_MODE_NONE` | `0x00` |  |
| `SNDRV_SB_CSP_MODE_DSP_READ` | `0x01` | Record from DSP |
| `SNDRV_SB_CSP_MODE_DSP_WRITE` | `0x02` | Play to DSP |
| `SNDRV_SB_CSP_MODE_QSOUND` | `0x04` | QSound |
| `SNDRV_SB_CSP_LOAD_FROMUSER` | `0x01` |  |
| `SNDRV_SB_CSP_LOAD_INITBLOCK` | `0x02` |  |
| `SNDRV_SB_CSP_SAMPLE_8BIT` | `0x01` |  |
| `SNDRV_SB_CSP_SAMPLE_16BIT` | `0x02` |  |
| `SNDRV_SB_CSP_MONO` | `0x01` |  |
| `SNDRV_SB_CSP_STEREO` | `0x02` |  |
| `SNDRV_SB_CSP_RATE_8000` | `0x01` |  |
| `SNDRV_SB_CSP_RATE_11025` | `0x02` |  |
| `SNDRV_SB_CSP_RATE_22050` | `0x04` |  |
| `SNDRV_SB_CSP_RATE_44100` | `0x08` |  |
| `SNDRV_SB_CSP_RATE_ALL` | `0x0f` |  |
| `SNDRV_SB_CSP_ST_IDLE` | `0x00` |  |
| `SNDRV_SB_CSP_ST_LOADED` | `0x01` |  |
| `SNDRV_SB_CSP_ST_RUNNING` | `0x02` |  |
| `SNDRV_SB_CSP_ST_PAUSED` | `0x04` |  |
| `SNDRV_SB_CSP_ST_AUTO` | `0x08` |  |
| `SNDRV_SB_CSP_ST_QSOUND` | `0x10` |  |
| `SNDRV_SB_CSP_QSOUND_MAX_RIGHT` | `0x20` |  |
| `SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE` | `0x3000` |  |
| `SNDRV_SB_CSP_IOCTL_INFO` | `_IOR('H', 0x10, struct snd_sb_csp_info)` |  |
| `SNDRV_SB_CSP_IOCTL_LOAD_CODE` | `` |  |
| `SNDRV_SB_CSP_IOCTL_UNLOAD_CODE` | `_IO('H', 0x12)` |  |
| `SNDRV_SB_CSP_IOCTL_START` | `_IOW('H', 0x13, struct snd_sb_csp_start)` |  |
| `SNDRV_SB_CSP_IOCTL_STOP` | `_IO('H', 0x14)` |  |
| `SNDRV_SB_CSP_IOCTL_PAUSE` | `_IO('H', 0x15)` |  |
| `SNDRV_SB_CSP_IOCTL_RESTART` | `_IO('H', 0x16)` |  |

## Structs (4)


### `struct snd_sb_csp_mc_header`

| Type | Field | Array |
|------|-------|-------|
| `char` | `codec_name` | `16` |

### `struct snd_sb_csp_microcode`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_sb_csp_start`

| Type | Field | Array |
|------|-------|-------|
| `int` | `sample_width` | `-` |
| `int` | `channels` | `-` |

### `struct snd_sb_csp_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `codec_name` | `16` |