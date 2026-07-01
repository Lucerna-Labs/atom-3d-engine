# in_route.h

**Source:** `in_route.h`


## Defines (18 total)


### UNCATEGORIZED (18)

| Name | Value | Comment |
|------|-------|---------|
| `RTCF_DEAD` | `RTNH_F_DEAD` |  |
| `RTCF_ONLINK` | `RTNH_F_ONLINK` |  |
| `RTCF_NOPMTUDISC` | `RTM_F_NOPMTUDISC` |  |
| `RTCF_NOTIFY` | `0x00010000` |  |
| `RTCF_DIRECTDST` | `0x00020000` | unused |
| `RTCF_REDIRECTED` | `0x00040000` |  |
| `RTCF_TPROXY` | `0x00080000` | unused |
| `RTCF_FAST` | `0x00200000` | unused |
| `RTCF_MASQ` | `0x00400000` | unused |
| `RTCF_SNAT` | `0x00800000` | unused |
| `RTCF_DOREDIRECT` | `0x01000000` |  |
| `RTCF_DIRECTSRC` | `0x04000000` |  |
| `RTCF_DNAT` | `0x08000000` |  |
| `RTCF_BROADCAST` | `0x10000000` |  |
| `RTCF_MULTICAST` | `0x20000000` |  |
| `RTCF_REJECT` | `0x40000000` | unused |
| `RTCF_LOCAL` | `0x80000000` |  |
| `RTCF_NAT` | `(RTCF_DNAT\|RTCF_SNAT)` |  |