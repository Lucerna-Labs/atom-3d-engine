# compress_params.h

**Source:** `compress_params.h`


## Includes

- `linux/types.h`

## Defines (127 total)


### MAX_NUM (4)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_NUM_CODECS` | `32` |  |
| `MAX_NUM_CODEC_DESCRIPTORS` | `32` |  |
| `MAX_NUM_BITRATES` | `32` |  |
| `MAX_NUM_SAMPLE_RATES` | `32` |  |

### SND_AUDIOCHANMODE (4)

| Name | Value | Comment |
|------|-------|---------|
| `SND_AUDIOCHANMODE_MP3_MONO` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOCHANMODE_MP3_STEREO` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOCHANMODE_MP3_JOINTSTEREO` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOCHANMODE_MP3_DUAL` | `((__u32) 0x00000008)` |  |

### SND_AUDIOCODEC (18)

| Name | Value | Comment |
|------|-------|---------|
| `SND_AUDIOCODEC_PCM` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOCODEC_MP3` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOCODEC_AMR` | `((__u32) 0x00000003)` |  |
| `SND_AUDIOCODEC_AMRWB` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOCODEC_AMRWBPLUS` | `((__u32) 0x00000005)` |  |
| `SND_AUDIOCODEC_AAC` | `((__u32) 0x00000006)` |  |
| `SND_AUDIOCODEC_WMA` | `((__u32) 0x00000007)` |  |
| `SND_AUDIOCODEC_REAL` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOCODEC_VORBIS` | `((__u32) 0x00000009)` |  |
| `SND_AUDIOCODEC_FLAC` | `((__u32) 0x0000000A)` |  |
| `SND_AUDIOCODEC_IEC61937` | `((__u32) 0x0000000B)` |  |
| `SND_AUDIOCODEC_G723_1` | `((__u32) 0x0000000C)` |  |
| `SND_AUDIOCODEC_G729` | `((__u32) 0x0000000D)` |  |
| `SND_AUDIOCODEC_BESPOKE` | `((__u32) 0x0000000E)` |  |
| `SND_AUDIOCODEC_ALAC` | `((__u32) 0x0000000F)` |  |
| `SND_AUDIOCODEC_APE` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOCODEC_OPUS_RAW` | `((__u32) 0x00000011)` |  |
| `SND_AUDIOCODEC_MAX` | `SND_AUDIOCODEC_OPUS_RAW` |  |

### SND_AUDIOMODE (62)

