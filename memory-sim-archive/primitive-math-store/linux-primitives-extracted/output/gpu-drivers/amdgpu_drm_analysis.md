# amdgpu_drm.h

**Source:** `amdgpu_drm.h`


## Includes

- `drm.h`

## Defines (357 total)


### AMDGPU_BO (3)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_BO_LIST_OP_CREATE` | `0` |  |
| `AMDGPU_BO_LIST_OP_DESTROY` | `1` |  |
| `AMDGPU_BO_LIST_OP_UPDATE` | `2` |  |

### AMDGPU_CHUNK (10)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_CHUNK_ID_IB` | `0x01` |  |
| `AMDGPU_CHUNK_ID_FENCE` | `0x02` |  |
| `AMDGPU_CHUNK_ID_DEPENDENCIES` | `0x03` |  |
| `AMDGPU_CHUNK_ID_SYNCOBJ_IN` | `0x04` |  |
| `AMDGPU_CHUNK_ID_SYNCOBJ_OUT` | `0x05` |  |
| `AMDGPU_CHUNK_ID_BO_HANDLES` | `0x06` |  |
| `AMDGPU_CHUNK_ID_SCHEDULED_DEPENDENCIES` | `0x07` |  |
| `AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT` | `0x08` |  |
| `AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL` | `0x09` |  |
| `AMDGPU_CHUNK_ID_CP_GFX_SHADOW` | `0x0a` |  |

### AMDGPU_CS (1)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_CS_CHUNK_CP_GFX_SHADOW_FLAGS_INIT_SHADOW` | `0x1` |  |

### AMDGPU_CTX (28)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_CTX_OP_ALLOC_CTX` | `1` |  |
| `AMDGPU_CTX_OP_FREE_CTX` | `2` |  |
| `AMDGPU_CTX_OP_QUERY_STATE` | `3` |  |
| `AMDGPU_CTX_OP_QUERY_STATE2` | `4` |  |
| `AMDGPU_CTX_OP_GET_STABLE_PSTATE` | `5` |  |
| `AMDGPU_CTX_OP_SET_STABLE_PSTATE` | `6` |  |
| `AMDGPU_CTX_NO_RESET` | `0` |  |
| `AMDGPU_CTX_GUILTY_RESET` | `1` |  |
| `AMDGPU_CTX_INNOCENT_RESET` | `2` |  |
| `AMDGPU_CTX_UNKNOWN_RESET` | `3` |  |
| `AMDGPU_CTX_QUERY2_FLAGS_RESET` | `(1<<0)` |  |
| `AMDGPU_CTX_QUERY2_FLAGS_VRAMLOST` | `(1<<1)` |  |
| `AMDGPU_CTX_QUERY2_FLAGS_GUILTY` | `(1<<2)` |  |
| `AMDGPU_CTX_QUERY2_FLAGS_RAS_CE` | `(1<<3)` |  |
| `AMDGPU_CTX_QUERY2_FLAGS_RAS_UE` | `(1<<4)` |  |
| `AMDGPU_CTX_QUERY2_FLAGS_RESET_IN_PROGRESS` | `(1<<5)` |  |
| `AMDGPU_CTX_PRIORITY_UNSET` | `-2048` |  |
| `AMDGPU_CTX_PRIORITY_VERY_LOW` | `-1023` |  |
| `AMDGPU_CTX_PRIORITY_LOW` | `-512` |  |
| `AMDGPU_CTX_PRIORITY_NORMAL` | `0` |  |
| `AMDGPU_CTX_PRIORITY_HIGH` | `512` |  |
| `AMDGPU_CTX_PRIORITY_VERY_HIGH` | `1023` |  |
| `AMDGPU_CTX_STABLE_PSTATE_FLAGS_MASK` | `0xf` |  |
| `AMDGPU_CTX_STABLE_PSTATE_NONE` | `0` |  |
| `AMDGPU_CTX_STABLE_PSTATE_STANDARD` | `1` |  |
| `AMDGPU_CTX_STABLE_PSTATE_MIN_SCLK` | `2` |  |
| `AMDGPU_CTX_STABLE_PSTATE_MIN_MCLK` | `3` |  |
| `AMDGPU_CTX_STABLE_PSTATE_PEAK` | `4` |  |

### AMDGPU_FAMILY (18)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_FAMILY_UNKNOWN` | `0` |  |
| `AMDGPU_FAMILY_SI` | `110` | Hainan, Oland, Verde, Pitcairn, Tahiti |
| `AMDGPU_FAMILY_CI` | `120` | Bonaire, Hawaii |
| `AMDGPU_FAMILY_KV` | `125` | Kaveri, Kabini, Mullins |
| `AMDGPU_FAMILY_VI` | `130` | Iceland, Tonga |
| `AMDGPU_FAMILY_CZ` | `135` | Carrizo, Stoney |
| `AMDGPU_FAMILY_AI` | `141` | Vega10 |
| `AMDGPU_FAMILY_RV` | `142` | Raven |
| `AMDGPU_FAMILY_NV` | `143` | Navi10 |
| `AMDGPU_FAMILY_VGH` | `144` | Van Gogh |
| `AMDGPU_FAMILY_GC_11_0_0` | `145` | GC 11.0.0 |
| `AMDGPU_FAMILY_YC` | `146` | Yellow Carp |
| `AMDGPU_FAMILY_GC_11_0_1` | `148` | GC 11.0.1 |
| `AMDGPU_FAMILY_GC_10_3_6` | `149` | GC 10.3.6 |
| `AMDGPU_FAMILY_GC_10_3_7` | `151` | GC 10.3.7 |
| `AMDGPU_FAMILY_GC_11_5_0` | `150` | GC 11.5.0 |
| `AMDGPU_FAMILY_GC_11_5_4` | `154` | GC 11.5.4 |
| `AMDGPU_FAMILY_GC_12_0_0` | `152` | GC 12.0.0 |

