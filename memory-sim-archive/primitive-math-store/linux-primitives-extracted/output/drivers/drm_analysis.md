# drm.h

**Source:** `drm.h`


## Includes

- `linux/types.h`
- `asm/ioctl.h`
- `linux/types.h`
- `asm/ioctl.h`
- `stdint.h`
- `sys/ioccom.h`
- `sys/types.h`
- `drm_mode.h`

## Defines (166 total)


### DRM_CAP (15)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_CAP_DUMB_BUFFER` | `0x1` |  |
| `DRM_CAP_VBLANK_HIGH_CRTC` | `0x2` |  |
| `DRM_CAP_DUMB_PREFERRED_DEPTH` | `0x3` |  |
| `DRM_CAP_DUMB_PREFER_SHADOW` | `0x4` |  |
| `DRM_CAP_PRIME` | `0x5` |  |
| `DRM_CAP_TIMESTAMP_MONOTONIC` | `0x6` |  |
| `DRM_CAP_ASYNC_PAGE_FLIP` | `0x7` |  |
| `DRM_CAP_CURSOR_WIDTH` | `0x8` |  |
| `DRM_CAP_CURSOR_HEIGHT` | `0x9` |  |
| `DRM_CAP_ADDFB2_MODIFIERS` | `0x10` |  |
| `DRM_CAP_PAGE_FLIP_TARGET` | `0x11` |  |
| `DRM_CAP_CRTC_IN_VBLANK_EVENT` | `0x12` |  |
| `DRM_CAP_SYNCOBJ` | `0x13` |  |
| `DRM_CAP_SYNCOBJ_TIMELINE` | `0x14` |  |
| `DRM_CAP_ATOMIC_ASYNC_PAGE_FLIP` | `0x15` |  |

### DRM_CLIENT (8)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_CLIENT_CAP_STEREO_3D` | `1` |  |
| `DRM_CLIENT_CAP_UNIVERSAL_PLANES` | `2` |  |
| `DRM_CLIENT_CAP_ATOMIC` | `3` |  |
| `DRM_CLIENT_CAP_ASPECT_RATIO` | `4` |  |
| `DRM_CLIENT_CAP_WRITEBACK_CONNECTORS` | `5` |  |
| `DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT` | `6` |  |
| `DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE` | `7` |  |
| `DRM_CLIENT_NAME_MAX_LEN` | `64` |  |

### DRM_COMMAND (2)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_COMMAND_BASE` | `0x40` |  |
| `DRM_COMMAND_END` | `0xA0` |  |

### DRM_CRTC (2)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_CRTC_SEQUENCE_RELATIVE` | `0x00000001` | sequence is relative to current |
| `DRM_CRTC_SEQUENCE_NEXT_ON_MISS` | `0x00000002` | Use next sequence if we've missed |

### DRM_EVENT (3)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_EVENT_VBLANK` | `0x01` |  |
| `DRM_EVENT_FLIP_COMPLETE` | `0x02` |  |
| `DRM_EVENT_CRTC_SEQUENCE` | `0x03` |  |

