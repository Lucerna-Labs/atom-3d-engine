# input.h

**Source:** `input.h`


## Includes

- `sys/time.h`
- `sys/ioctl.h`
- `sys/types.h`
- `linux/types.h`
- `input-event-codes.h`

## Defines (82 total)


### BUS_AMD (1)

| Name | Value | Comment |
|------|-------|---------|
| `BUS_AMD_SFH` | `0x20` |  |

### BUS_INTEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `BUS_INTEL_ISHTP` | `0x1F` |  |

### FF_EFFECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `FF_EFFECT_MIN` | `FF_HAPTIC` |  |
| `FF_EFFECT_MAX` | `FF_RAMP` |  |

### FF_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `FF_MAX_EFFECTS` | `FF_GAIN` |  |

### FF_SAW (2)

| Name | Value | Comment |
|------|-------|---------|
| `FF_SAW_UP` | `0x5b` |  |
| `FF_SAW_DOWN` | `0x5c` |  |

### FF_STATUS (3)

| Name | Value | Comment |
|------|-------|---------|
| `FF_STATUS_STOPPED` | `0x00` |  |
| `FF_STATUS_PLAYING` | `0x01` |  |
| `FF_STATUS_MAX` | `0x01` |  |

### FF_WAVEFORM (2)

| Name | Value | Comment |
|------|-------|---------|
| `FF_WAVEFORM_MIN` | `FF_SQUARE` |  |
| `FF_WAVEFORM_MAX` | `FF_CUSTOM` |  |

### INPUT_KEYMAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `INPUT_KEYMAP_BY_INDEX` | `(1 << 0)` |  |

### MT_TOOL (5)

| Name | Value | Comment |
|------|-------|---------|
| `MT_TOOL_FINGER` | `0x00` |  |
| `MT_TOOL_PEN` | `0x01` |  |
| `MT_TOOL_PALM` | `0x02` |  |
| `MT_TOOL_DIAL` | `0x0a` |  |
| `MT_TOOL_MAX` | `0x0f` |  |

### UNCATEGORIZED (64)

| Name | Value | Comment |
|------|-------|---------|
| `input_event_sec` | `time.tv_sec` |  |
| `input_event_usec` | `time.tv_usec` |  |
| `input_event_sec` | `__sec` |  |
| `input_event_usec` | `__usec` |  |
| `EV_VERSION` | `0x010001` |  |
| `EVIOCGVERSION` | `_IOR('E', 0x01, int)` | get driver version |
| `EVIOCGID` | `_IOR('E', 0x02, struct input_id)` | get device ID |
| `EVIOCGREP` | `_IOR('E', 0x03, unsigned int[2])` | get repeat settings |
| `EVIOCSREP` | `_IOW('E', 0x03, unsigned int[2])` | set repeat settings |
| `EVIOCGKEYCODE` | `_IOR('E', 0x04, unsigned int[2])` | get keycode |
| `EVIOCGKEYCODE_V2` | `_IOR('E', 0x04, struct input_keymap_entry)` |  |
| `EVIOCSKEYCODE` | `_IOW('E', 0x04, unsigned int[2])` | set keycode |
| `EVIOCSKEYCODE_V2` | `_IOW('E', 0x04, struct input_keymap_entry)` |  |
| `EVIOCSFF` | `_IOW('E', 0x80, struct ff_effect)` | send a force effect to a force feedback device |
| `EVIOCRMFF` | `_IOW('E', 0x81, int)` | Erase a force effect |
| `EVIOCGEFFECTS` | `_IOR('E', 0x84, int)` | Report number of effects playable at the same time |
| `EVIOCGRAB` | `_IOW('E', 0x90, int)` | Grab/Release device |
| `EVIOCREVOKE` | `_IOW('E', 0x91, int)` | Revoke device access |
| `EVIOCGMASK` | `_IOR('E', 0x92, struct input_mask)` | Get event-masks |
| `EVIOCSMASK` | `_IOW('E', 0x93, struct input_mask)` | Set event-masks |
| `EVIOCSCLOCKID` | `_IOW('E', 0xa0, int)` | Set clockid to be used for timestamps |
| `ID_BUS` | `0` |  |
| `ID_VENDOR` | `1` |  |
| `ID_PRODUCT` | `2` |  |
| `ID_VERSION` | `3` |  |
| `BUS_PCI` | `0x01` |  |
| `BUS_ISAPNP` | `0x02` |  |
| `BUS_USB` | `0x03` |  |
| `BUS_HIL` | `0x04` |  |
| `BUS_BLUETOOTH` | `0x05` |  |
| `BUS_VIRTUAL` | `0x06` |  |
| `BUS_ISA` | `0x10` |  |
| `BUS_I8042` | `0x11` |  |
| `BUS_XTKBD` | `0x12` |  |
| `BUS_RS232` | `0x13` |  |
| `BUS_GAMEPORT` | `0x14` |  |
| `BUS_PARPORT` | `0x15` |  |
| `BUS_AMIGA` | `0x16` |  |
| `BUS_ADB` | `0x17` |  |
| `BUS_I2C` | `0x18` |  |
| `BUS_HOST` | `0x19` |  |
| `BUS_GSC` | `0x1A` |  |
| `BUS_ATARI` | `0x1B` |  |
| `BUS_SPI` | `0x1C` |  |
| `BUS_RMI` | `0x1D` |  |
| `BUS_CEC` | `0x1E` |  |
| `BUS_SDW` | `0x21` |  |
| `FF_HAPTIC` | `0x4f` |  |
| `FF_RUMBLE` | `0x50` |  |
| `FF_PERIODIC` | `0x51` |  |

