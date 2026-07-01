# hdspm.h

**Source:** `hdspm.h`


## Includes

- `linux/types.h`

## Defines (9 total)


### HDSPM_ADDON (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDSPM_ADDON_TCO` | `1` |  |

### HDSPM_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDSPM_MAX_CHANNELS` | `64` |  |

### HDSPM_MIXER (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDSPM_MIXER_CHANNELS` | `HDSPM_MAX_CHANNELS` |  |

### SNDRV_HDSPM (6)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_HDSPM_IOCTL_GET_PEAK_RMS` | `` |  |
| `SNDRV_HDSPM_IOCTL_GET_CONFIG` | `` |  |
| `SNDRV_HDSPM_IOCTL_GET_LTC` | `_IOR('H', 0x46, struct hdspm_ltc)` |  |
| `SNDRV_HDSPM_IOCTL_GET_STATUS` | `` |  |
| `SNDRV_HDSPM_IOCTL_GET_VERSION` | `_IOR('H', 0x48, struct hdspm_version)` |  |
| `SNDRV_HDSPM_IOCTL_GET_MIXER` | `_IOR('H', 0x44, struct hdspm_mixer_ioctl)` |  |

## Structs (9)


### `struct hdspm_peak_rms`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `input_peaks` | `64` |
| `__u32` | `playback_peaks` | `64` |
| `__u32` | `output_peaks` | `64` |
| `__u64` | `input_rms` | `64` |
| `__u64` | `playback_rms` | `64` |
| `__u64` | `output_rms` | `64` |
| `__u8` | `speed` | `-` |
| `int` | `status2` | `-` |

### `struct hdspm_config`

| Type | Field | Array |
|------|-------|-------|

### `struct hdspm_ltc`

| Type | Field | Array |
|------|-------|-------|

### `struct hdspm_status`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `card_type` | `-` |
| `__u64` | `card_clock` | `-` |
| `__u32` | `master_period` | `-` |
| `__u8` | `sync_wc` | `-` |
| `__u8` | `sync_madi` | `-` |
| `__u8` | `sync_tco` | `-` |
| `__u8` | `sync_in` | `-` |
| `__u8` | `madi_input` | `-` |
| `__u8` | `channel_format` | `-` |
| `__u8` | `frame_format` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sync_wc` | `-` |
| `__u8` | `sync_madi` | `-` |
| `__u8` | `sync_tco` | `-` |
| `__u8` | `sync_in` | `-` |
| `__u8` | `madi_input` | `-` |
| `__u8` | `channel_format` | `-` |
| `__u8` | `frame_format` | `-` |

### `struct hdspm_version`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `card_type` | `-` |
| `char` | `cardname` | `20` |
| `int` | `addons` | `-` |

### `struct hdspm_channelfader`

| Type | Field | Array |
|------|-------|-------|

### `struct hdspm_mixer`

| Type | Field | Array |
|------|-------|-------|

### `struct hdspm_mixer_ioctl`

| Type | Field | Array |
|------|-------|-------|