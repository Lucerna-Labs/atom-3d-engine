# tee.h

**Source:** `tee.h`


## Includes

- `linux/ioctl.h`
- `linux/types.h`

## Defines (52 total)


### TEE_GEN (5)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_GEN_CAP_GP` | `(1 << 0)` | GlobalPlatform compliant TEE |
| `TEE_GEN_CAP_PRIVILEGED` | `(1 << 1)` | Privileged device (for supplicant) |
| `TEE_GEN_CAP_REG_MEM` | `(1 << 2)` | Supports registering shared memory |
| `TEE_GEN_CAP_MEMREF_NULL` | `(1 << 3)` | NULL MemRef support |
| `TEE_GEN_CAP_OBJREF` | `(1 << 4)` | Supports generic object reference |

### TEE_IMPL (4)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_IMPL_ID_OPTEE` | `1` |  |
| `TEE_IMPL_ID_AMDTEE` | `2` |  |
| `TEE_IMPL_ID_TSTEE` | `3` |  |
| `TEE_IMPL_ID_QTEE` | `4` |  |

### TEE_IOC (13)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_IOC_MAGIC` | `0xa4` |  |
| `TEE_IOC_BASE` | `0` |  |
| `TEE_IOC_VERSION` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 0, ` |  |
| `TEE_IOC_SHM_ALLOC` | `_IOWR(TEE_IOC_MAGIC, TEE_IOC_BASE + 1, ` |  |
| `TEE_IOC_OPEN_SESSION` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 2, ` |  |
| `TEE_IOC_INVOKE` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 3, ` |  |
| `TEE_IOC_CANCEL` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 4, ` |  |
| `TEE_IOC_CLOSE_SESSION` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 5, ` |  |
| `TEE_IOC_SUPPL_RECV` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 6, ` |  |
| `TEE_IOC_SUPPL_SEND` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 7, ` |  |
| `TEE_IOC_SHM_REGISTER_FD` | `_IOWR(TEE_IOC_MAGIC, TEE_IOC_BASE + 8, ` |  |
| `TEE_IOC_SHM_REGISTER` | `_IOWR(TEE_IOC_MAGIC, TEE_IOC_BASE + 9, ` |  |
| `TEE_IOC_OBJECT_INVOKE` | `_IOR(TEE_IOC_MAGIC, TEE_IOC_BASE + 10, ` |  |

### TEE_IOCTL (26)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_IOCTL_PARAM_ATTR_TYPE_NONE` | `0` | parameter not used |
| `TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT` | `1` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT` | `2` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INOUT` | `3` | input and output |
| `TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT` | `5` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT` | `6` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT` | `7` | input and output |
| `TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_INPUT` | `8` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_OUTPUT` | `9` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_INOUT` | `10` | input and output |
| `TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_INPUT` | `11` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_OUTPUT` | `12` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_INOUT` | `13` |  |
| `TEE_IOCTL_PARAM_ATTR_TYPE_MASK` | `0xff` |  |
| `TEE_IOCTL_PARAM_ATTR_META` | `0x100` |  |
| `TEE_IOCTL_PARAM_ATTR_MASK` | `` |  |
| `TEE_IOCTL_LOGIN_PUBLIC` | `0` |  |
| `TEE_IOCTL_LOGIN_USER` | `1` |  |
| `TEE_IOCTL_LOGIN_GROUP` | `2` |  |
| `TEE_IOCTL_LOGIN_APPLICATION` | `4` |  |
| `TEE_IOCTL_LOGIN_USER_APPLICATION` | `5` |  |
| `TEE_IOCTL_LOGIN_GROUP_APPLICATION` | `6` |  |
| `TEE_IOCTL_LOGIN_REE_KERNEL_MIN` | `0x80000000` |  |
| `TEE_IOCTL_LOGIN_REE_KERNEL_MAX` | `0xBFFFFFFF` |  |
| `TEE_IOCTL_LOGIN_REE_KERNEL` | `0x80000000` |  |
| `TEE_IOCTL_UUID_LEN` | `16` |  |

### TEE_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_MAX_ARG_SIZE` | `4096` |  |

### TEE_MEMREF (1)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_MEMREF_NULL` | `((__u64)(-1))` | NULL MemRef Buffer |

### TEE_OBJREF (1)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_OBJREF_NULL` | `((__u64)(-1))` | NULL ObjRef Object |

### TEE_OPTEE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TEE_OPTEE_CAP_TZ` | `(1 << 0)` |  |

## Structs (13)


### `struct tee_ioctl_version_data`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `impl_id` | `-` |
| `__u32` | `impl_caps` | `-` |
| `__u32` | `gen_caps` | `-` |

### `struct tee_ioctl_shm_alloc_data`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `id` | `-` |

### `struct tee_ioctl_buf_data`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `buf_ptr` | `-` |
| `__u64` | `buf_len` | `-` |

### `struct tee_ioctl_param`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `attr` | `-` |
| `__u64` | `a` | `-` |
| `__u64` | `b` | `-` |
| `__u64` | `c` | `-` |

### `struct tee_ioctl_open_session_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `uuid` | `TEE_IOCTL_UUID_LEN` |
| `__u8` | `clnt_uuid` | `TEE_IOCTL_UUID_LEN` |
| `__u32` | `clnt_login` | `-` |
| `__u32` | `cancel_id` | `-` |
| `__u32` | `session` | `-` |
| `__u32` | `ret` | `-` |
| `__u32` | `ret_origin` | `-` |
| `__u32` | `num_params` | `-` |

### `struct tee_ioctl_invoke_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `func` | `-` |
| `__u32` | `session` | `-` |
| `__u32` | `cancel_id` | `-` |
| `__u32` | `ret` | `-` |
| `__u32` | `ret_origin` | `-` |
| `__u32` | `num_params` | `-` |

### `struct tee_ioctl_cancel_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cancel_id` | `-` |
| `__u32` | `session` | `-` |

### `struct tee_ioctl_close_session_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `session` | `-` |

### `struct tee_iocl_supp_recv_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `func` | `-` |
| `__u32` | `num_params` | `-` |

### `struct tee_iocl_supp_send_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ret` | `-` |
| `__u32` | `num_params` | `-` |

### `struct tee_ioctl_shm_register_data`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `length` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `id` | `-` |

### `struct tee_ioctl_shm_register_fd_data`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `fd` | `-` |
| `__u64` | `size` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `id` | `-` |

### `struct tee_ioctl_object_invoke_arg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |
| `__u32` | `op` | `-` |
| `__u32` | `ret` | `-` |
| `__u32` | `num_params` | `-` |