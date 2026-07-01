# kcm.h

**Source:** `kcm.h`


## Defines (5 total)


### KCM_RECV (1)

| Name | Value | Comment |
|------|-------|---------|
| `KCM_RECV_DISABLE` | `1` |  |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `SIOCKCMATTACH` | `(SIOCPROTOPRIVATE + 0)` |  |
| `SIOCKCMUNATTACH` | `(SIOCPROTOPRIVATE + 1)` |  |
| `SIOCKCMCLONE` | `(SIOCPROTOPRIVATE + 2)` |  |
| `KCMPROTO_CONNECTED` | `0` |  |

## Structs (3)


### `struct kcm_attach`

| Type | Field | Array |
|------|-------|-------|
| `int` | `fd` | `-` |
| `int` | `bpf_fd` | `-` |

### `struct kcm_unattach`

| Type | Field | Array |
|------|-------|-------|
| `int` | `fd` | `-` |

### `struct kcm_clone`

| Type | Field | Array |
|------|-------|-------|
| `int` | `fd` | `-` |