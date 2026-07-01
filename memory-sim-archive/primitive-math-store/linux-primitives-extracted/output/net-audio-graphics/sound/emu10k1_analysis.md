# emu10k1.h

**Source:** `emu10k1.h`


## Includes

- `linux/types.h`

## Defines (224 total)


### A_C (22)

| Name | Value | Comment |
|------|-------|---------|
| `A_C_00000000` | `0xc0` |  |
| `A_C_00000001` | `0xc1` |  |
| `A_C_00000002` | `0xc2` |  |
| `A_C_00000003` | `0xc3` |  |
| `A_C_00000004` | `0xc4` |  |
| `A_C_00000008` | `0xc5` |  |
| `A_C_00000010` | `0xc6` |  |
| `A_C_00000020` | `0xc7` |  |
| `A_C_00000100` | `0xc8` |  |
| `A_C_00010000` | `0xc9` |  |
| `A_C_00000800` | `0xca` |  |
| `A_C_10000000` | `0xcb` |  |
| `A_C_20000000` | `0xcc` |  |
| `A_C_40000000` | `0xcd` |  |
| `A_C_80000000` | `0xce` |  |
| `A_C_7fffffff` | `0xcf` |  |
| `A_C_ffffffff` | `0xd0` |  |
| `A_C_fffffffe` | `0xd1` |  |
| `A_C_c0000000` | `0xd2` |  |
| `A_C_4f1bbcdc` | `0xd3` |  |
| `A_C_5a7ef9db` | `0xd4` |  |
| `A_C_00100000` | `0xd5` |  |

### A_CC (6)

| Name | Value | Comment |
|------|-------|---------|
| `A_CC_REG_NORMALIZED` | `A_C_00000001` |  |
| `A_CC_REG_BORROW` | `A_C_00000002` |  |
| `A_CC_REG_MINUS` | `A_C_00000004` |  |
| `A_CC_REG_ZERO` | `A_C_00000008` |  |
| `A_CC_REG_SATURATE` | `A_C_00000010` |  |
| `A_CC_REG_NONZERO` | `A_C_00000100` |  |

### A_DBG (7)

| Name | Value | Comment |
|------|-------|---------|
| `A_DBG_ZC` | `0x40000000` | zero tram counter |
| `A_DBG_SATURATION_OCCURED` | `0x20000000` |  |
| `A_DBG_SATURATION_ADDR` | `0x0ffc0000` |  |
| `A_DBG_SINGLE_STEP` | `0x00020000` | Set to zero to start dsp |
| `A_DBG_STEP` | `0x00010000` |  |
| `A_DBG_CONDITION_CODE` | `0x0000f800` |  |
| `A_DBG_STEP_ADDR` | `0x000003ff` |  |

### A_EXTIN (12)

| Name | Value | Comment |
|------|-------|---------|
| `A_EXTIN_AC97_L` | `0x00` | AC'97 capture channel - left |
| `A_EXTIN_AC97_R` | `0x01` | AC'97 capture channel - right |
| `A_EXTIN_SPDIF_CD_L` | `0x02` | digital CD left |
| `A_EXTIN_SPDIF_CD_R` | `0x03` | digital CD left |
| `A_EXTIN_OPT_SPDIF_L` | `0x04` | audigy drive Optical SPDIF - left |
| `A_EXTIN_OPT_SPDIF_R` | `0x05` | right |
| `A_EXTIN_LINE2_L` | `0x08` | audigy drive line2/mic2 - left |
| `A_EXTIN_LINE2_R` | `0x09` | right |
| `A_EXTIN_ADC_L` | `0x0a` | Philips ADC - left |
| `A_EXTIN_ADC_R` | `0x0b` | right |
| `A_EXTIN_AUX2_L` | `0x0c` | audigy drive aux2 - left |
| `A_EXTIN_AUX2_R` | `0x0d` | - right |

### A_EXTOUT (21)

