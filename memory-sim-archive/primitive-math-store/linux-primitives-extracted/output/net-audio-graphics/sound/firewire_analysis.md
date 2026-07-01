# firewire.h

**Source:** `firewire.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (33 total)


### SNDRV_FIREWIRE (32)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_FIREWIRE_EVENT_LOCK_STATUS` | `0x000010cc` |  |
| `SNDRV_FIREWIRE_EVENT_DICE_NOTIFICATION` | `0xd1ce004e` |  |
| `SNDRV_FIREWIRE_EVENT_EFW_RESPONSE` | `0x4e617475` |  |
| `SNDRV_FIREWIRE_EVENT_DIGI00X_MESSAGE` | `0x746e736c` |  |
| `SNDRV_FIREWIRE_EVENT_MOTU_NOTIFICATION` | `0x64776479` |  |
| `SNDRV_FIREWIRE_EVENT_TASCAM_CONTROL` | `0x7473636d` |  |
| `SNDRV_FIREWIRE_EVENT_MOTU_REGISTER_DSP_CHANGE` | `0x4d545244` |  |
| `SNDRV_FIREWIRE_EVENT_FF400_MESSAGE` | `0x4f6c6761` |  |
| `SNDRV_FIREWIRE_IOCTL_GET_INFO` | `_IOR('H', 0xf8, struct snd_firewire_get_info)` |  |
| `SNDRV_FIREWIRE_IOCTL_LOCK` | `_IO('H', 0xf9)` |  |
| `SNDRV_FIREWIRE_IOCTL_UNLOCK` | `_IO('H', 0xfa)` |  |
| `SNDRV_FIREWIRE_IOCTL_TASCAM_STATE` | `_IOR('H', 0xfb, struct snd_firewire_tascam_state)` |  |
| `SNDRV_FIREWIRE_IOCTL_MOTU_REGISTER_DSP_METER` | `_IOR('H', 0xfc, struct snd_firewire_motu_register_dsp_meter)` |  |
| `SNDRV_FIREWIRE_IOCTL_MOTU_COMMAND_DSP_METER` | `_IOR('H', 0xfd, struct snd_firewire_motu_command_dsp_meter)` |  |
| `SNDRV_FIREWIRE_IOCTL_MOTU_REGISTER_DSP_PARAMETER` | `_IOR('H', 0xfe, struct snd_firewire_motu_register_dsp_parame` |  |
| `SNDRV_FIREWIRE_TYPE_DICE` | `1` |  |
| `SNDRV_FIREWIRE_TYPE_FIREWORKS` | `2` |  |
| `SNDRV_FIREWIRE_TYPE_BEBOB` | `3` |  |
| `SNDRV_FIREWIRE_TYPE_OXFW` | `4` |  |
| `SNDRV_FIREWIRE_TYPE_DIGI00X` | `5` |  |
| `SNDRV_FIREWIRE_TYPE_TASCAM` | `6` |  |
| `SNDRV_FIREWIRE_TYPE_MOTU` | `7` |  |
| `SNDRV_FIREWIRE_TYPE_FIREFACE` | `8` |  |
| `SNDRV_FIREWIRE_TASCAM_STATE_COUNT` | `64` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_INPUT_COUNT` | `24` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_OUTPUT_COUNT` | `24` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_COUNT` | `` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` | `4` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` | `20` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_INPUT_COUNT` | `10` |  |
| `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT` | `(SNDRV_FIREWIRE_MOTU_REGISTER_DSP_INPUT_COUNT + 2)` |  |
| `SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT` | `400` |  |

### SND_EFW (1)

| Name | Value | Comment |
|------|-------|---------|
| `SND_EFW_TRANSACTION_USER_SEQNUM_MAX` | `((__u32)((__u16)~0) - 1)` |  |

## Structs (23)


### `struct snd_firewire_event_common`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_firewire_event_lock_status`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_firewire_event_dice_notification`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_efw_transaction`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `length` | `-` |
| `__be32` | `version` | `-` |
| `__be32` | `seqnum` | `-` |
| `__be32` | `category` | `-` |
| `__be32` | `command` | `-` |
| `__be32` | `status` | `-` |

### `struct snd_firewire_event_efw_response`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_firewire_event_digi00x_message`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `message` | `-` |

### `struct snd_firewire_event_motu_notification`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `message` | `-` |

### `struct snd_firewire_tascam_change`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `before` | `-` |
| `__be32` | `after` | `-` |

### `struct snd_firewire_event_tascam_control`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_firewire_event_motu_register_dsp_change`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |

### `struct snd_firewire_event_ff400_message`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `message` | `-` |
| `__u32` | `tstamp` | `-` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `message` | `-` |
| `__u32` | `tstamp` | `-` |

### `struct snd_firewire_get_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `device_name` | `16` |

### `struct snd_firewire_tascam_state`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `data` | `SNDRV_FIREWIRE_TASCAM_STATE_COUNT` |

### `struct snd_firewire_motu_register_dsp_meter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `data` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_COUNT` |

### `struct snd_firewire_motu_register_dsp_parameter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `gain` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `pan` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_balance` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_width` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_volume` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` |
| `__u8` | `paired_flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` |
| `__u8` | `main_paired_volume` | `-` |
| `__u8` | `hp_paired_volume` | `-` |
| `__u8` | `hp_paired_assignment` | `-` |
| `__u8` | `reserved` | `5` |
| `__u8` | `boost_flag` | `-` |
| `__u8` | `nominal_level_flag` | `-` |
| `__u8` | `reserved` | `6` |
| `__u8` | `gain_and_invert` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT` |
| `__u8` | `flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT` |
| `__u8` | `reserved` | `64` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `gain` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `pan` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_balance` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_width` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_volume` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` |
| `__u8` | `paired_flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` |

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `gain` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `pan` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_balance` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |
| `__u8` | `paired_width` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT` |

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `paired_volume` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` |
| `__u8` | `paired_flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT` |

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `main_paired_volume` | `-` |
| `__u8` | `hp_paired_volume` | `-` |
| `__u8` | `hp_paired_assignment` | `-` |
| `__u8` | `reserved` | `5` |

### `struct anonymous_20`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `boost_flag` | `-` |
| `__u8` | `nominal_level_flag` | `-` |
| `__u8` | `reserved` | `6` |

### `struct anonymous_21`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `gain_and_invert` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT` |
| `__u8` | `flag` | `SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT` |

### `struct snd_firewire_motu_command_dsp_meter`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `data` | `SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT` |
| `float` | `data` | `SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT` |