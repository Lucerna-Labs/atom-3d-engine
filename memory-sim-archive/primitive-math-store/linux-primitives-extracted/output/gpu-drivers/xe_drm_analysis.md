# xe_drm.h

**Source:** `xe_drm.h`


## Includes

- `drm.h`

## Defines (165 total)


### DRM_IOCTL (16)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_XE_DEVICE_QUERY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_DEVICE_QUERY, struct drm_` |  |
| `DRM_IOCTL_XE_GEM_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_GEM_CREATE, struct drm_xe` |  |
| `DRM_IOCTL_XE_GEM_MMAP_OFFSET` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_GEM_MMAP_OFFSET, struct d` |  |
| `DRM_IOCTL_XE_VM_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_VM_CREATE, struct drm_xe_` |  |
| `DRM_IOCTL_XE_VM_DESTROY` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_VM_DESTROY, struct drm_xe_` |  |
| `DRM_IOCTL_XE_VM_BIND` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_VM_BIND, struct drm_xe_vm_` |  |
| `DRM_IOCTL_XE_EXEC_QUEUE_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_CREATE, struct` |  |
| `DRM_IOCTL_XE_EXEC_QUEUE_DESTROY` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_DESTROY, struct` |  |
| `DRM_IOCTL_XE_EXEC_QUEUE_GET_PROPERTY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_GET_PROPERTY, ` |  |
| `DRM_IOCTL_XE_EXEC` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_EXEC, struct drm_xe_exec)` |  |
| `DRM_IOCTL_XE_WAIT_USER_FENCE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_WAIT_USER_FENCE, struct d` |  |
| `DRM_IOCTL_XE_OBSERVATION` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_OBSERVATION, struct drm_xe` |  |
| `DRM_IOCTL_XE_MADVISE` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_MADVISE, struct drm_xe_mad` |  |
| `DRM_IOCTL_XE_VM_QUERY_MEM_RANGE_ATTRS` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_VM_QUERY_MEM_RANGE_ATTRS,` |  |
| `DRM_IOCTL_XE_EXEC_QUEUE_SET_PROPERTY` | `DRM_IOW(DRM_COMMAND_BASE + DRM_XE_EXEC_QUEUE_SET_PROPERTY, s` |  |
| `DRM_IOCTL_XE_VM_GET_PROPERTY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_XE_VM_GET_PROPERTY, struct d` |  |