### AMDGPU_FENCE (3)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_FENCE_TO_HANDLE_GET_SYNCOBJ` | `0` |  |
| `AMDGPU_FENCE_TO_HANDLE_GET_SYNCOBJ_FD` | `1` |  |
| `AMDGPU_FENCE_TO_HANDLE_GET_SYNC_FILE_FD` | `2` |  |

### AMDGPU_GEM (34)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_GEM_DOMAIN_CPU` | `0x1` |  |
| `AMDGPU_GEM_DOMAIN_GTT` | `0x2` |  |
| `AMDGPU_GEM_DOMAIN_VRAM` | `0x4` |  |
| `AMDGPU_GEM_DOMAIN_GDS` | `0x8` |  |
| `AMDGPU_GEM_DOMAIN_GWS` | `0x10` |  |
| `AMDGPU_GEM_DOMAIN_OA` | `0x20` |  |
| `AMDGPU_GEM_DOMAIN_DOORBELL` | `0x40` |  |
| `AMDGPU_GEM_DOMAIN_MASK` | `(AMDGPU_GEM_DOMAIN_CPU \| ` |  |
| `AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED` | `(1 << 0)` |  |
| `AMDGPU_GEM_CREATE_NO_CPU_ACCESS` | `(1 << 1)` |  |
| `AMDGPU_GEM_CREATE_CPU_GTT_USWC` | `(1 << 2)` |  |
| `AMDGPU_GEM_CREATE_VRAM_CLEARED` | `(1 << 3)` |  |
| `AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS` | `(1 << 5)` |  |
| `AMDGPU_GEM_CREATE_VM_ALWAYS_VALID` | `(1 << 6)` |  |
| `AMDGPU_GEM_CREATE_EXPLICIT_SYNC` | `(1 << 7)` |  |
| `AMDGPU_GEM_CREATE_CP_MQD_GFX9` | `(1 << 8)` |  |
| `AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE` | `(1 << 9)` |  |
| `AMDGPU_GEM_CREATE_ENCRYPTED` | `(1 << 10)` |  |
| `AMDGPU_GEM_CREATE_PREEMPTIBLE` | `(1 << 11)` |  |
| `AMDGPU_GEM_CREATE_DISCARDABLE` | `(1 << 12)` |  |
| `AMDGPU_GEM_CREATE_COHERENT` | `(1 << 13)` |  |
| `AMDGPU_GEM_CREATE_UNCACHED` | `(1 << 14)` |  |
| `AMDGPU_GEM_CREATE_EXT_COHERENT` | `(1 << 15)` |  |
| `AMDGPU_GEM_CREATE_GFX12_DCC` | `(1 << 16)` |  |
| `AMDGPU_GEM_USERPTR_READONLY` | `(1 << 0)` |  |
| `AMDGPU_GEM_USERPTR_ANONONLY` | `(1 << 1)` |  |
| `AMDGPU_GEM_USERPTR_VALIDATE` | `(1 << 2)` |  |
| `AMDGPU_GEM_USERPTR_REGISTER` | `(1 << 3)` |  |
| `AMDGPU_GEM_METADATA_OP_SET_METADATA` | `1` |  |
| `AMDGPU_GEM_METADATA_OP_GET_METADATA` | `2` |  |
| `AMDGPU_GEM_OP_GET_GEM_CREATE_INFO` | `0` |  |
| `AMDGPU_GEM_OP_SET_PLACEMENT` | `1` |  |
| `AMDGPU_GEM_OP_GET_MAPPING_INFO` | `2` |  |
| `AMDGPU_GEM_LIST_HANDLES_FLAG_IS_IMPORT` | `(1 << 0)` |  |

### AMDGPU_HW (12)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_HW_IP_GFX` | `0` |  |
| `AMDGPU_HW_IP_COMPUTE` | `1` |  |
| `AMDGPU_HW_IP_DMA` | `2` |  |
| `AMDGPU_HW_IP_UVD` | `3` |  |
| `AMDGPU_HW_IP_VCE` | `4` |  |
| `AMDGPU_HW_IP_UVD_ENC` | `5` |  |
| `AMDGPU_HW_IP_VCN_DEC` | `6` |  |
| `AMDGPU_HW_IP_VCN_ENC` | `7` |  |
| `AMDGPU_HW_IP_VCN_JPEG` | `8` |  |
| `AMDGPU_HW_IP_VPE` | `9` |  |
| `AMDGPU_HW_IP_NUM` | `10` |  |
| `AMDGPU_HW_IP_INSTANCE_MAX_COUNT` | `1` |  |

### AMDGPU_IB (7)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_IB_FLAG_CE` | `(1<<0)` |  |
| `AMDGPU_IB_FLAG_PREAMBLE` | `(1<<1)` |  |
| `AMDGPU_IB_FLAG_PREEMPT` | `(1<<2)` |  |
| `AMDGPU_IB_FLAG_TC_WB_NOT_INVALIDATE` | `(1 << 3)` |  |
| `AMDGPU_IB_FLAG_RESET_GDS_MAX_WAVE_ID` | `(1 << 4)` |  |
| `AMDGPU_IB_FLAGS_SECURE` | `(1 << 5)` |  |
| `AMDGPU_IB_FLAG_EMIT_MEM_SYNC` | `(1 << 6)` |  |

