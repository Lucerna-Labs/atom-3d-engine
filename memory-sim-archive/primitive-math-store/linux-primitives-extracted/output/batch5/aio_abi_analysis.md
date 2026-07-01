# aio_abi.h

**Source:** `aio_abi.h`


## Includes

- `linux/types.h`
- `linux/fs.h`
- `asm/byteorder.h`

## Defines (2 total)


### IOCB_FLAG (2)

| Name | Value | Comment |
|------|-------|---------|
| `IOCB_FLAG_RESFD` | `(1 << 0)` |  |
| `IOCB_FLAG_IOPRIO` | `(1 << 1)` |  |

## Structs (2)


### `struct io_event`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data` | `-` |
| `__u64` | `obj` | `-` |
| `__s64` | `res` | `-` |
| `__s64` | `res2` | `-` |

### `struct iocb`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `aio_data` | `-` |
| `__u32` | `aio_key` | `-` |
| `__kernel_rwf_t` | `aio_rw_flags` | `-` |
| `__kernel_rwf_t` | `aio_rw_flags` | `-` |
| `__u32` | `aio_key` | `-` |
| `__u16` | `aio_lio_opcode` | `-` |
| `__s16` | `aio_reqprio` | `-` |
| `__u32` | `aio_fildes` | `-` |
| `__u64` | `aio_buf` | `-` |
| `__u64` | `aio_nbytes` | `-` |
| `__s64` | `aio_offset` | `-` |
| `__u64` | `aio_reserved2` | `-` |
| `__u32` | `aio_flags` | `-` |
| `__u32` | `aio_resfd` | `-` |