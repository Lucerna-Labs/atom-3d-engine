# ipmi_msgdefs.h

**Source:** `ipmi_msgdefs.h`


## Defines (64 total)


### IPMI_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_ADD_SEL_ENTRY_CMD` | `0x44` |  |

### IPMI_BMC (5)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_BMC_RCV_MSG_INTR` | `0x01` |  |
| `IPMI_BMC_EVT_MSG_INTR` | `0x02` |  |
| `IPMI_BMC_EVT_MSG_BUFF` | `0x04` |  |
| `IPMI_BMC_SYS_LOG` | `0x08` |  |
| `IPMI_BMC_SLAVE_ADDR` | `0x20` |  |

### IPMI_BUS (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_BUS_ERR` | `0x82` |  |

### IPMI_CC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_CC_NO_ERROR` | `0x00` |  |

### IPMI_CHANNEL (22)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_CHANNEL_PROTOCOL_IPMB` | `1` |  |
| `IPMI_CHANNEL_PROTOCOL_ICMB` | `2` |  |
| `IPMI_CHANNEL_PROTOCOL_SMBUS` | `4` |  |
| `IPMI_CHANNEL_PROTOCOL_KCS` | `5` |  |
| `IPMI_CHANNEL_PROTOCOL_SMIC` | `6` |  |
| `IPMI_CHANNEL_PROTOCOL_BT10` | `7` |  |
| `IPMI_CHANNEL_PROTOCOL_BT15` | `8` |  |
| `IPMI_CHANNEL_PROTOCOL_TMODE` | `9` |  |
| `IPMI_CHANNEL_MEDIUM_IPMB` | `1` |  |
| `IPMI_CHANNEL_MEDIUM_ICMB10` | `2` |  |
| `IPMI_CHANNEL_MEDIUM_ICMB09` | `3` |  |
| `IPMI_CHANNEL_MEDIUM_8023LAN` | `4` |  |
| `IPMI_CHANNEL_MEDIUM_ASYNC` | `5` |  |
| `IPMI_CHANNEL_MEDIUM_OTHER_LAN` | `6` |  |
| `IPMI_CHANNEL_MEDIUM_PCI_SMBUS` | `7` |  |
| `IPMI_CHANNEL_MEDIUM_SMBUS1` | `8` |  |
| `IPMI_CHANNEL_MEDIUM_SMBUS2` | `9` |  |
| `IPMI_CHANNEL_MEDIUM_USB1` | `10` |  |
| `IPMI_CHANNEL_MEDIUM_USB2` | `11` |  |
| `IPMI_CHANNEL_MEDIUM_SYSINTF` | `12` |  |
| `IPMI_CHANNEL_MEDIUM_OEM_MIN` | `0x60` |  |
| `IPMI_CHANNEL_MEDIUM_OEM_MAX` | `0x7f` |  |

### IPMI_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_CLEAR_MSG_FLAGS_CMD` | `0x30` |  |

### IPMI_COLD (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_COLD_RESET_CMD` | `0x02` |  |

### IPMI_DEVICE (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_DEVICE_IN_FW_UPDATE_ERR` | `0xd1` |  |
| `IPMI_DEVICE_IN_INIT_ERR` | `0xd2` |  |

### IPMI_ERR (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_ERR_MSG_TRUNCATED` | `0xc6` |  |
| `IPMI_ERR_UNSPECIFIED` | `0xff` |  |

### IPMI_GET (7)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_GET_EVENT_RECEIVER_CMD` | `0x01` |  |
| `IPMI_GET_DEVICE_ID_CMD` | `0x01` |  |
| `IPMI_GET_DEVICE_GUID_CMD` | `0x08` |  |
| `IPMI_GET_MSG_FLAGS_CMD` | `0x31` |  |
| `IPMI_GET_MSG_CMD` | `0x33` |  |
| `IPMI_GET_BMC_GLOBAL_ENABLES_CMD` | `0x2f` |  |
| `IPMI_GET_CHANNEL_INFO_CMD` | `0x42` |  |

### IPMI_INVALID (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_INVALID_COMMAND_ERR` | `0xc1` |  |

### IPMI_LOST (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_LOST_ARBITRATION_ERR` | `0x81` |  |

### IPMI_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_MAX_MSG_LENGTH` | `272` | multiple of 16 |

### IPMI_NAK (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_NAK_ON_WRITE_ERR` | `0x83` |  |

### IPMI_NETFN (8)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_NETFN_SENSOR_EVENT_REQUEST` | `0x04` |  |
| `IPMI_NETFN_SENSOR_EVENT_RESPONSE` | `0x05` |  |
| `IPMI_NETFN_APP_REQUEST` | `0x06` |  |
| `IPMI_NETFN_APP_RESPONSE` | `0x07` |  |
| `IPMI_NETFN_STORAGE_REQUEST` | `0x0a` |  |
| `IPMI_NETFN_STORAGE_RESPONSE` | `0x0b` |  |
| `IPMI_NETFN_FIRMWARE_REQUEST` | `0x08` |  |
| `IPMI_NETFN_FIRMWARE_RESPONSE` | `0x09` |  |

### IPMI_NODE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_NODE_BUSY_ERR` | `0xc0` |  |

### IPMI_NOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_NOT_IN_MY_STATE_ERR` | `0xd5` | IPMI 2.0 |

### IPMI_READ (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_READ_EVENT_MSG_BUFFER_CMD` | `0x35` |  |

### IPMI_REQ (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_REQ_LEN_INVALID_ERR` | `0xc7` |  |
| `IPMI_REQ_LEN_EXCEEDED_ERR` | `0xc8` |  |

### IPMI_SEND (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_SEND_MSG_CMD` | `0x34` |  |

### IPMI_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_SET_BMC_GLOBAL_ENABLES_CMD` | `0x2e` |  |

### IPMI_TIMEOUT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_TIMEOUT_ERR` | `0xc3` |  |

### IPMI_WARM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_WARM_RESET_CMD` | `0x03` |  |