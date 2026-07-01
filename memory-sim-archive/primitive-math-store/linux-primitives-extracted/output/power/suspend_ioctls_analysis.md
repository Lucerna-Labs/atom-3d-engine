# suspend_ioctls.h

**Source:** `suspend_ioctls.h`


## Includes

- `linux/types.h`

## Defines (16 total)


### SNAPSHOT_ALLOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_ALLOC_SWAP_PAGE` | `_IOR(SNAPSHOT_IOC_MAGIC, 20, __kernel_loff_t)` |  |

### SNAPSHOT_ATOMIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_ATOMIC_RESTORE` | `_IO(SNAPSHOT_IOC_MAGIC, 4)` |  |

### SNAPSHOT_AVAIL (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_AVAIL_SWAP_SIZE` | `_IOR(SNAPSHOT_IOC_MAGIC, 19, __kernel_loff_t)` |  |

### SNAPSHOT_CREATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_CREATE_IMAGE` | `_IOW(SNAPSHOT_IOC_MAGIC, 17, int)` |  |

### SNAPSHOT_FREE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_FREE_SWAP_PAGES` | `_IO(SNAPSHOT_IOC_MAGIC, 9)` |  |

### SNAPSHOT_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_GET_IMAGE_SIZE` | `_IOR(SNAPSHOT_IOC_MAGIC, 14, __kernel_loff_t)` |  |

### SNAPSHOT_IOC (2)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_IOC_MAGIC` | `'3'` |  |
| `SNAPSHOT_IOC_MAXNR` | `20` |  |

### SNAPSHOT_PLATFORM (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_PLATFORM_SUPPORT` | `_IO(SNAPSHOT_IOC_MAGIC, 15)` |  |

### SNAPSHOT_POWER (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_POWER_OFF` | `_IO(SNAPSHOT_IOC_MAGIC, 16)` |  |

### SNAPSHOT_PREF (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_PREF_IMAGE_SIZE` | `_IO(SNAPSHOT_IOC_MAGIC, 18)` |  |

### SNAPSHOT_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_SET_SWAP_AREA` | `_IOW(SNAPSHOT_IOC_MAGIC, 13, ` |  |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `SNAPSHOT_FREEZE` | `_IO(SNAPSHOT_IOC_MAGIC, 1)` |  |
| `SNAPSHOT_UNFREEZE` | `_IO(SNAPSHOT_IOC_MAGIC, 2)` |  |
| `SNAPSHOT_FREE` | `_IO(SNAPSHOT_IOC_MAGIC, 5)` |  |
| `SNAPSHOT_S2RAM` | `_IO(SNAPSHOT_IOC_MAGIC, 11)` |  |

## Structs (1)


### `struct resume_swap_area`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_loff_t` | `offset` | `-` |
| `__u32` | `dev` | `-` |