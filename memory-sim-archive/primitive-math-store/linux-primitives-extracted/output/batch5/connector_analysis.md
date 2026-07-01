# connector.h

**Source:** `connector.h`


## Includes

- `linux/types.h`

## Defines (21 total)


### CN_DST (2)

| Name | Value | Comment |
|------|-------|---------|
| `CN_DST_IDX` | `0x6` |  |
| `CN_DST_VAL` | `0x1` |  |

### CN_IDX (6)

| Name | Value | Comment |
|------|-------|---------|
| `CN_IDX_PROC` | `0x1` |  |
| `CN_IDX_CIFS` | `0x2` |  |
| `CN_IDX_V86D` | `0x4` |  |
| `CN_IDX_BB` | `0x5` | BlackBoard, from the TSP GPL sampling framework |
| `CN_IDX_DM` | `0x7` | Device Mapper |
| `CN_IDX_DRBD` | `0x8` |  |

### CN_KVP (2)

| Name | Value | Comment |
|------|-------|---------|
| `CN_KVP_IDX` | `0x9` | HyperV KVP |
| `CN_KVP_VAL` | `0x1` | queries from the kernel |

### CN_NETLINK (1)

| Name | Value | Comment |
|------|-------|---------|
| `CN_NETLINK_USERS` | `11` | Highest index + 1 |

### CN_VAL (5)

| Name | Value | Comment |
|------|-------|---------|
| `CN_VAL_PROC` | `0x1` |  |
| `CN_VAL_CIFS` | `0x1` |  |
| `CN_VAL_V86D_UVESAFB` | `0x1` |  |
| `CN_VAL_DM_USERSPACE_LOG` | `0x1` |  |
| `CN_VAL_DRBD` | `0x1` |  |

### CN_VSS (2)

| Name | Value | Comment |
|------|-------|---------|
| `CN_VSS_IDX` | `0xA` | HyperV VSS |
| `CN_VSS_VAL` | `0x1` | queries from the kernel |

### CONNECTOR_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `CONNECTOR_MAX_MSG_SIZE` | `16384` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `CN_W1_IDX` | `0x3` | w1 communication |
| `CN_W1_VAL` | `0x1` |  |

## Structs (2)


### `struct cb_id`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `idx` | `-` |
| `__u32` | `val` | `-` |

### `struct cn_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `seq` | `-` |
| `__u32` | `ack` | `-` |
| `__u16` | `len` | `-` |
| `__u16` | `flags` | `-` |