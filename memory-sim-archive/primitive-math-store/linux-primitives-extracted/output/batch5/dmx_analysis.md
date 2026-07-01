# dmx.h

**Source:** `dmx.h`


## Includes

- `linux/types.h`
- `time.h`

## Defines (23 total)


### DMX_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_ADD_PID` | `_IOW('o', 51, __u16)` |  |

### DMX_CHECK (1)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_CHECK_CRC` | `1` |  |

### DMX_FILTER (1)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_FILTER_SIZE` | `16` |  |

### DMX_GET (2)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_GET_PES_PIDS` | `_IOR('o', 47, __u16[5])` |  |
| `DMX_GET_STC` | `_IOWR('o', 50, struct dmx_stc)` |  |

### DMX_IMMEDIATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_IMMEDIATE_START` | `4` |  |

### DMX_PES (5)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_PES_AUDIO` | `DMX_PES_AUDIO0` |  |
| `DMX_PES_VIDEO` | `DMX_PES_VIDEO0` |  |
| `DMX_PES_TELETEXT` | `DMX_PES_TELETEXT0` |  |
| `DMX_PES_SUBTITLE` | `DMX_PES_SUBTITLE0` |  |
| `DMX_PES_PCR` | `DMX_PES_PCR0` |  |

### DMX_REMOVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_REMOVE_PID` | `_IOW('o', 52, __u16)` |  |

### DMX_SET (3)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_SET_FILTER` | `_IOW('o', 43, struct dmx_sct_filter_params)` |  |
| `DMX_SET_PES_FILTER` | `_IOW('o', 44, struct dmx_pes_filter_params)` |  |
| `DMX_SET_BUFFER_SIZE` | `_IO('o', 45)` |  |

### UNCATEGORIZED (8)

| Name | Value | Comment |
|------|-------|---------|
| `DMX_ONESHOT` | `2` |  |
| `DMX_START` | `_IO('o', 41)` |  |
| `DMX_STOP` | `_IO('o', 42)` |  |
| `DMX_REQBUFS` | `_IOWR('o', 60, struct dmx_requestbuffers)` |  |
| `DMX_QUERYBUF` | `_IOWR('o', 61, struct dmx_buffer)` |  |
| `DMX_EXPBUF` | `_IOWR('o', 62, struct dmx_exportbuffer)` |  |
| `DMX_QBUF` | `_IOWR('o', 63, struct dmx_buffer)` |  |
| `DMX_DQBUF` | `_IOWR('o', 64, struct dmx_buffer)` |  |

## Structs (7)


### `struct dmx_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `filter` | `DMX_FILTER_SIZE` |
| `__u8` | `mask` | `DMX_FILTER_SIZE` |
| `__u8` | `mode` | `DMX_FILTER_SIZE` |

### `struct dmx_sct_filter_params`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `pid` | `-` |
| `__u32` | `timeout` | `-` |
| `__u32` | `flags` | `-` |

### `struct dmx_pes_filter_params`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `pid` | `-` |
| `__u32` | `flags` | `-` |

### `struct dmx_stc`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `stc` | `-` |

### `struct dmx_buffer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `index` | `-` |
| `__u32` | `bytesused` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `length` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `count` | `-` |

### `struct dmx_requestbuffers`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |
| `__u32` | `size` | `-` |

### `struct dmx_exportbuffer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `index` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `fd` | `-` |

## Typedefs

- `dmx_output_t`
- `dmx_input_t`
- `dmx_pes_type_t`
- `dmx_filter_t`