### DRM_XE (136)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_XE_DEVICE_QUERY` | `0x00` |  |
| `DRM_XE_GEM_CREATE` | `0x01` |  |
| `DRM_XE_GEM_MMAP_OFFSET` | `0x02` |  |
| `DRM_XE_VM_CREATE` | `0x03` |  |
| `DRM_XE_VM_DESTROY` | `0x04` |  |
| `DRM_XE_VM_BIND` | `0x05` |  |
| `DRM_XE_EXEC_QUEUE_CREATE` | `0x06` |  |
| `DRM_XE_EXEC_QUEUE_DESTROY` | `0x07` |  |
| `DRM_XE_EXEC_QUEUE_GET_PROPERTY` | `0x08` |  |
| `DRM_XE_EXEC` | `0x09` |  |
| `DRM_XE_WAIT_USER_FENCE` | `0x0a` |  |
| `DRM_XE_OBSERVATION` | `0x0b` |  |
| `DRM_XE_MADVISE` | `0x0c` |  |
| `DRM_XE_VM_QUERY_MEM_RANGE_ATTRS` | `0x0d` |  |
| `DRM_XE_EXEC_QUEUE_SET_PROPERTY` | `0x0e` |  |
| `DRM_XE_VM_GET_PROPERTY` | `0x0f` |  |
| `DRM_XE_ENGINE_CLASS_RENDER` | `0` |  |
| `DRM_XE_ENGINE_CLASS_COPY` | `1` |  |
| `DRM_XE_ENGINE_CLASS_VIDEO_DECODE` | `2` |  |
| `DRM_XE_ENGINE_CLASS_VIDEO_ENHANCE` | `3` |  |
| `DRM_XE_ENGINE_CLASS_COMPUTE` | `4` |  |
| `DRM_XE_ENGINE_CLASS_VM_BIND` | `5` |  |
| `DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID` | `0` |  |
| `DRM_XE_QUERY_CONFIG_FLAGS` | `1` |  |
| `DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM` | `(1 << 0)` |  |
| `DRM_XE_QUERY_CONFIG_FLAG_HAS_LOW_LATENCY` | `(1 << 1)` |  |
| `DRM_XE_QUERY_CONFIG_FLAG_HAS_CPU_ADDR_MIRROR` | `(1 << 2)` |  |
| `DRM_XE_QUERY_CONFIG_FLAG_HAS_NO_COMPRESSION_HINT` | `(1 << 3)` |  |
| `DRM_XE_QUERY_CONFIG_FLAG_HAS_DISABLE_STATE_CACHE_PERF_FIX` | `(1 << 4)` |  |
| `DRM_XE_QUERY_CONFIG_FLAG_HAS_PURGING_SUPPORT` | `(1 << 5)` |  |
| `DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT` | `2` |  |
| `DRM_XE_QUERY_CONFIG_VA_BITS` | `3` |  |
| `DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY` | `4` |  |
| `DRM_XE_QUERY_GT_TYPE_MAIN` | `0` |  |
| `DRM_XE_QUERY_GT_TYPE_MEDIA` | `1` |  |
| `DRM_XE_TOPO_DSS_GEOMETRY` | `1` |  |
| `DRM_XE_TOPO_DSS_COMPUTE` | `2` |  |
| `DRM_XE_TOPO_L3_BANK` | `3` |  |
| `DRM_XE_TOPO_EU_PER_DSS` | `4` |  |
| `DRM_XE_TOPO_SIMD16_EU_PER_DSS` | `5` |  |
| `DRM_XE_DEVICE_QUERY_ENGINES` | `0` |  |
| `DRM_XE_DEVICE_QUERY_MEM_REGIONS` | `1` |  |
| `DRM_XE_DEVICE_QUERY_CONFIG` | `2` |  |
| `DRM_XE_DEVICE_QUERY_GT_LIST` | `3` |  |
| `DRM_XE_DEVICE_QUERY_HWCONFIG` | `4` |  |
| `DRM_XE_DEVICE_QUERY_GT_TOPOLOGY` | `5` |  |
| `DRM_XE_DEVICE_QUERY_ENGINE_CYCLES` | `6` |  |
| `DRM_XE_DEVICE_QUERY_UC_FW_VERSION` | `7` |  |
| `DRM_XE_DEVICE_QUERY_OA_UNITS` | `8` |  |
| `DRM_XE_DEVICE_QUERY_PXP_STATUS` | `9` |  |

*...and 86 more*

### FAULT_ACCESS (3)

| Name | Value | Comment |
|------|-------|---------|
| `FAULT_ACCESS_TYPE_READ` | `0` |  |
| `FAULT_ACCESS_TYPE_WRITE` | `1` |  |
| `FAULT_ACCESS_TYPE_ATOMIC` | `2` |  |

### FAULT_LEVEL (5)

| Name | Value | Comment |
|------|-------|---------|
| `FAULT_LEVEL_PTE` | `0` |  |
| `FAULT_LEVEL_PDE` | `1` |  |
| `FAULT_LEVEL_PDP` | `2` |  |
| `FAULT_LEVEL_PML4` | `3` |  |
| `FAULT_LEVEL_PML5` | `4` |  |

### FAULT_TYPE (3)

| Name | Value | Comment |
|------|-------|---------|
| `FAULT_TYPE_NOT_PRESENT` | `0` |  |
| `FAULT_TYPE_WRITE_ACCESS` | `1` |  |
| `FAULT_TYPE_ATOMIC_ACCESS` | `2` |  |

### XE_QUERY (2)

| Name | Value | Comment |
|------|-------|---------|
| `XE_QUERY_UC_TYPE_GUC_SUBMISSION` | `0` |  |
| `XE_QUERY_UC_TYPE_HUC` | `1` |  |

## Structs (47)