### AMDGPU_IDS (10)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_IDS_FLAGS_FUSION` | `0x01` |  |
| `AMDGPU_IDS_FLAGS_PREEMPTION` | `0x02` |  |
| `AMDGPU_IDS_FLAGS_TMZ` | `0x04` |  |
| `AMDGPU_IDS_FLAGS_CONFORMANT_TRUNC_COORD` | `0x08` |  |
| `AMDGPU_IDS_FLAGS_GANG_SUBMIT` | `0x10` |  |
| `AMDGPU_IDS_FLAGS_MODE_MASK` | `0x300` |  |
| `AMDGPU_IDS_FLAGS_MODE_SHIFT` | `0x8` |  |
| `AMDGPU_IDS_FLAGS_MODE_PF` | `0x0` |  |
| `AMDGPU_IDS_FLAGS_MODE_VF` | `0x1` |  |
| `AMDGPU_IDS_FLAGS_MODE_PT` | `0x2` |  |

### AMDGPU_INFO (98)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_INFO_ACCEL_WORKING` | `0x00` |  |
| `AMDGPU_INFO_CRTC_FROM_ID` | `0x01` |  |
| `AMDGPU_INFO_HW_IP_INFO` | `0x02` |  |
| `AMDGPU_INFO_HW_IP_COUNT` | `0x03` |  |
| `AMDGPU_INFO_TIMESTAMP` | `0x05` |  |
| `AMDGPU_INFO_FW_VERSION` | `0x0e` |  |
| `AMDGPU_INFO_FW_VCE` | `0x1` |  |
| `AMDGPU_INFO_FW_UVD` | `0x2` |  |
| `AMDGPU_INFO_FW_GMC` | `0x03` |  |
| `AMDGPU_INFO_FW_GFX_ME` | `0x04` |  |
| `AMDGPU_INFO_FW_GFX_PFP` | `0x05` |  |
| `AMDGPU_INFO_FW_GFX_CE` | `0x06` |  |
| `AMDGPU_INFO_FW_GFX_RLC` | `0x07` |  |
| `AMDGPU_INFO_FW_GFX_MEC` | `0x08` |  |
| `AMDGPU_INFO_FW_SMC` | `0x0a` |  |
| `AMDGPU_INFO_FW_SDMA` | `0x0b` |  |
| `AMDGPU_INFO_FW_SOS` | `0x0c` |  |
| `AMDGPU_INFO_FW_ASD` | `0x0d` |  |
| `AMDGPU_INFO_FW_VCN` | `0x0e` |  |
| `AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL` | `0x0f` |  |
| `AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM` | `0x10` |  |
| `AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM` | `0x11` |  |
| `AMDGPU_INFO_FW_DMCU` | `0x12` |  |
| `AMDGPU_INFO_FW_TA` | `0x13` |  |
| `AMDGPU_INFO_FW_DMCUB` | `0x14` |  |
| `AMDGPU_INFO_FW_TOC` | `0x15` |  |
| `AMDGPU_INFO_FW_CAP` | `0x16` |  |
| `AMDGPU_INFO_FW_GFX_RLCP` | `0x17` |  |
| `AMDGPU_INFO_FW_GFX_RLCV` | `0x18` |  |
| `AMDGPU_INFO_FW_MES_KIQ` | `0x19` |  |
| `AMDGPU_INFO_FW_MES` | `0x1a` |  |
| `AMDGPU_INFO_FW_IMU` | `0x1b` |  |
| `AMDGPU_INFO_FW_VPE` | `0x1c` |  |
| `AMDGPU_INFO_NUM_BYTES_MOVED` | `0x0f` |  |
| `AMDGPU_INFO_VRAM_USAGE` | `0x10` |  |
| `AMDGPU_INFO_GTT_USAGE` | `0x11` |  |
| `AMDGPU_INFO_GDS_CONFIG` | `0x13` |  |
| `AMDGPU_INFO_VRAM_GTT` | `0x14` |  |
| `AMDGPU_INFO_READ_MMR_REG` | `0x15` |  |
| `AMDGPU_INFO_DEV_INFO` | `0x16` |  |
| `AMDGPU_INFO_VIS_VRAM_USAGE` | `0x17` |  |
| `AMDGPU_INFO_NUM_EVICTIONS` | `0x18` |  |
| `AMDGPU_INFO_MEMORY` | `0x19` |  |
| `AMDGPU_INFO_VCE_CLOCK_TABLE` | `0x1A` |  |
| `AMDGPU_INFO_VBIOS` | `0x1B` |  |
| `AMDGPU_INFO_VBIOS_SIZE` | `0x1` |  |
| `AMDGPU_INFO_VBIOS_IMAGE` | `0x2` |  |
| `AMDGPU_INFO_VBIOS_INFO` | `0x3` |  |
| `AMDGPU_INFO_NUM_HANDLES` | `0x1C` |  |
| `AMDGPU_INFO_SENSOR` | `0x1D` |  |

*...and 48 more*

### AMDGPU_SCHED (2)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_SCHED_OP_PROCESS_PRIORITY_OVERRIDE` | `1` |  |
| `AMDGPU_SCHED_OP_CONTEXT_PRIORITY_OVERRIDE` | `2` |  |

