# mptcp.h

**Source:** `mptcp.h`


## Includes

- `netinet/in.h`
- `sys/socket.h`
- `linux/const.h`
- `linux/types.h`
- `linux/in.h`
- `linux/in6.h`
- `linux/socket.h`
- `linux/mptcp_pm.h`

## Defines (38 total)


### MPTCP_FULL (1)

| Name | Value | Comment |
|------|-------|---------|
| `MPTCP_FULL_INFO` | `4` |  |

### MPTCP_INFO (2)

| Name | Value | Comment |
|------|-------|---------|
| `MPTCP_INFO_FLAG_FALLBACK` | `_BITUL(0)` |  |
| `MPTCP_INFO_FLAG_REMOTE_KEY_RECEIVED` | `_BITUL(1)` |  |

### MPTCP_PM (11)

| Name | Value | Comment |
|------|-------|---------|
| `MPTCP_PM_CMD_GRP_NAME` | `"mptcp_pm_cmds"` |  |
| `MPTCP_PM_EV_GRP_NAME` | `"mptcp_pm_events"` |  |
| `MPTCP_PM_EV_FLAG_DENY_JOIN_ID0` | `_BITUL(0)` |  |
| `MPTCP_PM_EV_FLAG_SERVER_SIDE` | `_BITUL(1)` |  |
| `MPTCP_PM_ADDR_FLAG_SIGNAL` | `_BITUL(0)` |  |
| `MPTCP_PM_ADDR_FLAG_SUBFLOW` | `_BITUL(1)` |  |
| `MPTCP_PM_ADDR_FLAG_BACKUP` | `_BITUL(2)` |  |
| `MPTCP_PM_ADDR_FLAG_FULLMESH` | `_BITUL(3)` |  |
| `MPTCP_PM_ADDR_FLAG_IMPLICIT` | `_BITUL(4)` |  |
| `MPTCP_PM_ADDR_FLAG_LAMINAR` | `_BITUL(5)` |  |
| `MPTCP_PM_ADDR_FLAGS_MASK` | `GENMASK(5, 0)` |  |

### MPTCP_RST (7)

| Name | Value | Comment |
|------|-------|---------|
| `MPTCP_RST_EUNSPEC` | `0` |  |
| `MPTCP_RST_EMPTCP` | `1` |  |
| `MPTCP_RST_ERESOURCE` | `2` |  |
| `MPTCP_RST_EPROHIBIT` | `3` |  |
| `MPTCP_RST_EWQ2BIG` | `4` |  |
| `MPTCP_RST_EBADPERF` | `5` |  |
| `MPTCP_RST_EMIDDLEBOX` | `6` |  |

### MPTCP_SUBFLOW (10)

| Name | Value | Comment |
|------|-------|---------|
| `MPTCP_SUBFLOW_FLAG_MCAP_REM` | `_BITUL(0)` |  |
| `MPTCP_SUBFLOW_FLAG_MCAP_LOC` | `_BITUL(1)` |  |
| `MPTCP_SUBFLOW_FLAG_JOIN_REM` | `_BITUL(2)` |  |
| `MPTCP_SUBFLOW_FLAG_JOIN_LOC` | `_BITUL(3)` |  |
| `MPTCP_SUBFLOW_FLAG_BKUP_REM` | `_BITUL(4)` |  |
| `MPTCP_SUBFLOW_FLAG_BKUP_LOC` | `_BITUL(5)` |  |
| `MPTCP_SUBFLOW_FLAG_FULLY_ESTABLISHED` | `_BITUL(6)` |  |
| `MPTCP_SUBFLOW_FLAG_CONNECTED` | `_BITUL(7)` |  |
| `MPTCP_SUBFLOW_FLAG_MAPVALID` | `_BITUL(8)` |  |
| `MPTCP_SUBFLOW_ADDRS` | `3` |  |

### UNCATEGORIZED (7)

| Name | Value | Comment |
|------|-------|---------|
| `mptcpi_extra_subflows` | `mptcpi_subflows` |  |
| `mptcpi_limit_extra_subflows` | `mptcpi_subflows_max` |  |
| `mptcpi_endp_signal_max` | `mptcpi_add_addr_signal_max` |  |
| `mptcpi_limit_add_addr_accepted` | `mptcpi_add_addr_accepted_max` |  |
| `mptcpi_endp_subflow_max` | `mptcpi_local_addr_max` |  |
| `MPTCP_INFO` | `1` |  |
| `MPTCP_TCPINFO` | `2` |  |

## Structs (5)


### `struct mptcp_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `mptcpi_subflows` | `-` |
| `__u8` | `mptcpi_add_addr_signal` | `-` |
| `__u8` | `mptcpi_add_addr_accepted` | `-` |
| `__u8` | `mptcpi_subflows_max` | `-` |
| `__u8` | `mptcpi_add_addr_signal_max` | `-` |
| `__u8` | `mptcpi_add_addr_accepted_max` | `-` |
| `__u32` | `mptcpi_flags` | `-` |
| `__u32` | `mptcpi_token` | `-` |
| `__u64` | `mptcpi_write_seq` | `-` |
| `__u64` | `mptcpi_snd_una` | `-` |
| `__u64` | `mptcpi_rcv_nxt` | `-` |
| `__u8` | `mptcpi_local_addr_used` | `-` |
| `__u8` | `mptcpi_local_addr_max` | `-` |
| `__u8` | `mptcpi_csum_enabled` | `-` |
| `__u32` | `mptcpi_retransmits` | `-` |
| `__u64` | `mptcpi_bytes_retrans` | `-` |
| `__u64` | `mptcpi_bytes_sent` | `-` |
| `__u64` | `mptcpi_bytes_received` | `-` |
| `__u64` | `mptcpi_bytes_acked` | `-` |
| `__u8` | `mptcpi_subflows_total` | `-` |
| `__u8` | `mptcpi_endp_laminar_max` | `-` |
| `__u8` | `mptcpi_endp_fullmesh_max` | `-` |
| `__u8` | `reserved` | `-` |
| `__u32` | `mptcpi_last_data_sent` | `-` |
| `__u32` | `mptcpi_last_data_recv` | `-` |
| `__u32` | `mptcpi_last_ack_recv` | `-` |

### `struct mptcp_subflow_data`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size_subflow_data` | `-` |
| `__u32` | `num_subflows` | `-` |
| `__u32` | `size_kernel` | `-` |
| `__u32` | `size_user` | `-` |

### `struct mptcp_subflow_addrs`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `sa_family` | `-` |

### `struct mptcp_subflow_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |

### `struct mptcp_full_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size_tcpinfo_kernel` | `-` |
| `__u32` | `size_tcpinfo_user` | `-` |
| `__u32` | `size_sfinfo_kernel` | `-` |
| `__u32` | `size_sfinfo_user` | `-` |
| `__u32` | `num_subflows` | `-` |
| `__u32` | `size_arrays_user` | `-` |
| `__aligned_u64` | `subflow_info` | `-` |
| `__aligned_u64` | `tcp_info` | `-` |