### `struct drm_xe_user_extension`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `next_extension` | `-` |
| `__u32` | `name` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_xe_ext_set_property`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `property` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |
| `__u64` | `ptr` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_engine_class_instance`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `engine_class` | `-` |
| `__u16` | `engine_instance` | `-` |
| `__u16` | `gt_id` | `-` |
| `__u16` | `pad` | `-` |

### `struct drm_xe_engine`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved` | `3` |

### `struct drm_xe_query_engines`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_engines` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_xe_mem_region`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `mem_class` | `-` |
| `__u16` | `instance` | `-` |
| `__u32` | `min_page_size` | `-` |
| `__u64` | `total_size` | `-` |
| `__u64` | `used` | `-` |
| `__u64` | `cpu_visible_size` | `-` |
| `__u64` | `cpu_visible_used` | `-` |
| `__u64` | `reserved` | `6` |

### `struct drm_xe_query_mem_regions`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_mem_regions` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_xe_query_config`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_params` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_xe_gt`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `type` | `-` |
| `__u16` | `tile_id` | `-` |
| `__u16` | `gt_id` | `-` |
| `__u16` | `pad` | `3` |
| `__u32` | `reference_clock` | `-` |
| `__u64` | `near_mem_regions` | `-` |
| `__u64` | `far_mem_regions` | `-` |
| `__u16` | `ip_ver_major` | `-` |
| `__u16` | `ip_ver_minor` | `-` |
| `__u16` | `ip_ver_rev` | `-` |
| `__u16` | `pad2` | `-` |
| `__u64` | `reserved` | `7` |

### `struct drm_xe_query_gt_list`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_gt` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_xe_query_topology_mask`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `gt_id` | `-` |
| `__u16` | `type` | `-` |
| `__u32` | `num_bytes` | `-` |

### `struct drm_xe_query_engine_cycles`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `clockid` | `-` |
| `__u32` | `width` | `-` |
| `__u64` | `engine_cycles` | `-` |
| `__u64` | `cpu_timestamp` | `-` |
| `__u64` | `cpu_delta` | `-` |

### `struct drm_xe_query_uc_fw_version`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `uc_type` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `branch_ver` | `-` |
| `__u32` | `major_ver` | `-` |
| `__u32` | `minor_ver` | `-` |
| `__u32` | `patch_ver` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `reserved` | `-` |

### `struct drm_xe_query_pxp_status`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `status` | `-` |
| `__u32` | `supported_session_types` | `-` |

### `struct drm_xe_device_query`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `query` | `-` |
| `__u32` | `size` | `-` |
| `__u64` | `data` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_gem_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `placement` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `handle` | `-` |
| `__u16` | `cpu_caching` | `-` |
| `__u16` | `pad` | `3` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_gem_mmap_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_vm_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_vm_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vm_id` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_vm_bind_op`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `obj` | `-` |
| `__u16` | `pat_index` | `-` |
| `__u16` | `pad` | `-` |
| `__u64` | `obj_offset` | `-` |
| `__u64` | `userptr` | `-` |
| `__s64` | `cpu_addr_mirror_offset` | `-` |
| `__u64` | `range` | `-` |
| `__u64` | `addr` | `-` |
| `__u32` | `op` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `prefetch_mem_region_instance` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `reserved` | `3` |

### `struct drm_xe_vm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `exec_queue_id` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `num_binds` | `-` |
| `__u64` | `vector_of_binds` | `-` |
| `__u32` | `pad2` | `-` |
| `__u32` | `num_syncs` | `-` |
| `__u64` | `syncs` | `-` |
| `__u64` | `reserved` | `2` |

### `struct xe_vm_fault`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `address` | `-` |
| `__u32` | `address_precision` | `-` |
| `__u8` | `access_type` | `-` |
| `__u8` | `fault_type` | `-` |
| `__u8` | `fault_level` | `-` |
| `__u8` | `pad` | `-` |
| `__u64` | `reserved` | `4` |

### `struct drm_xe_vm_get_property`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `property` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `data` | `-` |
| `__u64` | `value` | `-` |
| `__u64` | `reserved` | `3` |