### AMDGPU_TILING (40)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_TILING_ARRAY_MODE_SHIFT` | `0` |  |
| `AMDGPU_TILING_ARRAY_MODE_MASK` | `0xf` |  |
| `AMDGPU_TILING_PIPE_CONFIG_SHIFT` | `4` |  |
| `AMDGPU_TILING_PIPE_CONFIG_MASK` | `0x1f` |  |
| `AMDGPU_TILING_TILE_SPLIT_SHIFT` | `9` |  |
| `AMDGPU_TILING_TILE_SPLIT_MASK` | `0x7` |  |
| `AMDGPU_TILING_MICRO_TILE_MODE_SHIFT` | `12` |  |
| `AMDGPU_TILING_MICRO_TILE_MODE_MASK` | `0x7` |  |
| `AMDGPU_TILING_BANK_WIDTH_SHIFT` | `15` |  |
| `AMDGPU_TILING_BANK_WIDTH_MASK` | `0x3` |  |
| `AMDGPU_TILING_BANK_HEIGHT_SHIFT` | `17` |  |
| `AMDGPU_TILING_BANK_HEIGHT_MASK` | `0x3` |  |
| `AMDGPU_TILING_MACRO_TILE_ASPECT_SHIFT` | `19` |  |
| `AMDGPU_TILING_MACRO_TILE_ASPECT_MASK` | `0x3` |  |
| `AMDGPU_TILING_NUM_BANKS_SHIFT` | `21` |  |
| `AMDGPU_TILING_NUM_BANKS_MASK` | `0x3` |  |
| `AMDGPU_TILING_SWIZZLE_MODE_SHIFT` | `0` |  |
| `AMDGPU_TILING_SWIZZLE_MODE_MASK` | `0x1f` |  |
| `AMDGPU_TILING_DCC_OFFSET_256B_SHIFT` | `5` |  |
| `AMDGPU_TILING_DCC_OFFSET_256B_MASK` | `0xFFFFFF` |  |
| `AMDGPU_TILING_DCC_PITCH_MAX_SHIFT` | `29` |  |
| `AMDGPU_TILING_DCC_PITCH_MAX_MASK` | `0x3FFF` |  |
| `AMDGPU_TILING_DCC_INDEPENDENT_64B_SHIFT` | `43` |  |
| `AMDGPU_TILING_DCC_INDEPENDENT_64B_MASK` | `0x1` |  |
| `AMDGPU_TILING_DCC_INDEPENDENT_128B_SHIFT` | `44` |  |
| `AMDGPU_TILING_DCC_INDEPENDENT_128B_MASK` | `0x1` |  |
| `AMDGPU_TILING_SCANOUT_SHIFT` | `63` |  |
| `AMDGPU_TILING_SCANOUT_MASK` | `0x1` |  |
| `AMDGPU_TILING_GFX12_SWIZZLE_MODE_SHIFT` | `0` |  |
| `AMDGPU_TILING_GFX12_SWIZZLE_MODE_MASK` | `0x7` |  |
| `AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_SHIFT` | `3` |  |
| `AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_MASK` | `0x3` | 0:64B, 1:128B, 2:256B |
| `AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_SHIFT` | `5` |  |
| `AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_MASK` | `0x7` | CB_COLOR0_INFO.NUMBER_TYPE |
| `AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_SHIFT` | `8` |  |
| `AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_MASK` | `0x3f` | [0:4]:CB_COLOR0_INFO.FORMAT, [5]:MM |
| `AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_SHIFT` | `14` |  |
| `AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_MASK` | `0x1` |  |
| `AMDGPU_TILING_GFX12_SCANOUT_SHIFT` | `63` |  |
| `AMDGPU_TILING_GFX12_SCANOUT_MASK` | `0x1` |  |

### AMDGPU_USERQ (9)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_USERQ_OP_CREATE` | `1` |  |
| `AMDGPU_USERQ_OP_FREE` | `2` |  |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_MASK` | `0x3` |  |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_SHIFT` | `0` |  |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_LOW` | `0` |  |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_LOW` | `1` |  |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_HIGH` | `2` |  |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_HIGH` | `3` | admin only |
| `AMDGPU_USERQ_CREATE_FLAGS_QUEUE_SECURE` | `(1 << 2)` |  |

### AMDGPU_VA (4)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_VA_OP_MAP` | `1` |  |
| `AMDGPU_VA_OP_UNMAP` | `2` |  |
| `AMDGPU_VA_OP_CLEAR` | `3` |  |
| `AMDGPU_VA_OP_REPLACE` | `4` |  |

### AMDGPU_VCE (1)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_VCE_CLOCK_TABLE_ENTRIES` | `6` |  |

### AMDGPU_VM (15)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_VM_OP_RESERVE_VMID` | `1` |  |
| `AMDGPU_VM_OP_UNRESERVE_VMID` | `2` |  |
| `AMDGPU_VM_DELAY_UPDATE` | `(1 << 0)` |  |
| `AMDGPU_VM_PAGE_READABLE` | `(1 << 1)` |  |
| `AMDGPU_VM_PAGE_WRITEABLE` | `(1 << 2)` |  |
| `AMDGPU_VM_PAGE_EXECUTABLE` | `(1 << 3)` |  |
| `AMDGPU_VM_PAGE_PRT` | `(1 << 4)` |  |
| `AMDGPU_VM_MTYPE_MASK` | `(0xf << 5)` |  |
| `AMDGPU_VM_MTYPE_DEFAULT` | `(0 << 5)` |  |
| `AMDGPU_VM_MTYPE_NC` | `(1 << 5)` |  |
| `AMDGPU_VM_MTYPE_WC` | `(2 << 5)` |  |
| `AMDGPU_VM_MTYPE_CC` | `(3 << 5)` |  |
| `AMDGPU_VM_MTYPE_UC` | `(4 << 5)` |  |
| `AMDGPU_VM_MTYPE_RW` | `(5 << 5)` |  |
| `AMDGPU_VM_PAGE_NOALLOC` | `(1 << 9)` |  |

