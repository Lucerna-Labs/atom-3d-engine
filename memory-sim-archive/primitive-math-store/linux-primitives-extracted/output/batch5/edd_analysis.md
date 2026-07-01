# edd.h

**Source:** `edd.h`


## Includes

- `linux/types.h`

## Defines (27 total)


### EDD_EXT (4)

| Name | Value | Comment |
|------|-------|---------|
| `EDD_EXT_FIXED_DISK_ACCESS` | `(1 << 0)` |  |
| `EDD_EXT_DEVICE_LOCKING_AND_EJECTING` | `(1 << 1)` |  |
| `EDD_EXT_ENHANCED_DISK_DRIVE_SUPPORT` | `(1 << 2)` |  |
| `EDD_EXT_64BIT_EXTENSIONS` | `(1 << 3)` |  |

### EDD_INFO (8)

| Name | Value | Comment |
|------|-------|---------|
| `EDD_INFO_DMA_BOUNDARY_ERROR_TRANSPARENT` | `(1 << 0)` |  |
| `EDD_INFO_GEOMETRY_VALID` | `(1 << 1)` |  |
| `EDD_INFO_REMOVABLE` | `(1 << 2)` |  |
| `EDD_INFO_WRITE_VERIFY` | `(1 << 3)` |  |
| `EDD_INFO_MEDIA_CHANGE_NOTIFICATION` | `(1 << 4)` |  |
| `EDD_INFO_LOCKABLE` | `(1 << 5)` |  |
| `EDD_INFO_NO_MEDIA_PRESENT` | `(1 << 6)` |  |
| `EDD_INFO_USE_INT13_FN50` | `(1 << 7)` |  |

### EDD_MBR (4)

| Name | Value | Comment |
|------|-------|---------|
| `EDD_MBR_SIG_OFFSET` | `0x1B8` | offset of signature in the MBR |
| `EDD_MBR_SIG_BUF` | `0x290` | addr in boot params |
| `EDD_MBR_SIG_MAX` | `16` | max number of signatures to store |
| `EDD_MBR_SIG_NR_BUF` | `0x1ea  /* addr of number of MBR signtaures at EDD_MBR_SIG_BU` |  |

### UNCATEGORIZED (11)

| Name | Value | Comment |
|------|-------|---------|
| `EDDNR` | `0x1e9		/* addr of number of edd_info structs at EDDBUF` |  |
| `EDDBUF` | `0xd00` | addr of edd_info structs in boot_params |
| `EDDMAXNR` | `6` | number of edd_info structs starting at EDDBUF |
| `EDDEXTSIZE` | `8` | change these if you muck with the structures |
| `EDDPARMSIZE` | `74` |  |
| `CHECKEXTENSIONSPRESENT` | `0x41` |  |
| `GETDEVICEPARAMETERS` | `0x48` |  |
| `LEGACYGETDEVICEPARAMETERS` | `0x08` |  |
| `EDDMAGIC1` | `0x55AA` |  |
| `EDDMAGIC2` | `0xAA55` |  |
| `READ_SECTORS` | `0x02` | int13 AH=0x02 is READ_SECTORS command |

## Structs (19)


### `struct edd_device_params`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `length` | `-` |
| `__u16` | `info_flags` | `-` |
| `__u32` | `num_default_cylinders` | `-` |
| `__u32` | `num_default_heads` | `-` |
| `__u32` | `sectors_per_track` | `-` |
| `__u64` | `number_of_sectors` | `-` |
| `__u16` | `bytes_per_sector` | `-` |
| `__u32` | `dpte_ptr` | `-` |
| `__u16` | `key` | `-` |
| `__u8` | `device_path_info_length` | `-` |
| `__u8` | `reserved2` | `-` |
| `__u16` | `reserved3` | `-` |
| `__u8` | `host_bus_type` | `4` |
| `__u8` | `interface_type` | `8` |
| `__u16` | `base_address` | `-` |
| `__u16` | `reserved1` | `-` |
| `__u32` | `reserved2` | `-` |
| `__u8` | `bus` | `-` |
| `__u8` | `slot` | `-` |
| `__u8` | `function` | `-` |
| `__u8` | `channel` | `-` |
| `__u32` | `reserved` | `-` |
| `__u64` | `reserved` | `-` |
| `__u64` | `reserved` | `-` |
| `__u64` | `reserved` | `-` |
| `__u64` | `reserved` | `-` |
| `__u8` | `device` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `reserved4` | `-` |
| `__u8` | `device` | `-` |
| `__u8` | `lun` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u8` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `reserved4` | `-` |
| `__u16` | `id` | `-` |
| `__u64` | `lun` | `-` |
| `__u16` | `reserved1` | `-` |
| `__u32` | `reserved2` | `-` |
| `__u64` | `serial_number` | `-` |
| `__u64` | `reserved` | `-` |
| `__u64` | `eui` | `-` |
| `__u64` | `reserved` | `-` |
| `__u64` | `wwid` | `-` |
| `__u64` | `lun` | `-` |
| `__u64` | `identity_tag` | `-` |
| `__u64` | `reserved` | `-` |
| `__u32` | `array_number` | `-` |
| `__u32` | `reserved1` | `-` |
| `__u64` | `reserved2` | `-` |
| `__u8` | `device` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `reserved4` | `-` |
| `__u64` | `reserved1` | `-` |
| `__u64` | `reserved2` | `-` |
| `__u8` | `reserved4` | `-` |
| `__u8` | `checksum` | `-` |

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `base_address` | `-` |
| `__u16` | `reserved1` | `-` |
| `__u32` | `reserved2` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `bus` | `-` |
| `__u8` | `slot` | `-` |
| `__u8` | `function` | `-` |
| `__u8` | `channel` | `-` |
| `__u32` | `reserved` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved` | `-` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `device` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `reserved4` | `-` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `device` | `-` |
| `__u8` | `lun` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u8` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `reserved4` | `-` |

### `struct anonymous_9`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `id` | `-` |
| `__u64` | `lun` | `-` |
| `__u16` | `reserved1` | `-` |
| `__u32` | `reserved2` | `-` |

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `serial_number` | `-` |
| `__u64` | `reserved` | `-` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `eui` | `-` |
| `__u64` | `reserved` | `-` |

### `struct anonymous_12`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `wwid` | `-` |
| `__u64` | `lun` | `-` |

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `identity_tag` | `-` |
| `__u64` | `reserved` | `-` |

### `struct anonymous_14`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `array_number` | `-` |
| `__u32` | `reserved1` | `-` |
| `__u64` | `reserved2` | `-` |

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `device` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `reserved4` | `-` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved1` | `-` |
| `__u64` | `reserved2` | `-` |

### `struct edd_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `device` | `-` |
| `__u8` | `version` | `-` |
| `__u16` | `interface_support` | `-` |
| `__u16` | `legacy_max_cylinder` | `-` |
| `__u8` | `legacy_max_head` | `-` |
| `__u8` | `legacy_sectors_per_track` | `-` |

### `struct edd`

| Type | Field | Array |
|------|-------|-------|