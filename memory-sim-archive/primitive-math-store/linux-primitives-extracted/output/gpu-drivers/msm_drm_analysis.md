# msm_drm.h

**Source:** `msm_drm.h`


## Includes

- `drm.h`

## Defines (113 total)


### DRM_IOCTL (13)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_IOCTL_MSM_GET_PARAM` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_GET_PARAM, struct drm_ms` |  |
| `DRM_IOCTL_MSM_SET_PARAM` | `DRM_IOW (DRM_COMMAND_BASE + DRM_MSM_SET_PARAM, struct drm_ms` |  |
| `DRM_IOCTL_MSM_GEM_NEW` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_GEM_NEW, struct drm_msm_` |  |
| `DRM_IOCTL_MSM_GEM_INFO` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_GEM_INFO, struct drm_msm` |  |
| `DRM_IOCTL_MSM_GEM_CPU_PREP` | `DRM_IOW (DRM_COMMAND_BASE + DRM_MSM_GEM_CPU_PREP, struct drm` |  |
| `DRM_IOCTL_MSM_GEM_CPU_FINI` | `DRM_IOW (DRM_COMMAND_BASE + DRM_MSM_GEM_CPU_FINI, struct drm` |  |
| `DRM_IOCTL_MSM_GEM_SUBMIT` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_GEM_SUBMIT, struct drm_m` |  |
| `DRM_IOCTL_MSM_WAIT_FENCE` | `DRM_IOW (DRM_COMMAND_BASE + DRM_MSM_WAIT_FENCE, struct drm_m` |  |
| `DRM_IOCTL_MSM_GEM_MADVISE` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_GEM_MADVISE, struct drm_` |  |
| `DRM_IOCTL_MSM_SUBMITQUEUE_NEW` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_SUBMITQUEUE_NEW, struct ` |  |
| `DRM_IOCTL_MSM_SUBMITQUEUE_CLOSE` | `DRM_IOW (DRM_COMMAND_BASE + DRM_MSM_SUBMITQUEUE_CLOSE, __u32` |  |
| `DRM_IOCTL_MSM_SUBMITQUEUE_QUERY` | `DRM_IOW (DRM_COMMAND_BASE + DRM_MSM_SUBMITQUEUE_QUERY, struc` |  |
| `DRM_IOCTL_MSM_VM_BIND` | `DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_VM_BIND, struct drm_msm_` |  |

### DRM_MSM (14)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_MSM_GET_PARAM` | `0x00` |  |
| `DRM_MSM_SET_PARAM` | `0x01` |  |
| `DRM_MSM_GEM_NEW` | `0x02` |  |
| `DRM_MSM_GEM_INFO` | `0x03` |  |
| `DRM_MSM_GEM_CPU_PREP` | `0x04` |  |
| `DRM_MSM_GEM_CPU_FINI` | `0x05` |  |
| `DRM_MSM_GEM_SUBMIT` | `0x06` |  |
| `DRM_MSM_WAIT_FENCE` | `0x07` |  |
| `DRM_MSM_GEM_MADVISE` | `0x08` |  |
| `DRM_MSM_GEM_SVM_NEW` | `0x09` |  |
| `DRM_MSM_SUBMITQUEUE_NEW` | `0x0A` |  |
| `DRM_MSM_SUBMITQUEUE_CLOSE` | `0x0B` |  |
| `DRM_MSM_SUBMITQUEUE_QUERY` | `0x0C` |  |
| `DRM_MSM_VM_BIND` | `0x0D` |  |

### MSM_BO (9)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_BO_SCANOUT` | `0x00000001` | scanout capable |
| `MSM_BO_GPU_READONLY` | `0x00000002` |  |
| `MSM_BO_NO_SHARE` | `0x00000004` |  |
| `MSM_BO_CACHE_MASK` | `0x000f0000` |  |
| `MSM_BO_CACHED` | `0x00010000` |  |
| `MSM_BO_WC` | `0x00020000` |  |
| `MSM_BO_UNCACHED` | `0x00040000` | deprecated, use MSM_BO_WC |
| `MSM_BO_CACHED_COHERENT` | `0x080000` |  |
| `MSM_BO_FLAGS` | `(MSM_BO_SCANOUT \| ` |  |

### MSM_INFO (8)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_INFO_GET_OFFSET` | `0x00` | get mmap() offset, returned by value |
| `MSM_INFO_GET_IOVA` | `0x01` | get iova, returned by value |
| `MSM_INFO_SET_NAME` | `0x02` | set the debug name (by pointer) |
| `MSM_INFO_GET_NAME` | `0x03` | get debug name, returned by pointer |
| `MSM_INFO_SET_IOVA` | `0x04` | set the iova, passed by value |
| `MSM_INFO_GET_FLAGS` | `0x05` | get the MSM_BO_x flags |
| `MSM_INFO_SET_METADATA` | `0x06` | set userspace metadata |
| `MSM_INFO_GET_METADATA` | `0x07` | get userspace metadata |

### MSM_MADV (2)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_MADV_WILLNEED` | `0` | backing pages are needed, status returned in 'retained' |
| `MSM_MADV_DONTNEED` | `1` | backing pages not needed |

### MSM_PARAM (24)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_PARAM_GPU_ID` | `0x01` | RO |
| `MSM_PARAM_GMEM_SIZE` | `0x02` | RO |
| `MSM_PARAM_CHIP_ID` | `0x03` | RO |
| `MSM_PARAM_MAX_FREQ` | `0x04` | RO |
| `MSM_PARAM_TIMESTAMP` | `0x05` | RO |
| `MSM_PARAM_GMEM_BASE` | `0x06` | RO |
| `MSM_PARAM_PRIORITIES` | `0x07` | RO: The # of priority levels |
| `MSM_PARAM_PP_PGTABLE` | `0x08` | RO: Deprecated, always returns zero |
| `MSM_PARAM_FAULTS` | `0x09` | RO |
| `MSM_PARAM_SUSPENDS` | `0x0a` | RO |
| `MSM_PARAM_SYSPROF` | `0x0b` | WO: 1 preserves perfcntrs, 2 also disables suspend |
| `MSM_PARAM_COMM` | `0x0c` | WO: override for task->comm |
| `MSM_PARAM_CMDLINE` | `0x0d` | WO: override for task cmdline |
| `MSM_PARAM_VA_START` | `0x0e` | RO: start of valid GPU iova range |
| `MSM_PARAM_VA_SIZE` | `0x0f` | RO: size of valid GPU iova range (bytes) |
| `MSM_PARAM_HIGHEST_BANK_BIT` | `0x10` | RO |
| `MSM_PARAM_RAYTRACING` | `0x11` | RO |
| `MSM_PARAM_UBWC_SWIZZLE` | `0x12` | RO |
| `MSM_PARAM_MACROTILE_MODE` | `0x13` | RO |
| `MSM_PARAM_UCHE_TRAP_BASE` | `0x14` | RO |
| `MSM_PARAM_HAS_PRR` | `0x15` | RO |
| `MSM_PARAM_EN_VM_BIND` | `0x16` | WO, once |
| `MSM_PARAM_AQE` | `0x17` | RO |
| `MSM_PARAM_NR_RINGS` | `MSM_PARAM_PRIORITIES` |  |

### MSM_PIPE (5)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_PIPE_NONE` | `0x00` |  |
| `MSM_PIPE_2D0` | `0x01` |  |
| `MSM_PIPE_2D1` | `0x02` |  |
| `MSM_PIPE_3D0` | `0x10` |  |
| `MSM_PIPE_ID_MASK` | `0xffff` |  |

### MSM_PREP (5)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_PREP_READ` | `0x01` |  |
| `MSM_PREP_WRITE` | `0x02` |  |
| `MSM_PREP_NOSYNC` | `0x04` |  |
| `MSM_PREP_BOOST` | `0x08` |  |
| `MSM_PREP_FLAGS` | `(MSM_PREP_READ \| ` |  |

### MSM_SUBMIT (16)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_SUBMIT_CMD_BUF` | `0x0001` |  |
| `MSM_SUBMIT_CMD_IB_TARGET_BUF` | `0x0002` |  |
| `MSM_SUBMIT_CMD_CTX_RESTORE_BUF` | `0x0003` |  |
| `MSM_SUBMIT_BO_READ` | `0x0001` |  |
| `MSM_SUBMIT_BO_WRITE` | `0x0002` |  |
| `MSM_SUBMIT_BO_DUMP` | `0x0004` |  |
| `MSM_SUBMIT_BO_NO_IMPLICIT` | `0x0008` |  |
| `MSM_SUBMIT_BO_FLAGS` | `(MSM_SUBMIT_BO_READ \| ` |  |
| `MSM_SUBMIT_NO_IMPLICIT` | `0x80000000` | disable implicit sync |
| `MSM_SUBMIT_FENCE_FD_IN` | `0x40000000` | enable input fence_fd |
| `MSM_SUBMIT_FENCE_FD_OUT` | `0x20000000` | enable output fence_fd |
| `MSM_SUBMIT_SUDO` | `0x10000000` | run submitted cmds from RB |
| `MSM_SUBMIT_SYNCOBJ_IN` | `0x08000000` | enable input syncobj |
| `MSM_SUBMIT_SYNCOBJ_OUT` | `0x04000000` | enable output syncobj |
| `MSM_SUBMIT_FENCE_SN_IN` | `0x02000000` | userspace passes in seqno fence |
| `MSM_SUBMIT_FLAGS` | `( ` |  |

### MSM_SUBMITQUEUE (4)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_SUBMITQUEUE_ALLOW_PREEMPT` | `0x00000001` |  |
| `MSM_SUBMITQUEUE_VM_BIND` | `0x00000002` | virtual queue for VM_BIND ops |
| `MSM_SUBMITQUEUE_FLAGS` | `( ` |  |
| `MSM_SUBMITQUEUE_PARAM_FAULTS` | `0` |  |

### MSM_SYNCOBJ (2)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_SYNCOBJ_RESET` | `0x00000001` | Reset syncobj after wait. |
| `MSM_SYNCOBJ_FLAGS` | `( ` |  |

### MSM_VM (8)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_VM_BIND_OP_UNMAP` | `0` |  |
| `MSM_VM_BIND_OP_MAP` | `1` |  |
| `MSM_VM_BIND_OP_MAP_NULL` | `2` |  |
| `MSM_VM_BIND_OP_DUMP` | `1` |  |
| `MSM_VM_BIND_OP_FLAGS` | `( ` |  |
| `MSM_VM_BIND_FENCE_FD_IN` | `0x00000001` |  |
| `MSM_VM_BIND_FENCE_FD_OUT` | `0x00000002` |  |
| `MSM_VM_BIND_FLAGS` | `( ` |  |

### MSM_WAIT (2)

| Name | Value | Comment |
|------|-------|---------|
| `MSM_WAIT_FENCE_BOOST` | `0x00000001` |  |
| `MSM_WAIT_FENCE_FLAGS` | `( ` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `__MSM_MADV_PURGED` | `2` | internal state |

## Structs (17)


### `struct drm_msm_timespec`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `tv_sec` | `-` |
| `__s64` | `tv_nsec` | `-` |

### `struct drm_msm_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pipe` | `-` |
| `__u32` | `param` | `-` |
| `__u64` | `value` | `-` |
| `__u32` | `len` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_msm_gem_new`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_msm_gem_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `info` | `-` |
| `__u64` | `value` | `-` |
| `__u32` | `len` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_msm_gem_cpu_prep`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `op` | `-` |

### `struct drm_msm_gem_cpu_fini`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |

### `struct drm_msm_syncobj`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `point` | `-` |

### `struct drm_msm_gem_submit_reloc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `submit_offset` | `-` |
| `__u32` | `_or` | `-` |
| `__u32` | `or` | `-` |
| `__s32` | `shift` | `-` |
| `__u32` | `reloc_idx` | `-` |
| `__u64` | `reloc_offset` | `-` |

### `struct drm_msm_gem_submit_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `submit_idx` | `-` |
| `__u32` | `submit_offset` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `nr_relocs` | `-` |
| `__u64` | `relocs` | `-` |
| `__u64` | `iova` | `-` |

### `struct drm_msm_gem_submit_bo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `presumed` | `-` |

### `struct drm_msm_gem_submit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `fence` | `-` |
| `__u32` | `nr_bos` | `-` |
| `__u32` | `nr_cmds` | `-` |
| `__u64` | `bos` | `-` |
| `__u64` | `cmds` | `-` |
| `__s32` | `fence_fd` | `-` |
| `__u32` | `queueid` | `-` |
| `__u64` | `in_syncobjs` | `-` |
| `__u64` | `out_syncobjs` | `-` |
| `__u32` | `nr_in_syncobjs` | `-` |
| `__u32` | `nr_out_syncobjs` | `-` |
| `__u32` | `syncobj_stride` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_msm_vm_bind_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `handle` | `-` |
| `__u64` | `obj_offset` | `-` |
| `__u64` | `iova` | `-` |
| `__u64` | `range` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_msm_vm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `nr_ops` | `-` |
| `__s32` | `fence_fd` | `-` |
| `__u32` | `queue_id` | `-` |
| `__u64` | `in_syncobjs` | `-` |
| `__u64` | `out_syncobjs` | `-` |
| `__u32` | `nr_in_syncobjs` | `-` |
| `__u32` | `nr_out_syncobjs` | `-` |
| `__u32` | `syncobj_stride` | `-` |
| `__u32` | `op_stride` | `-` |
| `__u64` | `ops` | `-` |

### `struct drm_msm_wait_fence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fence` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `queueid` | `-` |

### `struct drm_msm_gem_madvise`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `madv` | `-` |
| `__u32` | `retained` | `-` |

### `struct drm_msm_submitqueue`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `prio` | `-` |
| `__u32` | `id` | `-` |

### `struct drm_msm_submitqueue_query`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `param` | `-` |
| `__u32` | `len` | `-` |
| `__u32` | `pad` | `-` |