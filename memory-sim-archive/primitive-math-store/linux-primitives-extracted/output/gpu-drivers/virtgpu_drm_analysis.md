# virtgpu_drm.h

**Source:** `virtgpu_drm.h`


## Includes

- `drm.h`

## Defines (54 total)


### DRM_IOCTL (11)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_VIRTGPU_MAP` | `` |  |
| `DRM_IOCTL_VIRTGPU_EXECBUFFER` | `` |  |
| `DRM_IOCTL_VIRTGPU_GETPARAM` | `` |  |
| `DRM_IOCTL_VIRTGPU_RESOURCE_CREATE` | `` |  |
| `DRM_IOCTL_VIRTGPU_RESOURCE_INFO` | `` |  |
| `DRM_IOCTL_VIRTGPU_TRANSFER_FROM_HOST` | `` |  |
| `DRM_IOCTL_VIRTGPU_TRANSFER_TO_HOST` | `` |  |
| `DRM_IOCTL_VIRTGPU_WAIT` | `` |  |
| `DRM_IOCTL_VIRTGPU_GET_CAPS` | `` |  |
| `DRM_IOCTL_VIRTGPU_RESOURCE_CREATE_BLOB` | `` |  |
| `DRM_IOCTL_VIRTGPU_CONTEXT_INIT` | `` |  |

### DRM_VIRTGPU (11)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_VIRTGPU_MAP` | `0x01` |  |
| `DRM_VIRTGPU_EXECBUFFER` | `0x02` |  |
| `DRM_VIRTGPU_GETPARAM` | `0x03` |  |
| `DRM_VIRTGPU_RESOURCE_CREATE` | `0x04` |  |
| `DRM_VIRTGPU_RESOURCE_INFO` | `0x05` |  |
| `DRM_VIRTGPU_TRANSFER_FROM_HOST` | `0x06` |  |
| `DRM_VIRTGPU_TRANSFER_TO_HOST` | `0x07` |  |
| `DRM_VIRTGPU_WAIT` | `0x08` |  |
| `DRM_VIRTGPU_GET_CAPS` | `0x09` |  |
| `DRM_VIRTGPU_RESOURCE_CREATE_BLOB` | `0x0a` |  |
| `DRM_VIRTGPU_CONTEXT_INIT` | `0x0b` |  |

### VIRTGPU_BLOB (6)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_BLOB_MEM_GUEST` | `0x0001` |  |
| `VIRTGPU_BLOB_MEM_HOST3D` | `0x0002` |  |
| `VIRTGPU_BLOB_MEM_HOST3D_GUEST` | `0x0003` |  |
| `VIRTGPU_BLOB_FLAG_USE_MAPPABLE` | `0x0001` |  |
| `VIRTGPU_BLOB_FLAG_USE_SHAREABLE` | `0x0002` |  |
| `VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE` | `0x0004` |  |

### VIRTGPU_CONTEXT (4)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_CONTEXT_PARAM_CAPSET_ID` | `0x0001` |  |
| `VIRTGPU_CONTEXT_PARAM_NUM_RINGS` | `0x0002` |  |
| `VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK` | `0x0003` |  |
| `VIRTGPU_CONTEXT_PARAM_DEBUG_NAME` | `0x0004` |  |

### VIRTGPU_DRM (6)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_DRM_CAPSET_VIRGL` | `1` |  |
| `VIRTGPU_DRM_CAPSET_VIRGL2` | `2` |  |
| `VIRTGPU_DRM_CAPSET_GFXSTREAM_VULKAN` | `3` |  |
| `VIRTGPU_DRM_CAPSET_VENUS` | `4` |  |
| `VIRTGPU_DRM_CAPSET_CROSS_DOMAIN` | `5` |  |
| `VIRTGPU_DRM_CAPSET_DRM` | `6` |  |

### VIRTGPU_EVENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_EVENT_FENCE_SIGNALED` | `0x90000000` |  |

