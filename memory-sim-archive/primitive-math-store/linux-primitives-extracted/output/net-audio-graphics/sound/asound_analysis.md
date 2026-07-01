# asound.h

**Source:** `asound.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`
- `endian.h`
- `sys/ioctl.h`
- `stdlib.h`
- `time.h`

## Defines (356 total)


### SNDRV_CHMAP (3)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_CHMAP_POSITION_MASK` | `0xffff` |  |
| `SNDRV_CHMAP_PHASE_INVERSE` | `(0x01 << 16)` |  |
| `SNDRV_CHMAP_DRIVER_SPEC` | `(0x02 << 16)` |  |

### SNDRV_CTL (81)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_CTL_VERSION` | `SNDRV_PROTOCOL_VERSION(2, 0, 9)` |  |
| `SNDRV_CTL_ELEM_TYPE_NONE` | `((__force snd_ctl_elem_type_t) 0)` | invalid |
| `SNDRV_CTL_ELEM_TYPE_BOOLEAN` | `((__force snd_ctl_elem_type_t) 1)` | boolean type |
| `SNDRV_CTL_ELEM_TYPE_INTEGER` | `((__force snd_ctl_elem_type_t) 2)` | integer type |
| `SNDRV_CTL_ELEM_TYPE_ENUMERATED` | `((__force snd_ctl_elem_type_t) 3)` | enumerated type |
| `SNDRV_CTL_ELEM_TYPE_BYTES` | `((__force snd_ctl_elem_type_t) 4)` | byte array |
| `SNDRV_CTL_ELEM_TYPE_IEC958` | `((__force snd_ctl_elem_type_t) 5)` | IEC958 (S/PDIF) setup |
| `SNDRV_CTL_ELEM_TYPE_INTEGER64` | `((__force snd_ctl_elem_type_t) 6)` | 64-bit integer type |
| `SNDRV_CTL_ELEM_TYPE_LAST` | `SNDRV_CTL_ELEM_TYPE_INTEGER64` |  |
| `SNDRV_CTL_ELEM_IFACE_CARD` | `((__force snd_ctl_elem_iface_t) 0)` | global control |
| `SNDRV_CTL_ELEM_IFACE_HWDEP` | `((__force snd_ctl_elem_iface_t) 1)` | hardware dependent device |
| `SNDRV_CTL_ELEM_IFACE_MIXER` | `((__force snd_ctl_elem_iface_t) 2)` | virtual mixer device |
| `SNDRV_CTL_ELEM_IFACE_PCM` | `((__force snd_ctl_elem_iface_t) 3)` | PCM device |
| `SNDRV_CTL_ELEM_IFACE_RAWMIDI` | `((__force snd_ctl_elem_iface_t) 4)` | RawMidi device |
| `SNDRV_CTL_ELEM_IFACE_TIMER` | `((__force snd_ctl_elem_iface_t) 5)` | timer device |
| `SNDRV_CTL_ELEM_IFACE_SEQUENCER` | `((__force snd_ctl_elem_iface_t) 6)` | sequencer client |
| `SNDRV_CTL_ELEM_IFACE_LAST` | `SNDRV_CTL_ELEM_IFACE_SEQUENCER` |  |
| `SNDRV_CTL_ELEM_ACCESS_READ` | `(1<<0)` |  |
| `SNDRV_CTL_ELEM_ACCESS_WRITE` | `(1<<1)` |  |
| `SNDRV_CTL_ELEM_ACCESS_READWRITE` | `(SNDRV_CTL_ELEM_ACCESS_READ\|SNDRV_CTL_ELEM_ACCESS_WRITE)` |  |
| `SNDRV_CTL_ELEM_ACCESS_VOLATILE` | `(1<<2)` | control value may be changed without a notification |
| `SNDRV_CTL_ELEM_ACCESS_TLV_READ` | `(1<<4)` | TLV read is possible |
| `SNDRV_CTL_ELEM_ACCESS_TLV_WRITE` | `(1<<5)` | TLV write is possible |
| `SNDRV_CTL_ELEM_ACCESS_TLV_READWRITE` | `(SNDRV_CTL_ELEM_ACCESS_TLV_READ\|SNDRV_CTL_ELEM_ACCESS_TLV_W` |  |
| `SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND` | `(1<<6)` | TLV command is possible |
| `SNDRV_CTL_ELEM_ACCESS_INACTIVE` | `(1<<8)` | control does actually nothing, but may be updated |
| `SNDRV_CTL_ELEM_ACCESS_LOCK` | `(1<<9)` | write lock |
| `SNDRV_CTL_ELEM_ACCESS_OWNER` | `(1<<10)` | write lock owner |
| `SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK` | `(1<<28)` | kernel use a TLV callback |
| `SNDRV_CTL_ELEM_ACCESS_USER` | `(1<<29)` | user space element |
| `SNDRV_CTL_POWER_D0` | `0x0000` | full On |
| `SNDRV_CTL_POWER_D1` | `0x0100` | partial On |
| `SNDRV_CTL_POWER_D2` | `0x0200` | partial On |
| `SNDRV_CTL_POWER_D3` | `0x0300` | Off |
| `SNDRV_CTL_POWER_D3hot` | `(SNDRV_CTL_POWER_D3\|0x0000)` | Off, with power |
| `SNDRV_CTL_POWER_D3cold` | `(SNDRV_CTL_POWER_D3\|0x0001)` | Off, without power |
| `SNDRV_CTL_ELEM_ID_NAME_MAXLEN` | `44` |  |
| `SNDRV_CTL_IOCTL_PVERSION` | `_IOR('U', 0x00, int)` |  |
| `SNDRV_CTL_IOCTL_CARD_INFO` | `_IOR('U', 0x01, struct snd_ctl_card_info)` |  |
| `SNDRV_CTL_IOCTL_ELEM_LIST` | `_IOWR('U', 0x10, struct snd_ctl_elem_list)` |  |
| `SNDRV_CTL_IOCTL_ELEM_INFO` | `_IOWR('U', 0x11, struct snd_ctl_elem_info)` |  |
| `SNDRV_CTL_IOCTL_ELEM_READ` | `_IOWR('U', 0x12, struct snd_ctl_elem_value)` |  |
| `SNDRV_CTL_IOCTL_ELEM_WRITE` | `_IOWR('U', 0x13, struct snd_ctl_elem_value)` |  |
| `SNDRV_CTL_IOCTL_ELEM_LOCK` | `_IOW('U', 0x14, struct snd_ctl_elem_id)` |  |
| `SNDRV_CTL_IOCTL_ELEM_UNLOCK` | `_IOW('U', 0x15, struct snd_ctl_elem_id)` |  |
| `SNDRV_CTL_IOCTL_SUBSCRIBE_EVENTS` | `_IOWR('U', 0x16, int)` |  |
| `SNDRV_CTL_IOCTL_ELEM_ADD` | `_IOWR('U', 0x17, struct snd_ctl_elem_info)` |  |
| `SNDRV_CTL_IOCTL_ELEM_REPLACE` | `_IOWR('U', 0x18, struct snd_ctl_elem_info)` |  |
| `SNDRV_CTL_IOCTL_ELEM_REMOVE` | `_IOWR('U', 0x19, struct snd_ctl_elem_id)` |  |
| `SNDRV_CTL_IOCTL_TLV_READ` | `_IOWR('U', 0x1a, struct snd_ctl_tlv)` |  |

*...and 31 more*

### SNDRV_HWDEP (5)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_HWDEP_VERSION` | `SNDRV_PROTOCOL_VERSION(1, 0, 1)` |  |
| `SNDRV_HWDEP_IOCTL_PVERSION` | `_IOR ('H', 0x00, int)` |  |
| `SNDRV_HWDEP_IOCTL_INFO` | `_IOR ('H', 0x01, struct snd_hwdep_info)` |  |
| `SNDRV_HWDEP_IOCTL_DSP_STATUS` | `_IOR('H', 0x02, struct snd_hwdep_dsp_status)` |  |
| `SNDRV_HWDEP_IOCTL_DSP_LOAD` | `_IOW('H', 0x03, struct snd_hwdep_dsp_image)` |  |

### SNDRV_MASK (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_MASK_MAX` | `256` |  |

### SNDRV_PCM (180)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_PCM_VERSION` | `SNDRV_PROTOCOL_VERSION(2, 0, 18)` |  |
| `SNDRV_PCM_ACCESS_MMAP_INTERLEAVED` | `((__force snd_pcm_access_t) 0)` | interleaved mmap |
| `SNDRV_PCM_ACCESS_MMAP_NONINTERLEAVED` | `((__force snd_pcm_access_t) 1)` | noninterleaved mmap |
| `SNDRV_PCM_ACCESS_MMAP_COMPLEX` | `((__force snd_pcm_access_t) 2)` | complex mmap |
| `SNDRV_PCM_ACCESS_RW_INTERLEAVED` | `((__force snd_pcm_access_t) 3)` | readi/writei |
| `SNDRV_PCM_ACCESS_RW_NONINTERLEAVED` | `((__force snd_pcm_access_t) 4)` | readn/writen |
| `SNDRV_PCM_ACCESS_LAST` | `SNDRV_PCM_ACCESS_RW_NONINTERLEAVED` |  |
| `SNDRV_PCM_FORMAT_S8` | `((__force snd_pcm_format_t) 0)` |  |
| `SNDRV_PCM_FORMAT_U8` | `((__force snd_pcm_format_t) 1)` |  |
| `SNDRV_PCM_FORMAT_S16_LE` | `((__force snd_pcm_format_t) 2)` |  |
| `SNDRV_PCM_FORMAT_S16_BE` | `((__force snd_pcm_format_t) 3)` |  |
| `SNDRV_PCM_FORMAT_U16_LE` | `((__force snd_pcm_format_t) 4)` |  |
| `SNDRV_PCM_FORMAT_U16_BE` | `((__force snd_pcm_format_t) 5)` |  |
| `SNDRV_PCM_FORMAT_S24_LE` | `((__force snd_pcm_format_t) 6)` | low three bytes |
| `SNDRV_PCM_FORMAT_S24_BE` | `((__force snd_pcm_format_t) 7)` | low three bytes |
| `SNDRV_PCM_FORMAT_U24_LE` | `((__force snd_pcm_format_t) 8)` | low three bytes |
| `SNDRV_PCM_FORMAT_U24_BE` | `((__force snd_pcm_format_t) 9)` | low three bytes |
| `SNDRV_PCM_FORMAT_S32_LE` | `((__force snd_pcm_format_t) 10)` |  |
| `SNDRV_PCM_FORMAT_S32_BE` | `((__force snd_pcm_format_t) 11)` |  |
| `SNDRV_PCM_FORMAT_U32_LE` | `((__force snd_pcm_format_t) 12)` |  |
| `SNDRV_PCM_FORMAT_U32_BE` | `((__force snd_pcm_format_t) 13)` |  |
| `SNDRV_PCM_FORMAT_FLOAT_LE` | `((__force snd_pcm_format_t) 14)` | 4-byte float, IEEE-754 32-bit, range -1.0 to 1.0 |
| `SNDRV_PCM_FORMAT_FLOAT_BE` | `((__force snd_pcm_format_t) 15)` | 4-byte float, IEEE-754 32-bit, range -1.0 to 1.0 |
| `SNDRV_PCM_FORMAT_FLOAT64_LE` | `((__force snd_pcm_format_t) 16)` | 8-byte float, IEEE-754 64-bit, range -1.0 to 1.0 |
| `SNDRV_PCM_FORMAT_FLOAT64_BE` | `((__force snd_pcm_format_t) 17)` | 8-byte float, IEEE-754 64-bit, range -1.0 to 1.0 |
| `SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE` | `((__force snd_pcm_format_t) 18)` | IEC-958 subframe, Little Endian |
| `SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE` | `((__force snd_pcm_format_t) 19)` | IEC-958 subframe, Big Endian |
| `SNDRV_PCM_FORMAT_MU_LAW` | `((__force snd_pcm_format_t) 20)` |  |
| `SNDRV_PCM_FORMAT_A_LAW` | `((__force snd_pcm_format_t) 21)` |  |
| `SNDRV_PCM_FORMAT_IMA_ADPCM` | `((__force snd_pcm_format_t) 22)` |  |
| `SNDRV_PCM_FORMAT_MPEG` | `((__force snd_pcm_format_t) 23)` |  |
| `SNDRV_PCM_FORMAT_GSM` | `((__force snd_pcm_format_t) 24)` |  |
| `SNDRV_PCM_FORMAT_S20_LE` | `((__force snd_pcm_format_t) 25)` | in four bytes, LSB justified |
| `SNDRV_PCM_FORMAT_S20_BE` | `((__force snd_pcm_format_t) 26)` | in four bytes, LSB justified |
| `SNDRV_PCM_FORMAT_U20_LE` | `((__force snd_pcm_format_t) 27)` | in four bytes, LSB justified |
| `SNDRV_PCM_FORMAT_U20_BE` | `((__force snd_pcm_format_t) 28)` | in four bytes, LSB justified |
| `SNDRV_PCM_FORMAT_SPECIAL` | `((__force snd_pcm_format_t) 31)` |  |
| `SNDRV_PCM_FORMAT_S24_3LE` | `((__force snd_pcm_format_t) 32)` | in three bytes |
| `SNDRV_PCM_FORMAT_S24_3BE` | `((__force snd_pcm_format_t) 33)` | in three bytes |
| `SNDRV_PCM_FORMAT_U24_3LE` | `((__force snd_pcm_format_t) 34)` | in three bytes |
| `SNDRV_PCM_FORMAT_U24_3BE` | `((__force snd_pcm_format_t) 35)` | in three bytes |
| `SNDRV_PCM_FORMAT_S20_3LE` | `((__force snd_pcm_format_t) 36)` | in three bytes |
| `SNDRV_PCM_FORMAT_S20_3BE` | `((__force snd_pcm_format_t) 37)` | in three bytes |
| `SNDRV_PCM_FORMAT_U20_3LE` | `((__force snd_pcm_format_t) 38)` | in three bytes |
| `SNDRV_PCM_FORMAT_U20_3BE` | `((__force snd_pcm_format_t) 39)` | in three bytes |
| `SNDRV_PCM_FORMAT_S18_3LE` | `((__force snd_pcm_format_t) 40)` | in three bytes |
| `SNDRV_PCM_FORMAT_S18_3BE` | `((__force snd_pcm_format_t) 41)` | in three bytes |
| `SNDRV_PCM_FORMAT_U18_3LE` | `((__force snd_pcm_format_t) 42)` | in three bytes |
| `SNDRV_PCM_FORMAT_U18_3BE` | `((__force snd_pcm_format_t) 43)` | in three bytes |
| `SNDRV_PCM_FORMAT_G723_24` | `((__force snd_pcm_format_t) 44)` | 8 samples in 3 bytes |

*...and 130 more*

### SNDRV_RAWMIDI (25)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_RAWMIDI_VERSION` | `SNDRV_PROTOCOL_VERSION(2, 0, 5)` |  |
| `SNDRV_RAWMIDI_INFO_OUTPUT` | `0x00000001` |  |
| `SNDRV_RAWMIDI_INFO_INPUT` | `0x00000002` |  |
| `SNDRV_RAWMIDI_INFO_DUPLEX` | `0x00000004` |  |
| `SNDRV_RAWMIDI_INFO_UMP` | `0x00000008` |  |
| `SNDRV_RAWMIDI_INFO_STREAM_INACTIVE` | `0x00000010` |  |
| `SNDRV_RAWMIDI_DEVICE_UNKNOWN` | `0` |  |
| `SNDRV_RAWMIDI_MODE_FRAMING_MASK` | `(7<<0)` |  |
| `SNDRV_RAWMIDI_MODE_FRAMING_SHIFT` | `0` |  |
| `SNDRV_RAWMIDI_MODE_FRAMING_NONE` | `(0<<0)` |  |
| `SNDRV_RAWMIDI_MODE_FRAMING_TSTAMP` | `(1<<0)` |  |
| `SNDRV_RAWMIDI_MODE_CLOCK_MASK` | `(7<<3)` |  |
| `SNDRV_RAWMIDI_MODE_CLOCK_SHIFT` | `3` |  |
| `SNDRV_RAWMIDI_MODE_CLOCK_NONE` | `(0<<3)` |  |
| `SNDRV_RAWMIDI_MODE_CLOCK_REALTIME` | `(1<<3)` |  |
| `SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC` | `(2<<3)` |  |
| `SNDRV_RAWMIDI_MODE_CLOCK_MONOTONIC_RAW` | `(3<<3)` |  |
| `SNDRV_RAWMIDI_FRAMING_DATA_LENGTH` | `16` |  |
| `SNDRV_RAWMIDI_IOCTL_PVERSION` | `_IOR('W', 0x00, int)` |  |
| `SNDRV_RAWMIDI_IOCTL_INFO` | `_IOR('W', 0x01, struct snd_rawmidi_info)` |  |
| `SNDRV_RAWMIDI_IOCTL_USER_PVERSION` | `_IOW('W', 0x02, int)` |  |
| `SNDRV_RAWMIDI_IOCTL_PARAMS` | `_IOWR('W', 0x10, struct snd_rawmidi_params)` |  |
| `SNDRV_RAWMIDI_IOCTL_STATUS` | `_IOWR('W', 0x20, struct snd_rawmidi_status)` |  |
| `SNDRV_RAWMIDI_IOCTL_DROP` | `_IOW('W', 0x30, int)` |  |
| `SNDRV_RAWMIDI_IOCTL_DRAIN` | `_IOW('W', 0x31, int)` |  |

