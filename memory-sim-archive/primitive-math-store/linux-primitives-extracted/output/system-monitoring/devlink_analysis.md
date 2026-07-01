# devlink.h

**Source:** `devlink.h`


## Includes

- `linux/const.h`

## Defines (19 total)


### DEVLINK_CMD (2)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_CMD_ESWITCH_MODE_GET` | `/* obsolete, never use this! */ ` |  |
| `DEVLINK_CMD_ESWITCH_MODE_SET` | `/* obsolete, never use this! */ ` |  |

### DEVLINK_FLASH (2)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_FLASH_OVERWRITE_SETTINGS` | `_BITUL(DEVLINK_FLASH_OVERWRITE_SETTINGS_BIT)` |  |
| `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS` | `_BITUL(DEVLINK_FLASH_OVERWRITE_IDENTIFIERS_BIT)` |  |

### DEVLINK_GENL (3)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_GENL_NAME` | `"devlink"` |  |
| `DEVLINK_GENL_VERSION` | `0x1` |  |
| `DEVLINK_GENL_MCGRP_CONFIG_NAME` | `"config"` |  |

### DEVLINK_INDEX (1)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_INDEX_BUS_NAME` | `"devlink_index"` |  |

### DEVLINK_PORT (4)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_PORT_FN_CAP_ROCE` | `_BITUL(DEVLINK_PORT_FN_ATTR_CAP_ROCE_BIT)` |  |
| `DEVLINK_PORT_FN_CAP_MIGRATABLE` | `` |  |
| `DEVLINK_PORT_FN_CAP_IPSEC_CRYPTO` | `_BITUL(DEVLINK_PORT_FN_ATTR_CAP_IPSEC_CRYPTO_BIT)` |  |
| `DEVLINK_PORT_FN_CAP_IPSEC_PACKET` | `_BITUL(DEVLINK_PORT_FN_ATTR_CAP_IPSEC_PACKET_BIT)` |  |

### DEVLINK_RATE (2)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_RATE_TCS_MAX` | `8` |  |
| `DEVLINK_RATE_TC_INDEX_MAX` | `(DEVLINK_RATE_TCS_MAX - 1)` |  |

### DEVLINK_RELOAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_RELOAD_LIMITS_VALID_MASK` | `(_BITUL(__DEVLINK_RELOAD_LIMIT_MAX) - 1)` |  |

### DEVLINK_RESOURCE (2)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_RESOURCE_SCOPE_DEV` | `` |  |
| `DEVLINK_RESOURCE_SCOPE_PORT` | `` |  |

### DEVLINK_SB (1)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_SB_THRESHOLD_TO_ALPHA_MAX` | `20` |  |

### DEVLINK_SUPPORTED (1)

| Name | Value | Comment |
|------|-------|---------|
| `DEVLINK_SUPPORTED_FLASH_OVERWRITE_SECTIONS` | `` |  |