*...and 14 more*

## Structs (16)


### `struct input_event`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_ulong_t` | `__sec` | `-` |
| `__kernel_ulong_t` | `__usec` | `-` |
| `__u16` | `type` | `-` |
| `__u16` | `code` | `-` |
| `__s32` | `value` | `-` |

### `struct input_id`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `bustype` | `-` |
| `__u16` | `vendor` | `-` |
| `__u16` | `product` | `-` |
| `__u16` | `version` | `-` |

### `struct input_absinfo`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `value` | `-` |
| `__s32` | `minimum` | `-` |
| `__s32` | `maximum` | `-` |
| `__s32` | `fuzz` | `-` |
| `__s32` | `flat` | `-` |
| `__s32` | `resolution` | `-` |

### `struct input_keymap_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `len` | `-` |
| `__u16` | `index` | `-` |
| `__u32` | `keycode` | `-` |
| `__u8` | `scancode` | `32` |

### `struct input_mask`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `codes_size` | `-` |
| `__u64` | `codes_ptr` | `-` |

### `struct input_mt_request_layout`

| Type | Field | Array |
|------|-------|-------|

### `struct ff_replay`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `length` | `-` |
| `__u16` | `delay` | `-` |

### `struct ff_trigger`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `button` | `-` |
| `__u16` | `interval` | `-` |

### `struct ff_envelope`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `attack_length` | `-` |
| `__u16` | `attack_level` | `-` |
| `__u16` | `fade_length` | `-` |
| `__u16` | `fade_level` | `-` |

### `struct ff_constant_effect`

| Type | Field | Array |
|------|-------|-------|
| `__s16` | `level` | `-` |

### `struct ff_ramp_effect`

| Type | Field | Array |
|------|-------|-------|
| `__s16` | `start_level` | `-` |
| `__s16` | `end_level` | `-` |

### `struct ff_condition_effect`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `right_saturation` | `-` |
| `__u16` | `left_saturation` | `-` |
| `__s16` | `right_coeff` | `-` |
| `__s16` | `left_coeff` | `-` |
| `__u16` | `deadband` | `-` |
| `__s16` | `center` | `-` |

### `struct ff_periodic_effect`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `waveform` | `-` |
| `__u16` | `period` | `-` |
| `__s16` | `magnitude` | `-` |
| `__s16` | `offset` | `-` |
| `__u16` | `phase` | `-` |
| `__u32` | `custom_len` | `-` |

### `struct ff_rumble_effect`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `strong_magnitude` | `-` |
| `__u16` | `weak_magnitude` | `-` |

### `struct ff_haptic_effect`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `hid_usage` | `-` |
| `__u16` | `vendor_id` | `-` |
| `__u8` | `vendor_waveform_page` | `-` |
| `__u16` | `intensity` | `-` |
| `__u16` | `repeat_count` | `-` |
| `__u16` | `retrigger_period` | `-` |

### `struct ff_effect`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `type` | `-` |
| `__s16` | `id` | `-` |
| `__u16` | `direction` | `-` |