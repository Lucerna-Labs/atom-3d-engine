# etnaviv_drm.h

**Source:** `etnaviv_drm.h`


## Includes

- `drm.h`

## Defines (78 total)


### DRM_ETNAVIV (13)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_ETNAVIV_GET_PARAM` | `0x00` |  |
| `DRM_ETNAVIV_SET_PARAM` | `0x01` |  |
| `DRM_ETNAVIV_GEM_NEW` | `0x02` |  |
| `DRM_ETNAVIV_GEM_INFO` | `0x03` |  |
| `DRM_ETNAVIV_GEM_CPU_PREP` | `0x04` |  |
| `DRM_ETNAVIV_GEM_CPU_FINI` | `0x05` |  |
| `DRM_ETNAVIV_GEM_SUBMIT` | `0x06` |  |
| `DRM_ETNAVIV_WAIT_FENCE` | `0x07` |  |
| `DRM_ETNAVIV_GEM_USERPTR` | `0x08` |  |
| `DRM_ETNAVIV_GEM_WAIT` | `0x09` |  |
| `DRM_ETNAVIV_PM_QUERY_DOM` | `0x0a` |  |
| `DRM_ETNAVIV_PM_QUERY_SIG` | `0x0b` |  |
| `DRM_ETNAVIV_NUM_IOCTLS` | `0x0c` |  |

### DRM_IOCTL (11)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_ETNAVIV_GET_PARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_GET_PARAM, struct dr` |  |
| `DRM_IOCTL_ETNAVIV_GEM_NEW` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_NEW, struct drm_` |  |
| `DRM_IOCTL_ETNAVIV_GEM_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_INFO, struct drm` |  |
| `DRM_IOCTL_ETNAVIV_GEM_CPU_PREP` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_CPU_PREP, struct ` |  |
| `DRM_IOCTL_ETNAVIV_GEM_CPU_FINI` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_CPU_FINI, struct ` |  |
| `DRM_IOCTL_ETNAVIV_GEM_SUBMIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_SUBMIT, struct d` |  |
| `DRM_IOCTL_ETNAVIV_WAIT_FENCE` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ETNAVIV_WAIT_FENCE, struct dr` |  |
| `DRM_IOCTL_ETNAVIV_GEM_USERPTR` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_USERPTR, struct ` |  |
| `DRM_IOCTL_ETNAVIV_GEM_WAIT` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ETNAVIV_GEM_WAIT, struct drm_` |  |
| `DRM_IOCTL_ETNAVIV_PM_QUERY_DOM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_PM_QUERY_DOM, struct` |  |
| `DRM_IOCTL_ETNAVIV_PM_QUERY_SIG` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ETNAVIV_PM_QUERY_SIG, struct` |  |

### ETNAVIV_PARAM (30)

| Name | Value | Comment |
|------|-------|---------|
| `ETNAVIV_PARAM_GPU_MODEL` | `0x01` |  |
| `ETNAVIV_PARAM_GPU_REVISION` | `0x02` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_0` | `0x03` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_1` | `0x04` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_2` | `0x05` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_3` | `0x06` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_4` | `0x07` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_5` | `0x08` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_6` | `0x09` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_7` | `0x0a` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_8` | `0x0b` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_9` | `0x0c` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_10` | `0x0d` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_11` | `0x0e` |  |
| `ETNAVIV_PARAM_GPU_FEATURES_12` | `0x0f` |  |
| `ETNAVIV_PARAM_GPU_STREAM_COUNT` | `0x10` |  |
| `ETNAVIV_PARAM_GPU_REGISTER_MAX` | `0x11` |  |
| `ETNAVIV_PARAM_GPU_THREAD_COUNT` | `0x12` |  |
| `ETNAVIV_PARAM_GPU_VERTEX_CACHE_SIZE` | `0x13` |  |
| `ETNAVIV_PARAM_GPU_SHADER_CORE_COUNT` | `0x14` |  |
| `ETNAVIV_PARAM_GPU_PIXEL_PIPES` | `0x15` |  |
| `ETNAVIV_PARAM_GPU_VERTEX_OUTPUT_BUFFER_SIZE` | `0x16` |  |
| `ETNAVIV_PARAM_GPU_BUFFER_SIZE` | `0x17` |  |
| `ETNAVIV_PARAM_GPU_INSTRUCTION_COUNT` | `0x18` |  |
| `ETNAVIV_PARAM_GPU_NUM_CONSTANTS` | `0x19` |  |
| `ETNAVIV_PARAM_GPU_NUM_VARYINGS` | `0x1a` |  |
| `ETNAVIV_PARAM_SOFTPIN_START_ADDR` | `0x1b` |  |
| `ETNAVIV_PARAM_GPU_PRODUCT_ID` | `0x1c` |  |
| `ETNAVIV_PARAM_GPU_CUSTOMER_ID` | `0x1d` |  |
| `ETNAVIV_PARAM_GPU_ECO_ID` | `0x1e` |  |

### ETNA_BO (5)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_BO_CACHE_MASK` | `0x000f0000` |  |
| `ETNA_BO_CACHED` | `0x00010000` |  |
| `ETNA_BO_WC` | `0x00020000` |  |
| `ETNA_BO_UNCACHED` | `0x00040000` |  |
| `ETNA_BO_FORCE_MMU` | `0x00100000` |  |

