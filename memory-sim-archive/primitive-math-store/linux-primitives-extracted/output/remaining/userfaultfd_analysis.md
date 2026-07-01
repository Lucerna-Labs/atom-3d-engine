# userfaultfd.h

**Source:** `userfaultfd.h`


## Includes

- `linux/types.h`

## Defines (68 total)


### UFFDIO_CONTINUE (2)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_CONTINUE_MODE_DONTWAKE` | `((__u64)1<<0)` |  |
| `UFFDIO_CONTINUE_MODE_WP` | `((__u64)1<<1)` |  |

### UFFDIO_COPY (2)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_COPY_MODE_DONTWAKE` | `((__u64)1<<0)` |  |
| `UFFDIO_COPY_MODE_WP` | `((__u64)1<<1)` |  |

### UFFDIO_MOVE (2)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_MOVE_MODE_DONTWAKE` | `((__u64)1<<0)` |  |
| `UFFDIO_MOVE_MODE_ALLOW_SRC_HOLES` | `((__u64)1<<1)` |  |

### UFFDIO_POISON (1)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_POISON_MODE_DONTWAKE` | `((__u64)1<<0)` |  |

### UFFDIO_REGISTER (3)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_REGISTER_MODE_MISSING` | `((__u64)1<<0)` |  |
| `UFFDIO_REGISTER_MODE_WP` | `((__u64)1<<1)` |  |
| `UFFDIO_REGISTER_MODE_MINOR` | `((__u64)1<<2)` |  |

### UFFDIO_WRITEPROTECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_WRITEPROTECT_MODE_WP` | `((__u64)1<<0)` |  |
| `UFFDIO_WRITEPROTECT_MODE_DONTWAKE` | `((__u64)1<<1)` |  |

### UFFDIO_ZEROPAGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `UFFDIO_ZEROPAGE_MODE_DONTWAKE` | `((__u64)1<<0)` |  |

### UFFD_API (5)

| Name | Value | Comment |
|------|-------|---------|
| `UFFD_API_REGISTER_MODES` | `(UFFDIO_REGISTER_MODE_MISSING \|	` |  |
| `UFFD_API_FEATURES` | `(UFFD_FEATURE_PAGEFAULT_FLAG_WP \|	` |  |
| `UFFD_API_IOCTLS` | `` |  |
| `UFFD_API_RANGE_IOCTLS` | `` |  |
| `UFFD_API_RANGE_IOCTLS_BASIC` | `` |  |

### UFFD_EVENT (5)

| Name | Value | Comment |
|------|-------|---------|
| `UFFD_EVENT_PAGEFAULT` | `0x12` |  |
| `UFFD_EVENT_FORK` | `0x13` |  |
| `UFFD_EVENT_REMAP` | `0x14` |  |
| `UFFD_EVENT_REMOVE` | `0x15` |  |
| `UFFD_EVENT_UNMAP` | `0x16` |  |

### UFFD_FEATURE (17)

| Name | Value | Comment |
|------|-------|---------|
| `UFFD_FEATURE_PAGEFAULT_FLAG_WP` | `(1<<0)` |  |
| `UFFD_FEATURE_EVENT_FORK` | `(1<<1)` |  |
| `UFFD_FEATURE_EVENT_REMAP` | `(1<<2)` |  |
| `UFFD_FEATURE_EVENT_REMOVE` | `(1<<3)` |  |
| `UFFD_FEATURE_MISSING_HUGETLBFS` | `(1<<4)` |  |
| `UFFD_FEATURE_MISSING_SHMEM` | `(1<<5)` |  |
| `UFFD_FEATURE_EVENT_UNMAP` | `(1<<6)` |  |
| `UFFD_FEATURE_SIGBUS` | `(1<<7)` |  |
| `UFFD_FEATURE_THREAD_ID` | `(1<<8)` |  |
| `UFFD_FEATURE_MINOR_HUGETLBFS` | `(1<<9)` |  |
| `UFFD_FEATURE_MINOR_SHMEM` | `(1<<10)` |  |
| `UFFD_FEATURE_EXACT_ADDRESS` | `(1<<11)` |  |
| `UFFD_FEATURE_WP_HUGETLBFS_SHMEM` | `(1<<12)` |  |
| `UFFD_FEATURE_WP_UNPOPULATED` | `(1<<13)` |  |
| `UFFD_FEATURE_POISON` | `(1<<14)` |  |
| `UFFD_FEATURE_WP_ASYNC` | `(1<<15)` |  |
| `UFFD_FEATURE_MOVE` | `(1<<16)` |  |

### UFFD_PAGEFAULT (3)

