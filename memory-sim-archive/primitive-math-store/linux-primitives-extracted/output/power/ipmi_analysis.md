# ipmi.h

**Source:** `ipmi.h`


## Includes

- `linux/ipmi_msgdefs.h`
- `linux/compiler.h`

## Defines (42 total)


### IPMICTL_GET (6)

| Name | Value | Comment |
|------|-------|---------|
| `IPMICTL_GET_MY_CHANNEL_ADDRESS_CMD` | `` |  |
| `IPMICTL_GET_MY_CHANNEL_LUN_CMD` | `` |  |
| `IPMICTL_GET_MY_ADDRESS_CMD` | `_IOR(IPMI_IOC_MAGIC, 18, unsigned int)` |  |
| `IPMICTL_GET_MY_LUN_CMD` | `_IOR(IPMI_IOC_MAGIC, 20, unsigned int)` |  |
| `IPMICTL_GET_TIMING_PARMS_CMD` | `_IOR(IPMI_IOC_MAGIC, 23, ` |  |
| `IPMICTL_GET_MAINTENANCE_MODE_CMD` | `_IOR(IPMI_IOC_MAGIC, 30, int)` |  |

### IPMICTL_RECEIVE (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMICTL_RECEIVE_MSG` | `_IOWR(IPMI_IOC_MAGIC, 12,	` |  |
| `IPMICTL_RECEIVE_MSG_TRUNC` | `_IOWR(IPMI_IOC_MAGIC, 11,	` |  |

### IPMICTL_REGISTER (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMICTL_REGISTER_FOR_CMD` | `_IOR(IPMI_IOC_MAGIC, 14,	` |  |
| `IPMICTL_REGISTER_FOR_CMD_CHANS` | `_IOR(IPMI_IOC_MAGIC, 28,	` |  |

### IPMICTL_SEND (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMICTL_SEND_COMMAND` | `_IOR(IPMI_IOC_MAGIC, 13,	` |  |
| `IPMICTL_SEND_COMMAND_SETTIME` | `_IOR(IPMI_IOC_MAGIC, 21,	` |  |

### IPMICTL_SET (7)

| Name | Value | Comment |
|------|-------|---------|
| `IPMICTL_SET_GETS_EVENTS_CMD` | `_IOR(IPMI_IOC_MAGIC, 16, int)` |  |
| `IPMICTL_SET_MY_CHANNEL_ADDRESS_CMD` | `` |  |
| `IPMICTL_SET_MY_CHANNEL_LUN_CMD` | `` |  |
| `IPMICTL_SET_MY_ADDRESS_CMD` | `_IOR(IPMI_IOC_MAGIC, 17, unsigned int)` |  |
| `IPMICTL_SET_MY_LUN_CMD` | `_IOR(IPMI_IOC_MAGIC, 19, unsigned int)` |  |
| `IPMICTL_SET_TIMING_PARMS_CMD` | `_IOR(IPMI_IOC_MAGIC, 22, ` |  |
| `IPMICTL_SET_MAINTENANCE_MODE_CMD` | `_IOW(IPMI_IOC_MAGIC, 31, int)` |  |

### IPMICTL_UNREGISTER (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMICTL_UNREGISTER_FOR_CMD` | `_IOR(IPMI_IOC_MAGIC, 15,	` |  |
| `IPMICTL_UNREGISTER_FOR_CMD_CHANS` | `_IOR(IPMI_IOC_MAGIC, 29,	` |  |

### IPMI_ASYNC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_ASYNC_EVENT_RECV_TYPE` | `2` | Something from the event queue |

### IPMI_BMC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_BMC_CHANNEL` | `0xf` |  |

### IPMI_CHAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_CHAN_ALL` | `(~0)` |  |

### IPMI_CMD (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_CMD_RECV_TYPE` | `3` | A command from somewhere else |

### IPMI_INVALID (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_INVALID_CMD_COMPLETION_CODE` | `0xC1` |  |

### IPMI_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_IOC_MAGIC` | `'i'` |  |

### IPMI_IPMB (3)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_IPMB_ADDR_TYPE` | `0x01` |  |
| `IPMI_IPMB_BROADCAST_ADDR_TYPE` | `0x41` |  |
| `IPMI_IPMB_DIRECT_ADDR_TYPE` | `0x81` |  |

### IPMI_LAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_LAN_ADDR_TYPE` | `0x04` |  |

### IPMI_MAINTENANCE (3)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_MAINTENANCE_MODE_AUTO` | `0` |  |
| `IPMI_MAINTENANCE_MODE_OFF` | `1` |  |
| `IPMI_MAINTENANCE_MODE_ON` | `2` |  |

### IPMI_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_MAX_ADDR_SIZE` | `32` |  |

### IPMI_NUM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_NUM_CHANNELS` | `0x10` |  |

### IPMI_OEM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_OEM_RECV_TYPE` | `5` | The response for OEM Channels |

### IPMI_RESPONSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_RESPONSE_RECV_TYPE` | `1` | A response to a command |
| `IPMI_RESPONSE_RESPONSE_TYPE` | `4 /* The response for` |  |

### IPMI_SYSTEM (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_SYSTEM_INTERFACE_ADDR_TYPE` | `0x0c` |  |

### IPMI_TIMEOUT (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_TIMEOUT_COMPLETION_CODE` | `0xC3` |  |

### IPMI_UNKNOWN (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPMI_UNKNOWN_ERR_COMPLETION_CODE` | `0xff` |  |

## Structs (14)


### `struct ipmi_addr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `addr_type` | `-` |
| `short` | `channel` | `-` |
| `char` | `data` | `IPMI_MAX_ADDR_SIZE` |

### `struct ipmi_system_interface_addr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `addr_type` | `-` |
| `short` | `channel` | `-` |

### `struct ipmi_ipmb_addr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `addr_type` | `-` |
| `short` | `channel` | `-` |

### `struct ipmi_ipmb_direct_addr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `addr_type` | `-` |
| `short` | `channel` | `-` |

### `struct ipmi_lan_addr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `addr_type` | `-` |
| `short` | `channel` | `-` |

### `struct ipmi_msg`

| Type | Field | Array |
|------|-------|-------|

### `struct kernel_ipmi_msg`

| Type | Field | Array |
|------|-------|-------|

### `struct ipmi_req`

| Type | Field | Array |
|------|-------|-------|
| `long` | `msgid` | `-` |

### `struct ipmi_req_settime`

| Type | Field | Array |
|------|-------|-------|
| `int` | `retries` | `-` |

### `struct ipmi_recv`

| Type | Field | Array |
|------|-------|-------|
| `int` | `recv_type` | `-` |
| `long` | `msgid` | `-` |

### `struct ipmi_cmdspec`

| Type | Field | Array |
|------|-------|-------|

### `struct ipmi_cmdspec_chans`

| Type | Field | Array |
|------|-------|-------|

### `struct ipmi_channel_lun_address_set`

| Type | Field | Array |
|------|-------|-------|

### `struct ipmi_timing_parms`

| Type | Field | Array |
|------|-------|-------|
| `int` | `retries` | `-` |