### AMDGPU_VMHUB (7)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_VMHUB_TYPE_MASK` | `0xff` |  |
| `AMDGPU_VMHUB_TYPE_SHIFT` | `0` |  |
| `AMDGPU_VMHUB_TYPE_GFX` | `0` |  |
| `AMDGPU_VMHUB_TYPE_MM0` | `1` |  |
| `AMDGPU_VMHUB_TYPE_MM1` | `2` |  |
| `AMDGPU_VMHUB_IDX_MASK` | `0xff00` |  |
| `AMDGPU_VMHUB_IDX_SHIFT` | `8` |  |

### AMDGPU_VRAM (15)

| Name | Value | Comment |
|------|-------|---------|
| `AMDGPU_VRAM_TYPE_UNKNOWN` | `0` |  |
| `AMDGPU_VRAM_TYPE_GDDR1` | `1` |  |
| `AMDGPU_VRAM_TYPE_DDR2` | `2` |  |
| `AMDGPU_VRAM_TYPE_GDDR3` | `3` |  |
| `AMDGPU_VRAM_TYPE_GDDR4` | `4` |  |
| `AMDGPU_VRAM_TYPE_GDDR5` | `5` |  |
| `AMDGPU_VRAM_TYPE_HBM` | `6` |  |
| `AMDGPU_VRAM_TYPE_DDR3` | `7` |  |
| `AMDGPU_VRAM_TYPE_DDR4` | `8` |  |
| `AMDGPU_VRAM_TYPE_GDDR6` | `9` |  |
| `AMDGPU_VRAM_TYPE_DDR5` | `10` |  |
| `AMDGPU_VRAM_TYPE_LPDDR4` | `11` |  |
| `AMDGPU_VRAM_TYPE_LPDDR5` | `12` |  |
| `AMDGPU_VRAM_TYPE_HBM3E` | `13` |  |
| `AMDGPU_VRAM_TYPE_HBM4` | `14` |  |

### DRM_AMDGPU (20)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_AMDGPU_GEM_CREATE` | `0x00` |  |
| `DRM_AMDGPU_GEM_MMAP` | `0x01` |  |
| `DRM_AMDGPU_CTX` | `0x02` |  |
| `DRM_AMDGPU_BO_LIST` | `0x03` |  |
| `DRM_AMDGPU_CS` | `0x04` |  |
| `DRM_AMDGPU_INFO` | `0x05` |  |
| `DRM_AMDGPU_GEM_METADATA` | `0x06` |  |
| `DRM_AMDGPU_GEM_WAIT_IDLE` | `0x07` |  |
| `DRM_AMDGPU_GEM_VA` | `0x08` |  |
| `DRM_AMDGPU_WAIT_CS` | `0x09` |  |
| `DRM_AMDGPU_GEM_OP` | `0x10` |  |
| `DRM_AMDGPU_GEM_USERPTR` | `0x11` |  |
| `DRM_AMDGPU_WAIT_FENCES` | `0x12` |  |
| `DRM_AMDGPU_VM` | `0x13` |  |
| `DRM_AMDGPU_FENCE_TO_HANDLE` | `0x14` |  |
| `DRM_AMDGPU_SCHED` | `0x15` |  |
| `DRM_AMDGPU_USERQ` | `0x16` |  |
| `DRM_AMDGPU_USERQ_SIGNAL` | `0x17` |  |
| `DRM_AMDGPU_USERQ_WAIT` | `0x18` |  |
| `DRM_AMDGPU_GEM_LIST_HANDLES` | `0x19` |  |

