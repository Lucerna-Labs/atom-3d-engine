# hidraw.h

**Source:** `hidraw.h`


## Includes

- `linux/hid.h`
- `linux/types.h`

## Defines (8 total)


### HIDRAW_BUFFER (1)

| Name | Value | Comment |
|------|-------|---------|
| `HIDRAW_BUFFER_SIZE` | `64` |  |

### HIDRAW_FIRST (1)

| Name | Value | Comment |
|------|-------|---------|
| `HIDRAW_FIRST_MINOR` | `0` |  |

### HIDRAW_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `HIDRAW_MAX_DEVICES` | `64` |  |

### UNCATEGORIZED (5)

| Name | Value | Comment |
|------|-------|---------|
| `HIDIOCGRDESCSIZE` | `_IOR('H', 0x01, int)` |  |
| `HIDIOCGRDESC` | `_IOR('H', 0x02, struct hidraw_report_descriptor)` |  |
| `HIDIOCGRAWINFO` | `_IOR('H', 0x03, struct hidraw_devinfo)` |  |
| `HIDIOCREVOKE` | `_IOW('H', 0x0D, int)` | Revoke device access |
| `HIDIOCTL_LAST` | `_IOC_NR(HIDIOCREVOKE)` |  |

## Structs (2)


### `struct hidraw_report_descriptor`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u8` | `value` | `HID_MAX_DESCRIPTOR_SIZE` |

### `struct hidraw_devinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bustype` | `-` |
| `__s16` | `vendor` | `-` |
| `__s16` | `product` | `-` |