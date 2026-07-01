# apm_bios.h

**Source:** `apm_bios.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (67 total)


### APM_BAD (4)

| Name | Value | Comment |
|------|-------|---------|
| `APM_BAD_DEVICE` | `0x09` |  |
| `APM_BAD_PARAM` | `0x0a` |  |
| `APM_BAD_FUNCTION` | `0x0c` |  |
| `APM_BAD_STATE` | `0x60` |  |

### APM_CAP (8)

| Name | Value | Comment |
|------|-------|---------|
| `APM_CAP_GLOBAL_STANDBY` | `0x0001` |  |
| `APM_CAP_GLOBAL_SUSPEND` | `0x0002` |  |
| `APM_CAP_RESUME_STANDBY_TIMER` | `0x0004` | Timer resume from standby |
| `APM_CAP_RESUME_SUSPEND_TIMER` | `0x0008` | Timer resume from suspend |
| `APM_CAP_RESUME_STANDBY_RING` | `0x0010` | Resume on Ring fr standby |
| `APM_CAP_RESUME_SUSPEND_RING` | `0x0020` | Resume on Ring fr suspend |
| `APM_CAP_RESUME_STANDBY_PCMCIA` | `0x0040` | Resume on PCMCIA Ring |
| `APM_CAP_RESUME_SUSPEND_PCMCIA` | `0x0080` | Resume on PCMCIA Ring |

### APM_CAPABILITY (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_CAPABILITY_CHANGE` | `0x000c` |  |

### APM_CRITICAL (2)

| Name | Value | Comment |
|------|-------|---------|
| `APM_CRITICAL_RESUME` | `0x0004` |  |
| `APM_CRITICAL_SUSPEND` | `0x0008` |  |

### APM_DEVICE (13)

| Name | Value | Comment |
|------|-------|---------|
| `APM_DEVICE_BIOS` | `0x0000` |  |
| `APM_DEVICE_ALL` | `0x0001` |  |
| `APM_DEVICE_DISPLAY` | `0x0100` |  |
| `APM_DEVICE_STORAGE` | `0x0200` |  |
| `APM_DEVICE_PARALLEL` | `0x0300` |  |
| `APM_DEVICE_SERIAL` | `0x0400` |  |
| `APM_DEVICE_NETWORK` | `0x0500` |  |
| `APM_DEVICE_PCMCIA` | `0x0600` |  |
| `APM_DEVICE_BATTERY` | `0x8000` |  |
| `APM_DEVICE_OEM` | `0xe000` |  |
| `APM_DEVICE_OLD_ALL` | `0xffff` |  |
| `APM_DEVICE_CLASS` | `0x00ff` |  |
| `APM_DEVICE_MASK` | `0xff00` |  |

### APM_HIBERNATION (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_HIBERNATION_RESUME` | `0x000e` |  |

### APM_IOC (2)

| Name | Value | Comment |
|------|-------|---------|
| `APM_IOC_STANDBY` | `_IO('A', 1)` |  |
| `APM_IOC_SUSPEND` | `_IO('A', 2)` |  |

### APM_LOW (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_LOW_BATTERY` | `0x0005` |  |

### APM_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_MAX_BATTERIES` | `2` |  |

### APM_NO (2)

| Name | Value | Comment |
|------|-------|---------|
| `APM_NO_ERROR` | `0x53` |  |
| `APM_NO_EVENTS` | `0x80` |  |

### APM_NORMAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_NORMAL_RESUME` | `0x0003` |  |

### APM_NOT (3)

| Name | Value | Comment |
|------|-------|---------|
| `APM_NOT_CONNECTED` | `0x03` |  |
| `APM_NOT_ENGAGED` | `0x0b` |  |
| `APM_NOT_PRESENT` | `0x86` |  |

### APM_POWER (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_POWER_STATUS_CHANGE` | `0x0006` |  |

### APM_RESUME (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_RESUME_DISABLED` | `0x0d` |  |

### APM_STANDBY (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_STANDBY_RESUME` | `0x000b` |  |

### APM_STATE (12)

| Name | Value | Comment |
|------|-------|---------|
| `APM_STATE_READY` | `0x0000` |  |
| `APM_STATE_STANDBY` | `0x0001` |  |
| `APM_STATE_SUSPEND` | `0x0002` |  |
| `APM_STATE_OFF` | `0x0003` |  |
| `APM_STATE_BUSY` | `0x0004` |  |
| `APM_STATE_REJECT` | `0x0005` |  |
| `APM_STATE_OEM_SYS` | `0x0020` |  |
| `APM_STATE_OEM_DEV` | `0x0040` |  |
| `APM_STATE_DISABLE` | `0x0000` |  |
| `APM_STATE_ENABLE` | `0x0001` |  |
| `APM_STATE_DISENGAGE` | `0x0000` |  |
| `APM_STATE_ENGAGE` | `0x0001` |  |

### APM_SYS (2)

| Name | Value | Comment |
|------|-------|---------|
| `APM_SYS_STANDBY` | `0x0001` |  |
| `APM_SYS_SUSPEND` | `0x0002` |  |

### APM_UPDATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `APM_UPDATE_TIME` | `0x0007` |  |

### APM_USER (3)

| Name | Value | Comment |
|------|-------|---------|
| `APM_USER_STANDBY` | `0x0009` |  |
| `APM_USER_SUSPEND` | `0x000a` |  |
| `APM_USER_HIBERNATION` | `0x000d` |  |

### UNCATEGORIZED (7)

| Name | Value | Comment |
|------|-------|---------|
| `APM_SUCCESS` | `0x00` |  |
| `APM_DISABLED` | `0x01` |  |
| `APM_CONNECTED` | `0x02` |  |
| `APM_16_CONNECTED` | `0x05` |  |
| `APM_16_UNSUPPORTED` | `0x06` |  |
| `APM_32_CONNECTED` | `0x07` |  |
| `APM_32_UNSUPPORTED` | `0x08` |  |

## Structs (1)


### `struct apm_bios_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `version` | `-` |
| `__u16` | `cseg` | `-` |
| `__u32` | `offset` | `-` |
| `__u16` | `cseg_16` | `-` |
| `__u16` | `dseg` | `-` |
| `__u16` | `flags` | `-` |
| `__u16` | `cseg_len` | `-` |
| `__u16` | `cseg_16_len` | `-` |
| `__u16` | `dseg_len` | `-` |