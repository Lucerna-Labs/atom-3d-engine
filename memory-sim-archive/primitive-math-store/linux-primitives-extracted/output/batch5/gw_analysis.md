# gw.h

**Source:** `gw.h`


## Includes

- `linux/types.h`
- `linux/can.h`

## Defines (19 total)


### CGW_CS (2)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_CS_XOR_LEN` | `sizeof(struct cgw_csum_xor)` |  |
| `CGW_CS_CRC8_LEN` | `sizeof(struct cgw_csum_crc8)` |  |

### CGW_FDMODATTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_FDMODATTR_LEN` | `sizeof(struct cgw_fdframe_mod)` |  |

### CGW_FLAGS (4)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_FLAGS_CAN_ECHO` | `0x01` |  |
| `CGW_FLAGS_CAN_SRC_TSTAMP` | `0x02` |  |
| `CGW_FLAGS_CAN_IIF_TX_OK` | `0x04` |  |
| `CGW_FLAGS_CAN_FD` | `0x08` |  |

### CGW_FRAME (1)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_FRAME_MODS` | `4` | ID DLC/LEN DATA FLAGS |

### CGW_MOD (6)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_MOD_FUNCS` | `4` | AND OR XOR SET |
| `CGW_MOD_ID` | `0x01` |  |
| `CGW_MOD_DLC` | `0x02` | Classical CAN data length code |
| `CGW_MOD_LEN` | `CGW_MOD_DLC` | CAN FD (plain) data length |
| `CGW_MOD_DATA` | `0x04` |  |
| `CGW_MOD_FLAGS` | `0x08` | CAN FD flags |

### CGW_MODATTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_MODATTR_LEN` | `sizeof(struct cgw_frame_mod)` |  |

### CGW_TYPE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_TYPE_MAX` | `(__CGW_TYPE_MAX - 1)` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `CGW_MAX` | `(__CGW_MAX - 1)` |  |
| `MAX_MODFUNCTIONS` | `(CGW_MOD_FUNCS * CGW_FRAME_MODS)` |  |
| `CGW_CRC8PRF_MAX` | `(__CGW_CRC8PRF_MAX - 1)` |  |

## Structs (5)


### `struct rtcanmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `can_family` | `-` |
| `__u8` | `gwtype` | `-` |
| `__u16` | `flags` | `-` |

### `struct cgw_frame_mod`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `modtype` | `-` |

### `struct cgw_fdframe_mod`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `modtype` | `-` |

### `struct cgw_csum_xor`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `from_idx` | `-` |
| `__s8` | `to_idx` | `-` |
| `__s8` | `result_idx` | `-` |
| `__u8` | `init_xor_val` | `-` |

### `struct cgw_csum_crc8`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `from_idx` | `-` |
| `__s8` | `to_idx` | `-` |
| `__s8` | `result_idx` | `-` |
| `__u8` | `init_crc_val` | `-` |
| `__u8` | `final_xor_val` | `-` |
| `__u8` | `crctab` | `256` |
| `__u8` | `profile` | `-` |
| `__u8` | `profile_data` | `20` |