### `struct drm_xe_exec_queue_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u16` | `width` | `-` |
| `__u16` | `num_placements` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `exec_queue_id` | `-` |
| `__u64` | `instances` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_exec_queue_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `exec_queue_id` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_exec_queue_get_property`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `exec_queue_id` | `-` |
| `__u32` | `property` | `-` |
| `__u64` | `value` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_sync`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `timeline_value` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_exec`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `exec_queue_id` | `-` |
| `__u32` | `num_syncs` | `-` |
| `__u64` | `syncs` | `-` |
| `__u64` | `address` | `-` |
| `__u16` | `num_batch_buffer` | `-` |
| `__u16` | `pad` | `3` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_wait_user_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `addr` | `-` |
| `__u16` | `op` | `-` |
| `__u16` | `flags` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |
| `__u64` | `mask` | `-` |
| `__s64` | `timeout` | `-` |
| `__u32` | `exec_queue_id` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_observation_param`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `observation_type` | `-` |
| `__u64` | `observation_op` | `-` |
| `__u64` | `param` | `-` |

### `struct drm_xe_oa_unit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `oa_unit_id` | `-` |
| `__u32` | `oa_unit_type` | `-` |
| `__u64` | `capabilities` | `-` |
| `__u64` | `oa_timestamp_freq` | `-` |
| `__u16` | `gt_id` | `-` |
| `__u16` | `reserved1` | `3` |
| `__u64` | `reserved` | `3` |
| `__u64` | `num_engines` | `-` |

### `struct drm_xe_query_oa_units`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `num_oa_units` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_xe_oa_config`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `char` | `uuid` | `36` |
| `__u32` | `n_regs` | `-` |
| `__u64` | `regs_ptr` | `-` |

### `struct drm_xe_oa_stream_status`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `oa_status` | `-` |
| `__u64` | `reserved` | `3` |

### `struct drm_xe_oa_stream_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `oa_buf_size` | `-` |
| `__u64` | `reserved` | `3` |

### `struct drm_xe_query_eu_stall`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `capabilities` | `-` |
| `__u64` | `record_size` | `-` |
| `__u64` | `per_xecore_buf_size` | `-` |
| `__u64` | `reserved` | `5` |
| `__u64` | `num_sampling_rates` | `-` |

### `struct drm_xe_madvise`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `range` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `devmem_fd` | `-` |
| `__u16` | `migration_policy` | `-` |
| `__u16` | `region_instance` | `-` |
| `__u64` | `reserved` | `-` |
| `__u32` | `val` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `reserved` | `-` |
| `__u32` | `val` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `reserved` | `-` |
| `__u32` | `val` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `retained_ptr` | `-` |
| `__u64` | `reserved` | `2` |

### `struct anonymous_37`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `devmem_fd` | `-` |
| `__u16` | `migration_policy` | `-` |
| `__u16` | `region_instance` | `-` |
| `__u64` | `reserved` | `-` |

### `struct anonymous_38`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `val` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `reserved` | `-` |

### `struct anonymous_39`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `val` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `reserved` | `-` |

### `struct anonymous_40`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `val` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `retained_ptr` | `-` |

### `struct drm_xe_mem_range_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |
| `__u32` | `devmem_fd` | `-` |
| `__u32` | `migration_policy` | `-` |
| `__u32` | `val` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `val` | `-` |
| `__u32` | `reserved` | `-` |
| `__u64` | `reserved` | `2` |

### `struct anonymous_42`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `devmem_fd` | `-` |
| `__u32` | `migration_policy` | `-` |

### `struct anonymous_43`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `val` | `-` |
| `__u32` | `reserved` | `-` |

### `struct anonymous_44`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `val` | `-` |
| `__u32` | `reserved` | `-` |

### `struct drm_xe_vm_query_mem_range_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `num_mem_ranges` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `range` | `-` |
| `__u64` | `sizeof_mem_range_attr` | `-` |
| `__u64` | `vector_of_mem_attr` | `-` |
| `__u64` | `reserved` | `2` |

### `struct drm_xe_exec_queue_set_property`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `extensions` | `-` |
| `__u32` | `exec_queue_id` | `-` |
| `__u32` | `property` | `-` |
| `__u64` | `value` | `-` |
| `__u64` | `reserved` | `2` |