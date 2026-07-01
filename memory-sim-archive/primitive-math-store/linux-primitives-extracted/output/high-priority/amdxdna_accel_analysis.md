# amdxdna_accel.h

**Source:** `amdxdna_accel.h`


## Includes

- `linux/stddef.h`
- `drm.h`

## Defines (26 total)


### AMDXDNA_HWCTX (2)

| Name | Value | Comment |
|------|-------|---------|
| `AMDXDNA_HWCTX_STATE_IDLE` | `0` |  |
| `AMDXDNA_HWCTX_STATE_ACTIVE` | `1` |  |

### AMDXDNA_INVALID (5)

| Name | Value | Comment |
|------|-------|---------|
| `AMDXDNA_INVALID_CMD_HANDLE` | `(~0UL)` |  |
| `AMDXDNA_INVALID_ADDR` | `(~0UL)` |  |
| `AMDXDNA_INVALID_CTX_HANDLE` | `0` |  |
| `AMDXDNA_INVALID_BO_HANDLE` | `0` |  |
| `AMDXDNA_INVALID_FENCE_HANDLE` | `0` |  |

### AMDXDNA_QOS (4)

| Name | Value | Comment |
|------|-------|---------|
| `AMDXDNA_QOS_REALTIME_PRIORITY` | `0x100` |  |
| `AMDXDNA_QOS_HIGH_PRIORITY` | `0x180` |  |
| `AMDXDNA_QOS_NORMAL_PRIORITY` | `0x200` |  |
| `AMDXDNA_QOS_LOW_PRIORITY` | `0x280` |  |

### DRM_AMDXDNA (3)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_AMDXDNA_HW_CONTEXT_ALL` | `0` |  |
| `DRM_AMDXDNA_HW_LAST_ASYNC_ERR` | `2` |  |
| `DRM_AMDXDNA_BO_USAGE` | `6` |  |

### DRM_IOCTL (10)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_AMDXDNA_CREATE_HWCTX` | `` |  |
| `DRM_IOCTL_AMDXDNA_DESTROY_HWCTX` | `` |  |
| `DRM_IOCTL_AMDXDNA_CONFIG_HWCTX` | `` |  |
| `DRM_IOCTL_AMDXDNA_CREATE_BO` | `` |  |
| `DRM_IOCTL_AMDXDNA_GET_BO_INFO` | `` |  |
| `DRM_IOCTL_AMDXDNA_SYNC_BO` | `` |  |
| `DRM_IOCTL_AMDXDNA_EXEC_CMD` | `` |  |
| `DRM_IOCTL_AMDXDNA_GET_INFO` | `` |  |
| `DRM_IOCTL_AMDXDNA_SET_STATE` | `` |  |
| `DRM_IOCTL_AMDXDNA_GET_ARRAY` | `` |  |

### SYNC_DIRECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `SYNC_DIRECT_TO_DEVICE` | `0U` |  |
| `SYNC_DIRECT_FROM_DEVICE` | `1U` |  |

## Structs (32)


### `struct amdxdna_qos_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gops` | `-` |
| `__u32` | `fps` | `-` |
| `__u32` | `dma_bandwidth` | `-` |
| `__u32` | `latency` | `-` |
| `__u32` | `frame_exec_time` | `-` |
| `__u32` | `priority` | `-` |

### `struct amdxdna_drm_create_hwctx`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ext` | `-` |
| `__u64` | `ext_flags` | `-` |
| `__u64` | `qos_p` | `-` |
| `__u32` | `umq_bo` | `-` |
| `__u32` | `log_buf_bo` | `-` |
| `__u32` | `max_opc` | `-` |
| `__u32` | `num_tiles` | `-` |
| `__u32` | `mem_size` | `-` |
| `__u32` | `umq_doorbell` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `syncobj_handle` | `-` |

### `struct amdxdna_drm_destroy_hwctx`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct amdxdna_cu_config`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cu_bo` | `-` |
| `__u8` | `cu_func` | `-` |
| `__u8` | `pad` | `3` |

### `struct amdxdna_hwctx_param_config_cu`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `num_cus` | `-` |
| `__u16` | `pad` | `3` |

### `struct amdxdna_drm_config_hwctx`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `param_type` | `-` |
| `__u64` | `param_val` | `-` |
| `__u32` | `param_val_size` | `-` |
| `__u32` | `pad` | `-` |

### `struct amdxdna_drm_va_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `vaddr` | `-` |
| `__u64` | `len` | `-` |

### `struct amdxdna_drm_va_tbl`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `dmabuf_fd` | `-` |
| `__u32` | `num_entries` | `-` |

### `struct amdxdna_drm_create_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `vaddr` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `handle` | `-` |

### `struct amdxdna_drm_get_bo_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ext` | `-` |
| `__u64` | `ext_flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `map_offset` | `-` |
| `__u64` | `vaddr` | `-` |
| `__u64` | `xdna_addr` | `-` |

### `struct amdxdna_drm_sync_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `direction` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |

### `struct amdxdna_drm_exec_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ext` | `-` |
| `__u64` | `ext_flags` | `-` |
| `__u32` | `hwctx` | `-` |
| `__u32` | `type` | `-` |
| `__u64` | `cmd_handles` | `-` |
| `__u64` | `args` | `-` |
| `__u32` | `cmd_count` | `-` |
| `__u32` | `arg_count` | `-` |
| `__u64` | `seq` | `-` |

### `struct amdxdna_drm_query_aie_status`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buffer` | `-` |
| `__u32` | `buffer_size` | `-` |
| `__u32` | `cols_filled` | `-` |

### `struct amdxdna_drm_query_aie_version`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |

### `struct amdxdna_drm_query_aie_tile_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `row_count` | `-` |
| `__u16` | `row_start` | `-` |
| `__u16` | `dma_channel_count` | `-` |
| `__u16` | `lock_count` | `-` |
| `__u16` | `event_reg_count` | `-` |
| `__u16` | `pad` | `3` |

### `struct amdxdna_drm_query_aie_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `col_size` | `-` |
| `__u16` | `cols` | `-` |
| `__u16` | `rows` | `-` |

### `struct amdxdna_drm_query_clock`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `name` | `16` |
| `__u32` | `freq_mhz` | `-` |
| `__u32` | `pad` | `-` |

### `struct amdxdna_drm_query_clock_metadata`

| Type | Field | Array |
|------|-------|-------|

### `struct amdxdna_drm_query_sensor`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `label` | `64` |
| `__u32` | `input` | `-` |
| `__u32` | `max` | `-` |
| `__u32` | `average` | `-` |
| `__u32` | `highest` | `-` |
| `__u8` | `status` | `64` |
| `__u8` | `units` | `16` |
| `__s8` | `unitm` | `-` |
| `__u8` | `type` | `-` |
| `__u8` | `pad` | `6` |

### `struct amdxdna_drm_query_hwctx`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `context_id` | `-` |
| `__u32` | `start_col` | `-` |
| `__u32` | `num_col` | `-` |
| `__u32` | `pad` | `-` |
| `__s64` | `pid` | `-` |
| `__u64` | `command_submissions` | `-` |
| `__u64` | `command_completions` | `-` |
| `__u64` | `migrations` | `-` |
| `__u64` | `preemptions` | `-` |
| `__u64` | `errors` | `-` |

### `struct amdxdna_drm_get_power_mode`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `power_mode` | `-` |
| `__u8` | `pad` | `7` |

### `struct amdxdna_drm_query_firmware_version`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u32` | `patch` | `-` |
| `__u32` | `build` | `-` |

### `struct amdxdna_drm_get_resource_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `npu_clk_max` | `-` |
| `__u64` | `npu_tops_max` | `-` |
| `__u64` | `npu_task_max` | `-` |
| `__u64` | `npu_tops_curr` | `-` |
| `__u64` | `npu_task_curr` | `-` |

### `struct amdxdna_drm_attribute_state`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `state` | `-` |
| `__u8` | `pad` | `7` |

### `struct amdxdna_drm_query_telemetry_header`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `map_num_elements` | `-` |

### `struct amdxdna_drm_get_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `buffer_size` | `-` |
| `__u64` | `buffer` | `-` |

### `struct amdxdna_drm_hwctx_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `context_id` | `-` |
| `__u32` | `start_col` | `-` |
| `__u32` | `num_col` | `-` |
| `__u32` | `hwctx_id` | `-` |
| `__s64` | `pid` | `-` |
| `__u64` | `command_submissions` | `-` |
| `__u64` | `command_completions` | `-` |
| `__u64` | `migrations` | `-` |
| `__u64` | `preemptions` | `-` |
| `__u64` | `errors` | `-` |
| `__u64` | `priority` | `-` |
| `__u64` | `heap_usage` | `-` |
| `__u64` | `suspensions` | `-` |
| `__u32` | `state` | `-` |
| `__u32` | `pasid` | `-` |
| `__u32` | `gops` | `-` |
| `__u32` | `fps` | `-` |
| `__u32` | `dma_bandwidth` | `-` |
| `__u32` | `latency` | `-` |
| `__u32` | `frame_exec_time` | `-` |
| `__u32` | `txn_op_idx` | `-` |
| `__u32` | `ctx_pc` | `-` |
| `__u32` | `fatal_error_type` | `-` |
| `__u32` | `fatal_error_exception_type` | `-` |
| `__u32` | `fatal_error_exception_pc` | `-` |
| `__u32` | `fatal_error_app_module` | `-` |
| `__u32` | `pad` | `-` |

### `struct amdxdna_async_error`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `err_code` | `-` |
| `__u64` | `ts_us` | `-` |
| `__u64` | `ex_err_code` | `-` |

### `struct amdxdna_drm_bo_usage`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `pid` | `-` |
| `__u64` | `total_usage` | `-` |
| `__u64` | `internal_usage` | `-` |
| `__u64` | `heap_usage` | `-` |

### `struct amdxdna_drm_get_array`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `element_size` | `-` |
| `__u32` | `num_element` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `buffer` | `-` |

### `struct amdxdna_drm_set_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `buffer_size` | `-` |
| `__u64` | `buffer` | `-` |

### `struct amdxdna_drm_set_power_mode`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `power_mode` | `-` |
| `__u8` | `pad` | `7` |