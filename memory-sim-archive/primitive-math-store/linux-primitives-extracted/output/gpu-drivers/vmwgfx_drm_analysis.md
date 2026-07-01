# vmwgfx_drm.h

**Source:** `vmwgfx_drm.h`


## Includes

- `drm.h`

## Defines (69 total)


### DRM_VMW (65)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_VMW_MAX_SURFACE_FACES` | `6` |  |
| `DRM_VMW_MAX_MIP_LEVELS` | `24` |  |
| `DRM_VMW_GET_PARAM` | `0` |  |
| `DRM_VMW_ALLOC_DMABUF` | `1` |  |
| `DRM_VMW_ALLOC_BO` | `1` |  |
| `DRM_VMW_UNREF_DMABUF` | `2` |  |
| `DRM_VMW_HANDLE_CLOSE` | `2` |  |
| `DRM_VMW_CURSOR_BYPASS` | `3` |  |
| `DRM_VMW_CONTROL_STREAM` | `4` |  |
| `DRM_VMW_CLAIM_STREAM` | `5` |  |
| `DRM_VMW_UNREF_STREAM` | `6` |  |
| `DRM_VMW_CREATE_CONTEXT` | `7` |  |
| `DRM_VMW_UNREF_CONTEXT` | `8` |  |
| `DRM_VMW_CREATE_SURFACE` | `9` |  |
| `DRM_VMW_UNREF_SURFACE` | `10` |  |
| `DRM_VMW_REF_SURFACE` | `11` |  |
| `DRM_VMW_EXECBUF` | `12` |  |
| `DRM_VMW_GET_3D_CAP` | `13` |  |
| `DRM_VMW_FENCE_WAIT` | `14` |  |
| `DRM_VMW_FENCE_SIGNALED` | `15` |  |
| `DRM_VMW_FENCE_UNREF` | `16` |  |
| `DRM_VMW_FENCE_EVENT` | `17` |  |
| `DRM_VMW_PRESENT` | `18` |  |
| `DRM_VMW_PRESENT_READBACK` | `19` |  |
| `DRM_VMW_UPDATE_LAYOUT` | `20` |  |
| `DRM_VMW_CREATE_SHADER` | `21` |  |
| `DRM_VMW_UNREF_SHADER` | `22` |  |
| `DRM_VMW_GB_SURFACE_CREATE` | `23` |  |
| `DRM_VMW_GB_SURFACE_REF` | `24` |  |
| `DRM_VMW_SYNCCPU` | `25` |  |
| `DRM_VMW_CREATE_EXTENDED_CONTEXT` | `26` |  |
| `DRM_VMW_GB_SURFACE_CREATE_EXT` | `27` |  |
| `DRM_VMW_GB_SURFACE_REF_EXT` | `28` |  |
| `DRM_VMW_MSG` | `29` |  |
| `DRM_VMW_MKSSTAT_RESET` | `30` |  |
| `DRM_VMW_MKSSTAT_ADD` | `31` |  |
| `DRM_VMW_MKSSTAT_REMOVE` | `32` |  |
| `DRM_VMW_PARAM_NUM_STREAMS` | `0` |  |
| `DRM_VMW_PARAM_NUM_FREE_STREAMS` | `1` |  |
| `DRM_VMW_PARAM_3D` | `2` |  |
| `DRM_VMW_PARAM_HW_CAPS` | `3` |  |
| `DRM_VMW_PARAM_FIFO_CAPS` | `4` |  |
| `DRM_VMW_PARAM_MAX_FB_SIZE` | `5` |  |
| `DRM_VMW_PARAM_FIFO_HW_VERSION` | `6` |  |
| `DRM_VMW_PARAM_MAX_SURF_MEMORY` | `7` |  |
| `DRM_VMW_PARAM_3D_CAPS_SIZE` | `8` |  |
| `DRM_VMW_PARAM_MAX_MOB_MEMORY` | `9` |  |
| `DRM_VMW_PARAM_MAX_MOB_SIZE` | `10` |  |
| `DRM_VMW_PARAM_SCREEN_TARGET` | `11` |  |
| `DRM_VMW_PARAM_DX` | `12` |  |

*...and 15 more*

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `drm_vmw_alloc_dmabuf_req` | `drm_vmw_alloc_bo_req` |  |
| `drm_vmw_dmabuf_rep` | `drm_vmw_bo_rep` |  |
| `drm_vmw_alloc_dmabuf_arg` | `drm_vmw_alloc_bo_arg` |  |
| `drm_vmw_unref_dmabuf_arg` | `drm_vmw_handle_close_arg` |  |

## Structs (34)


### `struct drm_vmw_getparam_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `value` | `-` |
| `__u32` | `param` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_context_arg`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `cid` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_surface_create_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `format` | `-` |
| `__u32` | `mip_levels` | `DRM_VMW_MAX_SURFACE_FACES` |
| `__u64` | `size_addr` | `-` |
| `__s32` | `shareable` | `-` |
| `__s32` | `scanout` | `-` |

### `struct drm_vmw_surface_arg`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `sid` | `-` |

### `struct drm_vmw_size`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `depth` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_execbuf_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `commands` | `-` |
| `__u32` | `command_size` | `-` |
| `__u32` | `throttle_us` | `-` |
| `__u64` | `fence_rep` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `context_handle` | `-` |
| `__s32` | `imported_fence_fd` | `-` |

### `struct drm_vmw_fence_rep`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `mask` | `-` |
| `__u32` | `seqno` | `-` |
| `__u32` | `passed_seqno` | `-` |
| `__s32` | `fd` | `-` |
| `__s32` | `error` | `-` |

### `struct drm_vmw_alloc_bo_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_bo_rep`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `map_handle` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `cur_gmr_id` | `-` |
| `__u32` | `cur_gmr_offset` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_rect`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `x` | `-` |
| `__s32` | `y` | `-` |
| `__u32` | `w` | `-` |
| `__u32` | `h` | `-` |

### `struct drm_vmw_control_stream_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `stream_id` | `-` |
| `__u32` | `enabled` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `color_key` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `offset` | `-` |
| `__s32` | `format` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `pitch` | `3` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_cursor_bypass_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__s32` | `xpos` | `-` |
| `__s32` | `ypos` | `-` |
| `__s32` | `xhot` | `-` |
| `__s32` | `yhot` | `-` |

### `struct drm_vmw_stream_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `stream_id` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_get_3d_cap_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buffer` | `-` |
| `__u32` | `max_size` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_fence_wait_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__s32` | `cookie_valid` | `-` |
| `__u64` | `kernel_cookie` | `-` |
| `__u64` | `timeout_us` | `-` |
| `__s32` | `lazy` | `-` |
| `__s32` | `flags` | `-` |
| `__s32` | `wait_options` | `-` |
| `__s32` | `pad64` | `-` |

### `struct drm_vmw_fence_signaled_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `signaled` | `-` |
| `__u32` | `passed_seqno` | `-` |
| `__u32` | `signaled_flags` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_fence_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_event_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_data` | `-` |
| `__u32` | `tv_sec` | `-` |
| `__u32` | `tv_usec` | `-` |

### `struct drm_vmw_fence_event_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fence_rep` | `-` |
| `__u64` | `user_data` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_vmw_present_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_id` | `-` |
| `__u32` | `sid` | `-` |
| `__s32` | `dest_x` | `-` |
| `__s32` | `dest_y` | `-` |
| `__u64` | `clips_ptr` | `-` |
| `__u32` | `num_clips` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_present_readback_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_id` | `-` |
| `__u32` | `num_clips` | `-` |
| `__u64` | `clips_ptr` | `-` |
| `__u64` | `fence_rep` | `-` |

### `struct drm_vmw_update_layout_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_outputs` | `-` |
| `__u32` | `pad64` | `-` |
| `__u64` | `rects` | `-` |

### `struct drm_vmw_shader_create_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `buffer_handle` | `-` |
| `__u32` | `shader_handle` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_vmw_shader_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_gb_surface_create_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `svga3d_flags` | `-` |
| `__u32` | `format` | `-` |
| `__u32` | `mip_levels` | `-` |
| `__u32` | `multisample_count` | `-` |
| `__u32` | `autogen_filter` | `-` |
| `__u32` | `buffer_handle` | `-` |
| `__u32` | `array_size` | `-` |

### `struct drm_vmw_gb_surface_create_rep`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `backup_size` | `-` |
| `__u32` | `buffer_handle` | `-` |
| `__u32` | `buffer_size` | `-` |
| `__u64` | `buffer_map_handle` | `-` |

### `struct drm_vmw_gb_surface_ref_rep`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_vmw_synccpu_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_handle_close_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad64` | `-` |

### `struct drm_vmw_gb_surface_create_ext_req`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `svga3d_flags_upper_32_bits` | `-` |
| `__u32` | `multisample_pattern` | `-` |
| `__u32` | `quality_level` | `-` |
| `__u32` | `buffer_byte_stride` | `-` |
| `__u32` | `must_be_zero` | `-` |

### `struct drm_vmw_gb_surface_ref_ext_rep`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_vmw_msg_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `send` | `-` |
| `__u64` | `receive` | `-` |
| `__s32` | `send_only` | `-` |
| `__u32` | `receive_len` | `-` |

### `struct drm_vmw_mksstat_add_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `stat` | `-` |
| `__u64` | `info` | `-` |
| `__u64` | `strs` | `-` |
| `__u64` | `stat_len` | `-` |
| `__u64` | `info_len` | `-` |
| `__u64` | `strs_len` | `-` |
| `__u64` | `description` | `-` |
| `__u64` | `id` | `-` |

### `struct drm_vmw_mksstat_remove_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |