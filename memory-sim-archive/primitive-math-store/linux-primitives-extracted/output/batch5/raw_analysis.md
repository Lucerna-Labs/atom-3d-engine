# raw.h

**Source:** `raw.h`


## Includes

- `linux/can.h`

## Defines (5 total)


### CAN_RAW (4)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_RAW_FILTER_MAX` | `512` | maximum number of can_filter set via setsockopt() |
| `CAN_RAW_XL_VCID_TX_SET` | `0x01` |  |
| `CAN_RAW_XL_VCID_TX_PASS` | `0x02` |  |
| `CAN_RAW_XL_VCID_RX_FILTER` | `0x04` |  |

### SOL_CAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOL_CAN_RAW` | `(SOL_CAN_BASE + CAN_RAW)` |  |

## Structs (1)


### `struct can_raw_vcid_options`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `tx_vcid` | `-` |
| `__u8` | `rx_vcid` | `-` |
| `__u8` | `rx_vcid_mask` | `-` |