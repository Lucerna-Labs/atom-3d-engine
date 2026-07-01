# asound_fm.h

**Source:** `asound_fm.h`


## Defines (19 total)


### FM_KEY (3)

| Name | Value | Comment |
|------|-------|---------|
| `FM_KEY_SBI` | `"SBI\032"` |  |
| `FM_KEY_2OP` | `"2OP\032"` |  |
| `FM_KEY_4OP` | `"4OP\032"` |  |

### SNDRV_DM (16)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_DM_FM_MODE_OPL2` | `0x00` |  |
| `SNDRV_DM_FM_MODE_OPL3` | `0x01` |  |
| `SNDRV_DM_FM_IOCTL_INFO` | `_IOR('H', 0x20, struct snd_dm_fm_info)` |  |
| `SNDRV_DM_FM_IOCTL_RESET` | `_IO ('H', 0x21)` |  |
| `SNDRV_DM_FM_IOCTL_PLAY_NOTE` | `_IOW('H', 0x22, struct snd_dm_fm_note)` |  |
| `SNDRV_DM_FM_IOCTL_SET_VOICE` | `_IOW('H', 0x23, struct snd_dm_fm_voice)` |  |
| `SNDRV_DM_FM_IOCTL_SET_PARAMS` | `_IOW('H', 0x24, struct snd_dm_fm_params)` |  |
| `SNDRV_DM_FM_IOCTL_SET_MODE` | `_IOW('H', 0x25, int)` |  |
| `SNDRV_DM_FM_IOCTL_SET_CONNECTION` | `_IOW('H', 0x26, int)` |  |
| `SNDRV_DM_FM_IOCTL_CLEAR_PATCHES` | `_IO ('H', 0x40)` |  |
| `SNDRV_DM_FM_OSS_IOCTL_RESET` | `0x20` |  |
| `SNDRV_DM_FM_OSS_IOCTL_PLAY_NOTE` | `0x21` |  |
| `SNDRV_DM_FM_OSS_IOCTL_SET_VOICE` | `0x22` |  |
| `SNDRV_DM_FM_OSS_IOCTL_SET_PARAMS` | `0x23` |  |
| `SNDRV_DM_FM_OSS_IOCTL_SET_MODE` | `0x24` |  |
| `SNDRV_DM_FM_OSS_IOCTL_SET_OPL` | `0x25` |  |

## Structs (5)


### `struct snd_dm_fm_info`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_dm_fm_voice`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_dm_fm_note`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_dm_fm_params`

| Type | Field | Array |
|------|-------|-------|

### `struct sbi_patch`

| Type | Field | Array |
|------|-------|-------|
| `char` | `key` | `4` |
| `char` | `name` | `25` |
| `char` | `extension` | `7` |