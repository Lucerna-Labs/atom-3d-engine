# i915_drm.h

**Source:** `i915_drm.h`


## Includes

- `drm.h`

## Defines (383 total)


### DRM_IOCTL (62)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_I915_INIT` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_INIT, drm_i915_init_t)` |  |
| `DRM_IOCTL_I915_FLUSH` | `DRM_IO ( DRM_COMMAND_BASE + DRM_I915_FLUSH)` |  |
| `DRM_IOCTL_I915_FLIP` | `DRM_IO ( DRM_COMMAND_BASE + DRM_I915_FLIP)` |  |
| `DRM_IOCTL_I915_BATCHBUFFER` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_BATCHBUFFER, drm_i915_b` |  |
| `DRM_IOCTL_I915_IRQ_EMIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_IRQ_EMIT, drm_i915_irq_` |  |
| `DRM_IOCTL_I915_IRQ_WAIT` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_IRQ_WAIT, drm_i915_irq_` |  |
| `DRM_IOCTL_I915_GETPARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GETPARAM, drm_i915_getp` |  |
| `DRM_IOCTL_I915_SETPARAM` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_SETPARAM, drm_i915_setp` |  |
| `DRM_IOCTL_I915_ALLOC` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_ALLOC, drm_i915_mem_all` |  |
| `DRM_IOCTL_I915_FREE` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_FREE, drm_i915_mem_free` |  |
| `DRM_IOCTL_I915_INIT_HEAP` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_INIT_HEAP, drm_i915_mem` |  |
| `DRM_IOCTL_I915_CMDBUFFER` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_CMDBUFFER, drm_i915_cmd` |  |
| `DRM_IOCTL_I915_DESTROY_HEAP` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_DESTROY_HEAP, drm_i915_` |  |
| `DRM_IOCTL_I915_SET_VBLANK_PIPE` | `DRM_IOW( DRM_COMMAND_BASE + DRM_I915_SET_VBLANK_PIPE, drm_i9` |  |
| `DRM_IOCTL_I915_GET_VBLANK_PIPE` | `DRM_IOR( DRM_COMMAND_BASE + DRM_I915_GET_VBLANK_PIPE, drm_i9` |  |
| `DRM_IOCTL_I915_VBLANK_SWAP` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_VBLANK_SWAP, drm_i915_v` |  |
| `DRM_IOCTL_I915_HWS_ADDR` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_HWS_ADDR, struct drm_i91` |  |
| `DRM_IOCTL_I915_GEM_INIT` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_GEM_INIT, struct drm_i91` |  |
| `DRM_IOCTL_I915_GEM_EXECBUFFER` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_GEM_EXECBUFFER, struct d` |  |
| `DRM_IOCTL_I915_GEM_EXECBUFFER2` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_GEM_EXECBUFFER2, struct ` |  |
| `DRM_IOCTL_I915_GEM_EXECBUFFER2_WR` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_EXECBUFFER2_WR, str` |  |
| `DRM_IOCTL_I915_GEM_PIN` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_PIN, struct drm_i91` |  |
| `DRM_IOCTL_I915_GEM_UNPIN` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_GEM_UNPIN, struct drm_i9` |  |
| `DRM_IOCTL_I915_GEM_BUSY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_BUSY, struct drm_i9` |  |
| `DRM_IOCTL_I915_GEM_SET_CACHING` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_GEM_SET_CACHING, struct ` |  |
| `DRM_IOCTL_I915_GEM_GET_CACHING` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_GET_CACHING, struct` |  |
| `DRM_IOCTL_I915_GEM_THROTTLE` | `DRM_IO ( DRM_COMMAND_BASE + DRM_I915_GEM_THROTTLE)` |  |
| `DRM_IOCTL_I915_GEM_ENTERVT` | `DRM_IO(DRM_COMMAND_BASE + DRM_I915_GEM_ENTERVT)` |  |
| `DRM_IOCTL_I915_GEM_LEAVEVT` | `DRM_IO(DRM_COMMAND_BASE + DRM_I915_GEM_LEAVEVT)` |  |
| `DRM_IOCTL_I915_GEM_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_CREATE, struct drm_` |  |
| `DRM_IOCTL_I915_GEM_CREATE_EXT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_CREATE_EXT, struct ` |  |
| `DRM_IOCTL_I915_GEM_PREAD` | `DRM_IOW (DRM_COMMAND_BASE + DRM_I915_GEM_PREAD, struct drm_i` |  |
| `DRM_IOCTL_I915_GEM_PWRITE` | `DRM_IOW (DRM_COMMAND_BASE + DRM_I915_GEM_PWRITE, struct drm_` |  |
| `DRM_IOCTL_I915_GEM_MMAP` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_MMAP, struct drm_i9` |  |
| `DRM_IOCTL_I915_GEM_MMAP_GTT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_MMAP_GTT, struct dr` |  |
| `DRM_IOCTL_I915_GEM_MMAP_OFFSET` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_MMAP_GTT, struct dr` |  |
| `DRM_IOCTL_I915_GEM_SET_DOMAIN` | `DRM_IOW (DRM_COMMAND_BASE + DRM_I915_GEM_SET_DOMAIN, struct ` |  |
| `DRM_IOCTL_I915_GEM_SW_FINISH` | `DRM_IOW (DRM_COMMAND_BASE + DRM_I915_GEM_SW_FINISH, struct d` |  |
| `DRM_IOCTL_I915_GEM_SET_TILING` | `DRM_IOWR (DRM_COMMAND_BASE + DRM_I915_GEM_SET_TILING, struct` |  |
| `DRM_IOCTL_I915_GEM_GET_TILING` | `DRM_IOWR (DRM_COMMAND_BASE + DRM_I915_GEM_GET_TILING, struct` |  |
| `DRM_IOCTL_I915_GEM_GET_APERTURE` | `DRM_IOR  (DRM_COMMAND_BASE + DRM_I915_GEM_GET_APERTURE, stru` |  |
| `DRM_IOCTL_I915_GET_PIPE_FROM_CRTC_ID` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GET_PIPE_FROM_CRTC_ID, ` |  |
| `DRM_IOCTL_I915_GEM_MADVISE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_MADVISE, struct drm` |  |
| `DRM_IOCTL_I915_OVERLAY_PUT_IMAGE` | `DRM_IOW(DRM_COMMAND_BASE + DRM_I915_OVERLAY_PUT_IMAGE, struc` |  |
| `DRM_IOCTL_I915_OVERLAY_ATTRS` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_OVERLAY_ATTRS, struct d` |  |
| `DRM_IOCTL_I915_SET_SPRITE_COLORKEY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_SET_SPRITE_COLORKEY, st` |  |
| `DRM_IOCTL_I915_GET_SPRITE_COLORKEY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GET_SPRITE_COLORKEY, st` |  |
| `DRM_IOCTL_I915_GEM_WAIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_I915_GEM_WAIT, struct drm_i9` |  |
| `DRM_IOCTL_I915_GEM_CONTEXT_CREATE` | `DRM_IOWR (DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_CREATE, st` |  |
| `DRM_IOCTL_I915_GEM_CONTEXT_CREATE_EXT` | `DRM_IOWR (DRM_COMMAND_BASE + DRM_I915_GEM_CONTEXT_CREATE, st` |  |

*...and 12 more*

### EXEC_OBJECT (8)

| Name | Value | Comment |
|------|-------|---------|
| `EXEC_OBJECT_NEEDS_FENCE` | `(1<<0)` |  |
| `EXEC_OBJECT_NEEDS_GTT` | `(1<<1)` |  |
| `EXEC_OBJECT_WRITE` | `(1<<2)` |  |
| `EXEC_OBJECT_SUPPORTS_48B_ADDRESS` | `(1<<3)` |  |
| `EXEC_OBJECT_PINNED` | `(1<<4)` |  |
| `EXEC_OBJECT_PAD_TO_SIZE` | `(1<<5)` |  |
| `EXEC_OBJECT_ASYNC` | `(1<<6)` |  |
| `EXEC_OBJECT_CAPTURE` | `(1<<7)` |  |

### UNCATEGORIZED (313)

| Name | Value | Comment |
|------|-------|---------|
| `I915_L3_PARITY_UEVENT` | `"L3_PARITY_ERROR"` |  |
| `I915_ERROR_UEVENT` | `"ERROR"` |  |
| `I915_RESET_UEVENT` | `"RESET"` |  |
| `I915_ENGINE_CLASS_INVALID_NONE` | `-1` |  |
| `I915_ENGINE_CLASS_INVALID_VIRTUAL` | `-2` |  |
| `I915_PMU_SAMPLE_BITS` | `(4)` |  |
| `I915_PMU_SAMPLE_MASK` | `(0xf)` |  |
| `I915_PMU_SAMPLE_INSTANCE_BITS` | `(8)` |  |
| `I915_PMU_CLASS_SHIFT` | `` |  |
| `__I915_PMU_GT_SHIFT` | `(60)` |  |
| `I915_PMU_ACTUAL_FREQUENCY` | `__I915_PMU_OTHER(0)` |  |
| `I915_PMU_REQUESTED_FREQUENCY` | `__I915_PMU_OTHER(1)` |  |
| `I915_PMU_INTERRUPTS` | `__I915_PMU_OTHER(2)` |  |
| `I915_PMU_RC6_RESIDENCY` | `__I915_PMU_OTHER(3)` |  |
| `I915_PMU_SOFTWARE_GT_AWAKE_TIME` | `__I915_PMU_OTHER(4)` |  |
| `I915_PMU_LAST` | `/* Deprecated - do not use */ I915_PMU_RC6_RESIDENCY` |  |
| `I915_NR_TEX_REGIONS` | `255	/* table size 2k - maximum due to use` |  |
| `I915_LOG_MIN_TEX_REGION_SIZE` | `14` |  |
| `planeA_x` | `pipeA_x` |  |
| `planeA_y` | `pipeA_y` |  |
| `planeA_w` | `pipeA_w` |  |
| `planeA_h` | `pipeA_h` |  |
| `planeB_x` | `pipeB_x` |  |
| `planeB_y` | `pipeB_y` |  |
| `planeB_w` | `pipeB_w` |  |
| `planeB_h` | `pipeB_h` |  |
| `I915_BOX_RING_EMPTY` | `0x1` |  |
| `I915_BOX_FLIP` | `0x2` |  |
| `I915_BOX_WAIT` | `0x4` |  |
| `I915_BOX_TEXTURE_LOAD` | `0x8` |  |
| `I915_BOX_LOST_CONTEXT` | `0x10` |  |
| `DRM_I915_INIT` | `0x00` |  |
| `DRM_I915_FLUSH` | `0x01` |  |
| `DRM_I915_FLIP` | `0x02` |  |
| `DRM_I915_BATCHBUFFER` | `0x03` |  |
| `DRM_I915_IRQ_EMIT` | `0x04` |  |
| `DRM_I915_IRQ_WAIT` | `0x05` |  |
| `DRM_I915_GETPARAM` | `0x06` |  |
| `DRM_I915_SETPARAM` | `0x07` |  |
| `DRM_I915_ALLOC` | `0x08` |  |
| `DRM_I915_FREE` | `0x09` |  |
| `DRM_I915_INIT_HEAP` | `0x0a` |  |
| `DRM_I915_CMDBUFFER` | `0x0b` |  |
| `DRM_I915_DESTROY_HEAP` | `0x0c` |  |
| `DRM_I915_SET_VBLANK_PIPE` | `0x0d` |  |
| `DRM_I915_GET_VBLANK_PIPE` | `0x0e` |  |
| `DRM_I915_VBLANK_SWAP` | `0x0f` |  |
| `DRM_I915_HWS_ADDR` | `0x11` |  |
| `DRM_I915_GEM_INIT` | `0x13` |  |
| `DRM_I915_GEM_EXECBUFFER` | `0x14` |  |

*...and 263 more*

## Structs (84)


### `struct i915_user_extension`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `next_extension` | `-` |
| `__u32` | `name` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `rsvd` | `4` |

### `struct i915_engine_class_instance`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `engine_class` | `-` |
| `__u16` | `engine_instance` | `-` |

### `struct _drm_i915_init`

| Type | Field | Array |
|------|-------|-------|
| `int` | `sarea_priv_offset` | `-` |

### `struct _drm_i915_sarea`

| Type | Field | Array |
|------|-------|-------|
| `int` | `last_upload` | `-` |
| `int` | `last_enqueue` | `-` |
| `int` | `last_dispatch` | `-` |
| `int` | `ctxOwner` | `-` |
| `int` | `texAge` | `-` |
| `int` | `pf_enabled` | `-` |
| `int` | `pf_active` | `-` |
| `int` | `pf_current_page` | `-` |
| `int` | `perf_boxes` | `-` |
| `drm_handle_t` | `front_handle` | `-` |
| `int` | `front_offset` | `-` |
| `int` | `front_size` | `-` |
| `drm_handle_t` | `back_handle` | `-` |
| `int` | `back_offset` | `-` |
| `int` | `back_size` | `-` |
| `drm_handle_t` | `depth_handle` | `-` |
| `int` | `depth_offset` | `-` |
| `int` | `depth_size` | `-` |
| `drm_handle_t` | `tex_handle` | `-` |
| `int` | `tex_offset` | `-` |
| `int` | `tex_size` | `-` |
| `int` | `log_tex_granularity` | `-` |
| `int` | `pitch` | `-` |
| `int` | `rotation` | `-` |
| `int` | `rotated_offset` | `-` |
| `int` | `rotated_size` | `-` |
| `int` | `rotated_pitch` | `-` |
| `int` | `pipeA_x` | `-` |
| `int` | `pipeA_y` | `-` |
| `int` | `pipeA_w` | `-` |
| `int` | `pipeA_h` | `-` |
| `int` | `pipeB_x` | `-` |
| `int` | `pipeB_y` | `-` |
| `int` | `pipeB_w` | `-` |
| `int` | `pipeB_h` | `-` |
| `drm_handle_t` | `unused_handle` | `-` |
| `__u32` | `front_bo_handle` | `-` |
| `__u32` | `back_bo_handle` | `-` |
| `__u32` | `unused_bo_handle` | `-` |
| `__u32` | `depth_bo_handle` | `-` |

### `struct drm_i915_batchbuffer`

| Type | Field | Array |
|------|-------|-------|
| `int` | `start` | `-` |
| `int` | `used` | `-` |
| `int` | `DR1` | `-` |
| `int` | `DR4` | `-` |
| `int` | `num_cliprects` | `-` |

### `struct _drm_i915_cmdbuffer`

| Type | Field | Array |
|------|-------|-------|
| `int` | `sz` | `-` |
| `int` | `DR1` | `-` |
| `int` | `DR4` | `-` |
| `int` | `num_cliprects` | `-` |

### `struct drm_i915_irq_emit`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_i915_irq_wait`

| Type | Field | Array |
|------|-------|-------|
| `int` | `irq_seq` | `-` |

### `struct drm_i915_getparam`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `param` | `-` |

### `struct drm_i915_setparam`

| Type | Field | Array |
|------|-------|-------|
| `int` | `param` | `-` |
| `int` | `value` | `-` |

### `struct drm_i915_mem_alloc`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |
| `int` | `alignment` | `-` |
| `int` | `size` | `-` |

### `struct drm_i915_mem_free`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |
| `int` | `region_offset` | `-` |

### `struct drm_i915_mem_init_heap`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |
| `int` | `size` | `-` |
| `int` | `start` | `-` |

### `struct drm_i915_mem_destroy_heap`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |

### `struct drm_i915_vblank_pipe`

| Type | Field | Array |
|------|-------|-------|
| `int` | `pipe` | `-` |

### `struct drm_i915_vblank_swap`

| Type | Field | Array |
|------|-------|-------|
| `drm_drawable_t` | `drawable` | `-` |

### `struct drm_i915_hws_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |

### `struct drm_i915_gem_init`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `gtt_start` | `-` |
| `__u64` | `gtt_end` | `-` |

### `struct drm_i915_gem_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_i915_gem_pread`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `data_ptr` | `-` |

### `struct drm_i915_gem_pwrite`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `data_ptr` | `-` |

### `struct drm_i915_gem_mmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `addr_ptr` | `-` |
| `__u64` | `flags` | `-` |

### `struct drm_i915_gem_mmap_gtt`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_i915_gem_mmap_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `extensions` | `-` |

### `struct drm_i915_gem_set_domain`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `read_domains` | `-` |
| `__u32` | `write_domain` | `-` |

### `struct drm_i915_gem_sw_finish`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |

### `struct drm_i915_gem_relocation_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `target_handle` | `-` |
| `__u32` | `delta` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `presumed_offset` | `-` |
| `__u32` | `read_domains` | `-` |
| `__u32` | `write_domain` | `-` |

### `struct drm_i915_gem_exec_object`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `relocation_count` | `-` |
| `__u64` | `relocs_ptr` | `-` |
| `__u64` | `alignment` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_i915_gem_execbuffer`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buffers_ptr` | `-` |
| `__u32` | `buffer_count` | `-` |
| `__u32` | `batch_start_offset` | `-` |
| `__u32` | `batch_len` | `-` |
| `__u32` | `DR1` | `-` |
| `__u32` | `DR4` | `-` |
| `__u32` | `num_cliprects` | `-` |
| `__u64` | `cliprects_ptr` | `-` |

### `struct drm_i915_gem_exec_object2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `relocation_count` | `-` |
| `__u64` | `relocs_ptr` | `-` |
| `__u64` | `alignment` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `rsvd1` | `-` |
| `__u64` | `pad_to_size` | `-` |
| `__u64` | `rsvd2` | `-` |

### `struct drm_i915_gem_exec_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_i915_gem_execbuffer_ext_timeline_fences`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fence_count` | `-` |
| `__u64` | `handles_ptr` | `-` |
| `__u64` | `values_ptr` | `-` |

### `struct drm_i915_gem_execbuffer2`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buffers_ptr` | `-` |
| `__u32` | `buffer_count` | `-` |
| `__u32` | `batch_start_offset` | `-` |
| `__u32` | `batch_len` | `-` |
| `__u32` | `DR1` | `-` |
| `__u32` | `DR4` | `-` |
| `__u32` | `num_cliprects` | `-` |
| `__u64` | `cliprects_ptr` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `rsvd1` | `-` |
| `__u64` | `rsvd2` | `-` |

### `struct drm_i915_gem_pin`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `alignment` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_i915_gem_unpin`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_i915_gem_busy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `busy` | `-` |

### `struct drm_i915_gem_caching`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `caching` | `-` |

### `struct drm_i915_gem_set_tiling`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `tiling_mode` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `swizzle_mode` | `-` |

### `struct drm_i915_gem_get_tiling`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `tiling_mode` | `-` |
| `__u32` | `swizzle_mode` | `-` |
| `__u32` | `phys_swizzle_mode` | `-` |

### `struct drm_i915_gem_get_aperture`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `aper_size` | `-` |
| `__u64` | `aper_available_size` | `-` |

### `struct drm_i915_get_pipe_from_crtc_id`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc_id` | `-` |
| `__u32` | `pipe` | `-` |

### `struct drm_i915_gem_madvise`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `madv` | `-` |
| `__u32` | `retained` | `-` |

### `struct drm_intel_overlay_put_image`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `bo_handle` | `-` |
| `__u16` | `stride_Y` | `-` |
| `__u16` | `stride_UV` | `-` |
| `__u32` | `offset_Y` | `-` |
| `__u32` | `offset_U` | `-` |
| `__u32` | `offset_V` | `-` |
| `__u16` | `src_width` | `-` |
| `__u16` | `src_height` | `-` |
| `__u16` | `src_scan_width` | `-` |
| `__u16` | `src_scan_height` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__u16` | `dst_x` | `-` |
| `__u16` | `dst_y` | `-` |
| `__u16` | `dst_width` | `-` |
| `__u16` | `dst_height` | `-` |

### `struct drm_intel_overlay_attrs`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `color_key` | `-` |
| `__s32` | `brightness` | `-` |
| `__u32` | `contrast` | `-` |
| `__u32` | `saturation` | `-` |
| `__u32` | `gamma0` | `-` |
| `__u32` | `gamma1` | `-` |
| `__u32` | `gamma2` | `-` |
| `__u32` | `gamma3` | `-` |
| `__u32` | `gamma4` | `-` |
| `__u32` | `gamma5` | `-` |

### `struct drm_intel_sprite_colorkey`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `plane_id` | `-` |
| `__u32` | `min_value` | `-` |
| `__u32` | `channel_mask` | `-` |
| `__u32` | `max_value` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_i915_gem_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bo_handle` | `-` |
| `__u32` | `flags` | `-` |
| `__s64` | `timeout_ns` | `-` |

### `struct drm_i915_gem_context_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_i915_gem_context_create_ext`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `extensions` | `-` |

### `struct drm_i915_gem_context_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `size` | `-` |
| `__u64` | `param` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_i915_gem_context_param_sseu`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u64` | `slice_mask` | `-` |
| `__u64` | `subslice_mask` | `-` |
| `__u16` | `min_eus_per_subslice` | `-` |
| `__u16` | `max_eus_per_subslice` | `-` |
| `__u32` | `rsvd` | `-` |

### `struct i915_context_engines_load_balance`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `engine_index` | `-` |
| `__u16` | `num_siblings` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `mbz64` | `-` |

### `struct anonymous_51`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `engine_index` | `-` |
| `__u16` | `num_siblings` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `mbz64` | `-` |

### `struct i915_context_engines_bond`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `virtual_index` | `-` |
| `__u16` | `num_bonds` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `mbz64` | `4` |

### `struct anonymous_53`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `virtual_index` | `-` |
| `__u16` | `num_bonds` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `mbz64` | `4` |

### `struct i915_context_engines_parallel_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `engine_index` | `-` |
| `__u16` | `width` | `-` |
| `__u16` | `num_siblings` | `-` |
| `__u16` | `mbz16` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `mbz64` | `3` |

### `struct anonymous_55`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `engine_index` | `-` |
| `__u16` | `width` | `-` |
| `__u16` | `num_siblings` | `-` |
| `__u16` | `mbz16` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `mbz64` | `3` |

### `struct i915_context_param_engines`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |

### `struct anonymous_57`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |

### `struct i915_gem_context_param_context_image`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `mbz` | `-` |
| `__u64` | `image` | `-` |

### `struct drm_i915_gem_context_create_ext_setparam`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_i915_gem_context_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_i915_gem_vm_control`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `vm_id` | `-` |

### `struct drm_i915_reg_read`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `offset` | `-` |
| `__u64` | `val` | `-` |

### `struct drm_i915_reset_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reset_count` | `-` |
| `__u32` | `batch_active` | `-` |
| `__u32` | `batch_pending` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_i915_gem_userptr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_ptr` | `-` |
| `__u64` | `user_size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_i915_perf_open_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `num_properties` | `-` |
| `__u64` | `properties_ptr` | `-` |

### `struct drm_i915_perf_record_header`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `pad` | `-` |
| `__u16` | `size` | `-` |

### `struct anonymous_67`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_i915_perf_oa_config`

| Type | Field | Array |
|------|-------|-------|
| `char` | `uuid` | `36` |
| `__u32` | `n_mux_regs` | `-` |
| `__u32` | `n_boolean_regs` | `-` |
| `__u32` | `n_flex_regs` | `-` |
| `__u64` | `mux_regs_ptr` | `-` |
| `__u64` | `boolean_regs_ptr` | `-` |
| `__u64` | `flex_regs_ptr` | `-` |

### `struct drm_i915_query_item`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `query_id` | `-` |
| `__s32` | `length` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `data_ptr` | `-` |

### `struct drm_i915_query`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_items` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `items_ptr` | `-` |

### `struct drm_i915_query_topology_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `flags` | `-` |
| `__u16` | `max_slices` | `-` |
| `__u16` | `max_subslices` | `-` |
| `__u16` | `max_eus_per_subslice` | `-` |
| `__u16` | `subslice_offset` | `-` |
| `__u16` | `subslice_stride` | `-` |
| `__u16` | `eu_offset` | `-` |
| `__u16` | `eu_stride` | `-` |

### `struct drm_i915_engine_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rsvd0` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `capabilities` | `-` |
| `__u16` | `logical_instance` | `-` |
| `__u16` | `rsvd1` | `3` |
| `__u64` | `rsvd2` | `3` |

### `struct drm_i915_query_engine_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_engines` | `-` |
| `__u32` | `rsvd` | `3` |

### `struct drm_i915_query_perf_config`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `n_configs` | `-` |
| `__u64` | `config` | `-` |
| `char` | `uuid` | `36` |
| `__u32` | `flags` | `-` |

### `struct drm_i915_gem_memory_class_instance`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `memory_class` | `-` |
| `__u16` | `memory_instance` | `-` |

### `struct drm_i915_memory_region_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rsvd0` | `-` |
| `__u64` | `probed_size` | `-` |
| `__u64` | `unallocated_size` | `-` |
| `__u64` | `rsvd1` | `8` |
| `__u64` | `probed_cpu_visible_size` | `-` |
| `__u64` | `unallocated_cpu_visible_size` | `-` |

### `struct anonymous_77`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `probed_cpu_visible_size` | `-` |
| `__u64` | `unallocated_cpu_visible_size` | `-` |

### `struct drm_i915_query_memory_regions`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_regions` | `-` |
| `__u32` | `rsvd` | `3` |

### `struct drm_i915_query_guc_submission_version`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `branch` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u32` | `patch` | `-` |

### `struct drm_i915_gem_create_ext`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `extensions` | `-` |

### `struct drm_i915_gem_create_ext_memory_regions`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pad` | `-` |
| `__u32` | `num_regions` | `-` |
| `__u64` | `regions` | `-` |

### `struct drm_i915_gem_create_ext_protected_content`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |

### `struct drm_i915_gem_create_ext_set_pat`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pat_index` | `-` |
| `__u32` | `rsvd` | `-` |

## Typedefs

- `_drm_i915_init`
- `_drm_i915_sarea`
- `drm_i915_batchbuffer`
- `_drm_i915_cmdbuffer`
- `drm_i915_irq_emit`
- `drm_i915_irq_wait`
- `drm_i915_getparam_t`
- `drm_i915_setparam`
- `drm_i915_mem_alloc`
- `drm_i915_mem_free`
- `drm_i915_mem_init_heap`
- `drm_i915_mem_destroy_heap`
- `drm_i915_vblank_pipe`
- `drm_i915_vblank_swap`
- `drm_i915_hws_addr`