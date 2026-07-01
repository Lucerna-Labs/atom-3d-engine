# futex.h

**Source:** `futex.h`


## Includes

- `linux/compiler.h`
- `linux/types.h`

## Defines (58 total)


### FUTEX_BITSET (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_BITSET_MATCH_ANY` | `0xffffffff` |  |

### FUTEX_CLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_CLOCK_REALTIME` | `256` |  |

### FUTEX_CMD (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_CMD_MASK` | `~(FUTEX_PRIVATE_FLAG \| FUTEX_CLOCK_REALTIME)` |  |

### FUTEX_CMP (4)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_CMP_REQUEUE` | `4` |  |
| `FUTEX_CMP_REQUEUE_PI` | `12` |  |
| `FUTEX_CMP_REQUEUE_PRIVATE` | `(FUTEX_CMP_REQUEUE \| FUTEX_PRIVATE_FLAG)` |  |
| `FUTEX_CMP_REQUEUE_PI_PRIVATE` | `(FUTEX_CMP_REQUEUE_PI \| ` |  |

### FUTEX_LOCK (4)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_LOCK_PI` | `6` |  |
| `FUTEX_LOCK_PI2` | `13` |  |
| `FUTEX_LOCK_PI_PRIVATE` | `(FUTEX_LOCK_PI \| FUTEX_PRIVATE_FLAG)` |  |
| `FUTEX_LOCK_PI2_PRIVATE` | `(FUTEX_LOCK_PI2 \| FUTEX_PRIVATE_FLAG)` |  |

### FUTEX_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_NO_NODE` | `(-1)` |  |

### FUTEX_OP (12)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_OP_SET` | `0` | *(int *)UADDR2 = OPARG; |
| `FUTEX_OP_ADD` | `1` | *(int *)UADDR2 += OPARG; |
| `FUTEX_OP_OR` | `2` | *(int *)UADDR2 |= OPARG; |
| `FUTEX_OP_ANDN` | `3` | *(int *)UADDR2 &= ~OPARG; |
| `FUTEX_OP_XOR` | `4` | *(int *)UADDR2 ^= OPARG; |
| `FUTEX_OP_OPARG_SHIFT` | `8` | Use (1 << OPARG) instead of OPARG. |
| `FUTEX_OP_CMP_EQ` | `0` | if (oldval == CMPARG) wake |
| `FUTEX_OP_CMP_NE` | `1` | if (oldval != CMPARG) wake |
| `FUTEX_OP_CMP_LT` | `2` | if (oldval < CMPARG) wake |
| `FUTEX_OP_CMP_LE` | `3` | if (oldval <= CMPARG) wake |
| `FUTEX_OP_CMP_GT` | `4` | if (oldval > CMPARG) wake |
| `FUTEX_OP_CMP_GE` | `5` | if (oldval >= CMPARG) wake |

### FUTEX_OWNER (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_OWNER_DIED` | `0x40000000` |  |

### FUTEX_PRIVATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_PRIVATE_FLAG` | `128` |  |

### FUTEX_REQUEUE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_REQUEUE_PRIVATE` | `(FUTEX_REQUEUE \| FUTEX_PRIVATE_FLAG)` |  |

### FUTEX_TID (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_TID_MASK` | `0x3fffffff` |  |

### FUTEX_TRYLOCK (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_TRYLOCK_PI` | `8` |  |
| `FUTEX_TRYLOCK_PI_PRIVATE` | `(FUTEX_TRYLOCK_PI \| FUTEX_PRIVATE_FLAG)` |  |

### FUTEX_UNLOCK (2)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_UNLOCK_PI` | `7` |  |
| `FUTEX_UNLOCK_PI_PRIVATE` | `(FUTEX_UNLOCK_PI \| FUTEX_PRIVATE_FLAG)` |  |

### FUTEX_WAIT (5)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_WAIT_BITSET` | `9` |  |
| `FUTEX_WAIT_REQUEUE_PI` | `11` |  |
| `FUTEX_WAIT_PRIVATE` | `(FUTEX_WAIT \| FUTEX_PRIVATE_FLAG)` |  |
| `FUTEX_WAIT_BITSET_PRIVATE` | `(FUTEX_WAIT_BITSET \| FUTEX_PRIVATE_FLAG)` |  |
| `FUTEX_WAIT_REQUEUE_PI_PRIVATE` | `(FUTEX_WAIT_REQUEUE_PI \| ` |  |

### FUTEX_WAITV (1)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_WAITV_MAX` | `128` |  |

### FUTEX_WAKE (5)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_WAKE_OP` | `5` |  |
| `FUTEX_WAKE_BITSET` | `10` |  |
| `FUTEX_WAKE_PRIVATE` | `(FUTEX_WAKE \| FUTEX_PRIVATE_FLAG)` |  |
| `FUTEX_WAKE_OP_PRIVATE` | `(FUTEX_WAKE_OP \| FUTEX_PRIVATE_FLAG)` |  |
| `FUTEX_WAKE_BITSET_PRIVATE` | `(FUTEX_WAKE_BITSET \| FUTEX_PRIVATE_FLAG)` |  |

### ROBUST_LIST (1)

| Name | Value | Comment |
|------|-------|---------|
| `ROBUST_LIST_LIMIT` | `2048` |  |

### UNCATEGORIZED (14)

| Name | Value | Comment |
|------|-------|---------|
| `FUTEX_WAIT` | `0` |  |
| `FUTEX_WAKE` | `1` |  |
| `FUTEX_FD` | `2` |  |
| `FUTEX_REQUEUE` | `3` |  |
| `FUTEX2_SIZE_U8` | `0x00` |  |
| `FUTEX2_SIZE_U16` | `0x01` |  |
| `FUTEX2_SIZE_U32` | `0x02` |  |
| `FUTEX2_SIZE_U64` | `0x03` |  |
| `FUTEX2_NUMA` | `0x04` |  |
| `FUTEX2_MPOL` | `0x08` |  |
| `FUTEX2_PRIVATE` | `FUTEX_PRIVATE_FLAG` |  |
| `FUTEX2_SIZE_MASK` | `0x03` |  |
| `FUTEX_32` | `FUTEX2_SIZE_U32` | historical accident :-( |
| `FUTEX_WAITERS` | `0x80000000` |  |

## Structs (4)


### `struct anonymous_0`

| Type | Field | Array |
|------|-------|-------|

### `struct futex_waitv`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `val` | `-` |
| `__u64` | `uaddr` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `__reserved` | `-` |

### `struct robust_list`

| Type | Field | Array |
|------|-------|-------|

### `struct robust_list_head`

| Type | Field | Array |
|------|-------|-------|
| `long` | `futex_offset` | `-` |