### SNDRV_TIMER (29)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_TIMER_VERSION` | `SNDRV_PROTOCOL_VERSION(2, 0, 8)` |  |
| `SNDRV_TIMER_GLOBAL_SYSTEM` | `0` |  |
| `SNDRV_TIMER_GLOBAL_RTC` | `1` | unused |
| `SNDRV_TIMER_GLOBAL_HPET` | `2` |  |
| `SNDRV_TIMER_GLOBAL_HRTIMER` | `3` |  |
| `SNDRV_TIMER_GLOBAL_UDRIVEN` | `4` |  |
| `SNDRV_TIMER_FLG_SLAVE` | `(1<<0)` | cannot be controlled |
| `SNDRV_TIMER_PSFLG_AUTO` | `(1<<0)` | auto start, otherwise one-shot |
| `SNDRV_TIMER_PSFLG_EXCLUSIVE` | `(1<<1)` | exclusive use, precise start/stop/pause/continue |
| `SNDRV_TIMER_PSFLG_EARLY_EVENT` | `(1<<2)` | write early event to the poll queue |
| `SNDRV_TIMER_IOCTL_PVERSION` | `_IOR('T', 0x00, int)` |  |
| `SNDRV_TIMER_IOCTL_NEXT_DEVICE` | `_IOWR('T', 0x01, struct snd_timer_id)` |  |
| `SNDRV_TIMER_IOCTL_TREAD_OLD` | `_IOW('T', 0x02, int)` |  |
| `SNDRV_TIMER_IOCTL_GINFO` | `_IOWR('T', 0x03, struct snd_timer_ginfo)` |  |
| `SNDRV_TIMER_IOCTL_GPARAMS` | `_IOW('T', 0x04, struct snd_timer_gparams)` |  |
| `SNDRV_TIMER_IOCTL_GSTATUS` | `_IOWR('T', 0x05, struct snd_timer_gstatus)` |  |
| `SNDRV_TIMER_IOCTL_SELECT` | `_IOW('T', 0x10, struct snd_timer_select)` |  |
| `SNDRV_TIMER_IOCTL_INFO` | `_IOR('T', 0x11, struct snd_timer_info)` |  |
| `SNDRV_TIMER_IOCTL_PARAMS` | `_IOW('T', 0x12, struct snd_timer_params)` |  |
| `SNDRV_TIMER_IOCTL_STATUS` | `_IOR('T', 0x14, struct snd_timer_status)` |  |
| `SNDRV_TIMER_IOCTL_START` | `_IO('T', 0xa0)` |  |
| `SNDRV_TIMER_IOCTL_STOP` | `_IO('T', 0xa1)` |  |
| `SNDRV_TIMER_IOCTL_CONTINUE` | `_IO('T', 0xa2)` |  |
| `SNDRV_TIMER_IOCTL_PAUSE` | `_IO('T', 0xa3)` |  |
| `SNDRV_TIMER_IOCTL_TREAD64` | `_IOW('T', 0xa4, int)` |  |
| `SNDRV_TIMER_IOCTL_CREATE` | `_IOWR('T', 0xa5, struct snd_timer_uinfo)` |  |
| `SNDRV_TIMER_IOCTL_TRIGGER` | `_IO('T', 0xa6)` |  |
| `SNDRV_TIMER_IOCTL_TREAD` | `SNDRV_TIMER_IOCTL_TREAD_OLD` |  |
| `SNDRV_TIMER_IOCTL_TREAD` | `((sizeof(__kernel_long_t) >= sizeof(time_t)) ? ` |  |

