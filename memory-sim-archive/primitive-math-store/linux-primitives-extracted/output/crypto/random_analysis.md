# random.h

**Source:** `random.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`
- `linux/irqnr.h`

## Defines (10 total)


### UNCATEGORIZED (10)

| Name | Value | Comment |
|------|-------|---------|
| `RNDGETENTCNT` | `_IOR( 'R', 0x00, int )` |  |
| `RNDADDTOENTCNT` | `_IOW( 'R', 0x01, int )` |  |
| `RNDGETPOOL` | `_IOR( 'R', 0x02, int [2] )` |  |
| `RNDADDENTROPY` | `_IOW( 'R', 0x03, int [2] )` |  |
| `RNDZAPENTCNT` | `_IO( 'R', 0x04 )` |  |
| `RNDCLEARPOOL` | `_IO( 'R', 0x06 )` |  |
| `RNDRESEEDCRNG` | `_IO( 'R', 0x07 )` |  |
| `GRND_NONBLOCK` | `0x0001` |  |
| `GRND_RANDOM` | `0x0002` |  |
| `GRND_INSECURE` | `0x0004` |  |

## Structs (2)


### `struct rand_pool_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `entropy_count` | `-` |
| `int` | `buf_size` | `-` |

### `struct vgetrandom_opaque_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `size_of_opaque_state` | `-` |
| `__u32` | `mmap_prot` | `-` |
| `__u32` | `mmap_flags` | `-` |
| `__u32` | `reserved` | `13` |