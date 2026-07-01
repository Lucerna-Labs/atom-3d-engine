# blkpg.h

**Source:** `blkpg.h`


## Includes

- `linux/compiler.h`
- `linux/ioctl.h`

## Defines (6 total)


### BLKPG_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `BLKPG_ADD_PARTITION` | `1` |  |

### BLKPG_DEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `BLKPG_DEL_PARTITION` | `2` |  |

### BLKPG_RESIZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BLKPG_RESIZE_PARTITION` | `3` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `BLKPG` | `_IO(0x12,105)` |  |
| `BLKPG_DEVNAMELTH` | `64` |  |
| `BLKPG_VOLNAMELTH` | `64` |  |

## Structs (2)


### `struct blkpg_ioctl_arg`

| Type | Field | Array |
|------|-------|-------|
| `int` | `op` | `-` |
| `int` | `flags` | `-` |
| `int` | `datalen` | `-` |

### `struct blkpg_partition`

| Type | Field | Array |
|------|-------|-------|
| `int` | `pno` | `-` |
| `char` | `devname` | `BLKPG_DEVNAMELTH` |
| `char` | `volname` | `BLKPG_VOLNAMELTH` |