### SNDRV_UMP (20)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_UMP_EP_INFO_STATIC_BLOCKS` | `0x01` |  |
| `SNDRV_UMP_EP_INFO_PROTO_MIDI_MASK` | `0x0300` |  |
| `SNDRV_UMP_EP_INFO_PROTO_MIDI1` | `0x0100` | MIDI 1.0 |
| `SNDRV_UMP_EP_INFO_PROTO_MIDI2` | `0x0200` | MIDI 2.0 |
| `SNDRV_UMP_EP_INFO_PROTO_JRTS_MASK` | `0x0003` |  |
| `SNDRV_UMP_EP_INFO_PROTO_JRTS_TX` | `0x0001` | JRTS Transmit |
| `SNDRV_UMP_EP_INFO_PROTO_JRTS_RX` | `0x0002` | JRTS Receive |
| `SNDRV_UMP_DIR_INPUT` | `0x01` |  |
| `SNDRV_UMP_DIR_OUTPUT` | `0x02` |  |
| `SNDRV_UMP_DIR_BIDIRECTION` | `0x03` |  |
| `SNDRV_UMP_BLOCK_IS_MIDI1` | `(1U << 0)` | MIDI 1.0 port w/o restrict |
| `SNDRV_UMP_BLOCK_IS_LOWSPEED` | `(1U << 1)` | 31.25Kbps B/W MIDI1 port |
| `SNDRV_UMP_BLOCK_UI_HINT_UNKNOWN` | `0x00` |  |
| `SNDRV_UMP_BLOCK_UI_HINT_RECEIVER` | `0x01` |  |
| `SNDRV_UMP_BLOCK_UI_HINT_SENDER` | `0x02` |  |
| `SNDRV_UMP_BLOCK_UI_HINT_BOTH` | `0x03` |  |
| `SNDRV_UMP_MAX_GROUPS` | `16` |  |
| `SNDRV_UMP_MAX_BLOCKS` | `32` |  |
| `SNDRV_UMP_IOCTL_ENDPOINT_INFO` | `_IOR('W', 0x40, struct snd_ump_endpoint_info)` |  |
| `SNDRV_UMP_IOCTL_BLOCK_INFO` | `_IOR('W', 0x41, struct snd_ump_block_info)` |  |

