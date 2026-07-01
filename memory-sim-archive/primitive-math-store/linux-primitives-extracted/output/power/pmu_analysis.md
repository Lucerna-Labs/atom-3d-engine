# pmu.h

**Source:** `pmu.h`


## Includes

- `linux/ioctl.h`

## Defines (64 total)


### PMU_ADB (2)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_ADB_CMD` | `0x20` | send ADB packet |
| `PMU_ADB_POLL_OFF` | `0x21` | disable ADB auto-poll |

### PMU_BACKLIGHT (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_BACKLIGHT_BRIGHT` | `0x41` | set backlight brightness |

### PMU_BATTERY (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_BATTERY_STATE` | `0x6b` | report battery state etc. |

### PMU_CPU (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_CPU_SPEED` | `0x7d` | control CPU speed on some models |

### PMU_DRIVER (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_DRIVER_VERSION` | `2` |  |

### PMU_ENV (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_ENV_LID_CLOSED` | `0x01` | The lid is closed |

### PMU_GET (4)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_GET_VOLBUTTON` | `0x48` | get volume up/down position |
| `PMU_GET_BRIGHTBUTTON` | `0xd9` | report brightness up/down pos |
| `PMU_GET_COVER` | `0xdc` | report cover open/closed |
| `PMU_GET_VERSION` | `0xea` | read the PMU version |

### PMU_INT (10)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_INT_ACK` | `0x78` | read interrupt bits |
| `PMU_INT_PCEJECT` | `0x04` | PC-card eject buttons |
| `PMU_INT_SNDBRT` | `0x08` | sound/brightness up/down buttons |
| `PMU_INT_ADB` | `0x10` | ADB autopoll or reply data |
| `PMU_INT_BATTERY` | `0x20` | Battery state change |
| `PMU_INT_ENVIRONMENT` | `0x40` | Environment interrupts |
| `PMU_INT_TICK` | `0x80` | 1-second tick interrupt |
| `PMU_INT_ADB_AUTO` | `0x04` | ADB autopoll, when PMU_INT_ADB |
| `PMU_INT_WAITING_CHARGER` | `0x01` | ??? |
| `PMU_INT_AUTO_SRQ_POLL` | `0x02` | ??? |

### PMU_IOC (7)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_IOC_SLEEP` | `_IO('B', 0)` |  |
| `PMU_IOC_GET_BACKLIGHT` | `_IOR('B', 1, size_t)` |  |
| `PMU_IOC_SET_BACKLIGHT` | `_IOW('B', 2, size_t)` |  |
| `PMU_IOC_GET_MODEL` | `_IOR('B', 3, size_t)` |  |
| `PMU_IOC_HAS_ADB` | `_IOR('B', 4, size_t)` |  |
| `PMU_IOC_CAN_SLEEP` | `_IOR('B', 5, size_t)` |  |
| `PMU_IOC_GRAB_BACKLIGHT` | `_IOR('B', 6, size_t)` |  |

### PMU_POW (6)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_POW_ON` | `0x80` | OR this to power ON the device |
| `PMU_POW_OFF` | `0x00` | leave bit 7 to 0 to power it OFF |
| `PMU_POW_BACKLIGHT` | `0x01` | backlight power |
| `PMU_POW_CHARGER` | `0x02` | battery charger power |
| `PMU_POW_IRLED` | `0x04` | IR led power (on wallstreet) |
| `PMU_POW_MEDIABAY` | `0x08` | media bay power (wallstreet/lombard ?) |

### PMU_POWER (3)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_POWER_CTRL0` | `0x10` | control power of some devices |
| `PMU_POWER_CTRL` | `0x11` | control power of some devices |
| `PMU_POWER_EVENTS` | `0x8f` | Send power-event commands to PMU |

### PMU_READ (3)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_READ_XPRAM` | `0x3a` | read eXtended Parameter RAM |
| `PMU_READ_NVRAM` | `0x3b` | read non-volatile RAM |
| `PMU_READ_RTC` | `0x38` | read real-time clock |

### PMU_SET (3)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_SET_RTC` | `0x30` | set real-time clock |
| `PMU_SET_VOLBUTTON` | `0x40` | set volume up/down position |
| `PMU_SET_INTR_MASK` | `0x70` | set PMU interrupt mask |

### PMU_SMART (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_SMART_BATTERY_STATE` | `0x6f` | report battery state (new way) |

### PMU_SYSTEM (1)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_SYSTEM_READY` | `0xdf` | tell PMU we are awake |

### PMU_WRITE (2)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_WRITE_XPRAM` | `0x32` | write eXtended Parameter RAM |
| `PMU_WRITE_NVRAM` | `0x33` | write non-volatile RAM |

### UNCATEGORIZED (17)

| Name | Value | Comment |
|------|-------|---------|
| `PMU_PCEJECT` | `0x4c` | eject PC-card from slot |
| `PMU_SHUTDOWN` | `0x7e` | turn power off |
| `PMU_SLEEP` | `0x7f` | put CPU to sleep |
| `PMU_I2C_CMD` | `0x9a` | I2C operations |
| `PMU_RESET` | `0xd0` | reset CPU |
| `PMU_POW0_ON` | `0x80` | OR this to power ON the device |
| `PMU_POW0_OFF` | `0x00` | leave bit 7 to 0 to power it OFF |
| `PMU_POW0_HARD_DRIVE` | `0x04` | Hard drive power (on wallstreet/lombard ?) |
| `PMU_I2C_MODE_SIMPLE` | `0` |  |
| `PMU_I2C_MODE_STDSUB` | `1` |  |
| `PMU_I2C_MODE_COMBINED` | `2` |  |
| `PMU_I2C_BUS_STATUS` | `0` |  |
| `PMU_I2C_BUS_SYSCLK` | `1` |  |
| `PMU_I2C_BUS_POWER` | `2` |  |
| `PMU_I2C_STATUS_OK` | `0` |  |
| `PMU_I2C_STATUS_DATAREAD` | `1` |  |
| `PMU_I2C_STATUS_BUSY` | `0xfe` |  |