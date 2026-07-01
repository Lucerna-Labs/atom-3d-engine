# tlv.h

**Source:** `tlv.h`


## Defines (21 total)


### SNDRV_CTL (21)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_CTL_TLVT_CONTAINER` | `0` | one level down - group of TLVs |
| `SNDRV_CTL_TLVT_DB_SCALE` | `1` | dB scale |
| `SNDRV_CTL_TLVT_DB_LINEAR` | `2` | linear volume |
| `SNDRV_CTL_TLVT_DB_RANGE` | `3` | dB range container |
| `SNDRV_CTL_TLVT_DB_MINMAX` | `4` | dB scale with min/max |
| `SNDRV_CTL_TLVT_DB_MINMAX_MUTE` | `5` | dB scale with min/max with mute |
| `SNDRV_CTL_TLVT_CHMAP_FIXED` | `0x101` | fixed channel position |
| `SNDRV_CTL_TLVT_CHMAP_VAR` | `0x102` | channels freely swappable |
| `SNDRV_CTL_TLVT_CHMAP_PAIRED` | `0x103` | pair-wise swappable |
| `SNDRV_CTL_TLVT_FCP_CHANNEL_LABELS` | `0x110` | channel labels |
| `SNDRV_CTL_TLVO_TYPE` | `0` |  |
| `SNDRV_CTL_TLVO_LEN` | `1` |  |
| `SNDRV_CTL_TLVD_DB_SCALE_MASK` | `0xffff` |  |
| `SNDRV_CTL_TLVD_DB_SCALE_MUTE` | `0x10000` |  |
| `SNDRV_CTL_TLVO_DB_SCALE_MIN` | `2` |  |
| `SNDRV_CTL_TLVO_DB_SCALE_MUTE_AND_STEP` | `3` |  |
| `SNDRV_CTL_TLVO_DB_MINMAX_MIN` | `2` |  |
| `SNDRV_CTL_TLVO_DB_MINMAX_MAX` | `3` |  |
| `SNDRV_CTL_TLVO_DB_LINEAR_MIN` | `2` |  |
| `SNDRV_CTL_TLVO_DB_LINEAR_MAX` | `3` |  |
| `SNDRV_CTL_TLVD_DB_GAIN_MUTE` | `-9999999` |  |