### UNCATEGORIZED (12)

| Name | Value | Comment |
|------|-------|---------|
| `AES_IEC958_STATUS_SIZE` | `24` |  |
| `__snd_pcm_mmap_status64` | `snd_pcm_mmap_status` |  |
| `__snd_pcm_mmap_control64` | `snd_pcm_mmap_control` |  |
| `__snd_pcm_sync_ptr64` | `snd_pcm_sync_ptr` |  |
| `__snd_timespec64` | `__kernel_timespec` |  |
| `__snd_timespec64` | `timespec` |  |
| `__snd_pcm_mmap_status` | `snd_pcm_mmap_status` |  |
| `__snd_pcm_mmap_control` | `snd_pcm_mmap_control` |  |
| `__snd_pcm_sync_ptr` | `snd_pcm_sync_ptr` |  |
| `__snd_timespec` | `timespec` |  |
| `__SNDRV_PCM_IOCTL_SYNC_PTR` | `_IOWR('A', 0x23, struct __snd_pcm_sync_ptr)` |  |
| `__SNDRV_PCM_IOCTL_SYNC_PTR64` | `_IOWR('A', 0x23, struct __snd_pcm_sync_ptr64)` |  |

## Structs (51)


### `struct snd_aes_iec958`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_cea_861_aud_if`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_hwdep_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `card` | `-` |
| `int` | `iface` | `-` |

