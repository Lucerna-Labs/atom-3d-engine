# sock_diag.h

**Source:** `sock_diag.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### SK_DIAB (1)

| Name | Value | Comment |
|------|-------|---------|
| `SK_DIAB_BPF_STORAGE_REP_MAX` | `(__SK_DIAG_BPF_STORAGE_REP_MAX - 1)` |  |

### SK_DIAG (2)

| Name | Value | Comment |
|------|-------|---------|
| `SK_DIAG_BPF_STORAGE_REQ_MAX` | `(__SK_DIAG_BPF_STORAGE_REQ_MAX - 1)` |  |
| `SK_DIAG_BPF_STORAGE_MAX` | `(__SK_DIAG_BPF_STORAGE_MAX - 1)` |  |

### SOCK_DIAG (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_DIAG_BY_FAMILY` | `20` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_DESTROY` | `21` |  |
| `SKNLGRP_MAX` | `(__SKNLGRP_MAX - 1)` |  |

## Structs (1)


### `struct sock_diag_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |