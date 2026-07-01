# mqueue.h

**Source:** `mqueue.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### MQ_BYTES (1)

| Name | Value | Comment |
|------|-------|---------|
| `MQ_BYTES_MAX` | `819200` |  |

### MQ_PRIO (1)

| Name | Value | Comment |
|------|-------|---------|
| `MQ_PRIO_MAX` | `32768` |  |

### NOTIFY_COOKIE (1)

| Name | Value | Comment |
|------|-------|---------|
| `NOTIFY_COOKIE_LEN` | `32` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `NOTIFY_NONE` | `0` |  |
| `NOTIFY_WOKENUP` | `1` |  |
| `NOTIFY_REMOVED` | `2` |  |

## Structs (1)


### `struct mq_attr`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_long_t` | `mq_flags` | `-` |
| `__kernel_long_t` | `mq_maxmsg` | `-` |
| `__kernel_long_t` | `mq_msgsize` | `-` |
| `__kernel_long_t` | `mq_curmsgs` | `-` |
| `__kernel_long_t` | `__reserved` | `4` |