### `struct snd_hwdep_dsp_status`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_hwdep_dsp_image`

| Type | Field | Array |
|------|-------|-------|
| `size_t` | `length` | `-` |

### `struct snd_pcm_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `stream` | `-` |
| `int` | `card` | `-` |
| `int` | `dev_class` | `-` |
| `int` | `dev_subclass` | `-` |

### `struct snd_interval`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_mask`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bits` | `(SNDRV_MASK_MAX+31)/32` |

### `struct snd_pcm_hw_params`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_uframes_t` | `fifo_size` | `-` |

### `struct snd_pcm_sw_params`

| Type | Field | Array |
|------|-------|-------|
| `int` | `tstamp_mode` | `-` |
| `snd_pcm_uframes_t` | `avail_min` | `-` |
| `snd_pcm_uframes_t` | `xfer_align` | `-` |
| `snd_pcm_uframes_t` | `start_threshold` | `-` |
| `snd_pcm_uframes_t` | `stop_threshold` | `-` |
| `snd_pcm_uframes_t` | `silence_threshold` | `-` |
| `snd_pcm_uframes_t` | `silence_size` | `-` |
| `snd_pcm_uframes_t` | `boundary` | `-` |

### `struct snd_pcm_channel_info`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_off_t` | `offset` | `-` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_pcm_status`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_state_t` | `state` | `-` |
| `__time_pad` | `pad1` | `-` |
| `snd_pcm_uframes_t` | `appl_ptr` | `-` |
| `snd_pcm_uframes_t` | `hw_ptr` | `-` |
| `snd_pcm_sframes_t` | `delay` | `-` |
| `snd_pcm_uframes_t` | `avail` | `-` |
| `snd_pcm_uframes_t` | `avail_max` | `-` |
| `snd_pcm_uframes_t` | `overrange` | `-` |
| `snd_pcm_state_t` | `suspended_state` | `-` |
| `__u32` | `audio_tstamp_data` | `-` |
| `__u32` | `audio_tstamp_accuracy` | `-` |

### `struct __snd_timespec`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `tv_sec` | `-` |
| `__s32` | `tv_nsec` | `-` |

### `struct __snd_timespec64`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `tv_sec` | `-` |
| `__s64` | `tv_nsec` | `-` |

### `struct __snd_pcm_mmap_status`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_state_t` | `state` | `-` |
| `int` | `pad1` | `-` |
| `snd_pcm_uframes_t` | `hw_ptr` | `-` |
| `snd_pcm_state_t` | `suspended_state` | `-` |

### `struct __snd_pcm_mmap_control`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_uframes_t` | `appl_ptr` | `-` |
| `snd_pcm_uframes_t` | `avail_min` | `-` |

### `struct __snd_pcm_sync_ptr`

| Type | Field | Array |
|------|-------|-------|

### `struct __snd_pcm_mmap_status64`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_state_t` | `state` | `-` |
| `__u32` | `pad1` | `-` |
| `__pad_before_uframe` | `__pad1` | `-` |
| `snd_pcm_uframes_t` | `hw_ptr` | `-` |
| `__pad_after_uframe` | `__pad2` | `-` |
| `snd_pcm_state_t` | `suspended_state` | `-` |
| `__u32` | `pad3` | `-` |

### `struct __snd_pcm_mmap_control64`

| Type | Field | Array |
|------|-------|-------|
| `__pad_before_uframe` | `__pad1` | `-` |
| `snd_pcm_uframes_t` | `appl_ptr` | `-` |
| `__pad_before_uframe` | `__pad2` | `-` |
| `__pad_before_uframe` | `__pad3` | `-` |
| `snd_pcm_uframes_t` | `avail_min` | `-` |
| `__pad_after_uframe` | `__pad4` | `-` |

