# ipmi_bmc.h

**Source:** `ipmi_bmc.h`


## Includes

- `linux/ioctl.h`

## Defines (4 total)


### IPMI_BMC (3)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_BMC_IOCTL_SET_SMS_ATN` | `_IO(__IPMI_BMC_IOCTL_MAGIC, 0x00)` |  |
| `IPMI_BMC_IOCTL_CLEAR_SMS_ATN` | `_IO(__IPMI_BMC_IOCTL_MAGIC, 0x01)` |  |
| `IPMI_BMC_IOCTL_FORCE_ABORT` | `_IO(__IPMI_BMC_IOCTL_MAGIC, 0x02)` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `__IPMI_BMC_IOCTL_MAGIC` | `0xB1` |  |