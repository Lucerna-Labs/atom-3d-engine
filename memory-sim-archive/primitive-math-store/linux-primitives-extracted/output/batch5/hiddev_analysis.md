# hiddev.h

**Source:** `hiddev.h`


## Includes

- `linux/types.h`

## Defines (44 total)


### HIDDEV_FLAG (2)

| Name | Value | Comment |
|------|-------|---------|
| `HIDDEV_FLAG_UREF` | `0x1` |  |
| `HIDDEV_FLAG_REPORT` | `0x2` |  |

### HID_FIELD (10)

| Name | Value | Comment |
|------|-------|---------|
| `HID_FIELD_CONSTANT` | `0x001` |  |
| `HID_FIELD_VARIABLE` | `0x002` |  |
| `HID_FIELD_RELATIVE` | `0x004` |  |
| `HID_FIELD_WRAP` | `0x008` |  |
| `HID_FIELD_NONLINEAR` | `0x010` |  |
| `HID_FIELD_NO_PREFERRED` | `0x020` |  |
| `HID_FIELD_NULL_STATE` | `0x040` |  |
| `HID_FIELD_VOLATILE` | `0x080` |  |
| `HID_FIELD_BUFFERED_BYTE` | `0x100` |  |
| `HID_FIELD_INDEX_NONE` | `0xffffffff` |  |

### HID_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `HID_MAX_MULTI_USAGES` | `1024` |  |

### HID_REPORT (10)

| Name | Value | Comment |
|------|-------|---------|
| `HID_REPORT_ID_UNKNOWN` | `0xffffffff` |  |
| `HID_REPORT_ID_FIRST` | `0x00000100` |  |
| `HID_REPORT_ID_NEXT` | `0x00000200` |  |
| `HID_REPORT_ID_MASK` | `0x000000ff` |  |
| `HID_REPORT_ID_MAX` | `0x000000ff` |  |
| `HID_REPORT_TYPE_INPUT` | `1` |  |
| `HID_REPORT_TYPE_OUTPUT` | `2` |  |
| `HID_REPORT_TYPE_FEATURE` | `3` |  |
| `HID_REPORT_TYPE_MIN` | `1` |  |
| `HID_REPORT_TYPE_MAX` | `3` |  |

### HID_STRING (1)

| Name | Value | Comment |
|------|-------|---------|
| `HID_STRING_SIZE` | `256` |  |

### UNCATEGORIZED (20)

| Name | Value | Comment |
|------|-------|---------|
| `HID_VERSION` | `0x010004` |  |
| `HIDIOCGVERSION` | `_IOR('H', 0x01, int)` |  |
| `HIDIOCAPPLICATION` | `_IO('H', 0x02)` |  |
| `HIDIOCGDEVINFO` | `_IOR('H', 0x03, struct hiddev_devinfo)` |  |
| `HIDIOCGSTRING` | `_IOR('H', 0x04, struct hiddev_string_descriptor)` |  |
| `HIDIOCINITREPORT` | `_IO('H', 0x05)` |  |
| `HIDIOCGREPORT` | `_IOW('H', 0x07, struct hiddev_report_info)` |  |
| `HIDIOCSREPORT` | `_IOW('H', 0x08, struct hiddev_report_info)` |  |
| `HIDIOCGREPORTINFO` | `_IOWR('H', 0x09, struct hiddev_report_info)` |  |
| `HIDIOCGFIELDINFO` | `_IOWR('H', 0x0A, struct hiddev_field_info)` |  |
| `HIDIOCGUSAGE` | `_IOWR('H', 0x0B, struct hiddev_usage_ref)` |  |
| `HIDIOCSUSAGE` | `_IOW('H', 0x0C, struct hiddev_usage_ref)` |  |
| `HIDIOCGUCODE` | `_IOWR('H', 0x0D, struct hiddev_usage_ref)` |  |
| `HIDIOCGFLAG` | `_IOR('H', 0x0E, int)` |  |
| `HIDIOCSFLAG` | `_IOW('H', 0x0F, int)` |  |
| `HIDIOCGCOLLECTIONINDEX` | `_IOW('H', 0x10, struct hiddev_usage_ref)` |  |
| `HIDIOCGCOLLECTIONINFO` | `_IOWR('H', 0x11, struct hiddev_collection_info)` |  |
| `HIDIOCGUSAGES` | `_IOWR('H', 0x13, struct hiddev_usage_ref_multi)` |  |
| `HIDIOCSUSAGES` | `_IOW('H', 0x14, struct hiddev_usage_ref_multi)` |  |
| `HIDDEV_FLAGS` | `0x3` |  |

## Structs (8)


### `struct hiddev_event`

| Type | Field | Array |
|------|-------|-------|
| `unsigned` | `hid` | `-` |

### `struct hiddev_devinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bustype` | `-` |
| `__u32` | `busnum` | `-` |
| `__u32` | `devnum` | `-` |
| `__u32` | `ifnum` | `-` |
| `__s16` | `vendor` | `-` |
| `__s16` | `product` | `-` |
| `__s16` | `version` | `-` |
| `__u32` | `num_applications` | `-` |

### `struct hiddev_collection_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `index` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `usage` | `-` |
| `__u32` | `level` | `-` |

### `struct hiddev_string_descriptor`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `index` | `-` |
| `char` | `value` | `HID_STRING_SIZE` |

### `struct hiddev_report_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `report_type` | `-` |
| `__u32` | `report_id` | `-` |
| `__u32` | `num_fields` | `-` |

### `struct hiddev_field_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `report_type` | `-` |
| `__u32` | `report_id` | `-` |
| `__u32` | `field_index` | `-` |
| `__u32` | `maxusage` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `physical` | `-` |
| `__u32` | `logical` | `-` |
| `__u32` | `application` | `-` |
| `__s32` | `logical_minimum` | `-` |
| `__s32` | `logical_maximum` | `-` |
| `__s32` | `physical_minimum` | `-` |
| `__s32` | `physical_maximum` | `-` |
| `__u32` | `unit_exponent` | `-` |
| `__u32` | `unit` | `-` |

### `struct hiddev_usage_ref`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `report_type` | `-` |
| `__u32` | `report_id` | `-` |
| `__u32` | `field_index` | `-` |
| `__u32` | `usage_index` | `-` |
| `__u32` | `usage_code` | `-` |
| `__s32` | `value` | `-` |

### `struct hiddev_usage_ref_multi`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_values` | `-` |
| `__s32` | `values` | `HID_MAX_MULTI_USAGES` |