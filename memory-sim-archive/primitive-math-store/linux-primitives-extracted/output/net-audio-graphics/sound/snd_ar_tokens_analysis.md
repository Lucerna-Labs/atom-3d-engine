# snd_ar_tokens.h

**Source:** `snd_ar_tokens.h`


## Includes

- `linux/types.h`

## Defines (80 total)


### APM_CONT (4)

| Name | Value | Comment |
|------|-------|---------|
| `APM_CONT_GRAPH_POS_STREAM` | `0x1` |  |
| `APM_CONT_GRAPH_POS_PER_STR_PER_DEV` | `0x2` |  |
| `APM_CONT_GRAPH_POS_STR_DEV` | `0x3` |  |
| `APM_CONT_GRAPH_POS_GLOBAL_DEV` | `0x4` |  |

### APM_CONTAINER (4)

| Name | Value | Comment |
|------|-------|---------|
| `APM_CONTAINER_CAP_ID_PP` | `0x1` |  |
| `APM_CONTAINER_CAP_ID_CD` | `0x2` |  |
| `APM_CONTAINER_CAP_ID_EP` | `0x3` |  |
| `APM_CONTAINER_CAP_ID_OLC` | `0x4` |  |

### APM_PROC (4)

| Name | Value | Comment |
|------|-------|---------|
| `APM_PROC_DOMAIN_ID_MDSP` | `0x1` |  |
| `APM_PROC_DOMAIN_ID_ADSP` | `0x2` |  |
| `APM_PROC_DOMAIN_ID_SDSP` | `0x4` |  |
| `APM_PROC_DOMAIN_ID_CDSP` | `0x5` |  |

### APM_SUB (7)

| Name | Value | Comment |
|------|-------|---------|
| `APM_SUB_GRAPH_PERF_MODE_LOW_POWER` | `0x1` |  |
| `APM_SUB_GRAPH_PERF_MODE_LOW_LATENCY` | `0x2` |  |
| `APM_SUB_GRAPH_DIRECTION_TX` | `0x1` |  |
| `APM_SUB_GRAPH_DIRECTION_RX` | `0x2` |  |
| `APM_SUB_GRAPH_SID_AUDIO_PLAYBACK` | `0x1` |  |
| `APM_SUB_GRAPH_SID_AUDIO_RECORD` | `0x2` |  |
| `APM_SUB_GRAPH_SID_VOICE_CALL` | `0x3` |  |

### AR_TKN (53)

| Name | Value | Comment |
|------|-------|---------|
| `AR_TKN_DAI_INDEX` | `1` |  |
| `AR_TKN_U32_SUB_GRAPH_INSTANCE_ID` | `2` |  |
| `AR_TKN_U32_SUB_GRAPH_PERF_MODE` | `3` |  |
| `AR_TKN_U32_SUB_GRAPH_DIRECTION` | `4` |  |
| `AR_TKN_U32_SUB_GRAPH_SCENARIO_ID` | `5` |  |
| `AR_TKN_U32_CONTAINER_INSTANCE_ID` | `100` |  |
| `AR_TKN_U32_CONTAINER_CAPABILITY_ID` | `101` |  |
| `AR_TKN_U32_CONTAINER_STACK_SIZE` | `102` |  |
| `AR_TKN_U32_CONTAINER_GRAPH_POS` | `103` |  |
| `AR_TKN_U32_CONTAINER_PROC_DOMAIN` | `104` |  |
| `AR_TKN_U32_MODULE_ID` | `200` |  |
| `AR_TKN_U32_MODULE_INSTANCE_ID` | `201` |  |
| `AR_TKN_U32_MODULE_MAX_IP_PORTS` | `202` |  |
| `AR_TKN_U32_MODULE_MAX_OP_PORTS` | `203` |  |
| `AR_TKN_U32_MODULE_IN_PORTS` | `204` | deprecated |
| `AR_TKN_U32_MODULE_OUT_PORTS` | `205` | deprecated |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID` | `206` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID` | `207` |  |
| `AR_TKN_U32_MODULE_SRC_INSTANCE_ID` | `208` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID` | `209` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID1` | `210` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID1` | `211` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID1` | `212` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID2` | `213` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID2` | `214` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID2` | `215` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID3` | `216` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID3` | `217` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID3` | `218` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID4` | `219` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID4` | `220` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID4` | `221` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID5` | `222` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID5` | `223` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID5` | `224` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID6` | `225` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID6` | `226` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID6` | `227` |  |
| `AR_TKN_U32_MODULE_SRC_OP_PORT_ID7` | `228` |  |
| `AR_TKN_U32_MODULE_DST_IN_PORT_ID7` | `229` |  |
| `AR_TKN_U32_MODULE_DST_INSTANCE_ID7` | `230` |  |
| `AR_TKN_U32_MODULE_HW_IF_IDX` | `250` |  |
| `AR_TKN_U32_MODULE_HW_IF_TYPE` | `251` |  |
| `AR_TKN_U32_MODULE_FMT_INTERLEAVE` | `252` |  |
| `AR_TKN_U32_MODULE_FMT_DATA` | `253` |  |
| `AR_TKN_U32_MODULE_FMT_SAMPLE_RATE` | `254` |  |
| `AR_TKN_U32_MODULE_FMT_BIT_DEPTH` | `255` |  |
| `AR_TKN_U32_MODULE_SD_LINE_IDX` | `256` |  |
| `AR_TKN_U32_MODULE_WS_SRC` | `257` |  |
| `AR_TKN_U32_MODULE_FRAME_SZ_FACTOR` | `258` |  |

*...and 3 more*

### PCM_DEINTERLEAVED (2)

| Name | Value | Comment |
|------|-------|---------|
| `PCM_DEINTERLEAVED_PACKED` | `2` |  |
| `PCM_DEINTERLEAVED_UNPACKED` | `3` |  |

### SND_SOC (3)

| Name | Value | Comment |
|------|-------|---------|
| `SND_SOC_AR_TPLG_FE_BE_GRAPH_CTL_MIX` | `256` |  |
| `SND_SOC_AR_TPLG_VOL_CTL` | `257` |  |
| `SND_SOC_AR_TPLG_MODULE_CFG_TYPE` | `0x01001006` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `PCM_INTERLEAVED` | `1` |  |
| `AR_I2S_WS_SRC_EXTERNAL` | `0` |  |
| `AR_I2S_WS_SRC_INTERNAL` | `1` |  |

## Structs (1)


### `struct audioreach_module_priv_data`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `type` | `-` |
| `__le32` | `priv` | `2` |
| `__le32` | `data` | `0` |