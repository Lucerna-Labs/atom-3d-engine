# lima_drm.h

**Source:** `lima_drm.h`


## Includes

- `drm.h`

## Defines (25 total)


### DRM_IOCTL (7)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_LIMA_GET_PARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_LIMA_GET_PARAM, struct drm_l` |  |
| `DRM_IOCTL_LIMA_GEM_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_LIMA_GEM_CREATE, struct drm_` |  |
| `DRM_IOCTL_LIMA_GEM_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_LIMA_GEM_INFO, struct drm_li` |  |
| `DRM_IOCTL_LIMA_GEM_SUBMIT` | `DRM_IOW(DRM_COMMAND_BASE + DRM_LIMA_GEM_SUBMIT, struct drm_l` |  |
| `DRM_IOCTL_LIMA_GEM_WAIT` | `DRM_IOW(DRM_COMMAND_BASE + DRM_LIMA_GEM_WAIT, struct drm_lim` |  |
| `DRM_IOCTL_LIMA_CTX_CREATE` | `DRM_IOR(DRM_COMMAND_BASE + DRM_LIMA_CTX_CREATE, struct drm_l` |  |
| `DRM_IOCTL_LIMA_CTX_FREE` | `DRM_IOW(DRM_COMMAND_BASE + DRM_LIMA_CTX_FREE, struct drm_lim` |  |

### DRM_LIMA (7)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_LIMA_GET_PARAM` | `0x00` |  |
| `DRM_LIMA_GEM_CREATE` | `0x01` |  |
| `DRM_LIMA_GEM_INFO` | `0x02` |  |
| `DRM_LIMA_GEM_SUBMIT` | `0x03` |  |
| `DRM_LIMA_GEM_WAIT` | `0x04` |  |
| `DRM_LIMA_CTX_CREATE` | `0x05` |  |
| `DRM_LIMA_CTX_FREE` | `0x06` |  |

### LIMA_BO (1)

| Name | Value | Comment |
|------|-------|---------|
| `LIMA_BO_FLAG_HEAP` | `(1 << 0)` |  |

### LIMA_GEM (2)

| Name | Value | Comment |
|------|-------|---------|
| `LIMA_GEM_WAIT_READ` | `0x01` |  |
| `LIMA_GEM_WAIT_WRITE` | `0x02` |  |

### LIMA_GP (1)

| Name | Value | Comment |
|------|-------|---------|
| `LIMA_GP_FRAME_REG_NUM` | `6` |  |

### LIMA_PIPE (2)

| Name | Value | Comment |
|------|-------|---------|
| `LIMA_PIPE_GP` | `0x00` |  |
| `LIMA_PIPE_PP` | `0x01` |  |

### LIMA_PP (2)

| Name | Value | Comment |
|------|-------|---------|
| `LIMA_PP_FRAME_REG_NUM` | `23` |  |
| `LIMA_PP_WB_REG_NUM` | `12` |  |

### LIMA_SUBMIT (3)

| Name | Value | Comment |
|------|-------|---------|
| `LIMA_SUBMIT_BO_READ` | `0x01` |  |
| `LIMA_SUBMIT_BO_WRITE` | `0x02` |  |
| `LIMA_SUBMIT_FLAG_EXPLICIT_FENCE` | `(1 << 0)` |  |

## Structs (11)


### `struct drm_lima_get_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_lima_gem_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_lima_gem_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `va` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_lima_gem_submit_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_lima_gp_frame`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `frame` | `LIMA_GP_FRAME_REG_NUM` |

### `struct drm_lima_m400_pp_frame`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `frame` | `LIMA_PP_FRAME_REG_NUM` |
| `__u32` | `num_pp` | `-` |
| `__u32` | `wb` | `3 * LIMA_PP_WB_REG_NUM` |
| `__u32` | `plbu_array_address` | `4` |
| `__u32` | `fragment_stack_address` | `4` |

### `struct drm_lima_m450_pp_frame`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `frame` | `LIMA_PP_FRAME_REG_NUM` |
| `__u32` | `num_pp` | `-` |
| `__u32` | `wb` | `3 * LIMA_PP_WB_REG_NUM` |
| `__u32` | `use_dlbu` | `-` |
| `__u32` | `_pad` | `-` |
| `__u32` | `plbu_array_address` | `8` |
| `__u32` | `dlbu_regs` | `4` |
| `__u32` | `fragment_stack_address` | `8` |

### `struct drm_lima_gem_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx` | `-` |
| `__u32` | `pipe` | `-` |
| `__u32` | `nr_bos` | `-` |
| `__u32` | `frame_size` | `-` |
| `__u64` | `bos` | `-` |
| `__u64` | `frame` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `out_sync` | `-` |
| `__u32` | `in_sync` | `2` |

### `struct drm_lima_gem_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `op` | `-` |
| `__s64` | `timeout_ns` | `-` |

### `struct drm_lima_ctx_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_lima_ctx_free`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `_pad` | `-` |