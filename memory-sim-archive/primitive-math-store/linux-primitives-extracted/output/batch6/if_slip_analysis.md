# if_slip.h

**Source:** `if_slip.h`


## Defines (11 total)


### SL_MODE (3)

| Name | Value | Comment |
|------|-------|---------|
| `SL_MODE_SLIP` | `0` |  |
| `SL_MODE_CSLIP` | `1` |  |
| `SL_MODE_KISS` | `4` |  |

### SL_OPT (2)

| Name | Value | Comment |
|------|-------|---------|
| `SL_OPT_SIXBIT` | `2` |  |
| `SL_OPT_ADAPTIVE` | `8` |  |

### UNCATEGORIZED (6)

| Name | Value | Comment |
|------|-------|---------|
| `SIOCSKEEPALIVE` | `(SIOCDEVPRIVATE)` | Set keepalive timeout in sec |
| `SIOCGKEEPALIVE` | `(SIOCDEVPRIVATE+1)` | Get keepalive timeout |
| `SIOCSOUTFILL` | `(SIOCDEVPRIVATE+2)` | Set outfill timeout |
| `SIOCGOUTFILL` | `(SIOCDEVPRIVATE+3)` | Get outfill timeout |
| `SIOCSLEASE` | `(SIOCDEVPRIVATE+4)` | Set "leased" line type |
| `SIOCGLEASE` | `(SIOCDEVPRIVATE+5)` | Get line type |