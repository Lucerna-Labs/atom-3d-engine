# eventpoll.h

**Source:** `eventpoll.h`


## Includes

- `linux/fcntl.h`
- `linux/types.h`

## Defines (25 total)


### EPOLL_CTL (3)

| Name | Value | Comment |
|------|-------|---------|
| `EPOLL_CTL_ADD` | `1` |  |
| `EPOLL_CTL_DEL` | `2` |  |
| `EPOLL_CTL_MOD` | `3` |  |

### EPOLL_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `EPOLL_IOC_TYPE` | `0x8A` |  |

### EPOLL_URING (1)

| Name | Value | Comment |
|------|-------|---------|
| `EPOLL_URING_WAKE` | `((__force __poll_t)(1U << 27))` |  |

### UNCATEGORIZED (20)

| Name | Value | Comment |
|------|-------|---------|
| `EPOLL_CLOEXEC` | `O_CLOEXEC` |  |
| `EPOLLIN` | `(__force __poll_t)0x00000001` |  |
| `EPOLLPRI` | `(__force __poll_t)0x00000002` |  |
| `EPOLLOUT` | `(__force __poll_t)0x00000004` |  |
| `EPOLLERR` | `(__force __poll_t)0x00000008` |  |
| `EPOLLHUP` | `(__force __poll_t)0x00000010` |  |
| `EPOLLNVAL` | `(__force __poll_t)0x00000020` |  |
| `EPOLLRDNORM` | `(__force __poll_t)0x00000040` |  |
| `EPOLLRDBAND` | `(__force __poll_t)0x00000080` |  |
| `EPOLLWRNORM` | `(__force __poll_t)0x00000100` |  |
| `EPOLLWRBAND` | `(__force __poll_t)0x00000200` |  |
| `EPOLLMSG` | `(__force __poll_t)0x00000400` |  |
| `EPOLLRDHUP` | `(__force __poll_t)0x00002000` |  |
| `EPOLLEXCLUSIVE` | `((__force __poll_t)(1U << 28))` |  |
| `EPOLLWAKEUP` | `((__force __poll_t)(1U << 29))` |  |
| `EPOLLONESHOT` | `((__force __poll_t)(1U << 30))` |  |
| `EPOLLET` | `((__force __poll_t)(1U << 31))` |  |
| `EPOLL_PACKED` | `__attribute__((packed))` |  |
| `EPIOCSPARAMS` | `_IOW(EPOLL_IOC_TYPE, 0x01, struct epoll_params)` |  |
| `EPIOCGPARAMS` | `_IOR(EPOLL_IOC_TYPE, 0x02, struct epoll_params)` |  |

## Structs (2)


### `struct epoll_event`

| Type | Field | Array |
|------|-------|-------|
| `__poll_t` | `events` | `-` |
| `__u64` | `data` | `-` |

### `struct epoll_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `busy_poll_usecs` | `-` |
| `__u16` | `busy_poll_budget` | `-` |
| `__u8` | `prefer_busy_poll` | `-` |
| `__u8` | `__pad` | `-` |