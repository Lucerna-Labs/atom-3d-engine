# asoc.h

**Source:** `asoc.h`


## Includes

- `linux/types.h`
- `sound/asound.h`

## Defines (101 total)


### SND_SOC (101)

| Name | Value | Comment |
|------|-------|---------|
| `SND_SOC_TPLG_MAX_CHAN` | `8` |  |
| `SND_SOC_TPLG_MAX_FORMATS` | `16` |  |
| `SND_SOC_TPLG_STREAM_CONFIG_MAX` | `8` |  |
| `SND_SOC_TPLG_HW_CONFIG_MAX` | `8` |  |
| `SND_SOC_TPLG_CTL_VOLSW` | `1` |  |
| `SND_SOC_TPLG_CTL_VOLSW_SX` | `2` |  |
| `SND_SOC_TPLG_CTL_VOLSW_XR_SX` | `3` |  |
| `SND_SOC_TPLG_CTL_ENUM` | `4` |  |
| `SND_SOC_TPLG_CTL_BYTES` | `5` |  |
| `SND_SOC_TPLG_CTL_ENUM_VALUE` | `6` |  |
| `SND_SOC_TPLG_CTL_RANGE` | `7` |  |
| `SND_SOC_TPLG_CTL_STROBE` | `8` |  |
| `SND_SOC_TPLG_DAPM_CTL_VOLSW` | `64` |  |
| `SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE` | `65` |  |
| `SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT` | `66` |  |
| `SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE` | `67` |  |
| `SND_SOC_TPLG_DAPM_CTL_PIN` | `68` |  |
| `SND_SOC_TPLG_DAPM_INPUT` | `0` |  |
| `SND_SOC_TPLG_DAPM_OUTPUT` | `1` |  |
| `SND_SOC_TPLG_DAPM_MUX` | `2` |  |
| `SND_SOC_TPLG_DAPM_MIXER` | `3` |  |
| `SND_SOC_TPLG_DAPM_PGA` | `4` |  |
| `SND_SOC_TPLG_DAPM_OUT_DRV` | `5` |  |
| `SND_SOC_TPLG_DAPM_ADC` | `6` |  |
| `SND_SOC_TPLG_DAPM_DAC` | `7` |  |
| `SND_SOC_TPLG_DAPM_SWITCH` | `8` |  |
| `SND_SOC_TPLG_DAPM_PRE` | `9` |  |
| `SND_SOC_TPLG_DAPM_POST` | `10` |  |
| `SND_SOC_TPLG_DAPM_AIF_IN` | `11` |  |
| `SND_SOC_TPLG_DAPM_AIF_OUT` | `12` |  |
| `SND_SOC_TPLG_DAPM_DAI_IN` | `13` |  |
| `SND_SOC_TPLG_DAPM_DAI_OUT` | `14` |  |
| `SND_SOC_TPLG_DAPM_DAI_LINK` | `15` |  |
| `SND_SOC_TPLG_DAPM_BUFFER` | `16` |  |
| `SND_SOC_TPLG_DAPM_SCHEDULER` | `17` |  |
| `SND_SOC_TPLG_DAPM_EFFECT` | `18` |  |
| `SND_SOC_TPLG_DAPM_SIGGEN` | `19` |  |
| `SND_SOC_TPLG_DAPM_SRC` | `20` |  |
| `SND_SOC_TPLG_DAPM_ASRC` | `21` |  |
| `SND_SOC_TPLG_DAPM_ENCODER` | `22` |  |
| `SND_SOC_TPLG_DAPM_DECODER` | `23` |  |
| `SND_SOC_TPLG_DAPM_LAST` | `SND_SOC_TPLG_DAPM_DECODER` |  |
| `SND_SOC_TPLG_MAGIC` | `0x41536F43` | ASoC |
| `SND_SOC_TPLG_NUM_TEXTS` | `16` |  |
| `SND_SOC_TPLG_ABI_VERSION` | `0x5` | current version |
| `SND_SOC_TPLG_ABI_VERSION_MIN` | `0x5` | oldest version supported |
| `SND_SOC_TPLG_TLV_SIZE` | `32` |  |
| `SND_SOC_TPLG_TYPE_MIXER` | `1` |  |
| `SND_SOC_TPLG_TYPE_BYTES` | `2` |  |
| `SND_SOC_TPLG_TYPE_ENUM` | `3` |  |

*...and 51 more*

## Structs (23)


### `struct snd_soc_tplg_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `magic` | `-` |
| `__le32` | `abi` | `-` |
| `__le32` | `version` | `-` |
| `__le32` | `type` | `-` |
| `__le32` | `size` | `-` |
| `__le32` | `vendor_type` | `-` |
| `__le32` | `payload_size` | `-` |
| `__le32` | `index` | `-` |
| `__le32` | `count` | `-` |

### `struct snd_soc_tplg_vendor_uuid_elem`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `token` | `-` |
| `char` | `uuid` | `16` |

### `struct snd_soc_tplg_vendor_value_elem`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `token` | `-` |
| `__le32` | `value` | `-` |

### `struct snd_soc_tplg_vendor_string_elem`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `token` | `-` |
| `char` | `string` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |

### `struct snd_soc_tplg_vendor_array`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `type` | `-` |
| `__le32` | `num_elems` | `-` |

### `struct snd_soc_tplg_private`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |

### `struct snd_soc_tplg_tlv_dbscale`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `min` | `-` |
| `__le32` | `step` | `-` |
| `__le32` | `mute` | `-` |

### `struct snd_soc_tplg_ctl_tlv`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `type` | `-` |
| `__le32` | `data` | `SND_SOC_TPLG_TLV_SIZE` |

### `struct snd_soc_tplg_channel`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `reg` | `-` |
| `__le32` | `shift` | `-` |
| `__le32` | `id` | `-` |

### `struct snd_soc_tplg_io_ops`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `get` | `-` |
| `__le32` | `put` | `-` |
| `__le32` | `info` | `-` |

### `struct snd_soc_tplg_ctl_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `type` | `-` |
| `char` | `name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le32` | `access` | `-` |

### `struct snd_soc_tplg_stream_caps`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `char` | `name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le64` | `formats` | `-` |
| `__le32` | `rates` | `-` |
| `__le32` | `rate_min` | `-` |
| `__le32` | `rate_max` | `-` |
| `__le32` | `channels_min` | `-` |
| `__le32` | `channels_max` | `-` |
| `__le32` | `periods_min` | `-` |
| `__le32` | `periods_max` | `-` |
| `__le32` | `period_size_min` | `-` |
| `__le32` | `period_size_max` | `-` |
| `__le32` | `buffer_size_min` | `-` |
| `__le32` | `buffer_size_max` | `-` |
| `__le32` | `sig_bits` | `-` |

### `struct snd_soc_tplg_stream`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `char` | `name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le64` | `format` | `-` |
| `__le32` | `rate` | `-` |
| `__le32` | `period_bytes` | `-` |
| `__le32` | `buffer_bytes` | `-` |
| `__le32` | `channels` | `-` |

### `struct snd_soc_tplg_hw_config`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `id` | `-` |
| `__le32` | `fmt` | `-` |
| `__u8` | `clock_gated` | `-` |
| `__u8` | `invert_bclk` | `-` |
| `__u8` | `invert_fsync` | `-` |
| `__u8` | `bclk_provider` | `-` |
| `__u8` | `fsync_provider` | `-` |
| `__u8` | `mclk_direction` | `-` |
| `__le16` | `reserved` | `-` |
| `__le32` | `mclk_rate` | `-` |
| `__le32` | `bclk_rate` | `-` |
| `__le32` | `fsync_rate` | `-` |
| `__le32` | `tdm_slots` | `-` |
| `__le32` | `tdm_slot_width` | `-` |
| `__le32` | `tx_slots` | `-` |
| `__le32` | `rx_slots` | `-` |
| `__le32` | `tx_channels` | `-` |
| `__le32` | `tx_chanmap` | `SND_SOC_TPLG_MAX_CHAN` |
| `__le32` | `rx_channels` | `-` |
| `__le32` | `rx_chanmap` | `SND_SOC_TPLG_MAX_CHAN` |

### `struct snd_soc_tplg_manifest`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `control_elems` | `-` |
| `__le32` | `widget_elems` | `-` |
| `__le32` | `graph_elems` | `-` |
| `__le32` | `pcm_elems` | `-` |
| `__le32` | `dai_link_elems` | `-` |
| `__le32` | `dai_elems` | `-` |
| `__le32` | `reserved` | `20` |

### `struct snd_soc_tplg_mixer_control`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `min` | `-` |
| `__le32` | `max` | `-` |
| `__le32` | `platform_max` | `-` |
| `__le32` | `invert` | `-` |
| `__le32` | `num_channels` | `-` |

### `struct snd_soc_tplg_enum_control`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `num_channels` | `-` |
| `__le32` | `items` | `-` |
| `__le32` | `mask` | `-` |
| `__le32` | `count` | `-` |
| `char` | `texts` | `SND_SOC_TPLG_NUM_TEXTS][SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le32` | `values` | `SND_SOC_TPLG_NUM_TEXTS * SNDRV_CTL_ELEM_ID_NAME_MAXLEN / 4` |

### `struct snd_soc_tplg_bytes_control`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `max` | `-` |
| `__le32` | `mask` | `-` |
| `__le32` | `base` | `-` |
| `__le32` | `num_regs` | `-` |

### `struct snd_soc_tplg_dapm_graph_elem`

| Type | Field | Array |
|------|-------|-------|
| `char` | `sink` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `char` | `control` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `char` | `source` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |

### `struct snd_soc_tplg_dapm_widget`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `id` | `-` |
| `char` | `name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `char` | `sname` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le32` | `reg` | `-` |
| `__le32` | `shift` | `-` |
| `__le32` | `mask` | `-` |
| `__le32` | `subseq` | `-` |
| `__le32` | `invert` | `-` |
| `__le32` | `ignore_suspend` | `-` |
| `__le16` | `event_flags` | `-` |
| `__le16` | `event_type` | `-` |
| `__le32` | `num_kcontrols` | `-` |

### `struct snd_soc_tplg_pcm`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `char` | `pcm_name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `char` | `dai_name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le32` | `pcm_id` | `-` |
| `__le32` | `dai_id` | `-` |
| `__le32` | `playback` | `-` |
| `__le32` | `capture` | `-` |
| `__le32` | `compress` | `-` |
| `__le32` | `num_streams` | `-` |
| `__le32` | `flag_mask` | `-` |
| `__le32` | `flags` | `-` |

### `struct snd_soc_tplg_link_config`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `id` | `-` |
| `char` | `name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `char` | `stream_name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le32` | `num_streams` | `-` |
| `__le32` | `num_hw_configs` | `-` |
| `__le32` | `default_hw_config_id` | `-` |
| `__le32` | `flag_mask` | `-` |
| `__le32` | `flags` | `-` |

### `struct snd_soc_tplg_dai`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `char` | `dai_name` | `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` |
| `__le32` | `dai_id` | `-` |
| `__le32` | `playback` | `-` |
| `__le32` | `capture` | `-` |
| `__le32` | `flag_mask` | `-` |
| `__le32` | `flags` | `-` |