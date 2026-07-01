# qaic_accel.h

**Source:** `qaic_accel.h`


## Includes

- `drm.h`

## Defines (49 total)


### DRM_IOCTL (9)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_QAIC_MANAGE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_MANAGE, struct qaic_man` |  |
| `DRM_IOCTL_QAIC_CREATE_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_CREATE_BO,	struct qaic_` |  |
| `DRM_IOCTL_QAIC_MMAP_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_MMAP_BO, struct qaic_mm` |  |
| `DRM_IOCTL_QAIC_ATTACH_SLICE_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_ATTACH_SLICE_BO, struct ` |  |
| `DRM_IOCTL_QAIC_EXECUTE_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_EXECUTE_BO,	struct qaic_` |  |
| `DRM_IOCTL_QAIC_PARTIAL_EXECUTE_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_PARTIAL_EXECUTE_BO,	stru` |  |
| `DRM_IOCTL_QAIC_WAIT_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_WAIT_BO, struct qaic_wai` |  |
| `DRM_IOCTL_QAIC_PERF_STATS_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_QAIC_PERF_STATS_BO, struct q` |  |
| `DRM_IOCTL_QAIC_DETACH_SLICE_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_QAIC_DETACH_SLICE_BO, struct ` |  |

### DRM_QAIC (9)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_QAIC_MANAGE` | `0x00` |  |
| `DRM_QAIC_CREATE_BO` | `0x01` |  |
| `DRM_QAIC_MMAP_BO` | `0x02` |  |
| `DRM_QAIC_ATTACH_SLICE_BO` | `0x03` |  |
| `DRM_QAIC_EXECUTE_BO` | `0x04` |  |
| `DRM_QAIC_PARTIAL_EXECUTE_BO` | `0x05` |  |
| `DRM_QAIC_WAIT_BO` | `0x06` |  |
| `DRM_QAIC_PERF_STATS_BO` | `0x07` |  |
| `DRM_QAIC_DETACH_SLICE_BO` | `0x08` |  |

### QAIC_MANAGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `QAIC_MANAGE_MAX_MSG_LENGTH` | `SZ_4K` |  |

### QAIC_SEM (9)

| Name | Value | Comment |
|------|-------|---------|
| `QAIC_SEM_INSYNCFENCE` | `2` |  |
| `QAIC_SEM_OUTSYNCFENCE` | `1` |  |
| `QAIC_SEM_NOP` | `0` |  |
| `QAIC_SEM_INIT` | `1` |  |
| `QAIC_SEM_INC` | `2` |  |
| `QAIC_SEM_DEC` | `3` |  |
| `QAIC_SEM_WAIT_EQUAL` | `4` |  |
| `QAIC_SEM_WAIT_GT_EQ` | `5` | Greater than or equal |
| `QAIC_SEM_WAIT_GT_0` | `6` | Greater than 0 |

### QAIC_TRANS (21)

| Name | Value | Comment |
|------|-------|---------|
| `QAIC_TRANS_UNDEFINED` | `0` |  |
| `QAIC_TRANS_PASSTHROUGH_FROM_USR` | `1` |  |
| `QAIC_TRANS_PASSTHROUGH_TO_USR` | `2` |  |
| `QAIC_TRANS_PASSTHROUGH_FROM_DEV` | `3` |  |
| `QAIC_TRANS_PASSTHROUGH_TO_DEV` | `4` |  |
| `QAIC_TRANS_DMA_XFER_FROM_USR` | `5` |  |
| `QAIC_TRANS_DMA_XFER_TO_DEV` | `6` |  |
| `QAIC_TRANS_ACTIVATE_FROM_USR` | `7` |  |
| `QAIC_TRANS_ACTIVATE_FROM_DEV` | `8` |  |
| `QAIC_TRANS_ACTIVATE_TO_DEV` | `9` |  |
| `QAIC_TRANS_DEACTIVATE_FROM_USR` | `10` |  |
| `QAIC_TRANS_DEACTIVATE_FROM_DEV` | `11` |  |
| `QAIC_TRANS_STATUS_FROM_USR` | `12` |  |
| `QAIC_TRANS_STATUS_TO_USR` | `13` |  |
| `QAIC_TRANS_STATUS_FROM_DEV` | `14` |  |
| `QAIC_TRANS_STATUS_TO_DEV` | `15` |  |
| `QAIC_TRANS_TERMINATE_FROM_DEV` | `16` |  |
| `QAIC_TRANS_TERMINATE_TO_DEV` | `17` |  |
| `QAIC_TRANS_DMA_XFER_CONT` | `18` |  |
| `QAIC_TRANS_VALIDATE_PARTITION_FROM_DEV` | `19` |  |
| `QAIC_TRANS_VALIDATE_PARTITION_TO_DEV` | `20` |  |

## Structs (24)


### `struct qaic_manage_trans_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `len` | `-` |

### `struct qaic_manage_trans_passthrough`

| Type | Field | Array |
|------|-------|-------|

### `struct qaic_manage_trans_dma_xfer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tag` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `size` | `-` |

### `struct qaic_manage_trans_activate_to_dev`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `queue_size` | `-` |
| `__u32` | `eventfd` | `-` |
| `__u32` | `options` | `-` |
| `__u32` | `pad` | `-` |

### `struct qaic_manage_trans_activate_from_dev`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `status` | `-` |
| `__u32` | `dbc_id` | `-` |
| `__u64` | `options` | `-` |

### `struct qaic_manage_trans_deactivate`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dbc_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct qaic_manage_trans_status_to_dev`

| Type | Field | Array |
|------|-------|-------|

### `struct qaic_manage_trans_status_from_dev`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `major` | `-` |
| `__u16` | `minor` | `-` |
| `__u32` | `status` | `-` |
| `__u64` | `status_flags` | `-` |

### `struct qaic_manage_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `len` | `-` |
| `__u32` | `count` | `-` |
| `__u64` | `data` | `-` |

### `struct qaic_create_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct qaic_mmap_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct qaic_sem`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `val` | `-` |
| `__u8` | `index` | `-` |
| `__u8` | `presync` | `-` |
| `__u8` | `cmd` | `-` |
| `__u8` | `flags` | `-` |
| `__u16` | `pad` | `-` |

### `struct qaic_attach_slice_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u64` | `dev_addr` | `-` |
| `__u64` | `db_addr` | `-` |
| `__u32` | `db_data` | `-` |
| `__u32` | `db_len` | `-` |
| `__u64` | `offset` | `-` |

### `struct qaic_attach_slice_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |
| `__u32` | `dbc_id` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `dir` | `-` |
| `__u64` | `size` | `-` |

### `struct qaic_attach_slice`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data` | `-` |

### `struct qaic_execute_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `dir` | `-` |

### `struct qaic_partial_execute_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `dir` | `-` |
| `__u64` | `resize` | `-` |

### `struct qaic_execute_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |
| `__u32` | `dbc_id` | `-` |

### `struct qaic_execute`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data` | `-` |

### `struct qaic_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `timeout` | `-` |
| `__u32` | `dbc_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct qaic_perf_stats_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `count` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `dbc_id` | `-` |

### `struct qaic_perf_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data` | `-` |

### `struct qaic_perf_stats_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `queue_level_before` | `-` |
| `__u32` | `num_queue_element` | `-` |
| `__u32` | `submit_latency_us` | `-` |
| `__u32` | `device_latency_us` | `-` |
| `__u32` | `pad` | `-` |

### `struct qaic_detach_slice`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |