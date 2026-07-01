# uinput.h

**Source:** `uinput.h`


## Includes

- `linux/types.h`
- `linux/input.h`

## Defines (26 total)


### UINPUT_IOCTL (1)

| Name | Value | Comment |
|------|-------|---------|
| `UINPUT_IOCTL_BASE` | `'U'` |  |

### UINPUT_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `UINPUT_MAX_NAME_SIZE` | `80` |  |

### UI_ABS (1)

| Name | Value | Comment |
|------|-------|---------|
| `UI_ABS_SETUP` | `_IOW(UINPUT_IOCTL_BASE, 4, struct uinput_abs_setup)` |  |

### UI_BEGIN (2)

| Name | Value | Comment |
|------|-------|---------|
| `UI_BEGIN_FF_UPLOAD` | `_IOWR(UINPUT_IOCTL_BASE, 200, struct uinput_ff_upload)` |  |
| `UI_BEGIN_FF_ERASE` | `_IOWR(UINPUT_IOCTL_BASE, 202, struct uinput_ff_erase)` |  |

### UI_DEV (3)

| Name | Value | Comment |
|------|-------|---------|
| `UI_DEV_CREATE` | `_IO(UINPUT_IOCTL_BASE, 1)` |  |
| `UI_DEV_DESTROY` | `_IO(UINPUT_IOCTL_BASE, 2)` |  |
| `UI_DEV_SETUP` | `_IOW(UINPUT_IOCTL_BASE, 3, struct uinput_setup)` |  |

### UI_END (2)

| Name | Value | Comment |
|------|-------|---------|
| `UI_END_FF_UPLOAD` | `_IOW(UINPUT_IOCTL_BASE, 201, struct uinput_ff_upload)` |  |
| `UI_END_FF_ERASE` | `_IOW(UINPUT_IOCTL_BASE, 203, struct uinput_ff_erase)` |  |

### UI_FF (2)

| Name | Value | Comment |
|------|-------|---------|
| `UI_FF_UPLOAD` | `1` |  |
| `UI_FF_ERASE` | `2` |  |

### UI_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `UI_GET_VERSION` | `_IOR(UINPUT_IOCTL_BASE, 45, unsigned int)` |  |

### UI_SET (11)

| Name | Value | Comment |
|------|-------|---------|
| `UI_SET_EVBIT` | `_IOW(UINPUT_IOCTL_BASE, 100, int)` |  |
| `UI_SET_KEYBIT` | `_IOW(UINPUT_IOCTL_BASE, 101, int)` |  |
| `UI_SET_RELBIT` | `_IOW(UINPUT_IOCTL_BASE, 102, int)` |  |
| `UI_SET_ABSBIT` | `_IOW(UINPUT_IOCTL_BASE, 103, int)` |  |
| `UI_SET_MSCBIT` | `_IOW(UINPUT_IOCTL_BASE, 104, int)` |  |
| `UI_SET_LEDBIT` | `_IOW(UINPUT_IOCTL_BASE, 105, int)` |  |
| `UI_SET_SNDBIT` | `_IOW(UINPUT_IOCTL_BASE, 106, int)` |  |
| `UI_SET_FFBIT` | `_IOW(UINPUT_IOCTL_BASE, 107, int)` |  |
| `UI_SET_PHYS` | `_IOW(UINPUT_IOCTL_BASE, 108, char*)` |  |
| `UI_SET_SWBIT` | `_IOW(UINPUT_IOCTL_BASE, 109, int)` |  |
| `UI_SET_PROPBIT` | `_IOW(UINPUT_IOCTL_BASE, 110, int)` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `UINPUT_VERSION` | `5` |  |
| `EV_UINPUT` | `0x0101` |  |

## Structs (5)


### `struct uinput_ff_upload`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `request_id` | `-` |
| `__s32` | `retval` | `-` |

### `struct uinput_ff_erase`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `request_id` | `-` |
| `__s32` | `retval` | `-` |
| `__u32` | `effect_id` | `-` |

### `struct uinput_setup`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `UINPUT_MAX_NAME_SIZE` |
| `__u32` | `ff_effects_max` | `-` |

### `struct uinput_abs_setup`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `code` | `-` |

### `struct uinput_user_dev`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `UINPUT_MAX_NAME_SIZE` |
| `__u32` | `ff_effects_max` | `-` |
| `__s32` | `absmax` | `ABS_CNT` |
| `__s32` | `absmin` | `ABS_CNT` |
| `__s32` | `absfuzz` | `ABS_CNT` |
| `__s32` | `absflat` | `ABS_CNT` |