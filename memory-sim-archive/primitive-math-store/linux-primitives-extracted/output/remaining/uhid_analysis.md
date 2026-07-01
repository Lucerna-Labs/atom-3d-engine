# uhid.h

**Source:** `uhid.h`


## Includes

- `linux/input.h`
- `linux/types.h`
- `linux/hid.h`

## Defines (1 total)


### UHID_DATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `UHID_DATA_MAX` | `4096` |  |

## Structs (14)


### `struct uhid_create2_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `name` | `128` |
| `__u8` | `phys` | `64` |
| `__u8` | `uniq` | `64` |
| `__u16` | `rd_size` | `-` |
| `__u16` | `bus` | `-` |
| `__u32` | `vendor` | `-` |
| `__u32` | `product` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `country` | `-` |
| `__u8` | `rd_data` | `HID_MAX_DESCRIPTOR_SIZE` |

### `struct uhid_start_req`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dev_flags` | `-` |

### `struct uhid_input2_req`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `size` | `-` |
| `__u8` | `data` | `UHID_DATA_MAX` |

### `struct uhid_output_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `data` | `UHID_DATA_MAX` |
| `__u16` | `size` | `-` |
| `__u8` | `rtype` | `-` |

### `struct uhid_get_report_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u8` | `rnum` | `-` |
| `__u8` | `rtype` | `-` |

### `struct uhid_get_report_reply_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u16` | `err` | `-` |
| `__u16` | `size` | `-` |
| `__u8` | `data` | `UHID_DATA_MAX` |

### `struct uhid_set_report_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u8` | `rnum` | `-` |
| `__u8` | `rtype` | `-` |
| `__u16` | `size` | `-` |
| `__u8` | `data` | `UHID_DATA_MAX` |

### `struct uhid_set_report_reply_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u16` | `err` | `-` |

### `struct uhid_create_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `name` | `128` |
| `__u8` | `phys` | `64` |
| `__u8` | `uniq` | `64` |
| `__u16` | `rd_size` | `-` |
| `__u16` | `bus` | `-` |
| `__u32` | `vendor` | `-` |
| `__u32` | `product` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `country` | `-` |

### `struct uhid_input_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `data` | `UHID_DATA_MAX` |
| `__u16` | `size` | `-` |

### `struct uhid_output_ev_req`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `type` | `-` |
| `__u16` | `code` | `-` |
| `__s32` | `value` | `-` |

### `struct uhid_feature_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u8` | `rnum` | `-` |
| `__u8` | `rtype` | `-` |

### `struct uhid_feature_answer_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u16` | `err` | `-` |
| `__u16` | `size` | `-` |
| `__u8` | `data` | `UHID_DATA_MAX` |

### `struct uhid_event`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |