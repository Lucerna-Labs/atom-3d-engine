# hyperv.h

**Source:** `hyperv.h`


## Includes

- `linux/types.h`

## Defines (38 total)


### ADDR_FAMILY (3)

| Name | Value | Comment |
|------|-------|---------|
| `ADDR_FAMILY_NONE` | `0x00` |  |
| `ADDR_FAMILY_IPV4` | `0x01` |  |
| `ADDR_FAMILY_IPV6` | `0x02` |  |

### FCOPY_CURRENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FCOPY_CURRENT_VERSION` | `FCOPY_VERSION_1` |  |

### FCOPY_VERSION (2)

| Name | Value | Comment |
|------|-------|---------|
| `FCOPY_VERSION_0` | `0` |  |
| `FCOPY_VERSION_1` | `1` |  |

### HV_E (1)

| Name | Value | Comment |
|------|-------|---------|
| `HV_E_FAIL` | `0x80004005` |  |

### HV_ERROR (5)

| Name | Value | Comment |
|------|-------|---------|
| `HV_ERROR_NOT_SUPPORTED` | `0x80070032` |  |
| `HV_ERROR_MACHINE_LOCKED` | `0x800704F7` |  |
| `HV_ERROR_DEVICE_NOT_CONNECTED` | `0x8007048F` |  |
| `HV_ERROR_ALREADY_EXISTS` | `0x80070050` |  |
| `HV_ERROR_DISK_FULL` | `0x80070070` |  |

### HV_GUID (1)

| Name | Value | Comment |
|------|-------|---------|
| `HV_GUID_NOTFOUND` | `0x80041002` |  |

### HV_KVP (2)

| Name | Value | Comment |
|------|-------|---------|
| `HV_KVP_EXCHANGE_MAX_VALUE_SIZE` | `(2048)` |  |
| `HV_KVP_EXCHANGE_MAX_KEY_SIZE` | `(512)` |  |

### HV_S (2)

| Name | Value | Comment |
|------|-------|---------|
| `HV_S_OK` | `0x00000000` |  |
| `HV_S_CONT` | `0x80070103` |  |

### KVP_OP (2)

| Name | Value | Comment |
|------|-------|---------|
| `KVP_OP_REGISTER` | `4` |  |
| `KVP_OP_REGISTER1` | `100` |  |

### MAX_ADAPTER (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_ADAPTER_ID_SIZE` | `128` |  |

### MAX_GATEWAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_GATEWAY_SIZE` | `512` |  |

### MAX_IP (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_IP_ADDR_SIZE` | `1024` |  |

### UNCATEGORIZED (9)

| Name | Value | Comment |
|------|-------|---------|
| `UTIL_WS2K8_FW_MAJOR` | `1` |  |
| `UTIL_WS2K8_FW_VERSION` | `(UTIL_WS2K8_FW_MAJOR << 16 \| UTIL_FW_MINOR)` |  |
| `OVER_WRITE` | `0x1` |  |
| `CREATE_PATH` | `0x2` |  |
| `DATA_FRAGMENT` | `(6 * 1024)` |  |
| `REG_SZ` | `1` |  |
| `REG_U32` | `4` |  |
| `REG_U64` | `8` |  |
| `HV_INVALIDARG` | `0x80070057` |  |

### UTIL_FW (3)

| Name | Value | Comment |
|------|-------|---------|
| `UTIL_FW_MINOR` | `0` |  |
| `UTIL_FW_MAJOR` | `3` |  |
| `UTIL_FW_VERSION` | `(UTIL_FW_MAJOR << 16 \| UTIL_FW_MINOR)` |  |

### VSS_HBU (1)

| Name | Value | Comment |
|------|-------|---------|
| `VSS_HBU_NO_AUTO_RECOVERY` | `0x00000005` |  |

### VSS_OP (2)

| Name | Value | Comment |
|------|-------|---------|
| `VSS_OP_REGISTER` | `128` |  |
| `VSS_OP_REGISTER1` | `129` |  |

### W_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `W_MAX_PATH` | `260` |  |

## Structs (17)


### `struct hv_vss_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `operation` | `-` |
| `__u8` | `reserved` | `7` |

### `struct hv_vss_check_feature`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |

### `struct hv_vss_check_dm_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |

### `struct hv_vss_msg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `error` | `-` |

### `struct hv_fcopy_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `operation` | `-` |
| `__u8` | `service_id0` | `16` |
| `__u8` | `service_id1` | `16` |

### `struct hv_start_fcopy`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `file_name` | `W_MAX_PATH` |
| `__u16` | `path_name` | `W_MAX_PATH` |
| `__u32` | `copy_flags` | `-` |
| `__u64` | `file_size` | `-` |

### `struct hv_do_fcopy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u32` | `size` | `-` |
| `__u8` | `data` | `DATA_FRAGMENT` |

### `struct hv_kvp_ipaddr_value`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `adapter_id` | `MAX_ADAPTER_ID_SIZE` |
| `__u8` | `addr_family` | `-` |
| `__u8` | `dhcp_enabled` | `-` |
| `__u16` | `ip_addr` | `MAX_IP_ADDR_SIZE` |
| `__u16` | `sub_net` | `MAX_IP_ADDR_SIZE` |
| `__u16` | `gate_way` | `MAX_GATEWAY_SIZE` |
| `__u16` | `dns_addr` | `MAX_IP_ADDR_SIZE` |

### `struct hv_kvp_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `operation` | `-` |
| `__u8` | `pool` | `-` |
| `__u16` | `pad` | `-` |

### `struct hv_kvp_exchg_msg_value`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `value_type` | `-` |
| `__u32` | `key_size` | `-` |
| `__u32` | `value_size` | `-` |
| `__u8` | `key` | `HV_KVP_EXCHANGE_MAX_KEY_SIZE` |
| `__u8` | `value` | `HV_KVP_EXCHANGE_MAX_VALUE_SIZE` |
| `__u32` | `value_u32` | `-` |
| `__u64` | `value_u64` | `-` |

### `struct hv_kvp_msg_enumerate`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `index` | `-` |

### `struct hv_kvp_msg_get`

| Type | Field | Array |
|------|-------|-------|

### `struct hv_kvp_msg_set`

| Type | Field | Array |
|------|-------|-------|

### `struct hv_kvp_msg_delete`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `key_size` | `-` |
| `__u8` | `key` | `HV_KVP_EXCHANGE_MAX_KEY_SIZE` |

### `struct hv_kvp_register`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `version` | `HV_KVP_EXCHANGE_MAX_KEY_SIZE` |

### `struct hv_kvp_msg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `error` | `-` |

### `struct hv_kvp_ip_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `operation` | `-` |
| `__u8` | `pool` | `-` |