# nitro_enclaves.h

**Source:** `nitro_enclaves.h`


## Includes

- `linux/types.h`

## Defines (32 total)


### NE_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_ADD_VCPU` | `_IOWR(0xAE, 0x21, __u32)` |  |

### NE_CREATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_CREATE_VM` | `_IOR(0xAE, 0x20, __u64)` |  |

### NE_DEFAULT (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_DEFAULT_MEMORY_REGION` | `(0x00)` |  |

### NE_EIF (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_EIF_IMAGE` | `(0x01)` |  |

### NE_ENCLAVE (3)

| Name | Value | Comment |
|------|-------|---------|
| `NE_ENCLAVE_PRODUCTION_MODE` | `(0x00)` |  |
| `NE_ENCLAVE_DEBUG_MODE` | `(0x01)` |  |
| `NE_ENCLAVE_START_MAX_FLAG_VAL` | `(0x02)` |  |

### NE_ERR (20)

| Name | Value | Comment |
|------|-------|---------|
| `NE_ERR_VCPU_ALREADY_USED` | `(256)` |  |
| `NE_ERR_VCPU_NOT_IN_CPU_POOL` | `(257)` |  |
| `NE_ERR_VCPU_INVALID_CPU_CORE` | `(258)` |  |
| `NE_ERR_INVALID_MEM_REGION_SIZE` | `(259)` |  |
| `NE_ERR_INVALID_MEM_REGION_ADDR` | `(260)` |  |
| `NE_ERR_UNALIGNED_MEM_REGION_ADDR` | `(261)` |  |
| `NE_ERR_MEM_REGION_ALREADY_USED` | `(262)` |  |
| `NE_ERR_MEM_NOT_HUGE_PAGE` | `(263)` |  |
| `NE_ERR_MEM_DIFFERENT_NUMA_NODE` | `(264)` |  |
| `NE_ERR_MEM_MAX_REGIONS` | `(265)` |  |
| `NE_ERR_NO_MEM_REGIONS_ADDED` | `(266)` |  |
| `NE_ERR_NO_VCPUS_ADDED` | `(267)` |  |
| `NE_ERR_ENCLAVE_MEM_MIN_SIZE` | `(268)` |  |
| `NE_ERR_FULL_CORES_NOT_USED` | `(269)` |  |
| `NE_ERR_NOT_IN_INIT_STATE` | `(270)` |  |
| `NE_ERR_INVALID_VCPU` | `(271)` |  |
| `NE_ERR_NO_CPUS_AVAIL_IN_POOL` | `(272)` |  |
| `NE_ERR_INVALID_PAGE_SIZE` | `(273)` |  |
| `NE_ERR_INVALID_FLAG_VALUE` | `(274)` |  |
| `NE_ERR_INVALID_ENCLAVE_CID` | `(275)` |  |

### NE_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_GET_IMAGE_LOAD_INFO` | `_IOWR(0xAE, 0x22, struct ne_image_load_info)` |  |

### NE_IMAGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_IMAGE_LOAD_MAX_FLAG_VAL` | `(0x02)` |  |

### NE_MEMORY (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_MEMORY_REGION_MAX_FLAG_VAL` | `(0x01)` |  |

### NE_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_SET_USER_MEMORY_REGION` | `_IOW(0xAE, 0x23, struct ne_user_memory_region)` |  |

### NE_START (1)

| Name | Value | Comment |
|------|-------|---------|
| `NE_START_ENCLAVE` | `_IOWR(0xAE, 0x24, struct ne_enclave_start_info)` |  |

## Structs (3)


### `struct ne_image_load_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `memory_offset` | `-` |

### `struct ne_user_memory_region`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `memory_size` | `-` |
| `__u64` | `userspace_addr` | `-` |

### `struct ne_enclave_start_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `enclave_cid` | `-` |