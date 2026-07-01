# vc4_drm.h

**Source:** `vc4_drm.h`


## Includes

- `drm.h`

## Defines (49 total)


### DRM_IOCTL (15)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_VC4_SUBMIT_CL` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_SUBMIT_CL, struct drm_vc` |  |
| `DRM_IOCTL_VC4_WAIT_SEQNO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_WAIT_SEQNO, struct drm_v` |  |
| `DRM_IOCTL_VC4_WAIT_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_WAIT_BO, struct drm_vc4_` |  |
| `DRM_IOCTL_VC4_CREATE_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_CREATE_BO, struct drm_vc` |  |
| `DRM_IOCTL_VC4_MMAP_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_MMAP_BO, struct drm_vc4_` |  |
| `DRM_IOCTL_VC4_CREATE_SHADER_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_CREATE_SHADER_BO, struct` |  |
| `DRM_IOCTL_VC4_GET_HANG_STATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_GET_HANG_STATE, struct d` |  |
| `DRM_IOCTL_VC4_GET_PARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_GET_PARAM, struct drm_vc` |  |
| `DRM_IOCTL_VC4_SET_TILING` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_SET_TILING, struct drm_v` |  |
| `DRM_IOCTL_VC4_GET_TILING` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_GET_TILING, struct drm_v` |  |
| `DRM_IOCTL_VC4_LABEL_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_LABEL_BO, struct drm_vc4` |  |
| `DRM_IOCTL_VC4_GEM_MADVISE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_GEM_MADVISE, struct drm_` |  |
| `DRM_IOCTL_VC4_PERFMON_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_PERFMON_CREATE, struct d` |  |
| `DRM_IOCTL_VC4_PERFMON_DESTROY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_PERFMON_DESTROY, struct ` |  |
| `DRM_IOCTL_VC4_PERFMON_GET_VALUES` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_VC4_PERFMON_GET_VALUES, stru` |  |

### UNCATEGORIZED (34)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_VC4_SUBMIT_CL` | `0x00` |  |
| `DRM_VC4_WAIT_SEQNO` | `0x01` |  |
| `DRM_VC4_WAIT_BO` | `0x02` |  |
| `DRM_VC4_CREATE_BO` | `0x03` |  |
| `DRM_VC4_MMAP_BO` | `0x04` |  |
| `DRM_VC4_CREATE_SHADER_BO` | `0x05` |  |
| `DRM_VC4_GET_HANG_STATE` | `0x06` |  |
| `DRM_VC4_GET_PARAM` | `0x07` |  |
| `DRM_VC4_SET_TILING` | `0x08` |  |
| `DRM_VC4_GET_TILING` | `0x09` |  |
| `DRM_VC4_LABEL_BO` | `0x0a` |  |
| `DRM_VC4_GEM_MADVISE` | `0x0b` |  |
| `DRM_VC4_PERFMON_CREATE` | `0x0c` |  |
| `DRM_VC4_PERFMON_DESTROY` | `0x0d` |  |
| `DRM_VC4_PERFMON_GET_VALUES` | `0x0e` |  |
| `VC4_SUBMIT_RCL_SURFACE_READ_IS_FULL_RES` | `(1 << 0)` |  |
| `VC4_SUBMIT_CL_USE_CLEAR_COLOR` | `(1 << 0)` |  |
| `VC4_SUBMIT_CL_FIXED_RCL_ORDER` | `(1 << 1)` |  |
| `VC4_SUBMIT_CL_RCL_ORDER_INCREASING_X` | `(1 << 2)` |  |
| `VC4_SUBMIT_CL_RCL_ORDER_INCREASING_Y` | `(1 << 3)` |  |
| `DRM_VC4_PARAM_V3D_IDENT0` | `0` |  |
| `DRM_VC4_PARAM_V3D_IDENT1` | `1` |  |
| `DRM_VC4_PARAM_V3D_IDENT2` | `2` |  |
| `DRM_VC4_PARAM_SUPPORTS_BRANCHES` | `3` |  |
| `DRM_VC4_PARAM_SUPPORTS_ETC1` | `4` |  |
| `DRM_VC4_PARAM_SUPPORTS_THREADED_FS` | `5` |  |
| `DRM_VC4_PARAM_SUPPORTS_FIXED_RCL_ORDER` | `6` |  |
| `DRM_VC4_PARAM_SUPPORTS_MADVISE` | `7` |  |
| `DRM_VC4_PARAM_SUPPORTS_PERFMON` | `8` |  |
| `VC4_MADV_WILLNEED` | `0` |  |
| `VC4_MADV_DONTNEED` | `1` |  |
| `__VC4_MADV_PURGED` | `2` |  |
| `__VC4_MADV_NOTSUPP` | `3` |  |
| `DRM_VC4_MAX_PERF_COUNTERS` | `16` |  |

## Structs (17)


### `struct drm_vc4_submit_rcl_surface`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `hindex` | `-` |
| `__u32` | `offset` | `-` |
| `__u16` | `bits` | `-` |
| `__u16` | `flags` | `-` |

### `struct drm_vc4_submit_cl`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bin_cl` | `-` |
| `__u64` | `shader_rec` | `-` |
| `__u64` | `uniforms` | `-` |
| `__u64` | `bo_handles` | `-` |
| `__u32` | `bin_cl_size` | `-` |
| `__u32` | `shader_rec_size` | `-` |
| `__u32` | `shader_rec_count` | `-` |
| `__u32` | `uniforms_size` | `-` |
| `__u32` | `bo_handle_count` | `-` |
| `__u16` | `width` | `-` |
| `__u16` | `height` | `-` |
| `__u8` | `min_x_tile` | `-` |
| `__u8` | `min_y_tile` | `-` |
| `__u8` | `max_x_tile` | `-` |
| `__u8` | `max_y_tile` | `-` |
| `__u32` | `clear_color` | `2` |
| `__u32` | `clear_z` | `-` |
| `__u8` | `clear_s` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `seqno` | `-` |
| `__u32` | `perfmonid` | `-` |
| `__u32` | `in_sync` | `-` |
| `__u32` | `out_sync` | `-` |
| `__u32` | `pad2` | `-` |

### `struct drm_vc4_wait_seqno`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `seqno` | `-` |
| `__u64` | `timeout_ns` | `-` |

### `struct drm_vc4_wait_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `timeout_ns` | `-` |

### `struct drm_vc4_create_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_vc4_mmap_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_vc4_create_shader_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `data` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_vc4_get_hang_state_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `paddr` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_vc4_get_hang_state`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bo` | `-` |
| `__u32` | `bo_count` | `-` |
| `__u32` | `vpmbase` | `-` |
| `__u32` | `dbge` | `-` |
| `__u32` | `fdbgo` | `-` |
| `__u32` | `fdbgb` | `-` |
| `__u32` | `fdbgr` | `-` |
| `__u32` | `fdbgs` | `-` |
| `__u32` | `errstat` | `-` |
| `__u32` | `pad` | `16` |

### `struct drm_vc4_get_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_vc4_get_tiling`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `modifier` | `-` |

### `struct drm_vc4_set_tiling`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `modifier` | `-` |

### `struct drm_vc4_label_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `len` | `-` |
| `__u64` | `name` | `-` |

### `struct drm_vc4_gem_madvise`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `madv` | `-` |
| `__u32` | `retained` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_vc4_perfmon_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `ncounters` | `-` |
| `__u8` | `events` | `DRM_VC4_MAX_PERF_COUNTERS` |

### `struct drm_vc4_perfmon_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |

### `struct drm_vc4_perfmon_get_values`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u64` | `values_ptr` | `-` |