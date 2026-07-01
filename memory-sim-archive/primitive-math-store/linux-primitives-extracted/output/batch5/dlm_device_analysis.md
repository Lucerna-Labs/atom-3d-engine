# dlm_device.h

**Source:** `dlm_device.h`


## Includes

- `linux/dlm.h`
- `linux/types.h`

## Defines (13 total)


### DLM_DEVICE (3)

| Name | Value | Comment |
|------|-------|---------|
| `DLM_DEVICE_VERSION_MAJOR` | `6` |  |
| `DLM_DEVICE_VERSION_MINOR` | `0` |  |
| `DLM_DEVICE_VERSION_PATCH` | `2` |  |

### DLM_USER (10)

| Name | Value | Comment |
|------|-------|---------|
| `DLM_USER_LVB_LEN` | `32` |  |
| `DLM_USER_LOCK` | `1` |  |
| `DLM_USER_UNLOCK` | `2` |  |
| `DLM_USER_QUERY` | `3` |  |
| `DLM_USER_CREATE_LOCKSPACE` | `4` |  |
| `DLM_USER_REMOVE_LOCKSPACE` | `5` |  |
| `DLM_USER_PURGE` | `6` |  |
| `DLM_USER_DEADLOCK` | `7` |  |
| `DLM_USER_LSFLG_AUTOFREE` | `1` |  |
| `DLM_USER_LSFLG_FORCEFREE` | `2` |  |

## Structs (6)


### `struct dlm_lock_params`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `mode` | `-` |
| `__u8` | `namelen` | `-` |
| `__u16` | `unused` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `lkid` | `-` |
| `__u32` | `parent` | `-` |
| `__u64` | `xid` | `-` |
| `__u64` | `timeout` | `-` |
| `char` | `lvb` | `DLM_USER_LVB_LEN` |

### `struct dlm_lspace_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `minor` | `-` |

### `struct dlm_purge_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nodeid` | `-` |
| `__u32` | `pid` | `-` |

### `struct dlm_write_request`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `3` |
| `__u8` | `cmd` | `-` |
| `__u8` | `is64bit` | `-` |
| `__u8` | `unused` | `2` |

### `struct dlm_device_version`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `3` |

### `struct dlm_lock_result`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `3` |
| `__u32` | `length` | `-` |
| `__u8` | `bast_mode` | `-` |
| `__u8` | `unused` | `3` |
| `__u32` | `lvb_offset` | `-` |