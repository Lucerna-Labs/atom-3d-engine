# radeon_drm.h

**Source:** `radeon_drm.h`


## Includes

- `drm.h`

## Defines (395 total)


### CIK_TILE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CIK_TILE_MODE_DEPTH_STENCIL_1D` | `5` |  |

### DRM_IOCTL (42)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_RADEON_CP_INIT` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_CP_INIT, drm_radeon_i` |  |
| `DRM_IOCTL_RADEON_CP_START` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_CP_START)` |  |
| `DRM_IOCTL_RADEON_CP_STOP` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_CP_STOP, drm_radeon_c` |  |
| `DRM_IOCTL_RADEON_CP_RESET` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_CP_RESET)` |  |
| `DRM_IOCTL_RADEON_CP_IDLE` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_CP_IDLE)` |  |
| `DRM_IOCTL_RADEON_RESET` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_RESET)` |  |
| `DRM_IOCTL_RADEON_FULLSCREEN` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_FULLSCREEN, drm_radeo` |  |
| `DRM_IOCTL_RADEON_SWAP` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_SWAP)` |  |
| `DRM_IOCTL_RADEON_CLEAR` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_CLEAR, drm_radeon_cle` |  |
| `DRM_IOCTL_RADEON_VERTEX` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_VERTEX, drm_radeon_ve` |  |
| `DRM_IOCTL_RADEON_INDICES` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_INDICES, drm_radeon_i` |  |
| `DRM_IOCTL_RADEON_STIPPLE` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_STIPPLE, drm_radeon_s` |  |
| `DRM_IOCTL_RADEON_INDIRECT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_INDIRECT, drm_radeon_` |  |
| `DRM_IOCTL_RADEON_TEXTURE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_TEXTURE, drm_radeon_t` |  |
| `DRM_IOCTL_RADEON_VERTEX2` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_VERTEX2, drm_radeon_v` |  |
| `DRM_IOCTL_RADEON_CMDBUF` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_CMDBUF, drm_radeon_cm` |  |
| `DRM_IOCTL_RADEON_GETPARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GETPARAM, drm_radeon_` |  |
| `DRM_IOCTL_RADEON_FLIP` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_FLIP)` |  |
| `DRM_IOCTL_RADEON_ALLOC` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_ALLOC, drm_radeon_mem` |  |
| `DRM_IOCTL_RADEON_FREE` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_FREE, drm_radeon_mem_` |  |
| `DRM_IOCTL_RADEON_INIT_HEAP` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_INIT_HEAP, drm_radeon` |  |
| `DRM_IOCTL_RADEON_IRQ_EMIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_IRQ_EMIT, drm_radeon_` |  |
| `DRM_IOCTL_RADEON_IRQ_WAIT` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_IRQ_WAIT, drm_radeon_` |  |
| `DRM_IOCTL_RADEON_CP_RESUME` | `DRM_IO(  DRM_COMMAND_BASE + DRM_RADEON_CP_RESUME)` |  |
| `DRM_IOCTL_RADEON_SETPARAM` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_SETPARAM, drm_radeon_` |  |
| `DRM_IOCTL_RADEON_SURF_ALLOC` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_SURF_ALLOC, drm_radeo` |  |
| `DRM_IOCTL_RADEON_SURF_FREE` | `DRM_IOW( DRM_COMMAND_BASE + DRM_RADEON_SURF_FREE, drm_radeon` |  |
| `DRM_IOCTL_RADEON_GEM_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_INFO, struct drm_` |  |
| `DRM_IOCTL_RADEON_GEM_CREATE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_CREATE, struct dr` |  |
| `DRM_IOCTL_RADEON_GEM_MMAP` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_MMAP, struct drm_` |  |
| `DRM_IOCTL_RADEON_GEM_PREAD` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_PREAD, struct drm` |  |
| `DRM_IOCTL_RADEON_GEM_PWRITE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_PWRITE, struct dr` |  |
| `DRM_IOCTL_RADEON_GEM_SET_DOMAIN` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_SET_DOMAIN, struc` |  |
| `DRM_IOCTL_RADEON_GEM_WAIT_IDLE` | `DRM_IOW(DRM_COMMAND_BASE + DRM_RADEON_GEM_WAIT_IDLE, struct ` |  |
| `DRM_IOCTL_RADEON_CS` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_CS, struct drm_radeon` |  |
| `DRM_IOCTL_RADEON_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_INFO, struct drm_rade` |  |
| `DRM_IOCTL_RADEON_GEM_SET_TILING` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_SET_TILING, struc` |  |
| `DRM_IOCTL_RADEON_GEM_GET_TILING` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_GET_TILING, struc` |  |
| `DRM_IOCTL_RADEON_GEM_BUSY` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_BUSY, struct drm_` |  |
| `DRM_IOCTL_RADEON_GEM_VA` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_VA, struct drm_ra` |  |
| `DRM_IOCTL_RADEON_GEM_OP` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_OP, struct drm_ra` |  |
| `DRM_IOCTL_RADEON_GEM_USERPTR` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_RADEON_GEM_USERPTR, struct d` |  |

### DRM_RADEON (44)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_RADEON_CP_INIT` | `0x00` |  |
| `DRM_RADEON_CP_START` | `0x01` |  |
| `DRM_RADEON_CP_STOP` | `0x02` |  |
| `DRM_RADEON_CP_RESET` | `0x03` |  |
| `DRM_RADEON_CP_IDLE` | `0x04` |  |
| `DRM_RADEON_RESET` | `0x05` |  |
| `DRM_RADEON_FULLSCREEN` | `0x06` |  |
| `DRM_RADEON_SWAP` | `0x07` |  |
| `DRM_RADEON_CLEAR` | `0x08` |  |
| `DRM_RADEON_VERTEX` | `0x09` |  |
| `DRM_RADEON_INDICES` | `0x0A` |  |
| `DRM_RADEON_STIPPLE` | `0x0C` |  |
| `DRM_RADEON_INDIRECT` | `0x0D` |  |
| `DRM_RADEON_TEXTURE` | `0x0E` |  |
| `DRM_RADEON_VERTEX2` | `0x0F` |  |
| `DRM_RADEON_CMDBUF` | `0x10` |  |
| `DRM_RADEON_GETPARAM` | `0x11` |  |
| `DRM_RADEON_FLIP` | `0x12` |  |
| `DRM_RADEON_ALLOC` | `0x13` |  |
| `DRM_RADEON_FREE` | `0x14` |  |
| `DRM_RADEON_INIT_HEAP` | `0x15` |  |
| `DRM_RADEON_IRQ_EMIT` | `0x16` |  |
| `DRM_RADEON_IRQ_WAIT` | `0x17` |  |
| `DRM_RADEON_CP_RESUME` | `0x18` |  |
| `DRM_RADEON_SETPARAM` | `0x19` |  |
| `DRM_RADEON_SURF_ALLOC` | `0x1a` |  |
| `DRM_RADEON_SURF_FREE` | `0x1b` |  |
| `DRM_RADEON_GEM_INFO` | `0x1c` |  |
| `DRM_RADEON_GEM_CREATE` | `0x1d` |  |
| `DRM_RADEON_GEM_MMAP` | `0x1e` |  |
| `DRM_RADEON_GEM_PREAD` | `0x21` |  |
| `DRM_RADEON_GEM_PWRITE` | `0x22` |  |
| `DRM_RADEON_GEM_SET_DOMAIN` | `0x23` |  |
| `DRM_RADEON_GEM_WAIT_IDLE` | `0x24` |  |
| `DRM_RADEON_CS` | `0x26` |  |
| `DRM_RADEON_INFO` | `0x27` |  |
| `DRM_RADEON_GEM_SET_TILING` | `0x28` |  |
| `DRM_RADEON_GEM_GET_TILING` | `0x29` |  |
| `DRM_RADEON_GEM_BUSY` | `0x2a` |  |
| `DRM_RADEON_GEM_VA` | `0x2b` |  |
| `DRM_RADEON_GEM_OP` | `0x2c` |  |
| `DRM_RADEON_GEM_USERPTR` | `0x2d` |  |
| `DRM_RADEON_VBLANK_CRTC1` | `1` |  |
| `DRM_RADEON_VBLANK_CRTC2` | `2` |  |

### RADEON_BUFFER (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_BUFFER_SIZE` | `65536` |  |

### RADEON_CARD (3)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_CARD_PCI` | `0` |  |
| `RADEON_CARD_AGP` | `1` |  |
| `RADEON_CARD_PCIE` | `2` |  |

### RADEON_CHUNK (4)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_CHUNK_ID_RELOCS` | `0x01` |  |
| `RADEON_CHUNK_ID_IB` | `0x02` |  |
| `RADEON_CHUNK_ID_FLAGS` | `0x03` |  |
| `RADEON_CHUNK_ID_CONST_IB` | `0x04` |  |

### RADEON_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_CLEAR_FASTZ` | `0x80000000` |  |

### RADEON_CMD (9)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_CMD_PACKET` | `1` | emit one of the register packets above |
| `RADEON_CMD_SCALARS` | `2` | emit scalar data |
| `RADEON_CMD_VECTORS` | `3` | emit vector data |
| `RADEON_CMD_DMA_DISCARD` | `4` | discard current dma buf |
| `RADEON_CMD_PACKET3` | `5` | emit hw packet |
| `RADEON_CMD_PACKET3_CLIP` | `6` | emit hw packet wrapped in cliprects |
| `RADEON_CMD_SCALARS2` | `7` | r200 stopgap |
| `RADEON_CMD_WAIT` | `8	/* emit hw wait commands -- note:` |  |
| `RADEON_CMD_VECLINEAR` | `9` | another r200 stopgap |

### RADEON_CS (8)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_CS_KEEP_TILING_FLAGS` | `0x01` |  |
| `RADEON_CS_USE_VM` | `0x02` |  |
| `RADEON_CS_END_OF_FRAME` | `0x04` | a hint from userspace which CS is the last one |
| `RADEON_CS_RING_GFX` | `0` |  |
| `RADEON_CS_RING_COMPUTE` | `1` |  |
| `RADEON_CS_RING_DMA` | `2` |  |
| `RADEON_CS_RING_UVD` | `3` |  |
| `RADEON_CS_RING_VCE` | `4` |  |

### RADEON_EMIT (30)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_EMIT_PP_MISC` | `0` | context/7 |
| `RADEON_EMIT_PP_CNTL` | `1` | context/3 |
| `RADEON_EMIT_RB3D_COLORPITCH` | `2` | context/1 |
| `RADEON_EMIT_RE_LINE_PATTERN` | `3` | line/2 |
| `RADEON_EMIT_SE_LINE_WIDTH` | `4` | line/1 |
| `RADEON_EMIT_PP_LUM_MATRIX` | `5` | bumpmap/1 |
| `RADEON_EMIT_PP_ROT_MATRIX_0` | `6` | bumpmap/2 |
| `RADEON_EMIT_RB3D_STENCILREFMASK` | `7` | masks/3 |
| `RADEON_EMIT_SE_VPORT_XSCALE` | `8` | viewport/6 |
| `RADEON_EMIT_SE_CNTL` | `9` | setup/2 |
| `RADEON_EMIT_SE_CNTL_STATUS` | `10` | setup/1 |
| `RADEON_EMIT_RE_MISC` | `11` | misc/1 |
| `RADEON_EMIT_PP_TXFILTER_0` | `12` | tex0/6 |
| `RADEON_EMIT_PP_BORDER_COLOR_0` | `13` | tex0/1 |
| `RADEON_EMIT_PP_TXFILTER_1` | `14` | tex1/6 |
| `RADEON_EMIT_PP_BORDER_COLOR_1` | `15` | tex1/1 |
| `RADEON_EMIT_PP_TXFILTER_2` | `16` | tex2/6 |
| `RADEON_EMIT_PP_BORDER_COLOR_2` | `17` | tex2/1 |
| `RADEON_EMIT_SE_ZBIAS_FACTOR` | `18` | zbias/2 |
| `RADEON_EMIT_SE_TCL_OUTPUT_VTX_FMT` | `19` | tcl/11 |
| `RADEON_EMIT_SE_TCL_MATERIAL_EMMISSIVE_RED` | `20` | material/17 |
| `RADEON_EMIT_PP_TEX_SIZE_0` | `73` |  |
| `RADEON_EMIT_PP_TEX_SIZE_1` | `74` |  |
| `RADEON_EMIT_PP_TEX_SIZE_2` | `75` |  |
| `RADEON_EMIT_PP_CUBIC_FACES_0` | `78` |  |
| `RADEON_EMIT_PP_CUBIC_OFFSETS_T0` | `79` |  |
| `RADEON_EMIT_PP_CUBIC_FACES_1` | `80` |  |
| `RADEON_EMIT_PP_CUBIC_OFFSETS_T1` | `81` |  |
| `RADEON_EMIT_PP_CUBIC_FACES_2` | `82` |  |
| `RADEON_EMIT_PP_CUBIC_OFFSETS_T2` | `83` |  |

### RADEON_GART (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_GART_TEX_HEAP` | `1` |  |

### RADEON_GEM (14)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_GEM_DOMAIN_CPU` | `0x1` |  |
| `RADEON_GEM_DOMAIN_GTT` | `0x2` |  |
| `RADEON_GEM_DOMAIN_VRAM` | `0x4` |  |
| `RADEON_GEM_NO_BACKING_STORE` | `(1 << 0)` |  |
| `RADEON_GEM_GTT_UC` | `(1 << 1)` |  |
| `RADEON_GEM_GTT_WC` | `(1 << 2)` |  |
| `RADEON_GEM_CPU_ACCESS` | `(1 << 3)` |  |
| `RADEON_GEM_NO_CPU_ACCESS` | `(1 << 4)` |  |
| `RADEON_GEM_USERPTR_READONLY` | `(1 << 0)` |  |
| `RADEON_GEM_USERPTR_ANONONLY` | `(1 << 1)` |  |
| `RADEON_GEM_USERPTR_VALIDATE` | `(1 << 2)` |  |
| `RADEON_GEM_USERPTR_REGISTER` | `(1 << 3)` |  |
| `RADEON_GEM_OP_GET_INITIAL_DOMAIN` | `0` |  |
| `RADEON_GEM_OP_SET_INITIAL_DOMAIN` | `1` |  |

### RADEON_INDEX (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_INDEX_PRIM_OFFSET` | `20` |  |

### RADEON_INFO (39)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_INFO_DEVICE_ID` | `0x00` |  |
| `RADEON_INFO_NUM_GB_PIPES` | `0x01` |  |
| `RADEON_INFO_NUM_Z_PIPES` | `0x02` |  |
| `RADEON_INFO_ACCEL_WORKING` | `0x03` |  |
| `RADEON_INFO_CRTC_FROM_ID` | `0x04` |  |
| `RADEON_INFO_ACCEL_WORKING2` | `0x05` |  |
| `RADEON_INFO_TILING_CONFIG` | `0x06` |  |
| `RADEON_INFO_WANT_HYPERZ` | `0x07` |  |
| `RADEON_INFO_WANT_CMASK` | `0x08` | get access to CMASK on r300 |
| `RADEON_INFO_CLOCK_CRYSTAL_FREQ` | `0x09` | clock crystal frequency |
| `RADEON_INFO_NUM_BACKENDS` | `0x0a` | DB/backends for r600+ - need for OQ |
| `RADEON_INFO_NUM_TILE_PIPES` | `0x0b` | tile pipes for r600+ |
| `RADEON_INFO_FUSION_GART_WORKING` | `0x0c` | fusion writes to GTT were broken before this |
| `RADEON_INFO_BACKEND_MAP` | `0x0d` | pipe to backend map, needed by mesa |
| `RADEON_INFO_VA_START` | `0x0e` |  |
| `RADEON_INFO_IB_VM_MAX_SIZE` | `0x0f` |  |
| `RADEON_INFO_MAX_PIPES` | `0x10` |  |
| `RADEON_INFO_TIMESTAMP` | `0x11` |  |
| `RADEON_INFO_MAX_SE` | `0x12` |  |
| `RADEON_INFO_MAX_SH_PER_SE` | `0x13` |  |
| `RADEON_INFO_FASTFB_WORKING` | `0x14` |  |
| `RADEON_INFO_RING_WORKING` | `0x15` |  |
| `RADEON_INFO_SI_TILE_MODE_ARRAY` | `0x16` |  |
| `RADEON_INFO_SI_CP_DMA_COMPUTE` | `0x17` |  |
| `RADEON_INFO_CIK_MACROTILE_MODE_ARRAY` | `0x18` |  |
| `RADEON_INFO_SI_BACKEND_ENABLED_MASK` | `0x19` |  |
| `RADEON_INFO_MAX_SCLK` | `0x1a` |  |
| `RADEON_INFO_VCE_FW_VERSION` | `0x1b` |  |
| `RADEON_INFO_VCE_FB_VERSION` | `0x1c` |  |
| `RADEON_INFO_NUM_BYTES_MOVED` | `0x1d` |  |
| `RADEON_INFO_VRAM_USAGE` | `0x1e` |  |
| `RADEON_INFO_GTT_USAGE` | `0x1f` |  |
| `RADEON_INFO_ACTIVE_CU_COUNT` | `0x20` |  |
| `RADEON_INFO_CURRENT_GPU_TEMP` | `0x21` |  |
| `RADEON_INFO_CURRENT_GPU_SCLK` | `0x22` |  |
| `RADEON_INFO_CURRENT_GPU_MCLK` | `0x23` |  |
| `RADEON_INFO_READ_REG` | `0x24` |  |
| `RADEON_INFO_VA_UNMAP_WORKING` | `0x25` |  |
| `RADEON_INFO_GPU_RESET_COUNTER` | `0x26` |  |

### RADEON_LINE (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_LINE_STRIP` | `0x3` |  |

### RADEON_LOCAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_LOCAL_TEX_HEAP` | `0` |  |

### RADEON_LOG (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_LOG_TEX_GRANULARITY` | `16` |  |

### RADEON_MAX (4)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_MAX_STATE_PACKETS` | `95` |  |
| `RADEON_MAX_TEXTURE_LEVELS` | `12` |  |
| `RADEON_MAX_TEXTURE_UNITS` | `3` |  |
| `RADEON_MAX_SURFACES` | `8` |  |

### RADEON_MEM (2)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_MEM_REGION_GART` | `1` |  |
| `RADEON_MEM_REGION_FB` | `2` |  |

### RADEON_NR (3)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_NR_SAREA_CLIPRECTS` | `12` |  |
| `RADEON_NR_TEX_HEAPS` | `2` |  |
| `RADEON_NR_TEX_REGIONS` | `64` |  |

### RADEON_OFFSET (3)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_OFFSET_SHIFT` | `10` |  |
| `RADEON_OFFSET_ALIGN` | `(1 << RADEON_OFFSET_SHIFT)` |  |
| `RADEON_OFFSET_MASK` | `(RADEON_OFFSET_ALIGN - 1)` |  |

### RADEON_PARAM (17)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_PARAM_GART_BUFFER_OFFSET` | `1` | card offset of 1st GART buffer |
| `RADEON_PARAM_LAST_FRAME` | `2` |  |
| `RADEON_PARAM_LAST_DISPATCH` | `3` |  |
| `RADEON_PARAM_LAST_CLEAR` | `4` |  |
| `RADEON_PARAM_IRQ_NR` | `5` |  |
| `RADEON_PARAM_GART_BASE` | `6` | card offset of GART base |
| `RADEON_PARAM_REGISTER_HANDLE` | `7` | for drmMap() |
| `RADEON_PARAM_STATUS_HANDLE` | `8` |  |
| `RADEON_PARAM_SAREA_HANDLE` | `9` |  |
| `RADEON_PARAM_GART_TEX_HANDLE` | `10` |  |
| `RADEON_PARAM_SCRATCH_OFFSET` | `11` |  |
| `RADEON_PARAM_CARD_TYPE` | `12` |  |
| `RADEON_PARAM_VBLANK_CRTC` | `13` | VBLANK CRTC |
| `RADEON_PARAM_FB_LOCATION` | `14` | FB location |
| `RADEON_PARAM_NUM_GB_PIPES` | `15` | num GB pipes |
| `RADEON_PARAM_DEVICE_ID` | `16` |  |
| `RADEON_PARAM_NUM_Z_PIPES` | `17` | num Z pipes |

### RADEON_RELOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_RELOC_PRIO_MASK` | `(0xf << 0)` |  |

### RADEON_REQUIRE (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_REQUIRE_QUIESCENCE` | `0x00010000` |  |

### RADEON_SCRATCH (1)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_SCRATCH_REG_OFFSET` | `32` |  |

### RADEON_SETPARAM (6)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_SETPARAM_FB_LOCATION` | `1` | determined framebuffer location |
| `RADEON_SETPARAM_SWITCH_TILING` | `2` | enable/disable color tiling |
| `RADEON_SETPARAM_PCIGART_LOCATION` | `3` | PCI Gart Location |
| `RADEON_SETPARAM_NEW_MEMMAP` | `4` | Use new memory map |
| `RADEON_SETPARAM_PCIGART_TABLE_SIZE` | `5` | PCI GART Table Size |
| `RADEON_SETPARAM_VBLANK_CRTC` | `6` | VBLANK CRTC |

### RADEON_TILING (16)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_TILING_MACRO` | `0x1` |  |
| `RADEON_TILING_MICRO` | `0x2` |  |
| `RADEON_TILING_SWAP_16BIT` | `0x4` |  |
| `RADEON_TILING_SWAP_32BIT` | `0x8` |  |
| `RADEON_TILING_SURFACE` | `0x10` |  |
| `RADEON_TILING_MICRO_SQUARE` | `0x20` |  |
| `RADEON_TILING_EG_BANKW_SHIFT` | `8` |  |
| `RADEON_TILING_EG_BANKW_MASK` | `0xf` |  |
| `RADEON_TILING_EG_BANKH_SHIFT` | `12` |  |
| `RADEON_TILING_EG_BANKH_MASK` | `0xf` |  |
| `RADEON_TILING_EG_MACRO_TILE_ASPECT_SHIFT` | `16` |  |
| `RADEON_TILING_EG_MACRO_TILE_ASPECT_MASK` | `0xf` |  |
| `RADEON_TILING_EG_TILE_SPLIT_SHIFT` | `24` |  |
| `RADEON_TILING_EG_TILE_SPLIT_MASK` | `0xf` |  |
| `RADEON_TILING_EG_STENCIL_TILE_SPLIT_SHIFT` | `28` |  |
| `RADEON_TILING_EG_STENCIL_TILE_SPLIT_MASK` | `0xf` |  |

### RADEON_TRIANGLE (2)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_TRIANGLE_FAN` | `0x5` |  |
| `RADEON_TRIANGLE_STRIP` | `0x6` |  |

### RADEON_UPLOAD (19)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_UPLOAD_CONTEXT` | `0x00000001` |  |
| `RADEON_UPLOAD_VERTFMT` | `0x00000002` |  |
| `RADEON_UPLOAD_LINE` | `0x00000004` |  |
| `RADEON_UPLOAD_BUMPMAP` | `0x00000008` |  |
| `RADEON_UPLOAD_MASKS` | `0x00000010` |  |
| `RADEON_UPLOAD_VIEWPORT` | `0x00000020` |  |
| `RADEON_UPLOAD_SETUP` | `0x00000040` |  |
| `RADEON_UPLOAD_TCL` | `0x00000080` |  |
| `RADEON_UPLOAD_MISC` | `0x00000100` |  |
| `RADEON_UPLOAD_TEX0` | `0x00000200` |  |
| `RADEON_UPLOAD_TEX1` | `0x00000400` |  |
| `RADEON_UPLOAD_TEX2` | `0x00000800` |  |
| `RADEON_UPLOAD_TEX0IMAGES` | `0x00001000` |  |
| `RADEON_UPLOAD_TEX1IMAGES` | `0x00002000` |  |
| `RADEON_UPLOAD_TEX2IMAGES` | `0x00004000` |  |
| `RADEON_UPLOAD_CLIPRECTS` | `0x00008000` | handled client-side |
| `RADEON_UPLOAD_ZBIAS` | `0x00020000` | version 1.2 and newer |
| `RADEON_UPLOAD_ALL` | `0x003effff` |  |
| `RADEON_UPLOAD_CONTEXT_ALL` | `0x003e01ff` |  |

### RADEON_USE (2)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_USE_HIERZ` | `0x40000000` |  |
| `RADEON_USE_COMP_ZBUF` | `0x20000000` |  |

### RADEON_VA (5)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_VA_MAP` | `1` |  |
| `RADEON_VA_UNMAP` | `2` |  |
| `RADEON_VA_RESULT_OK` | `0` |  |
| `RADEON_VA_RESULT_ERROR` | `1` |  |
| `RADEON_VA_RESULT_VA_EXIST` | `2` |  |

### RADEON_VM (5)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_VM_PAGE_VALID` | `(1 << 0)` |  |
| `RADEON_VM_PAGE_READABLE` | `(1 << 1)` |  |
| `RADEON_VM_PAGE_WRITEABLE` | `(1 << 2)` |  |
| `RADEON_VM_PAGE_SYSTEM` | `(1 << 3)` |  |
| `RADEON_VM_PAGE_SNOOPED` | `(1 << 4)` |  |

### RADEON_WAIT (2)

| Name | Value | Comment |
|------|-------|---------|
| `RADEON_WAIT_2D` | `0x1` |  |
| `RADEON_WAIT_3D` | `0x2` |  |

### SI_TILE (14)

| Name | Value | Comment |
|------|-------|---------|
| `SI_TILE_MODE_COLOR_LINEAR_ALIGNED` | `8` |  |
| `SI_TILE_MODE_COLOR_1D` | `13` |  |
| `SI_TILE_MODE_COLOR_1D_SCANOUT` | `9` |  |
| `SI_TILE_MODE_COLOR_2D_8BPP` | `14` |  |
| `SI_TILE_MODE_COLOR_2D_16BPP` | `15` |  |
| `SI_TILE_MODE_COLOR_2D_32BPP` | `16` |  |
| `SI_TILE_MODE_COLOR_2D_64BPP` | `17` |  |
| `SI_TILE_MODE_COLOR_2D_SCANOUT_16BPP` | `11` |  |
| `SI_TILE_MODE_COLOR_2D_SCANOUT_32BPP` | `12` |  |
| `SI_TILE_MODE_DEPTH_STENCIL_1D` | `4` |  |
| `SI_TILE_MODE_DEPTH_STENCIL_2D` | `0` |  |
| `SI_TILE_MODE_DEPTH_STENCIL_2D_2AA` | `3` |  |
| `SI_TILE_MODE_DEPTH_STENCIL_2D_4AA` | `3` |  |
| `SI_TILE_MODE_DEPTH_STENCIL_2D_8AA` | `2` |  |

### UNCATEGORIZED (91)

| Name | Value | Comment |
|------|-------|---------|
| `R200_EMIT_PP_TXCBLEND_0` | `21` | tex0/4 |
| `R200_EMIT_PP_TXCBLEND_1` | `22` | tex1/4 |
| `R200_EMIT_PP_TXCBLEND_2` | `23` | tex2/4 |
| `R200_EMIT_PP_TXCBLEND_3` | `24` | tex3/4 |
| `R200_EMIT_PP_TXCBLEND_4` | `25` | tex4/4 |
| `R200_EMIT_PP_TXCBLEND_5` | `26` | tex5/4 |
| `R200_EMIT_PP_TXCBLEND_6` | `27` | /4 |
| `R200_EMIT_PP_TXCBLEND_7` | `28` | /4 |
| `R200_EMIT_TCL_LIGHT_MODEL_CTL_0` | `29` | tcl/7 |
| `R200_EMIT_TFACTOR_0` | `30` | tf/7 |
| `R200_EMIT_VTX_FMT_0` | `31` | vtx/5 |
| `R200_EMIT_VAP_CTL` | `32` | vap/1 |
| `R200_EMIT_MATRIX_SELECT_0` | `33` | msl/5 |
| `R200_EMIT_TEX_PROC_CTL_2` | `34` | tcg/5 |
| `R200_EMIT_TCL_UCP_VERT_BLEND_CTL` | `35` | tcl/1 |
| `R200_EMIT_PP_TXFILTER_0` | `36` | tex0/6 |
| `R200_EMIT_PP_TXFILTER_1` | `37` | tex1/6 |
| `R200_EMIT_PP_TXFILTER_2` | `38` | tex2/6 |
| `R200_EMIT_PP_TXFILTER_3` | `39` | tex3/6 |
| `R200_EMIT_PP_TXFILTER_4` | `40` | tex4/6 |
| `R200_EMIT_PP_TXFILTER_5` | `41` | tex5/6 |
| `R200_EMIT_PP_TXOFFSET_0` | `42` | tex0/1 |
| `R200_EMIT_PP_TXOFFSET_1` | `43` | tex1/1 |
| `R200_EMIT_PP_TXOFFSET_2` | `44` | tex2/1 |
| `R200_EMIT_PP_TXOFFSET_3` | `45` | tex3/1 |
| `R200_EMIT_PP_TXOFFSET_4` | `46` | tex4/1 |
| `R200_EMIT_PP_TXOFFSET_5` | `47` | tex5/1 |
| `R200_EMIT_VTE_CNTL` | `48` | vte/1 |
| `R200_EMIT_OUTPUT_VTX_COMP_SEL` | `49` | vtx/1 |
| `R200_EMIT_PP_TAM_DEBUG3` | `50` | tam/1 |
| `R200_EMIT_PP_CNTL_X` | `51` | cst/1 |
| `R200_EMIT_RB3D_DEPTHXY_OFFSET` | `52` | cst/1 |
| `R200_EMIT_RE_AUX_SCISSOR_CNTL` | `53` | cst/1 |
| `R200_EMIT_RE_SCISSOR_TL_0` | `54` | cst/2 |
| `R200_EMIT_RE_SCISSOR_TL_1` | `55` | cst/2 |
| `R200_EMIT_RE_SCISSOR_TL_2` | `56` | cst/2 |
| `R200_EMIT_SE_VAP_CNTL_STATUS` | `57` | cst/1 |
| `R200_EMIT_SE_VTX_STATE_CNTL` | `58` | cst/1 |
| `R200_EMIT_RE_POINTSIZE` | `59` | cst/1 |
| `R200_EMIT_TCL_INPUT_VTX_VECTOR_ADDR_0` | `60` | cst/4 |
| `R200_EMIT_PP_CUBIC_FACES_0` | `61` |  |
| `R200_EMIT_PP_CUBIC_OFFSETS_0` | `62` |  |
| `R200_EMIT_PP_CUBIC_FACES_1` | `63` |  |
| `R200_EMIT_PP_CUBIC_OFFSETS_1` | `64` |  |
| `R200_EMIT_PP_CUBIC_FACES_2` | `65` |  |
| `R200_EMIT_PP_CUBIC_OFFSETS_2` | `66` |  |
| `R200_EMIT_PP_CUBIC_FACES_3` | `67` |  |
| `R200_EMIT_PP_CUBIC_OFFSETS_3` | `68` |  |
| `R200_EMIT_PP_CUBIC_FACES_4` | `69` |  |
| `R200_EMIT_PP_CUBIC_OFFSETS_4` | `70` |  |

*...and 41 more*

## Structs (61)


### `struct anonymous_0`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_9`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_12`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_14`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_20`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_21`

| Type | Field | Array |
|------|-------|-------|
| `drm_radeon_context_regs_t` | `context` | `-` |
| `drm_radeon_texture_regs_t` | `tex` | `RADEON_MAX_TEXTURE_UNITS` |
| `drm_radeon_context2_regs_t` | `context2` | `-` |

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|
| `drm_radeon_context_regs_t` | `context_state` | `-` |
| `drm_radeon_texture_regs_t` | `tex_state` | `RADEON_MAX_TEXTURE_UNITS` |
| `int` | `ctx_owner` | `-` |
| `int` | `pfState` | `-` |
| `int` | `pfCurrentPage` | `-` |
| `int` | `crtc2_base` | `-` |
| `int` | `tiling_enabled` | `-` |

### `struct drm_radeon_init`

| Type | Field | Array |
|------|-------|-------|
| `int` | `is_pci` | `-` |
| `int` | `cp_mode` | `-` |
| `int` | `gart_size` | `-` |
| `int` | `ring_size` | `-` |
| `int` | `usec_timeout` | `-` |

### `struct drm_radeon_cp_stop`

| Type | Field | Array |
|------|-------|-------|
| `int` | `flush` | `-` |
| `int` | `idle` | `-` |

### `struct drm_radeon_fullscreen`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_clear`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_vertex`

| Type | Field | Array |
|------|-------|-------|
| `int` | `prim` | `-` |
| `int` | `idx` | `-` |
| `int` | `count` | `-` |
| `int` | `discard` | `-` |

### `struct drm_radeon_indices`

| Type | Field | Array |
|------|-------|-------|
| `int` | `prim` | `-` |
| `int` | `idx` | `-` |
| `int` | `start` | `-` |
| `int` | `end` | `-` |
| `int` | `discard` | `-` |

### `struct drm_radeon_vertex2`

| Type | Field | Array |
|------|-------|-------|
| `int` | `idx` | `-` |
| `int` | `discard` | `-` |
| `int` | `nr_states` | `-` |
| `int` | `nr_prims` | `-` |

### `struct drm_radeon_cmd_buffer`

| Type | Field | Array |
|------|-------|-------|
| `int` | `bufsz` | `-` |
| `int` | `nbox` | `-` |

### `struct drm_radeon_tex_image`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_texture`

| Type | Field | Array |
|------|-------|-------|
| `int` | `pitch` | `-` |
| `int` | `format` | `-` |
| `int` | `width` | `-` |
| `int` | `height` | `-` |

### `struct drm_radeon_stipple`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_indirect`

| Type | Field | Array |
|------|-------|-------|
| `int` | `idx` | `-` |
| `int` | `start` | `-` |
| `int` | `end` | `-` |
| `int` | `discard` | `-` |

### `struct drm_radeon_getparam`

| Type | Field | Array |
|------|-------|-------|
| `int` | `param` | `-` |

### `struct drm_radeon_mem_alloc`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |
| `int` | `alignment` | `-` |
| `int` | `size` | `-` |

### `struct drm_radeon_mem_free`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |
| `int` | `region_offset` | `-` |

### `struct drm_radeon_mem_init_heap`

| Type | Field | Array |
|------|-------|-------|
| `int` | `region` | `-` |
| `int` | `size` | `-` |
| `int` | `start` | `-` |

### `struct drm_radeon_irq_emit`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_irq_wait`

| Type | Field | Array |
|------|-------|-------|
| `int` | `irq_seq` | `-` |

### `struct drm_radeon_setparam`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `value` | `-` |

### `struct drm_radeon_surface_alloc`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_surface_free`

| Type | Field | Array |
|------|-------|-------|

### `struct drm_radeon_gem_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `gart_size` | `-` |
| `__u64` | `vram_size` | `-` |
| `__u64` | `vram_visible` | `-` |

### `struct drm_radeon_gem_create`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u64` | `alignment` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `initial_domain` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_radeon_gem_userptr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_radeon_gem_set_tiling`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `tiling_flags` | `-` |
| `__u32` | `pitch` | `-` |

### `struct drm_radeon_gem_get_tiling`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `tiling_flags` | `-` |
| `__u32` | `pitch` | `-` |

### `struct drm_radeon_gem_mmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `addr_ptr` | `-` |

### `struct drm_radeon_gem_set_domain`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `read_domains` | `-` |
| `__u32` | `write_domain` | `-` |

### `struct drm_radeon_gem_wait_idle`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_radeon_gem_busy`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `domain` | `-` |

### `struct drm_radeon_gem_pread`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `data_ptr` | `-` |

### `struct drm_radeon_gem_pwrite`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `data_ptr` | `-` |

### `struct drm_radeon_gem_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `op` | `-` |
| `__u64` | `value` | `-` |

### `struct drm_radeon_gem_va`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `operation` | `-` |
| `__u32` | `vm_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_radeon_cs_chunk`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `chunk_id` | `-` |
| `__u32` | `length_dw` | `-` |
| `__u64` | `chunk_data` | `-` |

### `struct drm_radeon_cs_reloc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `read_domains` | `-` |
| `__u32` | `write_domain` | `-` |
| `__u32` | `flags` | `-` |

### `struct drm_radeon_cs`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_chunks` | `-` |
| `__u32` | `cs_id` | `-` |
| `__u64` | `chunks` | `-` |
| `__u64` | `gart_limit` | `-` |
| `__u64` | `vram_limit` | `-` |

### `struct drm_radeon_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `request` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `value` | `-` |

## Typedefs

- `drm_radeon_init`
- `drm_radeon_cp_stop`
- `drm_radeon_fullscreen`
- `drm_radeon_clear_rect`
- `drm_radeon_clear`
- `drm_radeon_vertex`
- `drm_radeon_indices`
- `drm_radeon_vertex2`
- `drm_radeon_cmd_buffer`
- `drm_radeon_tex_image`
- `drm_radeon_texture`
- `drm_radeon_stipple`
- `drm_radeon_indirect`
- `drm_radeon_getparam`
- `drm_radeon_mem_alloc`
- `drm_radeon_mem_free`
- `drm_radeon_mem_init_heap`
- `drm_radeon_irq_emit`
- `drm_radeon_irq_wait`
- `drm_radeon_setparam`
- `drm_radeon_surface_alloc`
- `drm_radeon_surface_free`