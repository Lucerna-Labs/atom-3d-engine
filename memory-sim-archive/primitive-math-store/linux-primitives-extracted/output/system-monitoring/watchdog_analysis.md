# watchdog.h

**Source:** `watchdog.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (29 total)


### UNCATEGORIZED (28)

| Name | Value | Comment |
|------|-------|---------|
| `WDIOC_GETSUPPORT` | `_IOR(WATCHDOG_IOCTL_BASE, 0, struct watchdog_info)` |  |
| `WDIOC_GETSTATUS` | `_IOR(WATCHDOG_IOCTL_BASE, 1, int)` |  |
| `WDIOC_GETBOOTSTATUS` | `_IOR(WATCHDOG_IOCTL_BASE, 2, int)` |  |
| `WDIOC_GETTEMP` | `_IOR(WATCHDOG_IOCTL_BASE, 3, int)` |  |
| `WDIOC_SETOPTIONS` | `_IOR(WATCHDOG_IOCTL_BASE, 4, int)` |  |
| `WDIOC_KEEPALIVE` | `_IOR(WATCHDOG_IOCTL_BASE, 5, int)` |  |
| `WDIOC_SETTIMEOUT` | `_IOWR(WATCHDOG_IOCTL_BASE, 6, int)` |  |
| `WDIOC_GETTIMEOUT` | `_IOR(WATCHDOG_IOCTL_BASE, 7, int)` |  |
| `WDIOC_SETPRETIMEOUT` | `_IOWR(WATCHDOG_IOCTL_BASE, 8, int)` |  |
| `WDIOC_GETPRETIMEOUT` | `_IOR(WATCHDOG_IOCTL_BASE, 9, int)` |  |
| `WDIOC_GETTIMELEFT` | `_IOR(WATCHDOG_IOCTL_BASE, 10, int)` |  |
| `WDIOF_UNKNOWN` | `-1` | Unknown flag error |
| `WDIOS_UNKNOWN` | `-1` | Unknown status error |
| `WDIOF_OVERHEAT` | `0x0001` | Reset due to CPU overheat |
| `WDIOF_FANFAULT` | `0x0002` | Fan failed |
| `WDIOF_EXTERN1` | `0x0004` | External relay 1 |
| `WDIOF_EXTERN2` | `0x0008` | External relay 2 |
| `WDIOF_POWERUNDER` | `0x0010` | Power bad/power fault |
| `WDIOF_CARDRESET` | `0x0020` | Card previously reset the CPU |
| `WDIOF_POWEROVER` | `0x0040` | Power over voltage |
| `WDIOF_SETTIMEOUT` | `0x0080` | Set timeout (in seconds) |
| `WDIOF_MAGICCLOSE` | `0x0100` | Supports magic close char |
| `WDIOF_PRETIMEOUT` | `0x0200` | Pretimeout (in seconds), get/set |
| `WDIOF_ALARMONLY` | `0x0400	/* Watchdog triggers a management or` |  |
| `WDIOF_KEEPALIVEPING` | `0x8000` | Keep alive ping reply |
| `WDIOS_DISABLECARD` | `0x0001` | Turn off the watchdog timer |
| `WDIOS_ENABLECARD` | `0x0002` | Turn on the watchdog timer |
| `WDIOS_TEMPPANIC` | `0x0004` | Kernel panic on temperature trip |

### WATCHDOG_IOCTL (1)

| Name | Value | Comment |
|------|-------|---------|
| `WATCHDOG_IOCTL_BASE` | `'W'` |  |

## Structs (1)


### `struct watchdog_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `options` | `-` |
| `__u32` | `firmware_version` | `-` |
| `__u8` | `identity` | `32` |