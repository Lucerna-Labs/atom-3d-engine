# v3d_drm.h

**Source:** `v3d_drm.h`


## Includes

- `drm.h`

## Defines (42 total)


### DRM_IOCTL (14)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_V3D_SUBMIT_CL` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_CL, struct drm_v3` |  |
| `DRM_IOCTL_V3D_WAIT_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_WAIT_BO, struct drm_v3d_` |  |
| `DRM_IOCTL_V3D_CREATE_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_CREATE_BO, struct drm_v3` |  |
| `DRM_IOCTL_V3D_MMAP_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_MMAP_BO, struct drm_v3d_` |  |
| `DRM_IOCTL_V3D_GET_PARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_GET_PARAM, struct drm_v3` |  |
| `DRM_IOCTL_V3D_GET_BO_OFFSET` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_GET_BO_OFFSET, struct dr` |  |
| `DRM_IOCTL_V3D_SUBMIT_TFU` | `DRM_IOW(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_TFU, struct drm_v3` |  |
| `DRM_IOCTL_V3D_SUBMIT_CSD` | `DRM_IOW(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_CSD, struct drm_v3` |  |
| `DRM_IOCTL_V3D_PERFMON_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_PERFMON_CREATE, ` |  |
| `DRM_IOCTL_V3D_PERFMON_DESTROY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_PERFMON_DESTROY, ` |  |
| `DRM_IOCTL_V3D_PERFMON_GET_VALUES` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_PERFMON_GET_VALUES, ` |  |
| `DRM_IOCTL_V3D_SUBMIT_CPU` | `DRM_IOW(DRM_COMMAND_BASE + DRM_V3D_SUBMIT_CPU, struct drm_v3` |  |
| `DRM_IOCTL_V3D_PERFMON_GET_COUNTER` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_V3D_PERFMON_GET_COUNTER, ` |  |
| `DRM_IOCTL_V3D_PERFMON_SET_GLOBAL` | `DRM_IOW(DRM_COMMAND_BASE + DRM_V3D_PERFMON_SET_GLOBAL, ` |  |

### UNCATEGORIZED (28)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_V3D_SUBMIT_CL` | `0x00` |  |
| `DRM_V3D_WAIT_BO` | `0x01` |  |
| `DRM_V3D_CREATE_BO` | `0x02` |  |
| `DRM_V3D_MMAP_BO` | `0x03` |  |
| `DRM_V3D_GET_PARAM` | `0x04` |  |
| `DRM_V3D_GET_BO_OFFSET` | `0x05` |  |
| `DRM_V3D_SUBMIT_TFU` | `0x06` |  |
| `DRM_V3D_SUBMIT_CSD` | `0x07` |  |
| `DRM_V3D_PERFMON_CREATE` | `0x08` |  |
| `DRM_V3D_PERFMON_DESTROY` | `0x09` |  |
| `DRM_V3D_PERFMON_GET_VALUES` | `0x0a` |  |
| `DRM_V3D_SUBMIT_CPU` | `0x0b` |  |
| `DRM_V3D_PERFMON_GET_COUNTER` | `0x0c` |  |
| `DRM_V3D_PERFMON_SET_GLOBAL` | `0x0d` |  |
| `DRM_V3D_SUBMIT_CL_FLUSH_CACHE` | `0x01` |  |
| `DRM_V3D_SUBMIT_EXTENSION` | `0x02` |  |
| `DRM_V3D_EXT_ID_MULTI_SYNC` | `0x01` |  |
| `DRM_V3D_EXT_ID_CPU_INDIRECT_CSD` | `0x02` |  |
| `DRM_V3D_EXT_ID_CPU_TIMESTAMP_QUERY` | `0x03` |  |
| `DRM_V3D_EXT_ID_CPU_RESET_TIMESTAMP_QUERY` | `0x04` |  |
| `DRM_V3D_EXT_ID_CPU_COPY_TIMESTAMP_QUERY` | `0x05` |  |
| `DRM_V3D_EXT_ID_CPU_RESET_PERFORMANCE_QUERY` | `0x06` |  |
| `DRM_V3D_EXT_ID_CPU_COPY_PERFORMANCE_QUERY` | `0x07` |  |
| `DRM_V3D_MAX_PERF_COUNTERS` | `32` |  |
| `DRM_V3D_PERFCNT_MAX_NAME` | `64` |  |
| `DRM_V3D_PERFCNT_MAX_CATEGORY` | `32` |  |
| `DRM_V3D_PERFCNT_MAX_DESCRIPTION` | `256` |  |
| `DRM_V3D_PERFMON_CLEAR_GLOBAL` | `0x0001` |  |

## Structs (24)


### `struct drm_v3d_extension`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `next` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_v3d_sem`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `point` | `-` |
| `__u64` | `mbz` | `2` |

### `struct drm_v3d_multi_sync`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `in_syncs` | `-` |
| `__u64` | `out_syncs` | `-` |
| `__u32` | `in_sync_count` | `-` |
| `__u32` | `out_sync_count` | `-` |
| `__u32` | `wait_stage` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_v3d_submit_cl`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bcl_start` | `-` |
| `__u32` | `bcl_end` | `-` |
| `__u32` | `rcl_start` | `-` |
| `__u32` | `rcl_end` | `-` |
| `__u32` | `in_sync_bcl` | `-` |
| `__u32` | `in_sync_rcl` | `-` |
| `__u32` | `out_sync` | `-` |
| `__u32` | `qma` | `-` |
| `__u32` | `qms` | `-` |
| `__u32` | `qts` | `-` |
| `__u64` | `bo_handles` | `-` |
| `__u32` | `bo_handle_count` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `perfmon_id` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `extensions` | `-` |

### `struct drm_v3d_wait_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `timeout_ns` | `-` |

### `struct drm_v3d_create_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `offset` | `-` |

### `struct drm_v3d_mmap_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_v3d_get_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_v3d_get_bo_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `offset` | `-` |

### `struct drm_v3d_submit_tfu`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `icfg` | `-` |
| `__u32` | `iia` | `-` |
| `__u32` | `iis` | `-` |
| `__u32` | `ica` | `-` |
| `__u32` | `iua` | `-` |
| `__u32` | `ioa` | `-` |
| `__u32` | `ios` | `-` |
| `__u32` | `coef` | `4` |
| `__u32` | `bo_handles` | `4` |
| `__u32` | `in_sync` | `-` |
| `__u32` | `out_sync` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `extensions` | `-` |
| `__u32` | `ioc` | `-` |
| `__u32` | `pad` | `-` |

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ioc` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_v3d_submit_csd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cfg` | `7` |
| `__u32` | `coef` | `4` |
| `__u64` | `bo_handles` | `-` |
| `__u32` | `bo_handle_count` | `-` |
| `__u32` | `in_sync` | `-` |
| `__u32` | `out_sync` | `-` |
| `__u32` | `perfmon_id` | `-` |
| `__u64` | `extensions` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_v3d_indirect_csd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `indirect` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `wg_size` | `-` |
| `__u32` | `wg_uniform_offsets` | `3` |

### `struct drm_v3d_timestamp_query`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `offsets` | `-` |
| `__u64` | `syncs` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_v3d_reset_timestamp_query`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `syncs` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `count` | `-` |

### `struct drm_v3d_copy_timestamp_query`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `do_64bit` | `-` |
| `__u8` | `do_partial` | `-` |
| `__u8` | `availability_bit` | `-` |
| `__u8` | `pad` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `count` | `-` |
| `__u64` | `offsets` | `-` |
| `__u64` | `syncs` | `-` |

### `struct drm_v3d_reset_performance_query`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `syncs` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `nperfmons` | `-` |
| `__u64` | `kperfmon_ids` | `-` |

### `struct drm_v3d_copy_performance_query`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `do_64bit` | `-` |
| `__u8` | `do_partial` | `-` |
| `__u8` | `availability_bit` | `-` |
| `__u8` | `pad` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `nperfmons` | `-` |
| `__u32` | `ncounters` | `-` |
| `__u32` | `count` | `-` |
| `__u64` | `syncs` | `-` |
| `__u64` | `kperfmon_ids` | `-` |

### `struct drm_v3d_submit_cpu`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bo_handles` | `-` |
| `__u32` | `bo_handle_count` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `extensions` | `-` |

### `struct drm_v3d_perfmon_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `ncounters` | `-` |
| `__u8` | `counters` | `DRM_V3D_MAX_PERF_COUNTERS` |

### `struct drm_v3d_perfmon_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |

### `struct drm_v3d_perfmon_get_values`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `values_ptr` | `-` |

### `struct drm_v3d_perfmon_get_counter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `counter` | `-` |
| `__u8` | `name` | `DRM_V3D_PERFCNT_MAX_NAME` |
| `__u8` | `category` | `DRM_V3D_PERFCNT_MAX_CATEGORY` |
| `__u8` | `description` | `DRM_V3D_PERFCNT_MAX_DESCRIPTION` |
| `__u8` | `reserved` | `7` |

### `struct drm_v3d_perfmon_set_global`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `id` | `-` |