| Name | Value | Comment |
|------|-------|---------|
| `A_EXTOUT_FRONT_L` | `0x00` | digital front left |
| `A_EXTOUT_FRONT_R` | `0x01` | right |
| `A_EXTOUT_CENTER` | `0x02` | digital front center |
| `A_EXTOUT_LFE` | `0x03` | digital front lfe |
| `A_EXTOUT_HEADPHONE_L` | `0x04` | headphone audigy drive left |
| `A_EXTOUT_HEADPHONE_R` | `0x05` | right |
| `A_EXTOUT_REAR_L` | `0x06` | digital rear left |
| `A_EXTOUT_REAR_R` | `0x07` | right |
| `A_EXTOUT_AFRONT_L` | `0x08` | analog front left |
| `A_EXTOUT_AFRONT_R` | `0x09` | right |
| `A_EXTOUT_ACENTER` | `0x0a` | analog center |
| `A_EXTOUT_ALFE` | `0x0b` | analog LFE |
| `A_EXTOUT_ASIDE_L` | `0x0c` | analog side left  - Audigy 2 ZS |
| `A_EXTOUT_ASIDE_R` | `0x0d` | right - Audigy 2 ZS |
| `A_EXTOUT_AREAR_L` | `0x0e` | analog rear left |
| `A_EXTOUT_AREAR_R` | `0x0f` | right |
| `A_EXTOUT_AC97_L` | `0x10` | AC97 left (front) |
| `A_EXTOUT_AC97_R` | `0x11` | right |
| `A_EXTOUT_ADC_CAP_L` | `0x16` | ADC capture buffer left |
| `A_EXTOUT_ADC_CAP_R` | `0x17` | right |
| `A_EXTOUT_MIC_CAP` | `0x18` | Mic capture buffer |

### A_GPR (7)

| Name | Value | Comment |
|------|-------|---------|
| `A_GPR_ACCU` | `0xd6` | ACCUM, accumulator |
| `A_GPR_COND` | `0xd7` | CCR, condition register |
| `A_GPR_NOISE0` | `0xd8` | noise source |
| `A_GPR_NOISE1` | `0xd9` | noise source |
| `A_GPR_IRQ` | `0xda` | IRQ register |
| `A_GPR_DBAC` | `0xdb` | TRAM Delay Base Address Counter - internal |
| `A_GPR_DBACE` | `0xde` | TRAM Delay Base Address Counter - external |

### A_HIWORD (3)

| Name | Value | Comment |
|------|-------|---------|
| `A_HIWORD_OPCODE_MASK` | `0x0f000000` |  |
| `A_HIWORD_RESULT_MASK` | `0x007ff000` |  |
| `A_HIWORD_OPA_MASK` | `0x000007ff` |  |

### A_LOWORD (2)

| Name | Value | Comment |
|------|-------|---------|
| `A_LOWORD_OPX_MASK` | `0x007ff000` |  |
| `A_LOWORD_OPY_MASK` | `0x000007ff` |  |

### A_TANKMEMCTLREG (1)

| Name | Value | Comment |
|------|-------|---------|
| `A_TANKMEMCTLREG_MASK` | `0x1f` | only 5 bits used - only for Audigy |

### CC_REG (6)

| Name | Value | Comment |
|------|-------|---------|
| `CC_REG_NORMALIZED` | `C_00000001` |  |
| `CC_REG_BORROW` | `C_00000002` |  |
| `CC_REG_MINUS` | `C_00000004` |  |
| `CC_REG_ZERO` | `C_00000008` |  |
| `CC_REG_SATURATE` | `C_00000010` |  |
| `CC_REG_NONZERO` | `C_00000100` |  |

### EXTIN_COAX (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTIN_COAX_SPDIF_L` | `0x0a` | LiveDrive - Coaxial S/PDIF - left |
| `EXTIN_COAX_SPDIF_R` | `0x0b` | LiveDrive - Coaxial S/PDIF - right |

### EXTIN_SPDIF (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTIN_SPDIF_CD_L` | `0x02` | internal S/PDIF CD - onboard - left |
| `EXTIN_SPDIF_CD_R` | `0x03` | internal S/PDIF CD - onboard - right |

### EXTIN_TOSLINK (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTIN_TOSLINK_L` | `0x06` | LiveDrive - TOSLink Optical - left |
| `EXTIN_TOSLINK_R` | `0x07` | LiveDrive - TOSLink Optical - right |

### EXTIN_ZOOM (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTIN_ZOOM_L` | `0x04` | Zoom Video I2S - left |
| `EXTIN_ZOOM_R` | `0x05` | Zoom Video I2S - right |

### EXTOUT_ADC (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTOUT_ADC_CAP_L` | `0x0a` | ADC Capture buffer - left |
| `EXTOUT_ADC_CAP_R` | `0x0b` | ADC Capture buffer - right |

### EXTOUT_HEADPHONE (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTOUT_HEADPHONE_L` | `0x06` | LiveDrive - Headphone - left |
| `EXTOUT_HEADPHONE_R` | `0x07` | LiveDrive - Headphone - right |

### EXTOUT_MIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `EXTOUT_MIC_CAP` | `0x0c` | MIC Capture buffer |

### EXTOUT_REAR (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTOUT_REAR_L` | `0x08` | Rear channel - left |
| `EXTOUT_REAR_R` | `0x09` | Rear channel - right |

### EXTOUT_TOSLINK (2)

| Name | Value | Comment |
|------|-------|---------|
| `EXTOUT_TOSLINK_L` | `0x02` | LiveDrive - TOSLink Optical - left |
| `EXTOUT_TOSLINK_R` | `0x03` | LiveDrive - TOSLink Optical - right |

### FXBUS_MIDI (4)

| Name | Value | Comment |
|------|-------|---------|
| `FXBUS_MIDI_LEFT` | `0x04` |  |
| `FXBUS_MIDI_RIGHT` | `0x05` |  |
| `FXBUS_MIDI_REVERB` | `0x0c` |  |
| `FXBUS_MIDI_CHORUS` | `0x0d` |  |

### FXBUS_PCM (10)

| Name | Value | Comment |
|------|-------|---------|
| `FXBUS_PCM_LEFT` | `0x00` |  |
| `FXBUS_PCM_RIGHT` | `0x01` |  |
| `FXBUS_PCM_LEFT_REAR` | `0x02` |  |
| `FXBUS_PCM_RIGHT_REAR` | `0x03` |  |
| `FXBUS_PCM_CENTER` | `0x06` |  |
| `FXBUS_PCM_LFE` | `0x07` |  |
| `FXBUS_PCM_LEFT_FRONT` | `0x08` |  |
| `FXBUS_PCM_RIGHT_FRONT` | `0x09` |  |
| `FXBUS_PCM_LEFT_SIDE` | `0x0e` |  |
| `FXBUS_PCM_RIGHT_SIDE` | `0x0f` |  |

### FXBUS_PT (2)

| Name | Value | Comment |
|------|-------|---------|
| `FXBUS_PT_LEFT` | `0x14` |  |
| `FXBUS_PT_RIGHT` | `0x15` |  |

### HIWORD_OPA (1)

| Name | Value | Comment |
|------|-------|---------|
| `HIWORD_OPA_MASK` | `0x000003ff` | Instruction operand A |

### HIWORD_OPCODE (1)

| Name | Value | Comment |
|------|-------|---------|
| `HIWORD_OPCODE_MASK` | `0x00f00000` | Instruction opcode |

### HIWORD_RESULT (1)

| Name | Value | Comment |
|------|-------|---------|
| `HIWORD_RESULT_MASK` | `0x000ffc00` | Instruction result |

### LOWORD_OPX (1)

| Name | Value | Comment |
|------|-------|---------|
| `LOWORD_OPX_MASK` | `0x000ffc00` | Instruction operand X |

### LOWORD_OPY (1)

| Name | Value | Comment |
|------|-------|---------|
| `LOWORD_OPY_MASK` | `0x000003ff` | Instruction operand Y |

### TANKMEMADDRREG_ADDR (1)

| Name | Value | Comment |
|------|-------|---------|
| `TANKMEMADDRREG_ADDR_MASK` | `0x000fffff` | 20 bit tank address field |

### UNCATEGORIZED (98)

| Name | Value | Comment |
|------|-------|---------|
| `EMU10K1_FX8010_PCM_COUNT` | `8` |  |
| `iMAC0` | `0x00` | R = A + (X * Y >> 31)   ; saturation |
| `iMAC1` | `0x01` | R = A + (-X * Y >> 31)  ; saturation |
| `iMAC2` | `0x02` | R = A + (X * Y >> 31)   ; wraparound |
| `iMAC3` | `0x03` | R = A + (-X * Y >> 31)  ; wraparound |
| `iMACINT0` | `0x04` | R = A + X * Y	   ; saturation |
| `iMACINT1` | `0x05` | R = A + X * Y	   ; wraparound (31-bit) |
| `iACC3` | `0x06` | R = A + X + Y	   ; saturation |
| `iMACMV` | `0x07` | R = A, acc += X * Y >> 31 |
| `iANDXOR` | `0x08` | R = (A & X) ^ Y |
| `iTSTNEG` | `0x09` | R = (A >= Y) ? X : ~X |
| `iLIMITGE` | `0x0a` | R = (A >= Y) ? X : Y |
| `iLIMITLT` | `0x0b` | R = (A < Y) ? X : Y |
| `iLOG` | `0x0c` | R = linear_data, A (log_data), X (max_exp), Y (format_word) |
| `iEXP` | `0x0d` | R = log_data, A (linear_data), X (max_exp), Y (format_word) |
| `iINTERP` | `0x0e` | R = A + (X * (Y - A) >> 31)  ; saturation |
| `iSKIP` | `0x0f` | R = A (cc_reg), X (count), Y (cc_test) |
| `C_00000000` | `0x40` |  |
| `C_00000001` | `0x41` |  |
| `C_00000002` | `0x42` |  |
| `C_00000003` | `0x43` |  |
| `C_00000004` | `0x44` |  |
| `C_00000008` | `0x45` |  |
| `C_00000010` | `0x46` |  |
| `C_00000020` | `0x47` |  |
| `C_00000100` | `0x48` |  |
| `C_00010000` | `0x49` |  |
| `C_00080000` | `0x4a` |  |
| `C_10000000` | `0x4b` |  |
| `C_20000000` | `0x4c` |  |
| `C_40000000` | `0x4d` |  |
| `C_80000000` | `0x4e` |  |
| `C_7fffffff` | `0x4f` |  |
| `C_ffffffff` | `0x50` |  |
| `C_fffffffe` | `0x51` |  |
| `C_c0000000` | `0x52` |  |
| `C_4f1bbcdc` | `0x53` |  |
| `C_5a7ef9db` | `0x54` |  |
| `C_00100000` | `0x55` | ?? |
| `GPR_ACCU` | `0x56` | ACCUM, accumulator |
| `GPR_COND` | `0x57` | CCR, condition register |
| `GPR_NOISE0` | `0x58` | noise source |
| `GPR_NOISE1` | `0x59` | noise source |
| `GPR_IRQ` | `0x5a` | IRQ register |
| `GPR_DBAC` | `0x5b` | TRAM Delay Base Address Counter |
| `FXGPREGBASE` | `0x100` | FX general purpose registers base |
| `A_FXGPREGBASE` | `0x400` | Audigy GPRs, 0x400 to 0x5ff |
| `A_TANKMEMCTLREGBASE` | `0x100` | Tank memory control registers base - only for Audigy |
| `TANKMEMDATAREGBASE` | `0x200` | Tank memory data registers base |
| `TANKMEMDATAREG_MASK` | `0x000fffff` | 20 bit tank audio data field |

*...and 48 more*

## Structs (7)


### `struct snd_emu10k1_fx8010_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `fxbus_names` | `16][32` |
| `char` | `extin_names` | `16][32` |
| `char` | `extout_names` | `32][32` |

### `struct emu10k1_ctl_elem_id`

| Type | Field | Array |
|------|-------|-------|
| `int` | `iface` | `-` |

### `struct snd_emu10k1_fx8010_control_gpr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `value` | `32` |
| `int` | `min` | `-` |
| `int` | `max` | `-` |

### `struct snd_emu10k1_fx8010_control_old_gpr`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_emu10k1_fx8010_code`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `128` |

### `struct snd_emu10k1_fx8010_tram`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_emu10k1_fx8010_pcm_rec`

| Type | Field | Array |
|------|-------|-------|