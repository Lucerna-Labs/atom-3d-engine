# rocket_accel.h

**Source:** `rocket_accel.h`


## Includes

- `drm.h`

## Defines (8 total)


### DRM_IOCTL (4)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_ROCKET_CREATE_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_ROCKET_CREATE_BO, struct drm` |  |
| `DRM_IOCTL_ROCKET_SUBMIT` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ROCKET_SUBMIT, struct drm_roc` |  |
| `DRM_IOCTL_ROCKET_PREP_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ROCKET_PREP_BO, struct drm_ro` |  |
| `DRM_IOCTL_ROCKET_FINI_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_ROCKET_FINI_BO, struct drm_ro` |  |

### DRM_ROCKET (4)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_ROCKET_CREATE_BO` | `0x00` |  |
| `DRM_ROCKET_SUBMIT` | `0x01` |  |
| `DRM_ROCKET_PREP_BO` | `0x02` |  |
| `DRM_ROCKET_FINI_BO` | `0x03` |  |

## Structs (6)


### `struct drm_rocket_create_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `dma_address` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_rocket_prep_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `reserved` | `-` |
| `__s64` | `timeout_ns` | `-` |

### `struct drm_rocket_fini_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `reserved` | `-` |

### `struct drm_rocket_task`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `regcmd` | `-` |
| `__u32` | `regcmd_count` | `-` |

### `struct drm_rocket_job`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `tasks` | `-` |
| `__u64` | `in_bo_handles` | `-` |
| `__u64` | `out_bo_handles` | `-` |
| `__u32` | `task_count` | `-` |
| `__u32` | `task_struct_size` | `-` |
| `__u32` | `in_bo_handle_count` | `-` |
| `__u32` | `out_bo_handle_count` | `-` |

### `struct drm_rocket_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `jobs` | `-` |
| `__u32` | `job_count` | `-` |
| `__u32` | `job_struct_size` | `-` |
| `__u64` | `reserved` | `-` |