| Name | Value | Comment |
|------|-------|---------|
| `SND_AUDIOMODE_AMR_DTX_OFF` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_AMR_VAD1` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_AMR_VAD2` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_AMRWB_DTX_OFF` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_AMRWB_VAD1` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_AMRWB_VAD2` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_AAC_MAIN` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_AAC_LC` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_AAC_SSR` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_AAC_LTP` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOMODE_AAC_HE` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOMODE_AAC_SCALABLE` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOMODE_AAC_ERLC` | `((__u32) 0x00000040)` |  |
| `SND_AUDIOMODE_AAC_LD` | `((__u32) 0x00000080)` |  |
| `SND_AUDIOMODE_AAC_HE_PS` | `((__u32) 0x00000100)` |  |
| `SND_AUDIOMODE_AAC_HE_MPS` | `((__u32) 0x00000200)` |  |
| `SND_AUDIOMODE_WMA_LEVEL1` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_WMA_LEVEL2` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_WMA_LEVEL3` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_WMA_LEVEL4` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOMODE_WMAPRO_LEVELM0` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOMODE_WMAPRO_LEVELM1` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOMODE_WMAPRO_LEVELM2` | `((__u32) 0x00000040)` |  |
| `SND_AUDIOMODE_WMAPRO_LEVELM3` | `((__u32) 0x00000080)` |  |
| `SND_AUDIOMODE_REALAUDIO_G2` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_REALAUDIO_8` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_REALAUDIO_10` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_REALAUDIO_SURROUND` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOMODE_VORBIS` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL0` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL1` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL2` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL3` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL4` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL5` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL6` | `((__u32) 0x00000040)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL7` | `((__u32) 0x00000080)` |  |
| `SND_AUDIOMODE_FLAC_LEVEL8` | `((__u32) 0x00000100)` |  |
| `SND_AUDIOMODE_IEC_REF_STREAM_HEADER` | `((__u32) 0x00000000)` |  |
| `SND_AUDIOMODE_IEC_LPCM` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOMODE_IEC_AC3` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOMODE_IEC_MPEG1` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOMODE_IEC_MP3` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOMODE_IEC_MPEG2` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOMODE_IEC_AACLC` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOMODE_IEC_DTS` | `((__u32) 0x00000040)` |  |
| `SND_AUDIOMODE_IEC_ATRAC` | `((__u32) 0x00000080)` |  |
| `SND_AUDIOMODE_IEC_SACD` | `((__u32) 0x00000100)` |  |
| `SND_AUDIOMODE_IEC_EAC3` | `((__u32) 0x00000200)` |  |
| `SND_AUDIOMODE_IEC_DTS_HD` | `((__u32) 0x00000400)` |  |

*...and 12 more*

### SND_AUDIOPROFILE (19)

| Name | Value | Comment |
|------|-------|---------|
| `SND_AUDIOPROFILE_PCM` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_AMR` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_AMRWB` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_AMRWBPLUS` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_AAC` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_WMA7` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_WMA8` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOPROFILE_WMA9` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOPROFILE_WMA10` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOPROFILE_WMA9_PRO` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOPROFILE_WMA9_LOSSLESS` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOPROFILE_WMA10_LOSSLESS` | `((__u32) 0x00000040)` |  |
| `SND_AUDIOPROFILE_REALAUDIO` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_VORBIS` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_FLAC` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_IEC61937` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_IEC61937_SPDIF` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOPROFILE_G723_1` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOPROFILE_G729` | `((__u32) 0x00000001)` |  |

### SND_AUDIOSTREAMFORMAT (18)

| Name | Value | Comment |
|------|-------|---------|
| `SND_AUDIOSTREAMFORMAT_UNDEFINED` | `((__u32) 0x00000000)` |  |
| `SND_AUDIOSTREAMFORMAT_CONFORMANCE` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOSTREAMFORMAT_IF1` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOSTREAMFORMAT_IF2` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOSTREAMFORMAT_FSF` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOSTREAMFORMAT_RTPPAYLOAD` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOSTREAMFORMAT_ITU` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOSTREAMFORMAT_MP2ADTS` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOSTREAMFORMAT_MP4ADTS` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOSTREAMFORMAT_MP4LOAS` | `((__u32) 0x00000004)` |  |
| `SND_AUDIOSTREAMFORMAT_MP4LATM` | `((__u32) 0x00000008)` |  |
| `SND_AUDIOSTREAMFORMAT_ADIF` | `((__u32) 0x00000010)` |  |
| `SND_AUDIOSTREAMFORMAT_MP4FF` | `((__u32) 0x00000020)` |  |
| `SND_AUDIOSTREAMFORMAT_RAW` | `((__u32) 0x00000040)` |  |
| `SND_AUDIOSTREAMFORMAT_WMA_ASF` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOSTREAMFORMAT_WMA_NOASF_HDR` | `((__u32) 0x00000002)` |  |
| `SND_AUDIOSTREAMFORMAT_FLAC` | `((__u32) 0x00000001)` |  |
| `SND_AUDIOSTREAMFORMAT_FLAC_OGG` | `((__u32) 0x00000002)` |  |

### SND_RATECONTROLMODE (2)

| Name | Value | Comment |
|------|-------|---------|
| `SND_RATECONTROLMODE_CONSTANTBITRATE` | `((__u32) 0x00000001)` |  |
| `SND_RATECONTROLMODE_VARIABLEBITRATE` | `((__u32) 0x00000002)` |  |

## Structs (15)


### `struct snd_enc_wma`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `super_block_align` | `-` |

### `struct snd_enc_vorbis`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `quality` | `-` |
| `__u32` | `managed` | `-` |
| `__u32` | `max_bit_rate` | `-` |
| `__u32` | `min_bit_rate` | `-` |
| `__u32` | `downmix` | `-` |

### `struct snd_enc_real`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `quant_bits` | `-` |
| `__u32` | `start_region` | `-` |
| `__u32` | `num_regions` | `-` |

### `struct snd_enc_flac`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num` | `-` |
| `__u32` | `gain` | `-` |

### `struct snd_enc_generic`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bw` | `-` |
| `__s32` | `reserved` | `15` |

### `struct snd_dec_flac`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `sample_size` | `-` |
| `__u16` | `min_blk_size` | `-` |
| `__u16` | `max_blk_size` | `-` |
| `__u16` | `min_frame_size` | `-` |
| `__u16` | `max_frame_size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct snd_dec_wma`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `encoder_option` | `-` |
| `__u32` | `adv_encoder_option` | `-` |
| `__u32` | `adv_encoder_option2` | `-` |
| `__u32` | `reserved` | `-` |

### `struct snd_dec_alac`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `frame_length` | `-` |
| `__u8` | `compatible_version` | `-` |
| `__u8` | `pb` | `-` |
| `__u8` | `mb` | `-` |
| `__u8` | `kb` | `-` |
| `__u32` | `max_run` | `-` |
| `__u32` | `max_frame_bytes` | `-` |

### `struct snd_dec_ape`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `compatible_version` | `-` |
| `__u16` | `compression_level` | `-` |
| `__u32` | `format_flags` | `-` |
| `__u32` | `blocks_per_frame` | `-` |
| `__u32` | `final_frame_blocks` | `-` |
| `__u32` | `total_frames` | `-` |
| `__u32` | `seek_table_present` | `-` |

### `struct snd_dec_opus`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `version` | `-` |
| `__u8` | `num_channels` | `-` |
| `__u16` | `pre_skip` | `-` |
| `__u32` | `sample_rate` | `-` |
| `__u16` | `output_gain` | `-` |
| `__u8` | `mapping_family` | `-` |
| `__u8` | `stream_count` | `-` |
| `__u8` | `coupled_count` | `-` |
| `__u8` | `channel_map` | `8` |

### `struct snd_dec_opus_ch_map`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `stream_count` | `-` |
| `__u8` | `coupled_count` | `-` |
| `__u8` | `channel_map` | `8` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `out_sample_rate` | `-` |

### `struct snd_codec_desc_src`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `out_sample_rate_min` | `-` |
| `__u32` | `out_sample_rate_max` | `-` |

### `struct snd_codec_desc`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_ch` | `-` |
| `__u32` | `sample_rates` | `MAX_NUM_SAMPLE_RATES` |
| `__u32` | `num_sample_rates` | `-` |
| `__u32` | `bit_rate` | `MAX_NUM_BITRATES` |
| `__u32` | `num_bitrates` | `-` |
| `__u32` | `rate_control` | `-` |
| `__u32` | `profiles` | `-` |
| `__u32` | `modes` | `-` |
| `__u32` | `formats` | `-` |
| `__u32` | `min_buffer` | `-` |
| `__u32` | `pcm_formats` | `-` |
| `__u32` | `u_space` | `6` |
| `__u32` | `reserved` | `8` |

### `struct snd_codec`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `ch_in` | `-` |
| `__u32` | `ch_out` | `-` |
| `__u32` | `sample_rate` | `-` |
| `__u32` | `bit_rate` | `-` |
| `__u32` | `rate_control` | `-` |
| `__u32` | `profile` | `-` |
| `__u32` | `level` | `-` |
| `__u32` | `ch_mode` | `-` |
| `__u32` | `format` | `-` |
| `__u32` | `align` | `-` |
| `__u32` | `pcm_format` | `-` |
| `__u32` | `reserved` | `2` |