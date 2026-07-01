# timerfd.h

**Source:** `timerfd.h`


## Includes

- `linux/types.h`
- `linux/fcntl.h`
- `linux/ioctl.h`

## Defines (5 total)


### TFD_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `TFD_IOC_SET_TICKS` | `_IOW('T', 0, __u64)` |  |

### TFD_TIMER (2)

| Name | Value | Comment |
|------|-------|---------|
| `TFD_TIMER_ABSTIME` | `(1 << 0)` |  |
| `TFD_TIMER_CANCEL_ON_SET` | `(1 << 1)` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `TFD_CLOEXEC` | `O_CLOEXEC` |  |
| `TFD_NONBLOCK` | `O_NONBLOCK` |  |