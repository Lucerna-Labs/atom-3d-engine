# binfmts.h

**Source:** `binfmts.h`


## Includes

- `linux/capability.h`

## Defines (5 total)


### AT_FLAGS (2)

| Name | Value | Comment |
|------|-------|---------|
| `AT_FLAGS_PRESERVE_ARGV0_BIT` | `0` |  |
| `AT_FLAGS_PRESERVE_ARGV0` | `(1 << AT_FLAGS_PRESERVE_ARGV0_BIT)` |  |

### BINPRM_BUF (1)

| Name | Value | Comment |
|------|-------|---------|
| `BINPRM_BUF_SIZE` | `256` |  |

### MAX_ARG (2)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_ARG_STRLEN` | `(PAGE_SIZE * 32)` |  |
| `MAX_ARG_STRINGS` | `0x7FFFFFFF` |  |