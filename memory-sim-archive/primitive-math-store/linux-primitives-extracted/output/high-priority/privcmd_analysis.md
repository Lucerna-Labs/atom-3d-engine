# privcmd.h

**Source:** `privcmd.h`


## Includes

- `linux/types.h`
- `linux/compiler.h`
- `xen/interface/xen.h`

## Defines (14 total)


### IOCTL_PRIVCMD (10)

| Name | Value | Comment |
|------|-------|---------|
| `IOCTL_PRIVCMD_HYPERCALL` | `` |  |
| `IOCTL_PRIVCMD_MMAP` | `` |  |
| `IOCTL_PRIVCMD_MMAPBATCH` | `` |  |
| `IOCTL_PRIVCMD_MMAPBATCH_V2` | `` |  |
| `IOCTL_PRIVCMD_DM_OP` | `` |  |
| `IOCTL_PRIVCMD_RESTRICT` | `` |  |
| `IOCTL_PRIVCMD_MMAP_RESOURCE` | `` |  |
| `IOCTL_PRIVCMD_IRQFD` | `` |  |
| `IOCTL_PRIVCMD_IOEVENTFD` | `` |  |
| `IOCTL_PRIVCMD_PCIDEV_GET_GSI` | `` |  |

### PRIVCMD_IOEVENTFD (1)

| Name | Value | Comment |
|------|-------|---------|
| `PRIVCMD_IOEVENTFD_FLAG_DEASSIGN` | `(1 << 0)` |  |

### PRIVCMD_IRQFD (1)

| Name | Value | Comment |
|------|-------|---------|
| `PRIVCMD_IRQFD_FLAG_DEASSIGN` | `(1 << 0)` |  |

### PRIVCMD_MMAPBATCH (2)

| Name | Value | Comment |
|------|-------|---------|
| `PRIVCMD_MMAPBATCH_MFN_ERROR` | `0xf0000000U` |  |
| `PRIVCMD_MMAPBATCH_PAGED_ERROR` | `0x80000000U` |  |

## Structs (11)


### `struct privcmd_hypercall`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `op` | `-` |
| `__u64` | `arg` | `5` |

### `struct privcmd_mmap_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `va` | `-` |
| `__u64` | `mfn` | `-` |
| `__u64` | `npages` | `-` |

### `struct privcmd_mmap`

| Type | Field | Array |
|------|-------|-------|
| `int` | `num` | `-` |
| `domid_t` | `dom` | `-` |

### `struct privcmd_mmapbatch`

| Type | Field | Array |
|------|-------|-------|
| `int` | `num` | `-` |
| `domid_t` | `dom` | `-` |
| `__u64` | `addr` | `-` |

### `struct privcmd_mmapbatch_v2`

| Type | Field | Array |
|------|-------|-------|
| `domid_t` | `dom` | `-` |
| `__u64` | `addr` | `-` |

### `struct privcmd_dm_op_buf`

| Type | Field | Array |
|------|-------|-------|
| `size_t` | `size` | `-` |

### `struct privcmd_dm_op`

| Type | Field | Array |
|------|-------|-------|
| `domid_t` | `dom` | `-` |
| `__u16` | `num` | `-` |

### `struct privcmd_mmap_resource`

| Type | Field | Array |
|------|-------|-------|
| `domid_t` | `dom` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `idx` | `-` |
| `__u64` | `num` | `-` |
| `__u64` | `addr` | `-` |

### `struct privcmd_irqfd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dm_op` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `fd` | `-` |
| `__u32` | `flags` | `-` |
| `domid_t` | `dom` | `-` |
| `__u8` | `pad` | `2` |

### `struct privcmd_ioeventfd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ioreq` | `-` |
| `__u64` | `ports` | `-` |
| `__u64` | `addr` | `-` |
| `__u32` | `addr_len` | `-` |
| `__u32` | `event_fd` | `-` |
| `__u32` | `vcpus` | `-` |
| `__u32` | `vq` | `-` |
| `__u32` | `flags` | `-` |
| `domid_t` | `dom` | `-` |
| `__u8` | `pad` | `2` |

### `struct privcmd_pcidev_get_gsi`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `sbdf` | `-` |
| `__u32` | `gsi` | `-` |