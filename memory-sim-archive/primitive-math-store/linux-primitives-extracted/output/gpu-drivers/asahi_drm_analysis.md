# asahi_drm.h

**Source:** `asahi_drm.h`


## Includes

- `drm.h`

## Defines (2 total)


### DRM_ASAHI (2)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_ASAHI_MAX_CLUSTERS` | `64` |  |
| `DRM_ASAHI_BARRIER_NONE` | `(0xFFFFu)` |  |

## Structs (23)


### `struct drm_asahi_params_global`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `features` | `-` |
| `__u32` | `gpu_generation` | `-` |
| `__u32` | `gpu_variant` | `-` |
| `__u32` | `gpu_revision` | `-` |
| `__u32` | `chip_id` | `-` |
| `__u32` | `num_dies` | `-` |
| `__u32` | `num_clusters_total` | `-` |
| `__u32` | `num_cores_per_cluster` | `-` |
| `__u32` | `max_frequency_khz` | `-` |
| `__u64` | `core_masks` | `DRM_ASAHI_MAX_CLUSTERS` |
| `__u64` | `vm_start` | `-` |
| `__u64` | `vm_end` | `-` |
| `__u64` | `vm_kernel_min_size` | `-` |
| `__u32` | `max_commands_per_submission` | `-` |
| `__u32` | `max_attachments` | `-` |
| `__u64` | `command_timestamp_frequency_hz` | `-` |

### `struct drm_asahi_get_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param_group` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `pointer` | `-` |
| `__u64` | `size` | `-` |

### `struct drm_asahi_vm_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `kernel_start` | `-` |
| `__u64` | `kernel_end` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_asahi_vm_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vm_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_asahi_gem_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_asahi_gem_mmap_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_asahi_gem_bind_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `range` | `-` |
| `__u64` | `addr` | `-` |

### `struct drm_asahi_vm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vm_id` | `-` |
| `__u32` | `num_binds` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `userptr` | `-` |

### `struct drm_asahi_gem_bind_object`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `range` | `-` |
| `__u32` | `object_handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_asahi_queue_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `priority` | `-` |
| `__u32` | `queue_id` | `-` |
| `__u64` | `usc_exec_base` | `-` |

### `struct drm_asahi_queue_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `queue_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_asahi_sync`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `sync_type` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `timeline_value` | `-` |

### `struct drm_asahi_cmd_header`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `cmd_type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `vdm_barrier` | `-` |
| `__u16` | `cdm_barrier` | `-` |

### `struct drm_asahi_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `syncs` | `-` |
| `__u64` | `cmdbuf` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `queue_id` | `-` |
| `__u32` | `in_sync_count` | `-` |
| `__u32` | `out_sync_count` | `-` |
| `__u32` | `cmdbuf_size` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_asahi_attachment`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `pointer` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_asahi_zls_buffer`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `base` | `-` |
| `__u64` | `comp_base` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `comp_stride` | `-` |

### `struct drm_asahi_timestamp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `offset` | `-` |

### `struct drm_asahi_timestamps`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_asahi_helper_program`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `binary` | `-` |
| `__u32` | `cfg` | `-` |
| `__u64` | `data` | `-` |

### `struct drm_asahi_bg_eot`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `usc` | `-` |
| `__u32` | `rsrc_spec` | `-` |

### `struct drm_asahi_cmd_render`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `isp_zls_pixels` | `-` |
| `__u64` | `vdm_ctrl_stream_base` | `-` |
| `__u64` | `isp_scissor_base` | `-` |
| `__u64` | `isp_dbias_base` | `-` |
| `__u64` | `isp_oclqry_base` | `-` |
| `__u64` | `zls_ctrl` | `-` |
| `__u64` | `ppp_multisamplectl` | `-` |
| `__u64` | `sampler_heap` | `-` |
| `__u32` | `ppp_ctrl` | `-` |
| `__u16` | `width_px` | `-` |
| `__u16` | `height_px` | `-` |
| `__u16` | `layers` | `-` |
| `__u16` | `sampler_count` | `-` |
| `__u8` | `utile_width_px` | `-` |
| `__u8` | `utile_height_px` | `-` |
| `__u8` | `samples` | `-` |
| `__u8` | `sample_size_B` | `-` |
| `__u32` | `isp_merge_upper_x` | `-` |
| `__u32` | `isp_merge_upper_y` | `-` |
| `__u32` | `isp_bgobjdepth` | `-` |
| `__u32` | `isp_bgobjvals` | `-` |

### `struct drm_asahi_cmd_compute`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `sampler_count` | `-` |
| `__u64` | `cdm_ctrl_stream_base` | `-` |
| `__u64` | `cdm_ctrl_stream_end` | `-` |
| `__u64` | `sampler_heap` | `-` |

### `struct drm_asahi_get_time`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `gpu_timestamp` | `-` |