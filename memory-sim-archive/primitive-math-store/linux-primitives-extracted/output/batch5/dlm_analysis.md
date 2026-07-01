# dlm.h

**Source:** `dlm.h`


## Includes

- `linux/dlmconstants.h`
- `linux/types.h`

## Defines (6 total)


### DLM_LSFL (2)

| Name | Value | Comment |
|------|-------|---------|
| `DLM_LSFL_TIMEWARN` | `0x00000002` |  |
| `DLM_LSFL_NEWEXCL` | `0x00000008` |  |

### DLM_SBF (3)

| Name | Value | Comment |
|------|-------|---------|
| `DLM_SBF_DEMOTED` | `0x01` |  |
| `DLM_SBF_VALNOTVALID` | `0x02` |  |
| `DLM_SBF_ALTMODE` | `0x04` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `__DLM_LSFL_RESERVED0` | `0x00000010` |  |

## Structs (1)


### `struct dlm_lksb`

| Type | Field | Array |
|------|-------|-------|
| `int` | `sb_status` | `-` |
| `__u32` | `sb_lkid` | `-` |
| `char` | `sb_flags` | `-` |
| `char *` | `sb_lvbptr` | `-` |