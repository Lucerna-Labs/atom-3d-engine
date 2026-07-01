# v4l2-common.h

**Source:** `v4l2-common.h`


## Includes

- `linux/types.h`

## Defines (20 total)


### UNCATEGORIZED (20)

| Name | Value | Comment |
|------|-------|---------|
| `V4L2_SEL_TGT_CROP` | `0x0000` |  |
| `V4L2_SEL_TGT_CROP_DEFAULT` | `0x0001` |  |
| `V4L2_SEL_TGT_CROP_BOUNDS` | `0x0002` |  |
| `V4L2_SEL_TGT_NATIVE_SIZE` | `0x0003` |  |
| `V4L2_SEL_TGT_COMPOSE` | `0x0100` |  |
| `V4L2_SEL_TGT_COMPOSE_DEFAULT` | `0x0101` |  |
| `V4L2_SEL_TGT_COMPOSE_BOUNDS` | `0x0102` |  |
| `V4L2_SEL_TGT_COMPOSE_PADDED` | `0x0103` |  |
| `V4L2_SEL_FLAG_GE` | `(1 << 0)` |  |
| `V4L2_SEL_FLAG_LE` | `(1 << 1)` |  |
| `V4L2_SEL_FLAG_KEEP_CONFIG` | `(1 << 2)` |  |
| `V4L2_SEL_TGT_CROP_ACTIVE` | `V4L2_SEL_TGT_CROP` |  |
| `V4L2_SEL_TGT_COMPOSE_ACTIVE` | `V4L2_SEL_TGT_COMPOSE` |  |
| `V4L2_SUBDEV_SEL_TGT_CROP_ACTUAL` | `V4L2_SEL_TGT_CROP` |  |
| `V4L2_SUBDEV_SEL_TGT_COMPOSE_ACTUAL` | `V4L2_SEL_TGT_COMPOSE` |  |
| `V4L2_SUBDEV_SEL_TGT_CROP_BOUNDS` | `V4L2_SEL_TGT_CROP_BOUNDS` |  |
| `V4L2_SUBDEV_SEL_TGT_COMPOSE_BOUNDS` | `V4L2_SEL_TGT_COMPOSE_BOUNDS` |  |
| `V4L2_SUBDEV_SEL_FLAG_SIZE_GE` | `V4L2_SEL_FLAG_GE` |  |
| `V4L2_SUBDEV_SEL_FLAG_SIZE_LE` | `V4L2_SEL_FLAG_LE` |  |
| `V4L2_SUBDEV_SEL_FLAG_KEEP_CONFIG` | `V4L2_SEL_FLAG_KEEP_CONFIG` |  |

## Structs (1)


### `struct v4l2_edid`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pad` | `-` |
| `__u32` | `start_block` | `-` |
| `__u32` | `blocks` | `-` |
| `__u32` | `reserved` | `5` |