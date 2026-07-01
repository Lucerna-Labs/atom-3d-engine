# j1939.h

**Source:** `j1939.h`


## Includes

- `linux/types.h`
- `linux/socket.h`
- `linux/can.h`

## Defines (12 total)


### SOL_CAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOL_CAN_J1939` | `(SOL_CAN_BASE + CAN_J1939)` |  |

### UNCATEGORIZED (11)

| Name | Value | Comment |
|------|-------|---------|
| `J1939_MAX_UNICAST_ADDR` | `0xfd` |  |
| `J1939_IDLE_ADDR` | `0xfe` |  |
| `J1939_NO_ADDR` | `0xff` | == broadcast or no addr |
| `J1939_NO_NAME` | `0` |  |
| `J1939_PGN_REQUEST` | `0x0ea00` | Request PG |
| `J1939_PGN_ADDRESS_CLAIMED` | `0x0ee00` | Address Claimed |
| `J1939_PGN_ADDRESS_COMMANDED` | `0x0fed8` | Commanded Address |
| `J1939_PGN_PDU1_MAX` | `0x3ff00` |  |
| `J1939_PGN_MAX` | `0x3ffff` |  |
| `J1939_NO_PGN` | `0x40000` |  |
| `J1939_FILTER_MAX` | `512` | maximum number of j1939_filter set via setsockopt() |

## Structs (1)


### `struct j1939_filter`

| Type | Field | Array |
|------|-------|-------|
| `name_t` | `name` | `-` |
| `name_t` | `name_mask` | `-` |
| `pgn_t` | `pgn` | `-` |
| `pgn_t` | `pgn_mask` | `-` |
| `__u8` | `addr` | `-` |
| `__u8` | `addr_mask` | `-` |