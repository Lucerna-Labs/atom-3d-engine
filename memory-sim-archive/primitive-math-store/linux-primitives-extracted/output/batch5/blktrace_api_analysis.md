# blktrace_api.h

**Source:** `blktrace_api.h`


## Includes

- `linux/types.h`

## Defines (29 total)


### BLKTRACE_BDEV (2)

| Name | Value | Comment |
|------|-------|---------|
| `BLKTRACE_BDEV_SIZE` | `32` |  |
| `BLKTRACE_BDEV_SIZE2` | `64` |  |

### BLK_IO (3)

| Name | Value | Comment |
|------|-------|---------|
| `BLK_IO_TRACE_MAGIC` | `0x65617400` |  |
| `BLK_IO_TRACE_VERSION` | `0x07` |  |
| `BLK_IO_TRACE2_VERSION` | `0x08` |  |

### BLK_TA (20)

| Name | Value | Comment |
|------|-------|---------|
| `BLK_TA_QUEUE` | `(__BLK_TA_QUEUE \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_BACKMERGE` | `(__BLK_TA_BACKMERGE \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_FRONTMERGE` | `(__BLK_TA_FRONTMERGE \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_GETRQ` | `(__BLK_TA_GETRQ \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_SLEEPRQ` | `(__BLK_TA_SLEEPRQ \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_REQUEUE` | `(__BLK_TA_REQUEUE \| BLK_TC_ACT(BLK_TC_REQUEUE))` |  |
| `BLK_TA_ISSUE` | `(__BLK_TA_ISSUE \| BLK_TC_ACT(BLK_TC_ISSUE))` |  |
| `BLK_TA_COMPLETE` | `(__BLK_TA_COMPLETE\| BLK_TC_ACT(BLK_TC_COMPLETE))` |  |
| `BLK_TA_PLUG` | `(__BLK_TA_PLUG \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_UNPLUG_IO` | `(__BLK_TA_UNPLUG_IO \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_UNPLUG_TIMER` | `(__BLK_TA_UNPLUG_TIMER \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_INSERT` | `(__BLK_TA_INSERT \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_SPLIT` | `(__BLK_TA_SPLIT)` |  |
| `BLK_TA_BOUNCE` | `(__BLK_TA_BOUNCE)` |  |
| `BLK_TA_REMAP` | `(__BLK_TA_REMAP \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_ABORT` | `(__BLK_TA_ABORT \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_DRV_DATA` | `(__BLK_TA_DRV_DATA \| BLK_TC_ACT(BLK_TC_DRV_DATA))` |  |
| `BLK_TA_ZONE_APPEND` | `(__BLK_TA_COMPLETE \|` |  |
| `BLK_TA_ZONE_PLUG` | `(__BLK_TA_ZONE_PLUG \| BLK_TC_ACT(BLK_TC_QUEUE))` |  |
| `BLK_TA_ZONE_UNPLUG` | `(__BLK_TA_ZONE_UNPLUG \|` |  |

### BLK_TC (1)

| Name | Value | Comment |
|------|-------|---------|
| `BLK_TC_SHIFT` | `(16)` |  |

### BLK_TN (3)

| Name | Value | Comment |
|------|-------|---------|
| `BLK_TN_PROCESS` | `(__BLK_TN_PROCESS \| BLK_TC_ACT(BLK_TC_NOTIFY))` |  |
| `BLK_TN_TIMESTAMP` | `(__BLK_TN_TIMESTAMP \| BLK_TC_ACT(BLK_TC_NOTIFY))` |  |
| `BLK_TN_MESSAGE` | `(__BLK_TN_MESSAGE \| BLK_TC_ACT(BLK_TC_NOTIFY))` |  |

## Structs (5)


### `struct blk_io_trace`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `magic` | `-` |
| `__u32` | `sequence` | `-` |
| `__u64` | `time` | `-` |
| `__u64` | `sector` | `-` |
| `__u32` | `bytes` | `-` |
| `__u32` | `action` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `device` | `-` |
| `__u32` | `cpu` | `-` |
| `__u16` | `error` | `-` |
| `__u16` | `pdu_len` | `-` |

### `struct blk_io_trace2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `magic` | `-` |
| `__u32` | `sequence` | `-` |
| `__u64` | `time` | `-` |
| `__u64` | `sector` | `-` |
| `__u32` | `bytes` | `-` |
| `__u32` | `pid` | `-` |
| `__u64` | `action` | `-` |
| `__u32` | `device` | `-` |
| `__u32` | `cpu` | `-` |
| `__u16` | `error` | `-` |
| `__u16` | `pdu_len` | `-` |
| `__u8` | `pad` | `12` |

### `struct blk_io_trace_remap`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `device_from` | `-` |
| `__be32` | `device_to` | `-` |
| `__be64` | `sector_from` | `-` |

### `struct blk_user_trace_setup`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `BLKTRACE_BDEV_SIZE` |
| `__u16` | `act_mask` | `-` |
| `__u32` | `buf_size` | `-` |
| `__u32` | `buf_nr` | `-` |
| `__u64` | `start_lba` | `-` |
| `__u64` | `end_lba` | `-` |
| `__u32` | `pid` | `-` |

### `struct blk_user_trace_setup2`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `BLKTRACE_BDEV_SIZE2` |
| `__u64` | `act_mask` | `-` |
| `__u32` | `buf_size` | `-` |
| `__u32` | `buf_nr` | `-` |
| `__u64` | `start_lba` | `-` |
| `__u64` | `end_lba` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `reserved` | `11` |