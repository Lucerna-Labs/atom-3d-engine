# can.h

**Source:** `can.h`


## Includes

- `linux/types.h`
- `linux/socket.h`
- `linux/stddef.h`

## Defines (45 total)


### CANFD_MAX (2)

| Name | Value | Comment |
|------|-------|---------|
| `CANFD_MAX_DLC` | `15` |  |
| `CANFD_MAX_DLEN` | `64` |  |

### CANXL_HDR (1)

| Name | Value | Comment |
|------|-------|---------|
| `CANXL_HDR_SIZE` | `(offsetof(struct canxl_frame, data))` |  |

### CANXL_MAX (4)

| Name | Value | Comment |
|------|-------|---------|
| `CANXL_MAX_DLC` | `2047` |  |
| `CANXL_MAX_DLC_MASK` | `0x07FF` |  |
| `CANXL_MAX_DLEN` | `2048` |  |
| `CANXL_MAX_MTU` | `CANXL_MTU` |  |

### CANXL_MIN (3)

| Name | Value | Comment |
|------|-------|---------|
| `CANXL_MIN_DLC` | `0` |  |
| `CANXL_MIN_DLEN` | `1` |  |
| `CANXL_MIN_MTU` | `(CANXL_HDR_SIZE + 64)` |  |

### CANXL_PRIO (2)

| Name | Value | Comment |
|------|-------|---------|
| `CANXL_PRIO_MASK` | `CAN_SFF_MASK` | 11 bit priority mask |
| `CANXL_PRIO_BITS` | `CAN_SFF_ID_BITS` |  |

### CANXL_VCID (3)

| Name | Value | Comment |
|------|-------|---------|
| `CANXL_VCID_OFFSET` | `16` | bit offset of VCID in prio element |
| `CANXL_VCID_VAL_MASK` | `0xFFUL` | VCID is an 8-bit value |
| `CANXL_VCID_MASK` | `(CANXL_VCID_VAL_MASK << CANXL_VCID_OFFSET)` |  |

### CAN_EFF (3)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_EFF_FLAG` | `0x80000000U` | EFF/SFF is set in the MSB |
| `CAN_EFF_MASK` | `0x1FFFFFFFU` | extended frame format (EFF) |
| `CAN_EFF_ID_BITS` | `29` |  |

### CAN_ERR (2)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_ERR_FLAG` | `0x20000000U` | error message frame |
| `CAN_ERR_MASK` | `0x1FFFFFFFU` | omit EFF, RTR, ERR flags |

### CAN_INV (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_INV_FILTER` | `0x20000000U` | to be set in can_filter.can_id |

### CAN_MAX (3)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_MAX_DLC` | `8` |  |
| `CAN_MAX_RAW_DLC` | `15` |  |
| `CAN_MAX_DLEN` | `8` |  |

### CAN_RTR (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_RTR_FLAG` | `0x40000000U` | remote transmission request |

### CAN_SFF (2)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_SFF_MASK` | `0x000007FFU` | standard frame format (SFF) |
| `CAN_SFF_ID_BITS` | `11` |  |

### SOL_CAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOL_CAN_BASE` | `100` |  |

### UNCATEGORIZED (17)

| Name | Value | Comment |
|------|-------|---------|
| `CANFD_BRS` | `0x01` | bit rate switch (second bitrate for payload data) |
| `CANFD_ESI` | `0x02` | error state indicator of the transmitting node |
| `CANFD_FDF` | `0x04` | mark CAN FD for dual use of struct canfd_frame |
| `CANXL_XLF` | `0x80` | mandatory CAN XL frame flag (must always be set!) |
| `CANXL_SEC` | `0x01` | Simple Extended Content (security/segmentation) |
| `CANXL_RRS` | `0x02` | Remote Request Substitution |
| `CAN_MTU` | `(sizeof(struct can_frame))` |  |
| `CANFD_MTU` | `(sizeof(struct canfd_frame))` |  |
| `CANXL_MTU` | `(sizeof(struct canxl_frame))` |  |
| `CAN_RAW` | `1` | RAW sockets |
| `CAN_BCM` | `2` | Broadcast Manager |
| `CAN_TP16` | `3` | VAG Transport Protocol v1.6 |
| `CAN_TP20` | `4` | VAG Transport Protocol v2.0 |
| `CAN_MCNET` | `5` | Bosch MCNet |
| `CAN_ISOTP` | `6` | ISO 15765-2 Transport Protocol |
| `CAN_J1939` | `7` | SAE J1939 |
| `CAN_NPROTO` | `8` |  |

## Structs (7)


### `struct can_frame`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `canid_t` | `can_id` | `-` |
| `__u8` | `len` | `-` |
| `__u8` | `can_dlc` | `-` |
| `__u8` | `__pad` | `-` |
| `__u8` | `__res0` | `-` |
| `__u8` | `len8_dlc` | `-` |

### `struct canfd_frame`

| Type | Field | Array |
|------|-------|-------|
| `canid_t` | `can_id` | `-` |
| `__u8` | `len` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `__res0` | `-` |
| `__u8` | `__res1` | `-` |

### `struct canxl_frame`

| Type | Field | Array |
|------|-------|-------|
| `canid_t` | `prio` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `sdt` | `-` |
| `__u16` | `len` | `-` |
| `__u32` | `af` | `-` |
| `__u8` | `data` | `CANXL_MAX_DLEN` |

### `struct sockaddr_can`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `can_family` | `-` |
| `int` | `can_ifindex` | `-` |
| `__u64` | `name` | `-` |
| `__u32` | `pgn` | `-` |
| `__u8` | `addr` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `name` | `-` |
| `__u32` | `pgn` | `-` |
| `__u8` | `addr` | `-` |

### `struct can_filter`

| Type | Field | Array |
|------|-------|-------|
| `canid_t` | `can_id` | `-` |
| `canid_t` | `can_mask` | `-` |