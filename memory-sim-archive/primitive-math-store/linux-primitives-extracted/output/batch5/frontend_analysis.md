# frontend.h

**Source:** `frontend.h`


## Includes

- `linux/types.h`

## Defines (100 total)


### DTV_API (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_API_VERSION` | `35` |  |

### DTV_ATSCMH (15)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_ATSCMH_FIC_VER` | `45` |  |
| `DTV_ATSCMH_PARADE_ID` | `46` |  |
| `DTV_ATSCMH_NOG` | `47` |  |
| `DTV_ATSCMH_TNOG` | `48` |  |
| `DTV_ATSCMH_SGN` | `49` |  |
| `DTV_ATSCMH_PRC` | `50` |  |
| `DTV_ATSCMH_RS_FRAME_MODE` | `51` |  |
| `DTV_ATSCMH_RS_FRAME_ENSEMBLE` | `52` |  |
| `DTV_ATSCMH_RS_CODE_MODE_PRI` | `53` |  |
| `DTV_ATSCMH_RS_CODE_MODE_SEC` | `54` |  |
| `DTV_ATSCMH_SCCC_BLOCK_MODE` | `55` |  |
| `DTV_ATSCMH_SCCC_CODE_MODE_A` | `56` |  |
| `DTV_ATSCMH_SCCC_CODE_MODE_B` | `57` |  |
| `DTV_ATSCMH_SCCC_CODE_MODE_C` | `58` |  |
| `DTV_ATSCMH_SCCC_CODE_MODE_D` | `59` |  |

### DTV_BANDWIDTH (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_BANDWIDTH_HZ` | `5` |  |

### DTV_CODE (2)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_CODE_RATE_HP` | `36` |  |
| `DTV_CODE_RATE_LP` | `37` |  |

### DTV_DELIVERY (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_DELIVERY_SYSTEM` | `17` |  |

### DTV_DISEQC (2)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_DISEQC_MASTER` | `7` |  |
| `DTV_DISEQC_SLAVE_REPLY` | `14` |  |

### DTV_ENUM (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_ENUM_DELSYS` | `44` |  |

### DTV_FE (2)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_FE_CAPABILITY_COUNT` | `15` |  |
| `DTV_FE_CAPABILITY` | `16` |  |

### DTV_GUARD (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_GUARD_INTERVAL` | `38` |  |

### DTV_INNER (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_INNER_FEC` | `9` |  |

### DTV_IOCTL (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_IOCTL_MAX_MSGS` | `64` |  |

### DTV_ISDBS (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_ISDBS_TS_ID_LEGACY` | `DTV_STREAM_ID` |  |

### DTV_ISDBT (18)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_ISDBT_PARTIAL_RECEPTION` | `18` |  |
| `DTV_ISDBT_SOUND_BROADCASTING` | `19` |  |
| `DTV_ISDBT_SB_SUBCHANNEL_ID` | `20` |  |
| `DTV_ISDBT_SB_SEGMENT_IDX` | `21` |  |
| `DTV_ISDBT_SB_SEGMENT_COUNT` | `22` |  |
| `DTV_ISDBT_LAYERA_FEC` | `23` |  |
| `DTV_ISDBT_LAYERA_MODULATION` | `24` |  |
| `DTV_ISDBT_LAYERA_SEGMENT_COUNT` | `25` |  |
| `DTV_ISDBT_LAYERA_TIME_INTERLEAVING` | `26` |  |
| `DTV_ISDBT_LAYERB_FEC` | `27` |  |
| `DTV_ISDBT_LAYERB_MODULATION` | `28` |  |
| `DTV_ISDBT_LAYERB_SEGMENT_COUNT` | `29` |  |
| `DTV_ISDBT_LAYERB_TIME_INTERLEAVING` | `30` |  |
| `DTV_ISDBT_LAYERC_FEC` | `31` |  |
| `DTV_ISDBT_LAYERC_MODULATION` | `32` |  |
| `DTV_ISDBT_LAYERC_SEGMENT_COUNT` | `33` |  |
| `DTV_ISDBT_LAYERC_TIME_INTERLEAVING` | `34` |  |
| `DTV_ISDBT_LAYER_ENABLED` | `41` |  |

### DTV_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_MAX_COMMAND` | `DTV_SCRAMBLING_SEQUENCE_INDEX` |  |

### DTV_SCRAMBLING (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_SCRAMBLING_SEQUENCE_INDEX` | `70` |  |

### DTV_STAT (8)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_STAT_SIGNAL_STRENGTH` | `62` |  |
| `DTV_STAT_CNR` | `63` |  |
| `DTV_STAT_PRE_ERROR_BIT_COUNT` | `64` |  |
| `DTV_STAT_PRE_TOTAL_BIT_COUNT` | `65` |  |
| `DTV_STAT_POST_ERROR_BIT_COUNT` | `66` |  |
| `DTV_STAT_POST_TOTAL_BIT_COUNT` | `67` |  |
| `DTV_STAT_ERROR_BLOCK_COUNT` | `68` |  |
| `DTV_STAT_TOTAL_BLOCK_COUNT` | `69` |  |

### DTV_STREAM (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_STREAM_ID` | `42` |  |

### DTV_SYMBOL (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_SYMBOL_RATE` | `8` |  |

### DTV_TRANSMISSION (1)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_TRANSMISSION_MODE` | `39` |  |

### FE_DISEQC (4)

| Name | Value | Comment |
|------|-------|---------|
| `FE_DISEQC_RESET_OVERLOAD` | `_IO('o', 62)` |  |
| `FE_DISEQC_SEND_MASTER_CMD` | `_IOW('o', 63, struct dvb_diseqc_master_cmd)` |  |
| `FE_DISEQC_RECV_SLAVE_REPLY` | `_IOR('o', 64, struct dvb_diseqc_slave_reply)` |  |
| `FE_DISEQC_SEND_BURST` | `_IO('o', 65)` | fe_sec_mini_cmd_t |

### FE_DISHNETWORK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FE_DISHNETWORK_SEND_LEGACY_CMD` | `_IO('o', 80)` | unsigned int |

### FE_ENABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FE_ENABLE_HIGH_LNB_VOLTAGE` | `_IO('o', 68)` | int |

### FE_GET (4)

| Name | Value | Comment |
|------|-------|---------|
| `FE_GET_INFO` | `_IOR('o', 61, struct dvb_frontend_info)` |  |
| `FE_GET_EVENT` | `_IOR('o', 78, struct dvb_frontend_event)` |  |
| `FE_GET_PROPERTY` | `_IOR('o', 83, struct dtv_properties)` |  |
| `FE_GET_FRONTEND` | `_IOR('o', 77, struct dvb_frontend_parameters)` |  |

### FE_READ (5)

| Name | Value | Comment |
|------|-------|---------|
| `FE_READ_STATUS` | `_IOR('o', 69, fe_status_t)` |  |
| `FE_READ_BER` | `_IOR('o', 70, __u32)` |  |
| `FE_READ_SIGNAL_STRENGTH` | `_IOR('o', 71, __u16)` |  |
| `FE_READ_SNR` | `_IOR('o', 72, __u16)` |  |
| `FE_READ_UNCORRECTED_BLOCKS` | `_IOR('o', 73, __u32)` |  |

### FE_SET (5)

| Name | Value | Comment |
|------|-------|---------|
| `FE_SET_TONE` | `_IO('o', 66)` | fe_sec_tone_mode_t |
| `FE_SET_VOLTAGE` | `_IO('o', 67)` | fe_sec_voltage_t |
| `FE_SET_FRONTEND_TUNE_MODE` | `_IO('o', 81)` | unsigned int |
| `FE_SET_PROPERTY` | `_IOW('o', 82, struct dtv_properties)` |  |
| `FE_SET_FRONTEND` | `_IOW('o', 76, struct dvb_frontend_parameters)` |  |

### FE_TUNE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FE_TUNE_MODE_ONESHOT` | `0x01` |  |

### MAX_DTV (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_DTV_STATS` | `4` |  |

### NO_STREAM (1)

| Name | Value | Comment |
|------|-------|---------|
| `NO_STREAM_ID_FILTER` | `(~0U)` |  |

### SYS_DVBC (1)

| Name | Value | Comment |
|------|-------|---------|
| `SYS_DVBC_ANNEX_AC` | `SYS_DVBC_ANNEX_A` |  |

### UNCATEGORIZED (16)

