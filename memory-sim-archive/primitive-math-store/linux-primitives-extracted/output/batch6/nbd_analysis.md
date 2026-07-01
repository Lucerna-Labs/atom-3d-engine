# nbd.h

**Source:** `nbd.h`


## Includes

- `linux/types.h`

## Defines (25 total)


### NBD_CFLAG (2)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_CFLAG_DESTROY_ON_DISCONNECT` | `(1 << 0) /* delete the nbd device on` |  |
| `NBD_CFLAG_DISCONNECT_ON_CLOSE` | `(1 << 1) /* disconnect the nbd device on` |  |

### NBD_CLEAR (2)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_CLEAR_SOCK` | `_IO( 0xab, 4 )` |  |
| `NBD_CLEAR_QUE` | `_IO( 0xab, 5 )` |  |

### NBD_CMD (2)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_CMD_FLAG_FUA` | `(1 << 16)` | FUA (forced unit access) op |
| `NBD_CMD_FLAG_NO_HOLE` | `(1 << 17)` | Do not punch a hole for WRITE_ZEROES |

### NBD_DO (1)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_DO_IT` | `_IO( 0xab, 3 )` |  |

### NBD_FLAG (8)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_FLAG_HAS_FLAGS` | `(1 << 0)` | nbd-server supports flags |
| `NBD_FLAG_READ_ONLY` | `(1 << 1)` | device is read-only |
| `NBD_FLAG_SEND_FLUSH` | `(1 << 2)` | can flush writeback cache |
| `NBD_FLAG_SEND_FUA` | `(1 << 3)` | send FUA (forced unit access) |
| `NBD_FLAG_ROTATIONAL` | `(1 << 4)` | device is rotational |
| `NBD_FLAG_SEND_TRIM` | `(1 << 5)` | send trim/discard |
| `NBD_FLAG_SEND_WRITE_ZEROES` | `(1 << 6)` | supports WRITE_ZEROES |
| `NBD_FLAG_CAN_MULTI_CONN` | `(1 << 8)` | Server supports multiple connections per export. |

### NBD_PRINT (1)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_PRINT_DEBUG` | `_IO( 0xab, 6 )` |  |

### NBD_REPLY (1)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_REPLY_MAGIC` | `0x67446698` |  |

### NBD_REQUEST (1)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_REQUEST_MAGIC` | `0x25609513` |  |

### NBD_SET (6)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_SET_SOCK` | `_IO( 0xab, 0 )` |  |
| `NBD_SET_BLKSIZE` | `_IO( 0xab, 1 )` |  |
| `NBD_SET_SIZE` | `_IO( 0xab, 2 )` |  |
| `NBD_SET_SIZE_BLOCKS` | `_IO( 0xab, 7 )` |  |
| `NBD_SET_TIMEOUT` | `_IO( 0xab, 9 )` |  |
| `NBD_SET_FLAGS` | `_IO( 0xab, 10)` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `NBD_DISCONNECT` | `_IO( 0xab, 8 )` |  |

## Structs (2)


### `struct nbd_request`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `magic` | `-` |
| `__be32` | `type` | `-` |
| `__be64` | `cookie` | `-` |
| `char` | `handle` | `8` |
| `__be64` | `from` | `-` |
| `__be32` | `len` | `-` |

### `struct nbd_reply`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `magic` | `-` |
| `__be32` | `error` | `-` |
| `__be64` | `cookie` | `-` |
| `char` | `handle` | `8` |