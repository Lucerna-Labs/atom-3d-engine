# matroxfb.h

**Source:** `matroxfb.h`


## Includes

- `asm/ioctl.h`
- `linux/types.h`
- `linux/videodev2.h`
- `linux/fb.h`

## Defines (15 total)


### MATROXFB_GET (4)

| Name | Value | Comment |
|------|-------|---------|
| `MATROXFB_GET_OUTPUT_MODE` | `_IOWR('n',0xFA,size_t)` |  |
| `MATROXFB_GET_OUTPUT_CONNECTION` | `_IOR('n',0xF8,size_t)` |  |
| `MATROXFB_GET_AVAILABLE_OUTPUTS` | `_IOR('n',0xF9,size_t)` |  |
| `MATROXFB_GET_ALL_OUTPUTS` | `_IOR('n',0xFB,size_t)` |  |

### MATROXFB_OUTPUT (9)

| Name | Value | Comment |
|------|-------|---------|
| `MATROXFB_OUTPUT_PRIMARY` | `0x0000` |  |
| `MATROXFB_OUTPUT_SECONDARY` | `0x0001` |  |
| `MATROXFB_OUTPUT_DFP` | `0x0002` |  |
| `MATROXFB_OUTPUT_MODE_PAL` | `0x0001` |  |
| `MATROXFB_OUTPUT_MODE_NTSC` | `0x0002` |  |
| `MATROXFB_OUTPUT_MODE_MONITOR` | `0x0080` |  |
| `MATROXFB_OUTPUT_CONN_PRIMARY` | `(1 << MATROXFB_OUTPUT_PRIMARY)` |  |
| `MATROXFB_OUTPUT_CONN_SECONDARY` | `(1 << MATROXFB_OUTPUT_SECONDARY)` |  |
| `MATROXFB_OUTPUT_CONN_DFP` | `(1 << MATROXFB_OUTPUT_DFP)` |  |

### MATROXFB_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `MATROXFB_SET_OUTPUT_MODE` | `_IOW('n',0xFA,size_t)` |  |
| `MATROXFB_SET_OUTPUT_CONNECTION` | `_IOW('n',0xF8,size_t)` |  |

## Structs (1)


### `struct matroxioc_output_mode`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `output` | `-` |
| `__u32` | `mode` | `-` |