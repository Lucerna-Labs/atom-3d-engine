# scarlett2.h

**Source:** `scarlett2.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (12 total)


### UNCATEGORIZED (12)

| Name | Value | Comment |
|------|-------|---------|
| `SCARLETT2_HWDEP_MAJOR` | `1` |  |
| `SCARLETT2_HWDEP_MINOR` | `0` |  |
| `SCARLETT2_HWDEP_SUBMINOR` | `0` |  |
| `SCARLETT2_HWDEP_VERSION` | `` |  |
| `SCARLETT2_IOCTL_PVERSION` | `_IOR('S', 0x60, int)` |  |
| `SCARLETT2_IOCTL_REBOOT` | `_IO('S', 0x61)` |  |
| `SCARLETT2_SEGMENT_ID_SETTINGS` | `0` |  |
| `SCARLETT2_SEGMENT_ID_FIRMWARE` | `1` |  |
| `SCARLETT2_SEGMENT_ID_COUNT` | `2` |  |
| `SCARLETT2_IOCTL_SELECT_FLASH_SEGMENT` | `_IOW('S', 0x62, int)` |  |
| `SCARLETT2_IOCTL_ERASE_FLASH_SEGMENT` | `_IO('S', 0x63)` |  |
| `SCARLETT2_IOCTL_GET_ERASE_PROGRESS` | `` |  |

## Structs (1)


### `struct scarlett2_flash_segment_erase_progress`

| Type | Field | Array |
|------|-------|-------|