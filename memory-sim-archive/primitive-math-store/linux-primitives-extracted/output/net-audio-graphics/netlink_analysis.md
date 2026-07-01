# netlink.h

**Source:** `netlink.h`


## Includes

- `linux/types.h`

## Defines (17 total)


### CAN_CTRLMODE (16)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_CTRLMODE_LOOPBACK` | `0x01` | Loopback mode |
| `CAN_CTRLMODE_LISTENONLY` | `0x02` | Listen-only mode |
| `CAN_CTRLMODE_3_SAMPLES` | `0x04` | Triple sampling mode |
| `CAN_CTRLMODE_ONE_SHOT` | `0x08` | One-Shot mode |
| `CAN_CTRLMODE_BERR_REPORTING` | `0x10` | Bus-error reporting |
| `CAN_CTRLMODE_FD` | `0x20` | CAN FD mode |
| `CAN_CTRLMODE_PRESUME_ACK` | `0x40` | Ignore missing CAN ACKs |
| `CAN_CTRLMODE_FD_NON_ISO` | `0x80` | CAN FD in non-ISO mode |
| `CAN_CTRLMODE_CC_LEN8_DLC` | `0x100` | Classic CAN DLC option |
| `CAN_CTRLMODE_TDC_AUTO` | `0x200` | FD transceiver automatically calculates TDCV |
| `CAN_CTRLMODE_TDC_MANUAL` | `0x400` | FD TDCV is manually set up by user |
| `CAN_CTRLMODE_RESTRICTED` | `0x800` | Restricted operation mode |
| `CAN_CTRLMODE_XL` | `0x1000` | CAN XL mode |
| `CAN_CTRLMODE_XL_TDC_AUTO` | `0x2000` | XL transceiver automatically calculates TDCV |
| `CAN_CTRLMODE_XL_TDC_MANUAL` | `0x4000` | XL TDCV is manually set up by user |
| `CAN_CTRLMODE_XL_TMS` | `0x8000` | Transceiver Mode Switching |

### CAN_TERMINATION (1)

| Name | Value | Comment |
|------|-------|---------|
| `CAN_TERMINATION_DISABLED` | `0` |  |

## Structs (6)


### `struct can_bittiming`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bitrate` | `-` |
| `__u32` | `sample_point` | `-` |
| `__u32` | `tq` | `-` |
| `__u32` | `prop_seg` | `-` |
| `__u32` | `phase_seg1` | `-` |
| `__u32` | `phase_seg2` | `-` |
| `__u32` | `sjw` | `-` |
| `__u32` | `brp` | `-` |

### `struct can_bittiming_const`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `16` |
| `__u32` | `tseg1_min` | `-` |
| `__u32` | `tseg1_max` | `-` |
| `__u32` | `tseg2_min` | `-` |
| `__u32` | `tseg2_max` | `-` |
| `__u32` | `sjw_max` | `-` |
| `__u32` | `brp_min` | `-` |
| `__u32` | `brp_max` | `-` |
| `__u32` | `brp_inc` | `-` |

### `struct can_clock`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `freq` | `-` |

### `struct can_berr_counter`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `txerr` | `-` |
| `__u16` | `rxerr` | `-` |

### `struct can_ctrlmode`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mask` | `-` |
| `__u32` | `flags` | `-` |

### `struct can_device_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bus_error` | `-` |
| `__u32` | `error_warning` | `-` |
| `__u32` | `error_passive` | `-` |
| `__u32` | `bus_off` | `-` |
| `__u32` | `arbitration_lost` | `-` |
| `__u32` | `restarts` | `-` |