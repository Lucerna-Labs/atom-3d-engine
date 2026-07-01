# skl-tplg-interface.h

**Source:** `skl-tplg-interface.h`


## Includes

- `linux/types.h`

## Defines (8 total)


### HDA_SST (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDA_SST_CFG_MAX` | `900` | size of copier cfg |

### MAX_IN (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_IN_QUEUE` | `8` |  |

### MAX_OUT (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_OUT_QUEUE` | `8` |  |

### SKL_CONTROL (4)

| Name | Value | Comment |
|------|-------|---------|
| `SKL_CONTROL_TYPE_BYTE_TLV` | `0x100` |  |
| `SKL_CONTROL_TYPE_MIC_SELECT` | `0x102` |  |
| `SKL_CONTROL_TYPE_MULTI_IO_SELECT` | `0x103` |  |
| `SKL_CONTROL_TYPE_MULTI_IO_SELECT_DMIC` | `0x104` |  |

### SKL_UUID (1)

| Name | Value | Comment |
|------|-------|---------|
| `SKL_UUID_STR_SZ` | `40` |  |

## Structs (1)


### `struct skl_dfw_algo_data`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `param_id` | `-` |
| `__u32` | `max` | `-` |