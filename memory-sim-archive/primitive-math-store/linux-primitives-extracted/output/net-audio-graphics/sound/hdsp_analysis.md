# hdsp.h

**Source:** `hdsp.h`


## Includes

- `linux/types.h`

## Defines (7 total)


### HDSP_MATRIX (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDSP_MATRIX_MIXER_SIZE` | `2048` |  |

### SNDRV_HDSP (6)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_HDSP_IOCTL_GET_PEAK_RMS` | `_IOR('H', 0x40, struct hdsp_peak_rms)` |  |
| `SNDRV_HDSP_IOCTL_GET_CONFIG_INFO` | `_IOR('H', 0x41, struct hdsp_config_info)` |  |
| `SNDRV_HDSP_IOCTL_UPLOAD_FIRMWARE` | `_IOW('H', 0x42, struct hdsp_firmware)` |  |
| `SNDRV_HDSP_IOCTL_GET_VERSION` | `_IOR('H', 0x43, struct hdsp_version)` |  |
| `SNDRV_HDSP_IOCTL_GET_MIXER` | `_IOR('H', 0x44, struct hdsp_mixer)` |  |
| `SNDRV_HDSP_IOCTL_GET_9632_AEB` | `_IOR('H', 0x45, struct hdsp_9632_aeb)` |  |

## Structs (6)


### `struct hdsp_peak_rms`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `input_peaks` | `26` |
| `__u32` | `playback_peaks` | `26` |
| `__u32` | `output_peaks` | `28` |
| `__u64` | `input_rms` | `26` |
| `__u64` | `playback_rms` | `26` |
| `__u64` | `output_rms` | `26` |

### `struct hdsp_config_info`

| Type | Field | Array |
|------|-------|-------|

### `struct hdsp_firmware`

| Type | Field | Array |
|------|-------|-------|

### `struct hdsp_version`

| Type | Field | Array |
|------|-------|-------|

### `struct hdsp_mixer`

| Type | Field | Array |
|------|-------|-------|

### `struct hdsp_9632_aeb`

| Type | Field | Array |
|------|-------|-------|
| `int` | `aebi` | `-` |
| `int` | `aebo` | `-` |