### DRM_IOCTL (111)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_BASE` | `'d'` |  |
| `DRM_IOCTL_VERSION` | `DRM_IOWR(0x00, struct drm_version)` |  |
| `DRM_IOCTL_GET_UNIQUE` | `DRM_IOWR(0x01, struct drm_unique)` |  |
| `DRM_IOCTL_GET_MAGIC` | `DRM_IOR( 0x02, struct drm_auth)` |  |
| `DRM_IOCTL_IRQ_BUSID` | `DRM_IOWR(0x03, struct drm_irq_busid)` |  |
| `DRM_IOCTL_GET_MAP` | `DRM_IOWR(0x04, struct drm_map)` |  |
| `DRM_IOCTL_GET_CLIENT` | `DRM_IOWR(0x05, struct drm_client)` |  |
| `DRM_IOCTL_GET_STATS` | `DRM_IOR( 0x06, struct drm_stats)` |  |
| `DRM_IOCTL_SET_VERSION` | `DRM_IOWR(0x07, struct drm_set_version)` |  |
| `DRM_IOCTL_MODESET_CTL` | `DRM_IOW(0x08, struct drm_modeset_ctl)` |  |
| `DRM_IOCTL_GEM_CLOSE` | `DRM_IOW (0x09, struct drm_gem_close)` |  |
| `DRM_IOCTL_GEM_FLINK` | `DRM_IOWR(0x0a, struct drm_gem_flink)` |  |
| `DRM_IOCTL_GEM_OPEN` | `DRM_IOWR(0x0b, struct drm_gem_open)` |  |
| `DRM_IOCTL_GET_CAP` | `DRM_IOWR(0x0c, struct drm_get_cap)` |  |
| `DRM_IOCTL_SET_CLIENT_CAP` | `DRM_IOW( 0x0d, struct drm_set_client_cap)` |  |
| `DRM_IOCTL_SET_UNIQUE` | `DRM_IOW( 0x10, struct drm_unique)` |  |
| `DRM_IOCTL_AUTH_MAGIC` | `DRM_IOW( 0x11, struct drm_auth)` |  |
| `DRM_IOCTL_BLOCK` | `DRM_IOWR(0x12, struct drm_block)` |  |
| `DRM_IOCTL_UNBLOCK` | `DRM_IOWR(0x13, struct drm_block)` |  |
| `DRM_IOCTL_CONTROL` | `DRM_IOW( 0x14, struct drm_control)` |  |
| `DRM_IOCTL_ADD_MAP` | `DRM_IOWR(0x15, struct drm_map)` |  |
| `DRM_IOCTL_ADD_BUFS` | `DRM_IOWR(0x16, struct drm_buf_desc)` |  |
| `DRM_IOCTL_MARK_BUFS` | `DRM_IOW( 0x17, struct drm_buf_desc)` |  |
| `DRM_IOCTL_INFO_BUFS` | `DRM_IOWR(0x18, struct drm_buf_info)` |  |
| `DRM_IOCTL_MAP_BUFS` | `DRM_IOWR(0x19, struct drm_buf_map)` |  |
| `DRM_IOCTL_FREE_BUFS` | `DRM_IOW( 0x1a, struct drm_buf_free)` |  |
| `DRM_IOCTL_RM_MAP` | `DRM_IOW( 0x1b, struct drm_map)` |  |
| `DRM_IOCTL_SET_SAREA_CTX` | `DRM_IOW( 0x1c, struct drm_ctx_priv_map)` |  |
| `DRM_IOCTL_GET_SAREA_CTX` | `DRM_IOWR(0x1d, struct drm_ctx_priv_map)` |  |
| `DRM_IOCTL_SET_MASTER` | `DRM_IO(0x1e)` |  |
| `DRM_IOCTL_DROP_MASTER` | `DRM_IO(0x1f)` |  |
| `DRM_IOCTL_ADD_CTX` | `DRM_IOWR(0x20, struct drm_ctx)` |  |
| `DRM_IOCTL_RM_CTX` | `DRM_IOWR(0x21, struct drm_ctx)` |  |
| `DRM_IOCTL_MOD_CTX` | `DRM_IOW( 0x22, struct drm_ctx)` |  |
| `DRM_IOCTL_GET_CTX` | `DRM_IOWR(0x23, struct drm_ctx)` |  |
| `DRM_IOCTL_SWITCH_CTX` | `DRM_IOW( 0x24, struct drm_ctx)` |  |
| `DRM_IOCTL_NEW_CTX` | `DRM_IOW( 0x25, struct drm_ctx)` |  |
| `DRM_IOCTL_RES_CTX` | `DRM_IOWR(0x26, struct drm_ctx_res)` |  |
| `DRM_IOCTL_ADD_DRAW` | `DRM_IOWR(0x27, struct drm_draw)` |  |
| `DRM_IOCTL_RM_DRAW` | `DRM_IOWR(0x28, struct drm_draw)` |  |
| `DRM_IOCTL_DMA` | `DRM_IOWR(0x29, struct drm_dma)` |  |
| `DRM_IOCTL_LOCK` | `DRM_IOW( 0x2a, struct drm_lock)` |  |
| `DRM_IOCTL_UNLOCK` | `DRM_IOW( 0x2b, struct drm_lock)` |  |
| `DRM_IOCTL_FINISH` | `DRM_IOW( 0x2c, struct drm_lock)` |  |
| `DRM_IOCTL_PRIME_HANDLE_TO_FD` | `DRM_IOWR(0x2d, struct drm_prime_handle)` |  |
| `DRM_IOCTL_PRIME_FD_TO_HANDLE` | `DRM_IOWR(0x2e, struct drm_prime_handle)` |  |
| `DRM_IOCTL_AGP_ACQUIRE` | `DRM_IO(  0x30)` |  |
| `DRM_IOCTL_AGP_RELEASE` | `DRM_IO(  0x31)` |  |
| `DRM_IOCTL_AGP_ENABLE` | `DRM_IOW( 0x32, struct drm_agp_mode)` |  |
| `DRM_IOCTL_AGP_INFO` | `DRM_IOR( 0x33, struct drm_agp_info)` |  |