| Name | Value | Comment |
|------|-------|---------|
| `UFFD_PAGEFAULT_FLAG_WRITE` | `(1<<0)` | If this was a write fault |
| `UFFD_PAGEFAULT_FLAG_WP` | `(1<<1)` | If reason is VM_UFFD_WP |
| `UFFD_PAGEFAULT_FLAG_MINOR` | `(1<<2)` | If reason is VM_UFFD_MINOR |

### UFFD_USER (1)

| Name | Value | Comment |
|------|-------|---------|
| `UFFD_USER_MODE_ONLY` | `1` |  |

### UNCATEGORIZED (23)

| Name | Value | Comment |
|------|-------|---------|
| `USERFAULTFD_IOC` | `0xAA` |  |
| `UFFD_API` | `((__u64)0xAA)` |  |
| `_UFFDIO_REGISTER` | `(0x00)` |  |
| `_UFFDIO_UNREGISTER` | `(0x01)` |  |
| `_UFFDIO_WAKE` | `(0x02)` |  |
| `_UFFDIO_COPY` | `(0x03)` |  |
| `_UFFDIO_ZEROPAGE` | `(0x04)` |  |
| `_UFFDIO_MOVE` | `(0x05)` |  |
| `_UFFDIO_WRITEPROTECT` | `(0x06)` |  |
| `_UFFDIO_CONTINUE` | `(0x07)` |  |
| `_UFFDIO_POISON` | `(0x08)` |  |
| `_UFFDIO_API` | `(0x3F)` |  |
| `UFFDIO` | `0xAA` |  |
| `UFFDIO_API` | `_IOWR(UFFDIO, _UFFDIO_API,	` |  |
| `UFFDIO_REGISTER` | `_IOWR(UFFDIO, _UFFDIO_REGISTER, ` |  |
| `UFFDIO_UNREGISTER` | `_IOR(UFFDIO, _UFFDIO_UNREGISTER,	` |  |
| `UFFDIO_WAKE` | `_IOR(UFFDIO, _UFFDIO_WAKE,	` |  |
| `UFFDIO_COPY` | `_IOWR(UFFDIO, _UFFDIO_COPY,	` |  |
| `UFFDIO_ZEROPAGE` | `_IOWR(UFFDIO, _UFFDIO_ZEROPAGE,	` |  |
| `UFFDIO_MOVE` | `_IOWR(UFFDIO, _UFFDIO_MOVE,	` |  |
| `UFFDIO_WRITEPROTECT` | `_IOWR(UFFDIO, _UFFDIO_WRITEPROTECT, ` |  |
| `UFFDIO_CONTINUE` | `_IOWR(UFFDIO, _UFFDIO_CONTINUE,	` |  |
| `UFFDIO_POISON` | `_IOWR(UFFDIO, _UFFDIO_POISON, ` |  |

### USERFAULTFD_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `USERFAULTFD_IOC_NEW` | `_IO(USERFAULTFD_IOC, 0x00)` |  |

## Structs (15)


### `struct uffd_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `event` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u32` | `reserved3` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `address` | `-` |
| `__u32` | `ptid` | `-` |
| `__u32` | `ufd` | `-` |
| `__u64` | `from` | `-` |
| `__u64` | `to` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |
| `__u64` | `reserved1` | `-` |
| `__u64` | `reserved2` | `-` |
| `__u64` | `reserved3` | `-` |

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `address` | `-` |
| `__u32` | `ptid` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ufd` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `from` | `-` |
| `__u64` | `to` | `-` |
| `__u64` | `len` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reserved1` | `-` |
| `__u64` | `reserved2` | `-` |
| `__u64` | `reserved3` | `-` |

### `struct uffdio_api`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `api` | `-` |
| `__u64` | `features` | `-` |
| `__u64` | `ioctls` | `-` |

### `struct uffdio_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `len` | `-` |

### `struct uffdio_register`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mode` | `-` |
| `__u64` | `ioctls` | `-` |

### `struct uffdio_copy`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dst` | `-` |
| `__u64` | `src` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `mode` | `-` |
| `__s64` | `copy` | `-` |

### `struct uffdio_zeropage`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mode` | `-` |
| `__s64` | `zeropage` | `-` |

### `struct uffdio_writeprotect`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mode` | `-` |

### `struct uffdio_continue`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mode` | `-` |
| `__s64` | `mapped` | `-` |

### `struct uffdio_poison`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `mode` | `-` |
| `__s64` | `updated` | `-` |

### `struct uffdio_move`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `dst` | `-` |
| `__u64` | `src` | `-` |
| `__u64` | `len` | `-` |
| `__u64` | `mode` | `-` |
| `__s64` | `move` | `-` |