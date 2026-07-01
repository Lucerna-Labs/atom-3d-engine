# errqueue.h

**Source:** `errqueue.h`


## Includes

- `linux/types.h`
- `linux/time_types.h`

## Defines (12 total)


### SO_EE (12)

| Name | Value | Comment |
|------|-------|---------|
| `SO_EE_ORIGIN_NONE` | `0` |  |
| `SO_EE_ORIGIN_LOCAL` | `1` |  |
| `SO_EE_ORIGIN_ICMP` | `2` |  |
| `SO_EE_ORIGIN_ICMP6` | `3` |  |
| `SO_EE_ORIGIN_TXSTATUS` | `4` |  |
| `SO_EE_ORIGIN_ZEROCOPY` | `5` |  |
| `SO_EE_ORIGIN_TXTIME` | `6` |  |
| `SO_EE_ORIGIN_TIMESTAMPING` | `SO_EE_ORIGIN_TXSTATUS` |  |
| `SO_EE_CODE_ZEROCOPY_COPIED` | `1` |  |
| `SO_EE_CODE_TXTIME_INVALID_PARAM` | `1` |  |
| `SO_EE_CODE_TXTIME_MISSED` | `2` |  |
| `SO_EE_RFC4884_FLAG_INVALID` | `1` |  |

## Structs (4)


### `struct sock_ee_data_rfc4884`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `len` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `reserved` | `-` |

### `struct sock_extended_err`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ee_errno` | `-` |
| `__u8` | `ee_origin` | `-` |
| `__u8` | `ee_type` | `-` |
| `__u8` | `ee_code` | `-` |
| `__u8` | `ee_pad` | `-` |
| `__u32` | `ee_info` | `-` |
| `__u32` | `ee_data` | `-` |

### `struct scm_timestamping`

| Type | Field | Array |
|------|-------|-------|

### `struct scm_timestamping64`

| Type | Field | Array |
|------|-------|-------|