| Name | Value | Comment |
|------|-------|---------|
| `DTV_UNDEFINED` | `0` |  |
| `DTV_TUNE` | `1` |  |
| `DTV_CLEAR` | `2` |  |
| `DTV_FREQUENCY` | `3` |  |
| `DTV_MODULATION` | `4` |  |
| `DTV_INVERSION` | `6` |  |
| `DTV_VOLTAGE` | `10` |  |
| `DTV_TONE` | `11` |  |
| `DTV_PILOT` | `12` |  |
| `DTV_ROLLOFF` | `13` |  |
| `DTV_HIERARCHY` | `40` |  |
| `DTV_DVBT2_PLP_ID_LEGACY` | `43` |  |
| `DTV_INTERLEAVING` | `60` |  |
| `DTV_LNA` | `61` |  |
| `SYS_DMBTH` | `SYS_DTMB` | DMB-TH is legacy name, use DTMB |
| `LNA_AUTO` | `(~0U)` |  |

## Structs (14)


### `struct dvb_frontend_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `128` |
| `__u32` | `frequency_min` | `-` |
| `__u32` | `frequency_max` | `-` |
| `__u32` | `frequency_stepsize` | `-` |
| `__u32` | `frequency_tolerance` | `-` |
| `__u32` | `symbol_rate_min` | `-` |
| `__u32` | `symbol_rate_max` | `-` |
| `__u32` | `symbol_rate_tolerance` | `-` |
| `__u32` | `notifier_delay` | `-` |

### `struct dvb_diseqc_master_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `msg` | `6` |
| `__u8` | `msg_len` | `-` |

### `struct dvb_diseqc_slave_reply`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `msg` | `4` |
| `__u8` | `msg_len` | `-` |
| `int` | `timeout` | `-` |

### `struct dtv_stats`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `scale` | `-` |
| `__u64` | `uvalue` | `-` |
| `__s64` | `svalue` | `-` |

### `struct dtv_fe_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `len` | `-` |

### `struct dtv_property`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u32` | `reserved` | `3` |
| `__u32` | `data` | `-` |
| `__u8` | `data` | `32` |
| `__u32` | `len` | `-` |
| `__u32` | `reserved1` | `3` |
| `int` | `result` | `-` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `data` | `32` |
| `__u32` | `len` | `-` |
| `__u32` | `reserved1` | `3` |

### `struct dtv_properties`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num` | `-` |

### `struct dvb_qpsk_parameters`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `symbol_rate` | `-` |
| `fe_code_rate_t` | `fec_inner` | `-` |

### `struct dvb_qam_parameters`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `symbol_rate` | `-` |
| `fe_code_rate_t` | `fec_inner` | `-` |
| `fe_modulation_t` | `modulation` | `-` |

### `struct dvb_vsb_parameters`

| Type | Field | Array |
|------|-------|-------|
| `fe_modulation_t` | `modulation` | `-` |

### `struct dvb_ofdm_parameters`

| Type | Field | Array |
|------|-------|-------|
| `fe_bandwidth_t` | `bandwidth` | `-` |
| `fe_code_rate_t` | `code_rate_HP` | `-` |
| `fe_code_rate_t` | `code_rate_LP` | `-` |
| `fe_modulation_t` | `constellation` | `-` |
| `fe_transmit_mode_t` | `transmission_mode` | `-` |
| `fe_guard_interval_t` | `guard_interval` | `-` |
| `fe_hierarchy_t` | `hierarchy_information` | `-` |

### `struct dvb_frontend_parameters`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `frequency` | `-` |
| `fe_spectral_inversion_t` | `inversion` | `-` |

### `struct dvb_frontend_event`

| Type | Field | Array |
|------|-------|-------|
| `fe_status_t` | `status` | `-` |

## Typedefs

- `fe_sec_voltage_t`
- `fe_caps_t`
- `fe_type_t`
- `fe_sec_tone_mode_t`
- `fe_sec_mini_cmd_t`
- `fe_status_t`
- `fe_spectral_inversion_t`
- `fe_code_rate_t`
- `fe_modulation_t`
- `fe_transmit_mode_t`
- `fe_bandwidth_t`
- `fe_guard_interval_t`
- `fe_hierarchy_t`
- `fe_pilot_t`
- `fe_rolloff_t`
- `fe_delivery_system_t`