*...and 61 more*

### DRM_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_MAX_ORDER` | `22` | *< Up to 2^22 bytes = 4MB |

### DRM_MIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_MIN_ORDER` | `5` | *< At least 2^5 bytes = 32 bytes |

### DRM_PRIME (2)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_PRIME_CAP_IMPORT` | `0x1` |  |
| `DRM_PRIME_CAP_EXPORT` | `0x2` |  |

### DRM_RAM (1)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_RAM_PERCENT` | `10` | *< How much system ram can we lock? |

### DRM_SYNCOBJ (10)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_SYNCOBJ_CREATE_SIGNALED` | `(1 << 0)` |  |
| `DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_IMPORT_SYNC_FILE` | `(1 << 0)` |  |
| `DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_TIMELINE` | `(1 << 1)` |  |
| `DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE` | `(1 << 0)` |  |
| `DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_TIMELINE` | `(1 << 1)` |  |
| `DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL` | `(1 << 0)` |  |
| `DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT` | `(1 << 1)` |  |
| `DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE` | `(1 << 2)` | wait for time point to become available |
| `DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE` | `(1 << 3)` | set fence deadline to deadline_nsec |
| `DRM_SYNCOBJ_QUERY_FLAGS_LAST_SUBMITTED` | `(1 << 0)` | last available point on timeline syncobj |

### UNCATEGORIZED (10)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_NAME` | `"drm"` | *< Name in kernel, /dev, and /proc |
| `_DRM_LOCK_HELD` | `0x80000000U` | *< Hardware lock is held |
| `_DRM_LOCK_CONT` | `0x40000000U` | *< Hardware lock is contended |
| `_DRM_VBLANK_HIGH_CRTC_SHIFT` | `1` |  |
| `_DRM_VBLANK_TYPES_MASK` | `(_DRM_VBLANK_ABSOLUTE \| _DRM_VBLANK_RELATIVE)` |  |
| `_DRM_VBLANK_FLAGS_MASK` | `(_DRM_VBLANK_EVENT \| _DRM_VBLANK_SIGNAL \| ` |  |
| `_DRM_PRE_MODESET` | `1` |  |
| `_DRM_POST_MODESET` | `2` |  |
| `DRM_RDWR` | `O_RDWR` |  |
| `DRM_CLOEXEC` | `O_CLOEXEC` |  |

## Structs (58)


### `struct drm_clip_rect`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_drawable_info`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_tex_region`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_hw_lock`