### DRM_IOCTL (20)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_AMDGPU_GEM_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_CREATE, union drm` |  |
| `DRM_IOCTL_AMDGPU_GEM_MMAP` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_MMAP, union drm_a` |  |
| `DRM_IOCTL_AMDGPU_CTX` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_CTX, union drm_amdgpu` |  |
| `DRM_IOCTL_AMDGPU_BO_LIST` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_BO_LIST, union drm_am` |  |
| `DRM_IOCTL_AMDGPU_CS` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_CS, union drm_amdgpu_` |  |
| `DRM_IOCTL_AMDGPU_INFO` | `DRM_IOW(DRM_COMMAND_BASE + DRM_AMDGPU_INFO, struct drm_amdgp` |  |
| `DRM_IOCTL_AMDGPU_GEM_METADATA` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_METADATA, struct ` |  |
| `DRM_IOCTL_AMDGPU_GEM_WAIT_IDLE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_WAIT_IDLE, union ` |  |
| `DRM_IOCTL_AMDGPU_GEM_VA` | `DRM_IOW(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_VA, struct drm_amd` |  |
| `DRM_IOCTL_AMDGPU_WAIT_CS` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_WAIT_CS, union drm_am` |  |
| `DRM_IOCTL_AMDGPU_GEM_OP` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_OP, struct drm_am` |  |
| `DRM_IOCTL_AMDGPU_GEM_USERPTR` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_USERPTR, struct d` |  |
| `DRM_IOCTL_AMDGPU_WAIT_FENCES` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_WAIT_FENCES, union dr` |  |
| `DRM_IOCTL_AMDGPU_VM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_VM, union drm_amdgpu_` |  |
| `DRM_IOCTL_AMDGPU_FENCE_TO_HANDLE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_FENCE_TO_HANDLE, unio` |  |
| `DRM_IOCTL_AMDGPU_SCHED` | `DRM_IOW(DRM_COMMAND_BASE + DRM_AMDGPU_SCHED, union drm_amdgp` |  |
| `DRM_IOCTL_AMDGPU_USERQ` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ, union drm_amdg` |  |
| `DRM_IOCTL_AMDGPU_USERQ_SIGNAL` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ_SIGNAL, struct ` |  |
| `DRM_IOCTL_AMDGPU_USERQ_WAIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ_WAIT, struct dr` |  |
| `DRM_IOCTL_AMDGPU_GEM_LIST_HANDLES` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_LIST_HANDLES, str` |  |

## Structs (75)


### `struct drm_amdgpu_gem_create_in`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bo_size` | `-` |
| `__u64` | `alignment` | `-` |
| `__u64` | `domains` | `-` |
| `__u64` | `domain_flags` | `-` |

### `struct drm_amdgpu_gem_create_out`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_bo_list_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `operation` | `-` |
| `__u32` | `list_handle` | `-` |
| `__u32` | `bo_number` | `-` |
| `__u32` | `bo_info_size` | `-` |
| `__u64` | `bo_info_ptr` | `-` |

### `struct drm_amdgpu_bo_list_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bo_handle` | `-` |
| `__u32` | `bo_priority` | `-` |

### `struct drm_amdgpu_bo_list_out`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `list_handle` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_ctx_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `ctx_id` | `-` |
| `__s32` | `priority` | `-` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `_pad` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u32` | `hangs` | `-` |
| `__u32` | `reset_status` | `-` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_userq_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `queue_id` | `-` |
| `__u32` | `ip_type` | `-` |
| `__u32` | `doorbell_handle` | `-` |
| `__u32` | `doorbell_offset` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `queue_va` | `-` |
| `__u64` | `queue_size` | `-` |
| `__u64` | `rptr_va` | `-` |
| `__u64` | `wptr_va` | `-` |
| `__u64` | `mqd` | `-` |
| `__u64` | `mqd_size` | `-` |

### `struct drm_amdgpu_userq_out`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `queue_id` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_userq_mqd_gfx11`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `shadow_va` | `-` |
| `__u64` | `csa_va` | `-` |

### `struct drm_amdgpu_userq_mqd_sdma_gfx11`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `csa_va` | `-` |

### `struct drm_amdgpu_userq_mqd_compute_gfx11`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `eop_va` | `-` |

### `struct drm_amdgpu_userq_signal`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `queue_id` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `syncobj_handles` | `-` |
| `__u16` | `num_syncobj_handles` | `-` |
| `__u16` | `pad0` | `-` |
| `__u32` | `pad1` | `-` |
| `__u64` | `bo_read_handles` | `-` |
| `__u64` | `bo_write_handles` | `-` |
| `__u32` | `num_bo_read_handles` | `-` |
| `__u32` | `num_bo_write_handles` | `-` |

### `struct drm_amdgpu_userq_fence_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `va` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_amdgpu_userq_wait`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `waitq_id` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `syncobj_handles` | `-` |
| `__u64` | `syncobj_timeline_handles` | `-` |
| `__u64` | `syncobj_timeline_points` | `-` |
| `__u64` | `bo_read_handles` | `-` |
| `__u64` | `bo_write_handles` | `-` |
| `__u16` | `num_syncobj_timeline_handles` | `-` |
| `__u16` | `num_fences` | `-` |
| `__u16` | `num_syncobj_handles` | `-` |
| `__u16` | `pad0` | `-` |
| `__u32` | `num_bo_read_handles` | `-` |
| `__u32` | `num_bo_write_handles` | `-` |
| `__u64` | `out_fences` | `-` |

### `struct drm_amdgpu_vm_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_amdgpu_vm_out`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |

### `struct drm_amdgpu_sched_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `fd` | `-` |
| `__s32` | `priority` | `-` |
| `__u32` | `ctx_id` | `-` |

### `struct drm_amdgpu_gem_userptr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_amdgpu_gem_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `op` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `tiling_info` | `-` |
| `__u32` | `data_size_bytes` | `-` |
| `__u32` | `data` | `64` |

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `tiling_info` | `-` |
| `__u32` | `data_size_bytes` | `-` |
| `__u32` | `data` | `64` |

### `struct drm_amdgpu_gem_mmap_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_gem_mmap_out`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr_ptr` | `-` |

### `struct drm_amdgpu_gem_wait_idle_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `timeout` | `-` |

### `struct drm_amdgpu_gem_wait_idle_out`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `status` | `-` |
| `__u32` | `domain` | `-` |

### `struct drm_amdgpu_wait_cs_in`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `handle` | `-` |
| `__u64` | `timeout` | `-` |
| `__u32` | `ip_type` | `-` |
| `__u32` | `ip_instance` | `-` |
| `__u32` | `ring` | `-` |
| `__u32` | `ctx_id` | `-` |

### `struct drm_amdgpu_wait_cs_out`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `status` | `-` |

### `struct drm_amdgpu_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `ip_type` | `-` |
| `__u32` | `ip_instance` | `-` |
| `__u32` | `ring` | `-` |
| `__u64` | `seq_no` | `-` |

### `struct drm_amdgpu_wait_fences_in`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fences` | `-` |
| `__u32` | `fence_count` | `-` |
| `__u32` | `wait_all` | `-` |
| `__u64` | `timeout_ns` | `-` |

### `struct drm_amdgpu_wait_fences_out`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `status` | `-` |
| `__u32` | `first_signaled` | `-` |

### `struct drm_amdgpu_gem_vm_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `flags` | `-` |

### `struct drm_amdgpu_gem_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `op` | `-` |
| `__u64` | `value` | `-` |
| `__u32` | `num_entries` | `-` |
| `__u32` | `padding` | `-` |

### `struct drm_amdgpu_gem_list_handles`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `entries` | `-` |
| `__u32` | `num_entries` | `-` |
| `__u32` | `padding` | `-` |

### `struct drm_amdgpu_gem_list_handles_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gem_handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `preferred_domains` | `-` |
| `__u64` | `alloc_flags` | `-` |
| `__u64` | `alignment` | `-` |

### `struct drm_amdgpu_gem_va`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `_pad` | `-` |
| `__u32` | `operation` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `va_address` | `-` |
| `__u64` | `offset_in_bo` | `-` |
| `__u64` | `map_size` | `-` |
| `__u64` | `vm_timeline_point` | `-` |
| `__u32` | `vm_timeline_syncobj_out` | `-` |
| `__u32` | `num_syncobj_handles` | `-` |
| `__u64` | `input_fence_syncobj_handles` | `-` |

### `struct drm_amdgpu_cs_chunk`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `chunk_id` | `-` |
| `__u32` | `length_dw` | `-` |
| `__u64` | `chunk_data` | `-` |

### `struct drm_amdgpu_cs_in`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ctx_id` | `-` |
| `__u32` | `bo_list_handle` | `-` |
| `__u32` | `num_chunks` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `chunks` | `-` |

### `struct drm_amdgpu_cs_out`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `handle` | `-` |

### `struct drm_amdgpu_cs_chunk_ib`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `_pad` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `va_start` | `-` |
| `__u32` | `ib_bytes` | `-` |
| `__u32` | `ip_type` | `-` |
| `__u32` | `ip_instance` | `-` |
| `__u32` | `ring` | `-` |

### `struct drm_amdgpu_cs_chunk_dep`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ip_type` | `-` |
| `__u32` | `ip_instance` | `-` |
| `__u32` | `ring` | `-` |
| `__u32` | `ctx_id` | `-` |
| `__u64` | `handle` | `-` |

### `struct drm_amdgpu_cs_chunk_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `offset` | `-` |

### `struct drm_amdgpu_cs_chunk_sem`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |

### `struct drm_amdgpu_cs_chunk_syncobj`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `point` | `-` |

### `struct anonymous_45`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `what` | `-` |
| `__u32` | `pad` | `-` |

### `struct anonymous_46`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |

### `struct drm_amdgpu_cs_chunk_data`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_amdgpu_cs_chunk_cp_gfx_shadow`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `shadow_va` | `-` |
| `__u64` | `csa_va` | `-` |
| `__u64` | `gds_va` | `-` |
| `__u64` | `flags` | `-` |

### `struct drm_amdgpu_query_fw`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fw_type` | `-` |
| `__u32` | `ip_instance` | `-` |
| `__u32` | `index` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `return_pointer` | `-` |
| `__u32` | `return_size` | `-` |
| `__u32` | `query` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `_pad` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `ip_instance` | `-` |
| `__u32` | `dword_offset` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `instance` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `type` | `-` |

### `struct anonymous_51`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `_pad` | `-` |

### `struct anonymous_52`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `ip_instance` | `-` |

### `struct anonymous_53`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dword_offset` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `instance` | `-` |
| `__u32` | `flags` | `-` |

### `struct anonymous_54`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `offset` | `-` |

### `struct anonymous_55`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |

### `struct anonymous_56`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |

### `struct drm_amdgpu_info_gds`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gds_gfx_partition_size` | `-` |
| `__u32` | `compute_partition_size` | `-` |
| `__u32` | `gds_total_size` | `-` |
| `__u32` | `gws_per_gfx_partition` | `-` |
| `__u32` | `gws_per_compute_partition` | `-` |
| `__u32` | `oa_per_gfx_partition` | `-` |
| `__u32` | `oa_per_compute_partition` | `-` |
| `__u32` | `_pad` | `-` |

### `struct drm_amdgpu_info_vram_gtt`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `vram_size` | `-` |
| `__u64` | `vram_cpu_accessible_size` | `-` |
| `__u64` | `gtt_size` | `-` |

### `struct drm_amdgpu_heap_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `total_heap_size` | `-` |
| `__u64` | `usable_heap_size` | `-` |
| `__u64` | `heap_usage` | `-` |
| `__u64` | `max_allocation` | `-` |

### `struct drm_amdgpu_memory_info`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_amdgpu_info_firmware`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ver` | `-` |
| `__u32` | `feature` | `-` |

### `struct drm_amdgpu_info_vbios`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `name` | `64` |
| `__u8` | `vbios_pn` | `64` |
| `__u32` | `version` | `-` |
| `__u32` | `pad` | `-` |
| `__u8` | `vbios_ver_str` | `32` |
| `__u8` | `date` | `32` |

### `struct drm_amdgpu_info_device`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `device_id` | `-` |
| `__u32` | `chip_rev` | `-` |
| `__u32` | `external_rev` | `-` |
| `__u32` | `pci_rev` | `-` |
| `__u32` | `family` | `-` |
| `__u32` | `num_shader_engines` | `-` |
| `__u32` | `num_shader_arrays_per_engine` | `-` |
| `__u32` | `gpu_counter_freq` | `-` |
| `__u64` | `max_engine_clock` | `-` |
| `__u64` | `max_memory_clock` | `-` |
| `__u32` | `cu_active_number` | `-` |
| `__u32` | `cu_ao_mask` | `-` |
| `__u32` | `cu_bitmap` | `4][4` |
| `__u32` | `enabled_rb_pipes_mask` | `-` |
| `__u32` | `num_rb_pipes` | `-` |
| `__u32` | `num_hw_gfx_contexts` | `-` |
| `__u32` | `pcie_gen` | `-` |
| `__u64` | `ids_flags` | `-` |
| `__u64` | `virtual_address_offset` | `-` |
| `__u64` | `virtual_address_max` | `-` |
| `__u32` | `virtual_address_alignment` | `-` |
| `__u32` | `pte_fragment_size` | `-` |
| `__u32` | `gart_page_size` | `-` |
| `__u32` | `ce_ram_size` | `-` |
| `__u32` | `vram_type` | `-` |
| `__u32` | `vram_bit_width` | `-` |
| `__u32` | `vce_harvest_config` | `-` |
| `__u32` | `gc_double_offchip_lds_buf` | `-` |
| `__u64` | `prim_buf_gpu_addr` | `-` |
| `__u64` | `pos_buf_gpu_addr` | `-` |
| `__u64` | `cntl_sb_buf_gpu_addr` | `-` |
| `__u64` | `param_buf_gpu_addr` | `-` |
| `__u32` | `prim_buf_size` | `-` |
| `__u32` | `pos_buf_size` | `-` |
| `__u32` | `cntl_sb_buf_size` | `-` |
| `__u32` | `param_buf_size` | `-` |
| `__u32` | `wave_front_size` | `-` |
| `__u32` | `num_shader_visible_vgprs` | `-` |
| `__u32` | `num_cu_per_sh` | `-` |
| `__u32` | `num_tcc_blocks` | `-` |
| `__u32` | `gs_vgt_table_depth` | `-` |
| `__u32` | `gs_prim_buffer_depth` | `-` |
| `__u32` | `max_gs_waves_per_vgt` | `-` |
| `__u32` | `pcie_num_lanes` | `-` |
| `__u32` | `cu_ao_bitmap` | `4][4` |
| `__u64` | `high_va_offset` | `-` |
| `__u64` | `high_va_max` | `-` |
| `__u32` | `pa_sc_tile_steering_override` | `-` |
| `__u64` | `tcc_disabled_mask` | `-` |
| `__u64` | `min_engine_clock` | `-` |
| `__u64` | `min_memory_clock` | `-` |
| `__u32` | `tcp_cache_size` | `-` |
| `__u32` | `num_sqc_per_wgp` | `-` |
| `__u32` | `sqc_data_cache_size` | `-` |
| `__u32` | `sqc_inst_cache_size` | `-` |
| `__u32` | `gl1c_cache_size` | `-` |
| `__u32` | `gl2c_cache_size` | `-` |
| `__u64` | `mall_size` | `-` |
| `__u32` | `enabled_rb_pipes_mask_hi` | `-` |
| `__u32` | `shadow_size` | `-` |
| `__u32` | `shadow_alignment` | `-` |
| `__u32` | `csa_size` | `-` |
| `__u32` | `csa_alignment` | `-` |
| `__u32` | `userq_ip_mask` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_amdgpu_info_hw_ip`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `hw_ip_version_major` | `-` |
| `__u32` | `hw_ip_version_minor` | `-` |
| `__u64` | `capabilities_flags` | `-` |
| `__u32` | `ib_start_alignment` | `-` |
| `__u32` | `ib_size_alignment` | `-` |
| `__u32` | `available_rings` | `-` |
| `__u32` | `ip_discovery_version` | `-` |
| `__u32` | `userq_num_slots` | `-` |

### `struct drm_amdgpu_info_num_handles`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `uvd_max_handles` | `-` |
| `__u32` | `uvd_used_handles` | `-` |

### `struct drm_amdgpu_info_vce_clock_table_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `sclk` | `-` |
| `__u32` | `mclk` | `-` |
| `__u32` | `eclk` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_amdgpu_info_vce_clock_table`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_valid_entries` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_amdgpu_info_video_codec_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `valid` | `-` |
| `__u32` | `max_width` | `-` |
| `__u32` | `max_height` | `-` |
| `__u32` | `max_pixels_per_frame` | `-` |
| `__u32` | `max_level` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_amdgpu_info_video_caps`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_amdgpu_info_gpuvm_fault`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u32` | `status` | `-` |
| `__u32` | `vmhub` | `-` |

### `struct drm_amdgpu_info_uq_metadata_gfx`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `shadow_size` | `-` |
| `__u32` | `shadow_alignment` | `-` |
| `__u32` | `csa_size` | `-` |
| `__u32` | `csa_alignment` | `-` |

### `struct drm_amdgpu_info_uq_metadata_compute`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `eop_size` | `-` |
| `__u32` | `eop_alignment` | `-` |

### `struct drm_amdgpu_info_uq_metadata_sdma`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `csa_size` | `-` |
| `__u32` | `csa_alignment` | `-` |

### `struct drm_amdgpu_info_uq_metadata`

| Type | Field | Array |
|------|-------|-------|