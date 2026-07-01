# joystick.h

**Source:** `joystick.h`


## Includes

- `linux/types.h`
- `linux/input.h`

## Defines (34 total)


### JS_CORR (2)

| Name | Value | Comment |
|------|-------|---------|
| `JS_CORR_NONE` | `0x00` | returns raw values |
| `JS_CORR_BROKEN` | `0x01` | broken line |

### JS_DEF (3)

| Name | Value | Comment |
|------|-------|---------|
| `JS_DEF_TIMEOUT` | `0x1300` |  |
| `JS_DEF_CORR` | `0` |  |
| `JS_DEF_TIMELIMIT` | `10L` |  |

### JS_EVENT (3)

| Name | Value | Comment |
|------|-------|---------|
| `JS_EVENT_BUTTON` | `0x01` | button pressed/released |
| `JS_EVENT_AXIS` | `0x02` | joystick moved |
| `JS_EVENT_INIT` | `0x80` | initial state of device |

### JS_GET (4)

| Name | Value | Comment |
|------|-------|---------|
| `JS_GET_CAL` | `2` |  |
| `JS_GET_TIMEOUT` | `4` |  |
| `JS_GET_TIMELIMIT` | `6` |  |
| `JS_GET_ALL` | `7` |  |

### JS_SET (4)

| Name | Value | Comment |
|------|-------|---------|
| `JS_SET_CAL` | `1` |  |
| `JS_SET_TIMEOUT` | `3` |  |
| `JS_SET_TIMELIMIT` | `5` |  |
| `JS_SET_ALL` | `8` |  |

### JS_X (2)

| Name | Value | Comment |
|------|-------|---------|
| `JS_X_0` | `0x01` |  |
| `JS_X_1` | `0x04` |  |

### JS_Y (2)

| Name | Value | Comment |
|------|-------|---------|
| `JS_Y_0` | `0x02` |  |
| `JS_Y_1` | `0x08` |  |

### UNCATEGORIZED (14)

| Name | Value | Comment |
|------|-------|---------|
| `JS_VERSION` | `0x020100` |  |
| `JSIOCGVERSION` | `_IOR('j', 0x01, __u32)` | get driver version |
| `JSIOCGAXES` | `_IOR('j', 0x11, __u8)` | get number of axes |
| `JSIOCGBUTTONS` | `_IOR('j', 0x12, __u8)` | get number of buttons |
| `JSIOCSCORR` | `_IOW('j', 0x21, struct js_corr)` | set correction values |
| `JSIOCGCORR` | `_IOR('j', 0x22, struct js_corr)` | get correction values |
| `JSIOCSAXMAP` | `_IOW('j', 0x31, __u8[ABS_CNT])` | set axis mapping |
| `JSIOCGAXMAP` | `_IOR('j', 0x32, __u8[ABS_CNT])` | get axis mapping |
| `JSIOCSBTNMAP` | `_IOW('j', 0x33, __u16[KEY_MAX - BTN_MISC + 1])` | set button mapping |
| `JSIOCGBTNMAP` | `_IOR('j', 0x34, __u16[KEY_MAX - BTN_MISC + 1])` | get button mapping |
| `JS_RETURN` | `sizeof(struct JS_DATA_TYPE)` |  |
| `JS_TRUE` | `1` |  |
| `JS_FALSE` | `0` |  |
| `JS_MAX` | `2` |  |

## Structs (5)


### `struct js_event`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `time` | `-` |
| `__s16` | `value` | `-` |
| `__u8` | `type` | `-` |
| `__u8` | `number` | `-` |

### `struct js_corr`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `coef` | `8` |
| `__s16` | `prec` | `-` |
| `__u16` | `type` | `-` |

### `struct JS_DATA_TYPE`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `buttons` | `-` |
| `__s32` | `x` | `-` |
| `__s32` | `y` | `-` |

### `struct JS_DATA_SAVE_TYPE_32`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `JS_TIMEOUT` | `-` |
| `__s32` | `BUSY` | `-` |
| `__s32` | `JS_EXPIRETIME` | `-` |
| `__s32` | `JS_TIMELIMIT` | `-` |

### `struct JS_DATA_SAVE_TYPE_64`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `JS_TIMEOUT` | `-` |
| `__s32` | `BUSY` | `-` |
| `__s64` | `JS_EXPIRETIME` | `-` |
| `__s64` | `JS_TIMELIMIT` | `-` |