| Type | Field | Array |
|------|-------|-------|
| `char` | `padding` | `60` |

### `struct drm_version`

| Type | Field | Array |
|------|-------|-------|
| `int` | `version_major` | `-` |
| `int` | `version_minor` | `-` |
| `int` | `version_patchlevel` | `-` |
| `__kernel_size_t` | `name_len` | `-` |
| `__kernel_size_t` | `date_len` | `-` |
| `__kernel_size_t` | `desc_len` | `-` |

### `struct drm_unique`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_size_t` | `unique_len` | `-` |

### `struct drm_list`

| Type | Field | Array |
|------|-------|-------|
| `int` | `count` | `-` |

### `struct drm_block`

| Type | Field | Array |
|------|-------|-------|
| `int` | `unused` | `-` |

### `struct drm_control`

| Type | Field | Array |
|------|-------|-------|
| `int` | `irq` | `-` |

### `struct drm_ctx_priv_map`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_map`

| Type | Field | Array |
|------|-------|-------|
| `int` | `mtrr` | `-` |

### `struct drm_client`

| Type | Field | Array |
|------|-------|-------|
| `int` | `idx` | `-` |
| `int` | `auth` | `-` |

### `struct drm_stats`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_lock`

| Type | Field | Array |
|------|-------|-------|
| `int` | `context` | `-` |

### `struct drm_buf_desc`

| Type | Field | Array |
|------|-------|-------|
| `int` | `count` | `-` |
| `int` | `size` | `-` |
| `int` | `low_mark` | `-` |
| `int` | `high_mark` | `-` |

### `struct drm_buf_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `count` | `-` |

### `struct drm_buf_free`

| Type | Field | Array |
|------|-------|-------|
| `int` | `count` | `-` |

### `struct drm_buf_pub`

| Type | Field | Array |
|------|-------|-------|
| `int` | `idx` | `-` |
| `int` | `total` | `-` |
| `int` | `used` | `-` |

### `struct drm_buf_map`

| Type | Field | Array |
|------|-------|-------|
| `int` | `count` | `-` |

### `struct drm_dma`

| Type | Field | Array |
|------|-------|-------|
| `int` | `context` | `-` |
| `int` | `send_count` | `-` |
| `int` | `request_count` | `-` |
| `int` | `request_size` | `-` |
| `int` | `granted_count` | `-` |

### `struct drm_ctx`

| Type | Field | Array |
|------|-------|-------|
| `drm_context_t` | `handle` | `-` |

### `struct drm_ctx_res`

| Type | Field | Array |
|------|-------|-------|
| `int` | `count` | `-` |

### `struct drm_draw`

| Type | Field | Array |
|------|-------|-------|
| `drm_drawable_t` | `handle` | `-` |

### `struct drm_update_draw`

| Type | Field | Array |
|------|-------|-------|
| `drm_drawable_t` | `handle` | `-` |

### `struct drm_auth`

| Type | Field | Array |
|------|-------|-------|
| `drm_magic_t` | `magic` | `-` |

### `struct drm_irq_busid`

| Type | Field | Array |
|------|-------|-------|
| `int` | `irq` | `-` |
| `int` | `busnum` | `-` |
| `int` | `devnum` | `-` |
| `int` | `funcnum` | `-` |

### `struct drm_wait_vblank_request`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_wait_vblank_reply`

| Type | Field | Array |
|------|-------|-------|
| `long` | `tval_sec` | `-` |
| `long` | `tval_usec` | `-` |

### `struct drm_modeset_ctl`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc` | `-` |
| `__u32` | `cmd` | `-` |

### `struct drm_agp_mode`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_agp_buffer`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_agp_binding`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_agp_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `agp_version_major` | `-` |
| `int` | `agp_version_minor` | `-` |

### `struct drm_scatter_gather`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_set_version`