### `struct __snd_pcm_sync_ptr64`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `pad1` | `-` |

### `struct snd_xferi`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_sframes_t` | `result` | `-` |
| `snd_pcm_uframes_t` | `frames` | `-` |

### `struct snd_xfern`

| Type | Field | Array |
|------|-------|-------|
| `snd_pcm_sframes_t` | `result` | `-` |
| `snd_pcm_uframes_t` | `frames` | `-` |

### `struct snd_rawmidi_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `stream` | `-` |
| `int` | `card` | `-` |
| `int` | `tied_device` | `-` |

### `struct snd_rawmidi_framing_tstamp`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `frame_type` | `-` |
| `__u8` | `length` | `-` |
| `__u8` | `reserved` | `2` |
| `__u32` | `tv_nsec` | `-` |
| `__u64` | `tv_sec` | `-` |
| `__u8` | `data` | `SNDRV_RAWMIDI_FRAMING_DATA_LENGTH` |

### `struct snd_rawmidi_params`

| Type | Field | Array |
|------|-------|-------|
| `int` | `stream` | `-` |
| `size_t` | `buffer_size` | `-` |
| `size_t` | `avail_min` | `-` |

### `struct snd_rawmidi_status`

| Type | Field | Array |
|------|-------|-------|
| `int` | `stream` | `-` |
| `__time_pad` | `pad1` | `-` |
| `size_t` | `avail` | `-` |
| `size_t` | `xruns` | `-` |

### `struct snd_ump_endpoint_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `card` | `-` |
| `int` | `device` | `-` |

### `struct snd_ump_block_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `card` | `-` |
| `int` | `device` | `-` |

### `struct snd_timer_id`

| Type | Field | Array |
|------|-------|-------|
| `int` | `dev_class` | `-` |
| `int` | `dev_sclass` | `-` |
| `int` | `card` | `-` |
| `int` | `device` | `-` |
| `int` | `subdevice` | `-` |

### `struct snd_timer_ginfo`

| Type | Field | Array |
|------|-------|-------|
| `int` | `card` | `-` |

### `struct snd_timer_gparams`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_timer_gstatus`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_timer_select`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_timer_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `card` | `-` |

### `struct snd_timer_params`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_timer_status`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_timer_uinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `resolution` | `-` |
| `int` | `fd` | `-` |

### `struct snd_timer_read`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_timer_tread`

| Type | Field | Array |
|------|-------|-------|
| `int` | `event` | `-` |
| `__time_pad` | `pad1` | `-` |
| `__time_pad` | `pad2` | `-` |

### `struct snd_ctl_card_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `card` | `-` |
| `int` | `pad` | `-` |

### `struct snd_ctl_elem_id`

| Type | Field | Array |
|------|-------|-------|
| `snd_ctl_elem_iface_t` | `iface` | `-` |

### `struct snd_ctl_elem_list`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_ctl_elem_info`

| Type | Field | Array |
|------|-------|-------|
| `snd_ctl_elem_type_t` | `type` | `-` |
| `__kernel_pid_t` | `owner` | `-` |
| `long` | `min` | `-` |
| `long` | `max` | `-` |
| `long` | `step` | `-` |
| `char` | `name` | `64` |
| `__u64` | `names_ptr` | `-` |

### `struct anonymous_44`

| Type | Field | Array |
|------|-------|-------|
| `long` | `min` | `-` |
| `long` | `max` | `-` |
| `long` | `step` | `-` |

### `struct anonymous_45`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_46`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `64` |
| `__u64` | `names_ptr` | `-` |

### `struct snd_ctl_elem_value`

| Type | Field | Array |
|------|-------|-------|
| `long` | `value` | `128` |

### `struct snd_ctl_tlv`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_ctl_event`

| Type | Field | Array |
|------|-------|-------|
| `int` | `type` | `-` |

### `struct anonymous_50`

| Type | Field | Array |
|------|-------|-------|