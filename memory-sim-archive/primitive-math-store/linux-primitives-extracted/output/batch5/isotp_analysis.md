# isotp.h

**Source:** `isotp.h`


## Includes

- `linux/types.h`
- `linux/can.h`

## Defines (31 total)


### CAN_ISOTP (30)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_ISOTP_OPTS` | `1` | pass struct can_isotp_options |
| `CAN_ISOTP_RECV_FC` | `2` | pass struct can_isotp_fc_options |
| `CAN_ISOTP_TX_STMIN` | `3` | pass __u32 value in nano secs |
| `CAN_ISOTP_RX_STMIN` | `4` | pass __u32 value in nano secs |
| `CAN_ISOTP_LL_OPTS` | `5` | pass struct can_isotp_ll_options |
| `CAN_ISOTP_LISTEN_MODE` | `0x0001` | listen only (do not send FC) |
| `CAN_ISOTP_EXTEND_ADDR` | `0x0002` | enable extended addressing |
| `CAN_ISOTP_TX_PADDING` | `0x0004` | enable CAN frame padding tx path |
| `CAN_ISOTP_RX_PADDING` | `0x0008` | enable CAN frame padding rx path |
| `CAN_ISOTP_CHK_PAD_LEN` | `0x0010` | check received CAN frame padding |
| `CAN_ISOTP_CHK_PAD_DATA` | `0x0020` | check received CAN frame padding |
| `CAN_ISOTP_HALF_DUPLEX` | `0x0040` | half duplex error state handling |
| `CAN_ISOTP_FORCE_TXSTMIN` | `0x0080` | ignore stmin from received FC |
| `CAN_ISOTP_FORCE_RXSTMIN` | `0x0100` | ignore CFs depending on rx stmin |
| `CAN_ISOTP_RX_EXT_ADDR` | `0x0200` | different rx extended addressing |
| `CAN_ISOTP_WAIT_TX_DONE` | `0x0400` | wait for tx completion |
| `CAN_ISOTP_SF_BROADCAST` | `0x0800` | 1-to-N functional addressing |
| `CAN_ISOTP_CF_BROADCAST` | `0x1000` | 1-to-N transmission w/o FC |
| `CAN_ISOTP_DYN_FC_PARMS` | `0x2000` | dynamic FC parameters BS/STmin |
| `CAN_ISOTP_DEFAULT_FLAGS` | `0` |  |
| `CAN_ISOTP_DEFAULT_EXT_ADDRESS` | `0x00` |  |
| `CAN_ISOTP_DEFAULT_PAD_CONTENT` | `0xCC` | prevent bit-stuffing |
| `CAN_ISOTP_DEFAULT_FRAME_TXTIME` | `50000` | 50 micro seconds |
| `CAN_ISOTP_DEFAULT_RECV_BS` | `0` |  |
| `CAN_ISOTP_DEFAULT_RECV_STMIN` | `0x00` |  |
| `CAN_ISOTP_DEFAULT_RECV_WFTMAX` | `0` |  |
| `CAN_ISOTP_DEFAULT_LL_MTU` | `CAN_MTU` |  |
| `CAN_ISOTP_DEFAULT_LL_TX_DL` | `CAN_MAX_DLEN` |  |
| `CAN_ISOTP_DEFAULT_LL_TX_FLAGS` | `0` |  |
| `CAN_ISOTP_FRAME_TXTIME_ZERO` | `0xFFFFFFFF` |  |

### SOL_CAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOL_CAN_ISOTP` | `(SOL_CAN_BASE + CAN_ISOTP)` |  |

## Structs (3)


### `struct can_isotp_options`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `frame_txtime` | `-` |
| `__u8` | `ext_address` | `-` |
| `__u8` | `txpad_content` | `-` |
| `__u8` | `rxpad_content` | `-` |
| `__u8` | `rx_ext_address` | `-` |

### `struct can_isotp_fc_options`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `bs` | `-` |
| `__u8` | `stmin` | `-` |
| `__u8` | `wftmax` | `-` |

### `struct can_isotp_ll_options`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `mtu` | `-` |
| `__u8` | `tx_dl` | `-` |
| `__u8` | `tx_flags` | `-` |