### ETNA_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_MAX_PIPES` | `4` |  |

### ETNA_PIPE (3)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_PIPE_3D` | `0x00` |  |
| `ETNA_PIPE_2D` | `0x01` |  |
| `ETNA_PIPE_VG` | `0x02` |  |

### ETNA_PM (2)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_PM_PROCESS_PRE` | `0x0001` |  |
| `ETNA_PM_PROCESS_POST` | `0x0002` |  |

### ETNA_PREP (3)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_PREP_READ` | `0x01` |  |
| `ETNA_PREP_WRITE` | `0x02` |  |
| `ETNA_PREP_NOSYNC` | `0x04` |  |

### ETNA_SUBMIT (7)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_SUBMIT_BO_READ` | `0x0001` |  |
| `ETNA_SUBMIT_BO_WRITE` | `0x0002` |  |
| `ETNA_SUBMIT_NO_IMPLICIT` | `0x0001` |  |
| `ETNA_SUBMIT_FENCE_FD_IN` | `0x0002` |  |
| `ETNA_SUBMIT_FENCE_FD_OUT` | `0x0004` |  |
| `ETNA_SUBMIT_SOFTPIN` | `0x0008` |  |
| `ETNA_SUBMIT_FLAGS` | `(ETNA_SUBMIT_NO_IMPLICIT \| ` |  |

### ETNA_USERPTR (2)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_USERPTR_READ` | `0x01` |  |
| `ETNA_USERPTR_WRITE` | `0x02` |  |

### ETNA_WAIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETNA_WAIT_NONBLOCK` | `0x01` |  |

## Structs (15)


### `struct drm_etnaviv_timespec`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `tv_sec` | `-` |
| `__s64` | `tv_nsec` | `-` |

### `struct drm_etnaviv_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pipe` | `-` |
| `__u32` | `param` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_etnaviv_gem_new`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_etnaviv_gem_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_etnaviv_gem_cpu_prep`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `op` | `-` |

### `struct drm_etnaviv_gem_cpu_fini`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_etnaviv_gem_submit_reloc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `submit_offset` | `-` |
| `__u32` | `reloc_idx` | `-` |
| `__u64` | `reloc_offset` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_etnaviv_gem_submit_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `presumed` | `-` |

### `struct drm_etnaviv_gem_submit_pmr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u8` | `domain` | `-` |
| `__u8` | `pad` | `-` |
| `__u16` | `signal` | `-` |
| `__u32` | `sequence` | `-` |
| `__u32` | `read_offset` | `-` |
| `__u32` | `read_idx` | `-` |

### `struct drm_etnaviv_gem_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fence` | `-` |
| `__u32` | `pipe` | `-` |
| `__u32` | `exec_state` | `-` |
| `__u32` | `nr_bos` | `-` |
| `__u32` | `nr_relocs` | `-` |
| `__u32` | `stream_size` | `-` |
| `__u64` | `bos` | `-` |
| `__u64` | `relocs` | `-` |
| `__u64` | `stream` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `fence_fd` | `-` |
| `__u64` | `pmrs` | `-` |
| `__u32` | `nr_pmrs` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_etnaviv_wait_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pipe` | `-` |
| `__u32` | `fence` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_etnaviv_gem_userptr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `user_ptr` | `-` |
| `__u64` | `user_size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_etnaviv_gem_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pipe` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_etnaviv_pm_domain`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pipe` | `-` |
| `__u8` | `iter` | `-` |
| `__u8` | `id` | `-` |
| `__u16` | `nr_signals` | `-` |
| `char` | `name` | `64` |

### `struct drm_etnaviv_pm_signal`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pipe` | `-` |
| `__u8` | `domain` | `-` |
| `__u8` | `pad` | `-` |
| `__u16` | `iter` | `-` |
| `__u16` | `id` | `-` |
| `char` | `name` | `64` |