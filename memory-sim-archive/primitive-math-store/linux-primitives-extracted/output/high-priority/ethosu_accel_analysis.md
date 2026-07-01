# ethosu_accel.h

**Source:** `ethosu_accel.h`


## Includes

- `drm.h`

## Defines (1 total)


### ETHOSU_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETHOSU_MAX_REGIONS` | `8` |  |

## Structs (8)


### `struct drm_ethosu_npu_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `config` | `-` |
| `__u32` | `sram_size` | `-` |

### `struct drm_ethosu_dev_query`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `size` | `-` |
| `__u64` | `pointer` | `-` |

### `struct drm_ethosu_bo_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_ethosu_bo_mmap_offset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_ethosu_bo_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__s64` | `timeout_ns` | `-` |

### `struct drm_ethosu_cmdstream_bo_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `data` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_ethosu_job`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd_bo` | `-` |
| `__u32` | `sram_size` | `-` |
| `__u32` | `region_bo_handles` | `ETHOSU_MAX_REGIONS` |

### `struct drm_ethosu_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `jobs` | `-` |
| `__u32` | `job_count` | `-` |
| `__u32` | `pad` | `-` |