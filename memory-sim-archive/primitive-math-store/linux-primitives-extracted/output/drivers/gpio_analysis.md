# gpio.h

**Source:** `gpio.h`


## Includes

- `linux/const.h`
- `linux/ioctl.h`
- `linux/types.h`

## Defines (40 total)


### GPIOEVENT_EVENT (2)

| Name | Value | Comment |
|------|-------|---------|
| `GPIOEVENT_EVENT_RISING_EDGE` | `0x01` |  |
| `GPIOEVENT_EVENT_FALLING_EDGE` | `0x02` |  |

### GPIOEVENT_REQUEST (3)

| Name | Value | Comment |
|------|-------|---------|
| `GPIOEVENT_REQUEST_RISING_EDGE` | `(1UL << 0)` |  |
| `GPIOEVENT_REQUEST_FALLING_EDGE` | `(1UL << 1)` |  |
| `GPIOEVENT_REQUEST_BOTH_EDGES` | `((1UL << 0) \| (1UL << 1))` |  |

### GPIOHANDLE_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPIOHANDLE_GET_LINE_VALUES_IOCTL` | `_IOWR(0xB4, 0x08, struct gpiohandle_data)` |  |

### GPIOHANDLE_REQUEST (8)

| Name | Value | Comment |
|------|-------|---------|
| `GPIOHANDLE_REQUEST_INPUT` | `(1UL << 0)` |  |
| `GPIOHANDLE_REQUEST_OUTPUT` | `(1UL << 1)` |  |
| `GPIOHANDLE_REQUEST_ACTIVE_LOW` | `(1UL << 2)` |  |
| `GPIOHANDLE_REQUEST_OPEN_DRAIN` | `(1UL << 3)` |  |
| `GPIOHANDLE_REQUEST_OPEN_SOURCE` | `(1UL << 4)` |  |
| `GPIOHANDLE_REQUEST_BIAS_PULL_UP` | `(1UL << 5)` |  |
| `GPIOHANDLE_REQUEST_BIAS_PULL_DOWN` | `(1UL << 6)` |  |
| `GPIOHANDLE_REQUEST_BIAS_DISABLE` | `(1UL << 7)` |  |

### GPIOHANDLE_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `GPIOHANDLE_SET_LINE_VALUES_IOCTL` | `_IOWR(0xB4, 0x09, struct gpiohandle_data)` |  |
| `GPIOHANDLE_SET_CONFIG_IOCTL` | `_IOWR(0xB4, 0x0A, struct gpiohandle_config)` |  |

### GPIOLINE_FLAG (8)

| Name | Value | Comment |
|------|-------|---------|
| `GPIOLINE_FLAG_KERNEL` | `(1UL << 0)` | Line used by the kernel |
| `GPIOLINE_FLAG_IS_OUT` | `(1UL << 1)` |  |
| `GPIOLINE_FLAG_ACTIVE_LOW` | `(1UL << 2)` |  |
| `GPIOLINE_FLAG_OPEN_DRAIN` | `(1UL << 3)` |  |
| `GPIOLINE_FLAG_OPEN_SOURCE` | `(1UL << 4)` |  |
| `GPIOLINE_FLAG_BIAS_PULL_UP` | `(1UL << 5)` |  |
| `GPIOLINE_FLAG_BIAS_PULL_DOWN` | `(1UL << 6)` |  |
| `GPIOLINE_FLAG_BIAS_DISABLE` | `(1UL << 7)` |  |

### GPIO_GET (6)

| Name | Value | Comment |
|------|-------|---------|
| `GPIO_GET_CHIPINFO_IOCTL` | `_IOR(0xB4, 0x01, struct gpiochip_info)` |  |
| `GPIO_GET_LINEINFO_UNWATCH_IOCTL` | `_IOWR(0xB4, 0x0C, __u32)` |  |
| `GPIO_GET_LINEINFO_IOCTL` | `_IOWR(0xB4, 0x02, struct gpioline_info)` |  |
| `GPIO_GET_LINEHANDLE_IOCTL` | `_IOWR(0xB4, 0x03, struct gpiohandle_request)` |  |
| `GPIO_GET_LINEEVENT_IOCTL` | `_IOWR(0xB4, 0x04, struct gpioevent_request)` |  |
| `GPIO_GET_LINEINFO_WATCH_IOCTL` | `_IOWR(0xB4, 0x0B, struct gpioline_info)` |  |

### GPIO_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPIO_MAX_NAME_SIZE` | `32` |  |

### UNCATEGORIZED (9)