### VIRTGPU_EXECBUF (6)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_EXECBUF_FENCE_FD_IN` | `0x01` |  |
| `VIRTGPU_EXECBUF_FENCE_FD_OUT` | `0x02` |  |
| `VIRTGPU_EXECBUF_RING_IDX` | `0x04` |  |
| `VIRTGPU_EXECBUF_FLAGS` | `(` |  |
| `VIRTGPU_EXECBUF_SYNCOBJ_RESET` | `0x01` |  |
| `VIRTGPU_EXECBUF_SYNCOBJ_FLAGS` | `( ` |  |

### VIRTGPU_PARAM (8)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_PARAM_3D_FEATURES` | `1` | do we have 3D features in the hw |
| `VIRTGPU_PARAM_CAPSET_QUERY_FIX` | `2` | do we have the capset fix |
| `VIRTGPU_PARAM_RESOURCE_BLOB` | `3` | DRM_VIRTGPU_RESOURCE_CREATE_BLOB |
| `VIRTGPU_PARAM_HOST_VISIBLE` | `4` | Host blob resources are mappable |
| `VIRTGPU_PARAM_CROSS_DEVICE` | `5` | Cross virtio-device resource sharing |
| `VIRTGPU_PARAM_CONTEXT_INIT` | `6` | DRM_VIRTGPU_CONTEXT_INIT |
| `VIRTGPU_PARAM_SUPPORTED_CAPSET_IDs` | `7` | Bitmask of supported capability set ids |
| `VIRTGPU_PARAM_EXPLICIT_DEBUG_NAME` | `8` | Ability to set debug name from userspace |

### VIRTGPU_WAIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTGPU_WAIT_NOWAIT` | `1` | like it |

## Structs (14)


### `struct drm_virtgpu_map`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `offset` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_virtgpu_execbuffer_syncobj`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `point` | `-` |

### `struct drm_virtgpu_execbuffer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `size` | `-` |
| `__u64` | `command` | `-` |
| `__u64` | `bo_handles` | `-` |
| `__u32` | `num_bo_handles` | `-` |
| `__s32` | `fence_fd` | `-` |
| `__u32` | `ring_idx` | `-` |
| `__u32` | `syncobj_stride` | `-` |
| `__u32` | `num_in_syncobjs` | `-` |
| `__u32` | `num_out_syncobjs` | `-` |
| `__u64` | `in_syncobjs` | `-` |
| `__u64` | `out_syncobjs` | `-` |

### `struct drm_virtgpu_getparam`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `param` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_virtgpu_resource_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `target` | `-` |
| `__u32` | `format` | `-` |
| `__u32` | `bind` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `depth` | `-` |
| `__u32` | `array_size` | `-` |
| `__u32` | `last_level` | `-` |
| `__u32` | `nr_samples` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `bo_handle` | `-` |
| `__u32` | `res_handle` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `stride` | `-` |

### `struct drm_virtgpu_resource_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bo_handle` | `-` |
| `__u32` | `res_handle` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `blob_mem` | `-` |

### `struct drm_virtgpu_3d_box`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `x` | `-` |
| `__u32` | `y` | `-` |
| `__u32` | `z` | `-` |
| `__u32` | `w` | `-` |
| `__u32` | `h` | `-` |
| `__u32` | `d` | `-` |

### `struct drm_virtgpu_3d_transfer_to_host`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bo_handle` | `-` |
| `__u32` | `level` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `layer_stride` | `-` |

### `struct drm_virtgpu_3d_transfer_from_host`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bo_handle` | `-` |
| `__u32` | `level` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `layer_stride` | `-` |

### `struct drm_virtgpu_3d_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_virtgpu_get_caps`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cap_set_id` | `-` |
| `__u32` | `cap_set_ver` | `-` |
| `__u64` | `addr` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_virtgpu_resource_create_blob`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `blob_mem` | `-` |
| `__u32` | `blob_flags` | `-` |
| `__u32` | `bo_handle` | `-` |
| `__u32` | `res_handle` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `cmd_size` | `-` |
| `__u64` | `cmd` | `-` |
| `__u64` | `blob_id` | `-` |

### `struct drm_virtgpu_context_set_param`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `param` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_virtgpu_context_init`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_params` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `ctx_set_params` | `-` |