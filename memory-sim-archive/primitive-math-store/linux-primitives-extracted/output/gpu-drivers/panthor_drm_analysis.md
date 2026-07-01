# panthor_drm.h

**Source:** `panthor_drm.h`


## Includes

- `drm.h`

## Defines (4 total)


### DRM_PANTHOR (4)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_PANTHOR_USER_MMIO_OFFSET_32BIT` | `(1ull << 43)` |  |
| `DRM_PANTHOR_USER_MMIO_OFFSET_64BIT` | `(1ull << 56)` |  |
| `DRM_PANTHOR_USER_MMIO_OFFSET` | `(sizeof(unsigned long) < 8 ? ` |  |
| `DRM_PANTHOR_USER_FLUSH_ID_MMIO_OFFSET` | `(DRM_PANTHOR_USER_MMIO_OFFSET \| 0)` |  |

## Structs (27)


### `struct drm_panthor_obj_array`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `stride` | `-` |
| `__u32` | `count` | `-` |
| `__u64` | `array` | `-` |

### `struct drm_panthor_sync_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `timeline_value` | `-` |

### `struct drm_panthor_gpu_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gpu_id` | `-` |
| `__u32` | `gpu_rev` | `-` |
| `__u32` | `csf_id` | `-` |
| `__u32` | `l2_features` | `-` |
| `__u32` | `tiler_features` | `-` |
| `__u32` | `mem_features` | `-` |
| `__u32` | `mmu_features` | `-` |
| `__u32` | `thread_features` | `-` |
| `__u32` | `max_threads` | `-` |
| `__u32` | `thread_max_workgroup_size` | `-` |
| `__u32` | `thread_max_barrier_size` | `-` |
| `__u32` | `coherency_features` | `-` |
| `__u32` | `texture_features` | `4` |
| `__u32` | `as_present` | `-` |
| `__u32` | `selected_coherency` | `-` |
| `__u64` | `shader_present` | `-` |
| `__u64` | `l2_present` | `-` |
| `__u64` | `tiler_present` | `-` |
| `__u32` | `core_features` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `gpu_features` | `-` |

### `struct drm_panthor_csif_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `csg_slot_count` | `-` |
| `__u32` | `cs_slot_count` | `-` |
| `__u32` | `cs_reg_count` | `-` |
| `__u32` | `scoreboard_slot_count` | `-` |
| `__u32` | `unpreserved_cs_reg_count` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_timestamp_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `timestamp_frequency` | `-` |
| `__u64` | `current_timestamp` | `-` |
| `__u64` | `timestamp_offset` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `duration_nsec` | `-` |
| `__u64` | `cycle_count` | `-` |
| `__u64` | `cpu_timestamp_sec` | `-` |
| `__u64` | `cpu_timestamp_nsec` | `-` |

### `struct drm_panthor_group_priorities_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `allowed_mask` | `-` |
| `__u8` | `pad` | `3` |

### `struct drm_panthor_dev_query`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `size` | `-` |
| `__u64` | `pointer` | `-` |

### `struct drm_panthor_vm_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `id` | `-` |
| `__u64` | `user_va_range` | `-` |

### `struct drm_panthor_vm_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_vm_bind_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `bo_handle` | `-` |
| `__u64` | `bo_offset` | `-` |
| `__u64` | `va` | `-` |
| `__u64` | `size` | `-` |

### `struct drm_panthor_vm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vm_id` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_panthor_vm_get_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vm_id` | `-` |
| `__u32` | `state` | `-` |

### `struct drm_panthor_bo_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `exclusive_vm_id` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_bo_mmap_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_panthor_queue_create`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `priority` | `-` |
| `__u8` | `pad` | `3` |
| `__u32` | `ringbuf_size` | `-` |

### `struct drm_panthor_group_create`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `max_compute_cores` | `-` |
| `__u8` | `max_fragment_cores` | `-` |
| `__u8` | `max_tiler_cores` | `-` |
| `__u8` | `priority` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `compute_core_mask` | `-` |
| `__u64` | `fragment_core_mask` | `-` |
| `__u64` | `tiler_core_mask` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `group_handle` | `-` |

### `struct drm_panthor_group_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `group_handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_queue_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `queue_index` | `-` |
| `__u32` | `stream_size` | `-` |
| `__u64` | `stream_addr` | `-` |
| `__u32` | `latest_flush` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_group_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `group_handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_group_get_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `group_handle` | `-` |
| `__u32` | `state` | `-` |
| `__u32` | `fatal_queues` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_tiler_heap_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vm_id` | `-` |
| `__u32` | `initial_chunk_count` | `-` |
| `__u32` | `chunk_size` | `-` |
| `__u32` | `max_chunks` | `-` |
| `__u32` | `target_in_flight` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `tiler_heap_ctx_gpu_va` | `-` |
| `__u64` | `first_heap_chunk_gpu_va` | `-` |

### `struct drm_panthor_tiler_heap_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panthor_bo_set_label`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `label` | `-` |

### `struct drm_panthor_set_user_mmio_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `offset` | `-` |

### `struct drm_panthor_bo_sync_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `type` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |

### `struct drm_panthor_bo_sync`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_panthor_bo_query_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `extra_flags` | `-` |
| `__u32` | `create_flags` | `-` |
| `__u32` | `pad` | `-` |