| Name | Value | Comment |
|------|-------|---------|
| `GPIO_V2_LINES_MAX` | `64` |  |
| `GPIO_V2_LINE_NUM_ATTRS_MAX` | `10` |  |
| `GPIOHANDLES_MAX` | `64` |  |
| `GPIO_V2_GET_LINEINFO_IOCTL` | `_IOWR(0xB4, 0x05, struct gpio_v2_line_info)` |  |
| `GPIO_V2_GET_LINEINFO_WATCH_IOCTL` | `_IOWR(0xB4, 0x06, struct gpio_v2_line_info)` |  |
| `GPIO_V2_GET_LINE_IOCTL` | `_IOWR(0xB4, 0x07, struct gpio_v2_line_request)` |  |
| `GPIO_V2_LINE_SET_CONFIG_IOCTL` | `_IOWR(0xB4, 0x0D, struct gpio_v2_line_config)` |  |
| `GPIO_V2_LINE_GET_VALUES_IOCTL` | `_IOWR(0xB4, 0x0E, struct gpio_v2_line_values)` |  |
| `GPIO_V2_LINE_SET_VALUES_IOCTL` | `_IOWR(0xB4, 0x0F, struct gpio_v2_line_values)` |  |

## Structs (16)


### `struct gpiochip_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `GPIO_MAX_NAME_SIZE` |
| `char` | `label` | `GPIO_MAX_NAME_SIZE` |
| `__u32` | `lines` | `-` |

### `struct gpio_v2_line_values`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `bits` | `-` |
| `__aligned_u64` | `mask` | `-` |

### `struct gpio_v2_line_attribute`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `padding` | `-` |
| `__aligned_u64` | `flags` | `-` |
| `__aligned_u64` | `values` | `-` |
| `__u32` | `debounce_period_us` | `-` |

### `struct gpio_v2_line_config_attribute`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `mask` | `-` |

### `struct gpio_v2_line_config`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `flags` | `-` |
| `__u32` | `num_attrs` | `-` |
| `__u32` | `padding` | `5` |

### `struct gpio_v2_line_request`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `offsets` | `GPIO_V2_LINES_MAX` |
| `char` | `consumer` | `GPIO_MAX_NAME_SIZE` |
| `__u32` | `num_lines` | `-` |
| `__u32` | `event_buffer_size` | `-` |
| `__u32` | `padding` | `5` |
| `__s32` | `fd` | `-` |

### `struct gpio_v2_line_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `GPIO_MAX_NAME_SIZE` |
| `char` | `consumer` | `GPIO_MAX_NAME_SIZE` |
| `__u32` | `offset` | `-` |
| `__u32` | `num_attrs` | `-` |
| `__aligned_u64` | `flags` | `-` |
| `__u32` | `padding` | `4` |

### `struct gpio_v2_line_info_changed`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `timestamp_ns` | `-` |
| `__u32` | `event_type` | `-` |
| `__u32` | `padding` | `5` |

### `struct gpio_v2_line_event`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `timestamp_ns` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `seqno` | `-` |
| `__u32` | `line_seqno` | `-` |
| `__u32` | `padding` | `6` |

### `struct gpioline_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `line_offset` | `-` |
| `__u32` | `flags` | `-` |
| `char` | `name` | `GPIO_MAX_NAME_SIZE` |
| `char` | `consumer` | `GPIO_MAX_NAME_SIZE` |

### `struct gpioline_info_changed`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `timestamp` | `-` |
| `__u32` | `event_type` | `-` |
| `__u32` | `padding` | `5` |

### `struct gpiohandle_request`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `lineoffsets` | `GPIOHANDLES_MAX` |
| `__u32` | `flags` | `-` |
| `__u8` | `default_values` | `GPIOHANDLES_MAX` |
| `char` | `consumer_label` | `GPIO_MAX_NAME_SIZE` |
| `__u32` | `lines` | `-` |
| `int` | `fd` | `-` |

### `struct gpiohandle_config`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u8` | `default_values` | `GPIOHANDLES_MAX` |
| `__u32` | `padding` | `4` |

### `struct gpiohandle_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `values` | `GPIOHANDLES_MAX` |

### `struct gpioevent_request`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `lineoffset` | `-` |
| `__u32` | `handleflags` | `-` |
| `__u32` | `eventflags` | `-` |
| `char` | `consumer_label` | `GPIO_MAX_NAME_SIZE` |
| `int` | `fd` | `-` |

### `struct gpioevent_data`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `timestamp` | `-` |
| `__u32` | `id` | `-` |