# ivpu_accel.h

**Source:** `ivpu_accel.h`


## Includes

- `drm.h`

## Defines (72 total)


### DRM_IOCTL (14)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_IVPU_GET_PARAM` | `` |  |
| `DRM_IOCTL_IVPU_SET_PARAM` | `` |  |
| `DRM_IOCTL_IVPU_BO_CREATE` | `` |  |
| `DRM_IOCTL_IVPU_BO_INFO` | `` |  |
| `DRM_IOCTL_IVPU_SUBMIT` | `` |  |
| `DRM_IOCTL_IVPU_BO_WAIT` | `` |  |
| `DRM_IOCTL_IVPU_METRIC_STREAMER_START` | `` |  |
| `DRM_IOCTL_IVPU_METRIC_STREAMER_STOP` | `` |  |
| `DRM_IOCTL_IVPU_METRIC_STREAMER_GET_DATA` | `` |  |
| `DRM_IOCTL_IVPU_METRIC_STREAMER_GET_INFO` | `` |  |
| `DRM_IOCTL_IVPU_CMDQ_CREATE` | `` |  |
| `DRM_IOCTL_IVPU_CMDQ_DESTROY` | `` |  |
| `DRM_IOCTL_IVPU_CMDQ_SUBMIT` | `` |  |
| `DRM_IOCTL_IVPU_BO_CREATE_FROM_USERPTR` | `` |  |

### DRM_IVPU (58)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IVPU_GET_PARAM` | `0x00` |  |
| `DRM_IVPU_SET_PARAM` | `0x01` |  |
| `DRM_IVPU_BO_CREATE` | `0x02` |  |
| `DRM_IVPU_BO_INFO` | `0x03` |  |
| `DRM_IVPU_SUBMIT` | `0x05` |  |
| `DRM_IVPU_BO_WAIT` | `0x06` |  |
| `DRM_IVPU_METRIC_STREAMER_START` | `0x07` |  |
| `DRM_IVPU_METRIC_STREAMER_STOP` | `0x08` |  |
| `DRM_IVPU_METRIC_STREAMER_GET_DATA` | `0x09` |  |
| `DRM_IVPU_METRIC_STREAMER_GET_INFO` | `0x0a` |  |
| `DRM_IVPU_CMDQ_CREATE` | `0x0b` |  |
| `DRM_IVPU_CMDQ_DESTROY` | `0x0c` |  |
| `DRM_IVPU_CMDQ_SUBMIT` | `0x0d` |  |
| `DRM_IVPU_BO_CREATE_FROM_USERPTR` | `0x0e` |  |
| `DRM_IVPU_PARAM_DEVICE_ID` | `0` |  |
| `DRM_IVPU_PARAM_DEVICE_REVISION` | `1` |  |
| `DRM_IVPU_PARAM_PLATFORM_TYPE` | `2` |  |
| `DRM_IVPU_PARAM_CORE_CLOCK_RATE` | `3` |  |
| `DRM_IVPU_PARAM_NUM_CONTEXTS` | `4` |  |
| `DRM_IVPU_PARAM_CONTEXT_BASE_ADDRESS` | `5` |  |
| `DRM_IVPU_PARAM_CONTEXT_PRIORITY` | `6` | Deprecated |
| `DRM_IVPU_PARAM_CONTEXT_ID` | `7` |  |
| `DRM_IVPU_PARAM_FW_API_VERSION` | `8` |  |
| `DRM_IVPU_PARAM_ENGINE_HEARTBEAT` | `9` |  |
| `DRM_IVPU_PARAM_UNIQUE_INFERENCE_ID` | `10` |  |
| `DRM_IVPU_PARAM_TILE_CONFIG` | `11` |  |
| `DRM_IVPU_PARAM_SKU` | `12` |  |
| `DRM_IVPU_PARAM_CAPABILITIES` | `13` |  |
| `DRM_IVPU_PARAM_PREEMPT_BUFFER_SIZE` | `14` |  |
| `DRM_IVPU_PLATFORM_TYPE_SILICON` | `0` |  |
| `DRM_IVPU_CONTEXT_PRIORITY_IDLE` | `0` |  |
| `DRM_IVPU_CONTEXT_PRIORITY_NORMAL` | `1` |  |
| `DRM_IVPU_CONTEXT_PRIORITY_FOCUS` | `2` |  |
| `DRM_IVPU_CONTEXT_PRIORITY_REALTIME` | `3` |  |
| `DRM_IVPU_JOB_PRIORITY_DEFAULT` | `0` |  |
| `DRM_IVPU_JOB_PRIORITY_IDLE` | `1` |  |
| `DRM_IVPU_JOB_PRIORITY_NORMAL` | `2` |  |
| `DRM_IVPU_JOB_PRIORITY_FOCUS` | `3` |  |
| `DRM_IVPU_JOB_PRIORITY_REALTIME` | `4` |  |
| `DRM_IVPU_CAP_METRIC_STREAMER` | `1` |  |
| `DRM_IVPU_CAP_DMA_MEMORY_RANGE` | `2` |  |
| `DRM_IVPU_CAP_MANAGE_CMDQ` | `3` |  |
| `DRM_IVPU_CAP_BO_CREATE_FROM_USERPTR` | `4` |  |
| `DRM_IVPU_BO_SHAVE_MEM` | `0x00000001` |  |
| `DRM_IVPU_BO_HIGH_MEM` | `DRM_IVPU_BO_SHAVE_MEM` |  |
| `DRM_IVPU_BO_MAPPABLE` | `0x00000002` |  |
| `DRM_IVPU_BO_DMA_MEM` | `0x00000004` |  |
| `DRM_IVPU_BO_READ_ONLY` | `0x00000008` |  |
| `DRM_IVPU_BO_CACHED` | `0x00000000` |  |
| `DRM_IVPU_BO_UNCACHED` | `0x00010000` |  |

