# panfrost_drm.h

**Source:** `panfrost_drm.h`


## Includes

- `drm.h`

## Defines (45 total)


### DRM_IOCTL (14)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_PANFROST_SUBMIT` | `DRM_IOW(DRM_COMMAND_BASE + DRM_PANFROST_SUBMIT, struct drm_p` |  |
| `DRM_IOCTL_PANFROST_WAIT_BO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_PANFROST_WAIT_BO, struct drm_` |  |
| `DRM_IOCTL_PANFROST_CREATE_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_CREATE_BO, struct d` |  |
| `DRM_IOCTL_PANFROST_MMAP_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_MMAP_BO, struct drm` |  |
| `DRM_IOCTL_PANFROST_GET_PARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_GET_PARAM, struct d` |  |
| `DRM_IOCTL_PANFROST_GET_BO_OFFSET` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_GET_BO_OFFSET, stru` |  |
| `DRM_IOCTL_PANFROST_MADVISE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_MADVISE, struct drm` |  |
| `DRM_IOCTL_PANFROST_SET_LABEL_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_SET_LABEL_BO, struc` |  |
| `DRM_IOCTL_PANFROST_JM_CTX_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_JM_CTX_CREATE, stru` |  |
| `DRM_IOCTL_PANFROST_JM_CTX_DESTROY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_JM_CTX_DESTROY, str` |  |
| `DRM_IOCTL_PANFROST_SYNC_BO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_SYNC_BO, struct drm` |  |
| `DRM_IOCTL_PANFROST_QUERY_BO_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_PANFROST_QUERY_BO_INFO, stru` |  |
| `DRM_IOCTL_PANFROST_PERFCNT_ENABLE` | `DRM_IOW(DRM_COMMAND_BASE + DRM_PANFROST_PERFCNT_ENABLE, stru` |  |
| `DRM_IOCTL_PANFROST_PERFCNT_DUMP` | `DRM_IOW(DRM_COMMAND_BASE + DRM_PANFROST_PERFCNT_DUMP, struct` |  |

### DRM_PANFROST (15)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_PANFROST_SUBMIT` | `0x00` |  |
| `DRM_PANFROST_WAIT_BO` | `0x01` |  |
| `DRM_PANFROST_CREATE_BO` | `0x02` |  |
| `DRM_PANFROST_MMAP_BO` | `0x03` |  |
| `DRM_PANFROST_GET_PARAM` | `0x04` |  |
| `DRM_PANFROST_GET_BO_OFFSET` | `0x05` |  |
| `DRM_PANFROST_PERFCNT_ENABLE` | `0x06` |  |
| `DRM_PANFROST_PERFCNT_DUMP` | `0x07` |  |
| `DRM_PANFROST_MADVISE` | `0x08` |  |
| `DRM_PANFROST_SET_LABEL_BO` | `0x09` |  |
| `DRM_PANFROST_JM_CTX_CREATE` | `0x0a` |  |
| `DRM_PANFROST_JM_CTX_DESTROY` | `0x0b` |  |
| `DRM_PANFROST_SYNC_BO` | `0x0c` |  |
| `DRM_PANFROST_QUERY_BO_INFO` | `0x0d` |  |
| `DRM_PANFROST_BO_IS_IMPORTED` | `(1 << 0)` |  |

### PANFROSTDUMP_BUF (4)

| Name | Value | Comment |
|------|-------|---------|
| `PANFROSTDUMP_BUF_REG` | `0` |  |
| `PANFROSTDUMP_BUF_BOMAP` | `(PANFROSTDUMP_BUF_REG + 1)` |  |
| `PANFROSTDUMP_BUF_BO` | `(PANFROSTDUMP_BUF_BOMAP + 1)` |  |
| `PANFROSTDUMP_BUF_TRAILER` | `(PANFROSTDUMP_BUF_BO + 1)` |  |

### PANFROST_BO (5)

| Name | Value | Comment |
|------|-------|---------|
| `PANFROST_BO_NOEXEC` | `1` |  |
| `PANFROST_BO_HEAP` | `2` |  |
| `PANFROST_BO_WB_MMAP` | `4` |  |
| `PANFROST_BO_SYNC_CPU_CACHE_FLUSH` | `0` |  |
| `PANFROST_BO_SYNC_CPU_CACHE_FLUSH_AND_INVALIDATE` | `1` |  |

### PANFROST_JD (2)

| Name | Value | Comment |
|------|-------|---------|
| `PANFROST_JD_REQ_FS` | `(1 << 0)` |  |
| `PANFROST_JD_REQ_CYCLE_COUNT` | `(1 << 1)` |  |

### PANFROST_MADV (2)

| Name | Value | Comment |
|------|-------|---------|
| `PANFROST_MADV_WILLNEED` | `0` | backing pages are needed, status returned in 'retained' |
| `PANFROST_MADV_DONTNEED` | `1` | backing pages not needed |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `PANFROSTDUMP_MAJOR` | `1` |  |
| `PANFROSTDUMP_MINOR` | `0` |  |
| `PANFROSTDUMP_MAGIC` | `0x464E4150` | PANF |

## Structs (19)


### `struct drm_panfrost_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `jc` | `-` |
| `__u64` | `in_syncs` | `-` |
| `__u32` | `in_sync_count` | `-` |
| `__u32` | `out_sync` | `-` |
| `__u64` | `bo_handles` | `-` |
| `__u32` | `bo_handle_count` | `-` |
| `__u32` | `requirements` | `-` |
| `__u32` | `jm_ctx_handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panfrost_wait_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__s64` | `timeout_ns` | `-` |

### `struct drm_panfrost_create_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_panfrost_mmap_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_panfrost_get_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_panfrost_get_bo_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_panfrost_perfcnt_enable`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `enable` | `-` |
| `__u32` | `counterset` | `-` |

### `struct drm_panfrost_perfcnt_dump`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buf_ptr` | `-` |

### `struct drm_panfrost_madvise`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `madv` | `-` |
| `__u32` | `retained` | `-` |

### `struct drm_panfrost_set_label_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `label` | `-` |

### `struct drm_panfrost_bo_sync_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `size` | `-` |

### `struct drm_panfrost_sync_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ops` | `-` |
| `__u32` | `op_count` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_panfrost_query_bo_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `extra_flags` | `-` |
| `__u32` | `create_flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct panfrost_dump_object_header`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `magic` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `file_size` | `-` |
| `__u32` | `file_offset` | `-` |
| `__u64` | `jc` | `-` |
| `__u32` | `gpu_id` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u64` | `nbos` | `-` |
| `__u32` | `valid` | `-` |
| `__u64` | `iova` | `-` |
| `__u32` | `data` | `2` |
| `__u32` | `sizer` | `496` |

### `struct anonymous_14`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `jc` | `-` |
| `__u32` | `gpu_id` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |
| `__u64` | `nbos` | `-` |

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `valid` | `-` |
| `__u64` | `iova` | `-` |
| `__u32` | `data` | `2` |

### `struct panfrost_dump_registers`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `reg` | `-` |
| `__u32` | `value` | `-` |

### `struct drm_panfrost_jm_ctx_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `priority` | `-` |

### `struct drm_panfrost_jm_ctx_destroy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |