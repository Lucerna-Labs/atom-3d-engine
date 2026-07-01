# socket.h

**Source:** `socket.h`


## Defines (7 total)


### SOCK_BUF (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_BUF_LOCK_MASK` | `(SOCK_SNDBUF_LOCK \| SOCK_RCVBUF_LOCK)` |  |

### SOCK_RCVBUF (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_RCVBUF_LOCK` | `2` |  |

### SOCK_SNDBUF (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_SNDBUF_LOCK` | `1` |  |

### SOCK_TXREHASH (3)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_TXREHASH_DEFAULT` | `255` |  |
| `SOCK_TXREHASH_DISABLED` | `0` |  |
| `SOCK_TXREHASH_ENABLED` | `1` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `_K_SS_MAXSIZE` | `128` | Implementation specific max size |

## Structs (2)


### `struct __kernel_sockaddr_storage`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `ss_family` | `-` |
| `char` | `__data` | `_K_SS_MAXSIZE - sizeof(unsigned short)` |

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `ss_family` | `-` |
| `char` | `__data` | `_K_SS_MAXSIZE - sizeof(unsigned short)` |