# nouveau_drm.h

**Source:** `nouveau_drm.h`


## Includes

- `drm.h`

## Defines (97 total)


### DRM_IOCTL (14)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_NOUVEAU_GETPARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_GETPARAM, struct drm` |  |
| `DRM_IOCTL_NOUVEAU_CHANNEL_ALLOC` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_CHANNEL_ALLOC, struc` |  |
| `DRM_IOCTL_NOUVEAU_CHANNEL_FREE` | `DRM_IOW (DRM_COMMAND_BASE + DRM_NOUVEAU_CHANNEL_FREE, struct` |  |
| `DRM_IOCTL_NOUVEAU_SVM_INIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_SVM_INIT, struct drm` |  |
| `DRM_IOCTL_NOUVEAU_SVM_BIND` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_SVM_BIND, struct drm` |  |
| `DRM_IOCTL_NOUVEAU_GEM_NEW` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_NEW, struct drm_` |  |
| `DRM_IOCTL_NOUVEAU_GEM_PUSHBUF` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_PUSHBUF, struct ` |  |
| `DRM_IOCTL_NOUVEAU_GEM_CPU_PREP` | `DRM_IOW (DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_CPU_PREP, struct` |  |
| `DRM_IOCTL_NOUVEAU_GEM_CPU_FINI` | `DRM_IOW (DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_CPU_FINI, struct` |  |
| `DRM_IOCTL_NOUVEAU_GEM_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_INFO, struct drm` |  |
| `DRM_IOCTL_NOUVEAU_VM_INIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_VM_INIT, struct drm_` |  |
| `DRM_IOCTL_NOUVEAU_VM_BIND` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_VM_BIND, struct drm_` |  |
| `DRM_IOCTL_NOUVEAU_EXEC` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_NOUVEAU_EXEC, struct drm_nou` |  |
| `DRM_IOCTL_NOUVEAU_GET_ZCULL_INFO` | `DRM_IOR (DRM_COMMAND_BASE + DRM_NOUVEAU_GET_ZCULL_INFO, stru` |  |

### DRM_NOUVEAU (28)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_NOUVEAU_EVENT_NVIF` | `0x80000000` |  |
| `DRM_NOUVEAU_SYNC_SYNCOBJ` | `0x0` |  |
| `DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ` | `0x1` |  |
| `DRM_NOUVEAU_SYNC_TYPE_MASK` | `0xf` |  |
| `DRM_NOUVEAU_VM_BIND_OP_MAP` | `0x0` |  |
| `DRM_NOUVEAU_VM_BIND_OP_UNMAP` | `0x1` |  |
| `DRM_NOUVEAU_VM_BIND_SPARSE` | `(1 << 8)` |  |
| `DRM_NOUVEAU_VM_BIND_RUN_ASYNC` | `0x1` |  |
| `DRM_NOUVEAU_EXEC_PUSH_NO_PREFETCH` | `0x1` |  |
| `DRM_NOUVEAU_GETPARAM` | `0x00` |  |
| `DRM_NOUVEAU_SETPARAM` | `0x01` | deprecated |
| `DRM_NOUVEAU_CHANNEL_ALLOC` | `0x02` |  |
| `DRM_NOUVEAU_CHANNEL_FREE` | `0x03` |  |
| `DRM_NOUVEAU_GROBJ_ALLOC` | `0x04` | deprecated |
| `DRM_NOUVEAU_NOTIFIEROBJ_ALLOC` | `0x05` | deprecated |
| `DRM_NOUVEAU_GPUOBJ_FREE` | `0x06` | deprecated |
| `DRM_NOUVEAU_NVIF` | `0x07` |  |
| `DRM_NOUVEAU_SVM_INIT` | `0x08` |  |
| `DRM_NOUVEAU_SVM_BIND` | `0x09` |  |
| `DRM_NOUVEAU_VM_INIT` | `0x10` |  |
| `DRM_NOUVEAU_VM_BIND` | `0x11` |  |
| `DRM_NOUVEAU_EXEC` | `0x12` |  |
| `DRM_NOUVEAU_GET_ZCULL_INFO` | `0x13` |  |
| `DRM_NOUVEAU_GEM_NEW` | `0x40` |  |
| `DRM_NOUVEAU_GEM_PUSHBUF` | `0x41` |  |
| `DRM_NOUVEAU_GEM_CPU_PREP` | `0x42` |  |
| `DRM_NOUVEAU_GEM_CPU_FINI` | `0x43` |  |
| `DRM_NOUVEAU_GEM_INFO` | `0x44` |  |

### NOUVEAU_FIFO (5)

| Name | Value | Comment |
|------|-------|---------|
| `NOUVEAU_FIFO_ENGINE_GR` | `0x01` |  |
| `NOUVEAU_FIFO_ENGINE_VP` | `0x02` |  |
| `NOUVEAU_FIFO_ENGINE_PPP` | `0x04` |  |
| `NOUVEAU_FIFO_ENGINE_BSP` | `0x08` |  |
| `NOUVEAU_FIFO_ENGINE_CE` | `0x30` |  |

### NOUVEAU_GEM (22)

| Name | Value | Comment |
|------|-------|---------|
| `NOUVEAU_GEM_DOMAIN_CPU` | `(1 << 0)` |  |
| `NOUVEAU_GEM_DOMAIN_VRAM` | `(1 << 1)` |  |
| `NOUVEAU_GEM_DOMAIN_GART` | `(1 << 2)` |  |
| `NOUVEAU_GEM_DOMAIN_MAPPABLE` | `(1 << 3)` |  |
| `NOUVEAU_GEM_DOMAIN_COHERENT` | `(1 << 4)` |  |
| `NOUVEAU_GEM_DOMAIN_NO_SHARE` | `(1 << 5)` |  |
| `NOUVEAU_GEM_TILE_COMP` | `0x00030000` | nv50-only |
| `NOUVEAU_GEM_TILE_LAYOUT_MASK` | `0x0000ff00` |  |
| `NOUVEAU_GEM_TILE_16BPP` | `0x00000001` |  |
| `NOUVEAU_GEM_TILE_32BPP` | `0x00000002` |  |
| `NOUVEAU_GEM_TILE_ZETA` | `0x00000004` |  |
| `NOUVEAU_GEM_TILE_NONCONTIG` | `0x00000008` |  |
| `NOUVEAU_GEM_MAX_BUFFERS` | `1024` |  |
| `NOUVEAU_GEM_RELOC_LOW` | `(1 << 0)` |  |
| `NOUVEAU_GEM_RELOC_HIGH` | `(1 << 1)` |  |
| `NOUVEAU_GEM_RELOC_OR` | `(1 << 2)` |  |
| `NOUVEAU_GEM_MAX_RELOCS` | `1024` |  |
| `NOUVEAU_GEM_MAX_PUSH` | `512` |  |
| `NOUVEAU_GEM_PUSHBUF_NO_PREFETCH` | `(1 << 23)` |  |
| `NOUVEAU_GEM_PUSHBUF_SYNC` | `(1ULL << 0)` |  |
| `NOUVEAU_GEM_CPU_PREP_NOWAIT` | `0x00000001` |  |
| `NOUVEAU_GEM_CPU_PREP_WRITE` | `0x00000004` |  |

### NOUVEAU_GETPARAM (15)

| Name | Value | Comment |
|------|-------|---------|
| `NOUVEAU_GETPARAM_PCI_VENDOR` | `3` |  |
| `NOUVEAU_GETPARAM_PCI_DEVICE` | `4` |  |
| `NOUVEAU_GETPARAM_BUS_TYPE` | `5` |  |
| `NOUVEAU_GETPARAM_FB_SIZE` | `8` |  |
| `NOUVEAU_GETPARAM_AGP_SIZE` | `9` |  |
| `NOUVEAU_GETPARAM_CHIPSET_ID` | `11` |  |
| `NOUVEAU_GETPARAM_VM_VRAM_BASE` | `12` |  |
| `NOUVEAU_GETPARAM_GRAPH_UNITS` | `13` |  |
| `NOUVEAU_GETPARAM_PTIMER_TIME` | `14` |  |
| `NOUVEAU_GETPARAM_HAS_BO_USAGE` | `15` |  |
| `NOUVEAU_GETPARAM_HAS_PAGEFLIP` | `16` |  |
| `NOUVEAU_GETPARAM_EXEC_PUSH_MAX` | `17` |  |
| `NOUVEAU_GETPARAM_VRAM_BAR_SIZE` | `18` |  |
| `NOUVEAU_GETPARAM_VRAM_USED` | `19` |  |
| `NOUVEAU_GETPARAM_HAS_VMA_TILEMODE` | `20` |  |

### NOUVEAU_SVM (13)

| Name | Value | Comment |
|------|-------|---------|
| `NOUVEAU_SVM_BIND_COMMAND_SHIFT` | `0` |  |
| `NOUVEAU_SVM_BIND_COMMAND_BITS` | `8` |  |
| `NOUVEAU_SVM_BIND_COMMAND_MASK` | `((1 << 8) - 1)` |  |
| `NOUVEAU_SVM_BIND_PRIORITY_SHIFT` | `8` |  |
| `NOUVEAU_SVM_BIND_PRIORITY_BITS` | `8` |  |
| `NOUVEAU_SVM_BIND_PRIORITY_MASK` | `((1 << 8) - 1)` |  |
| `NOUVEAU_SVM_BIND_TARGET_SHIFT` | `16` |  |
| `NOUVEAU_SVM_BIND_TARGET_BITS` | `32` |  |
| `NOUVEAU_SVM_BIND_TARGET_MASK` | `0xffffffff` |  |
| `NOUVEAU_SVM_BIND_VALID_BITS` | `48` |  |
| `NOUVEAU_SVM_BIND_VALID_MASK` | `((1ULL << NOUVEAU_SVM_BIND_VALID_BITS) - 1)` |  |
| `NOUVEAU_SVM_BIND_COMMAND__MIGRATE` | `0` |  |
| `NOUVEAU_SVM_BIND_TARGET__GPU_VRAM` | `(1UL << 31)` |  |

## Structs (24)


### `struct drm_nouveau_getparam`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `param` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_nouveau_channel_alloc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_ctxdma_handle` | `-` |
| `__u32` | `tt_ctxdma_handle` | `-` |
| `__s32` | `channel` | `-` |
| `__u32` | `pushbuf_domains` | `-` |
| `__u32` | `notifier_handle` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `grclass` | `-` |
| `__u32` | `nr_subchan` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `grclass` | `-` |

### `struct drm_nouveau_channel_free`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `channel` | `-` |

### `struct drm_nouveau_notifierobj_alloc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `channel` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `offset` | `-` |

### `struct drm_nouveau_gpuobj_free`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `channel` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_nouveau_gem_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `domain` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `map_handle` | `-` |
| `__u32` | `tile_mode` | `-` |
| `__u32` | `tile_flags` | `-` |

### `struct drm_nouveau_gem_new`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `channel_hint` | `-` |
| `__u32` | `align` | `-` |

### `struct drm_nouveau_gem_pushbuf_bo_presumed`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `valid` | `-` |
| `__u32` | `domain` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_nouveau_gem_pushbuf_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_priv` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `read_domains` | `-` |
| `__u32` | `write_domains` | `-` |
| `__u32` | `valid_domains` | `-` |

### `struct drm_nouveau_gem_pushbuf_reloc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `reloc_bo_index` | `-` |
| `__u32` | `reloc_bo_offset` | `-` |
| `__u32` | `bo_index` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `data` | `-` |
| `__u32` | `vor` | `-` |
| `__u32` | `tor` | `-` |

### `struct drm_nouveau_gem_pushbuf_push`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bo_index` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `length` | `-` |

### `struct drm_nouveau_gem_pushbuf`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `channel` | `-` |
| `__u32` | `nr_buffers` | `-` |
| `__u64` | `buffers` | `-` |
| `__u32` | `nr_relocs` | `-` |
| `__u32` | `nr_push` | `-` |
| `__u64` | `relocs` | `-` |
| `__u64` | `push` | `-` |
| `__u32` | `suffix0` | `-` |
| `__u32` | `suffix1` | `-` |
| `__u64` | `vram_available` | `-` |
| `__u64` | `gart_available` | `-` |

### `struct drm_nouveau_gem_cpu_prep`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_nouveau_gem_cpu_fini`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |

### `struct drm_nouveau_sync`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `timeline_value` | `-` |

### `struct drm_nouveau_vm_init`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `kernel_managed_addr` | `-` |
| `__u64` | `kernel_managed_size` | `-` |

### `struct drm_nouveau_vm_bind_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `bo_offset` | `-` |
| `__u64` | `range` | `-` |

### `struct drm_nouveau_vm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op_count` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `wait_count` | `-` |
| `__u32` | `sig_count` | `-` |
| `__u64` | `wait_ptr` | `-` |
| `__u64` | `sig_ptr` | `-` |
| `__u64` | `op_ptr` | `-` |

### `struct drm_nouveau_exec_push`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `va` | `-` |
| `__u32` | `va_len` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_nouveau_exec`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `channel` | `-` |
| `__u32` | `push_count` | `-` |
| `__u32` | `wait_count` | `-` |
| `__u32` | `sig_count` | `-` |
| `__u64` | `wait_ptr` | `-` |
| `__u64` | `sig_ptr` | `-` |
| `__u64` | `push_ptr` | `-` |

### `struct drm_nouveau_get_zcull_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `width_align_pixels` | `-` |
| `__u32` | `height_align_pixels` | `-` |
| `__u32` | `pixel_squares_by_aliquots` | `-` |
| `__u32` | `aliquot_total` | `-` |
| `__u32` | `zcull_region_byte_multiplier` | `-` |
| `__u32` | `zcull_region_header_size` | `-` |
| `__u32` | `zcull_subregion_header_size` | `-` |
| `__u32` | `subregion_count` | `-` |
| `__u32` | `subregion_width_align_pixels` | `-` |
| `__u32` | `subregion_height_align_pixels` | `-` |
| `__u32` | `ctxsw_size` | `-` |
| `__u32` | `ctxsw_align` | `-` |

### `struct drm_nouveau_svm_init`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `unmanaged_addr` | `-` |
| `__u64` | `unmanaged_size` | `-` |

### `struct drm_nouveau_svm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `header` | `-` |
| `__u64` | `va_start` | `-` |
| `__u64` | `va_end` | `-` |
| `__u64` | `npages` | `-` |
| `__u64` | `stride` | `-` |
| `__u64` | `result` | `-` |
| `__u64` | `reserved0` | `-` |
| `__u64` | `reserved1` | `-` |