| Type | Field | Array |
|------|-------|-------|
| `int` | `drm_di_major` | `-` |
| `int` | `drm_di_minor` | `-` |
| `int` | `drm_dd_major` | `-` |
| `int` | `drm_dd_minor` | `-` |

### `struct drm_gem_close`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_gem_flink`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `name` | `-` |

### `struct drm_gem_open`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `name` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `size` | `-` |

### `struct drm_gem_change_handle`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `new_handle` | `-` |

### `struct drm_get_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `capability` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_set_client_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `capability` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_prime_handle`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `fd` | `-` |

### `struct drm_syncobj_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_syncobj_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_syncobj_handle`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `fd` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `point` | `-` |

### `struct drm_syncobj_transfer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `src_handle` | `-` |
| `__u32` | `dst_handle` | `-` |
| `__u64` | `src_point` | `-` |
| `__u64` | `dst_point` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_syncobj_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `handles` | `-` |
| `__s64` | `timeout_nsec` | `-` |
| `__u32` | `count_handles` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `first_signaled` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `deadline_nsec` | `-` |

### `struct drm_syncobj_timeline_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `handles` | `-` |
| `__u64` | `points` | `-` |
| `__s64` | `timeout_nsec` | `-` |
| `__u32` | `count_handles` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `first_signaled` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `deadline_nsec` | `-` |

### `struct drm_syncobj_eventfd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `point` | `-` |
| `__s32` | `fd` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_syncobj_array`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `handles` | `-` |
| `__u32` | `count_handles` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_syncobj_timeline_array`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `handles` | `-` |
| `__u64` | `points` | `-` |
| `__u32` | `count_handles` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_crtc_get_sequence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc_id` | `-` |
| `__u32` | `active` | `-` |
| `__u64` | `sequence` | `-` |
| `__s64` | `sequence_ns` | `-` |

### `struct drm_crtc_queue_sequence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `sequence` | `-` |
| `__u64` | `user_data` | `-` |

### `struct drm_set_client_name`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `name_len` | `-` |
| `__u64` | `name` | `-` |

### `struct drm_event`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `length` | `-` |

### `struct drm_event_vblank`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_data` | `-` |
| `__u32` | `tv_sec` | `-` |
| `__u32` | `tv_usec` | `-` |
| `__u32` | `sequence` | `-` |
| `__u32` | `crtc_id` | `-` |

### `struct drm_event_crtc_sequence`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_data` | `-` |
| `__s64` | `time_ns` | `-` |
| `__u64` | `sequence` | `-` |

## Typedefs

- `drm_clip_rect_t`
- `drm_drawable_info_t`
- `drm_tex_region_t`
- `drm_hw_lock_t`
- `drm_version_t`
- `drm_unique_t`
- `drm_list_t`
- `drm_block_t`
- `drm_control_t`
- `drm_map_type_t`
- `drm_map_flags_t`
- `drm_ctx_priv_map_t`
- `drm_map_t`
- `drm_client_t`
- `drm_stat_type_t`
- `drm_stats_t`
- `drm_lock_flags_t`
- `drm_lock_t`
- `drm_dma_flags_t`
- `drm_buf_desc_t`
- `drm_buf_info_t`
- `drm_buf_free_t`
- `drm_buf_pub_t`
- `drm_buf_map_t`
- `drm_dma_t`
- `drm_wait_vblank_t`
- `drm_agp_mode_t`
- `drm_ctx_flags_t`
- `drm_ctx_t`
- `drm_ctx_res_t`
- `drm_draw_t`
- `drm_update_draw_t`
- `drm_auth_t`
- `drm_irq_busid_t`
- `drm_vblank_seq_type_t`
- `drm_agp_buffer_t`
- `drm_agp_binding_t`
- `drm_agp_info_t`
- `drm_scatter_gather_t`
- `drm_set_version_t`

## Enums


### `anonymous_enum_0`

- `DRM_DRAWABLE_CLIPRECTS` 