*...and 8 more*

## Structs (12)


### `struct drm_ivpu_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `index` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_ivpu_bo_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `vpu_addr` | `-` |

### `struct drm_ivpu_bo_create_from_userptr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_ptr` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `vpu_addr` | `-` |

### `struct drm_ivpu_bo_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `vpu_addr` | `-` |
| `__u64` | `mmap_offset` | `-` |
| `__u64` | `size` | `-` |

### `struct drm_ivpu_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buffers_ptr` | `-` |
| `__u32` | `buffer_count` | `-` |
| `__u32` | `engine` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `commands_offset` | `-` |
| `__u32` | `priority` | `-` |

### `struct drm_ivpu_cmdq_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buffers_ptr` | `-` |
| `__u32` | `buffer_count` | `-` |
| `__u32` | `cmdq_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `commands_offset` | `-` |
| `__u32` | `preempt_buffer_index` | `-` |
| `__u32` | `reserved` | `-` |

### `struct drm_ivpu_bo_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__s64` | `timeout_ns` | `-` |
| `__u32` | `job_status` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_ivpu_metric_streamer_start`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `metric_group_mask` | `-` |
| `__u64` | `sampling_period_ns` | `-` |
| `__u32` | `read_period_samples` | `-` |
| `__u32` | `sample_size` | `-` |
| `__u32` | `max_data_size` | `-` |

### `struct drm_ivpu_metric_streamer_get_data`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `metric_group_mask` | `-` |
| `__u64` | `buffer_ptr` | `-` |
| `__u64` | `buffer_size` | `-` |
| `__u64` | `data_size` | `-` |

### `struct drm_ivpu_cmdq_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmdq_id` | `-` |
| `__u32` | `priority` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_ivpu_cmdq_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmdq_id` | `-` |

### `struct drm_ivpu_metric_streamer_stop`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `metric_group_mask` | `-` |