# inet_diag.h

**Source:** `inet_diag.h`


## Includes

- `linux/types.h`

## Defines (7 total)


### INET_DIAG (4)

| Name | Value | Comment |
|------|-------|---------|
| `INET_DIAG_GETSOCK_MAX` | `24` |  |
| `INET_DIAG_NOCOOKIE` | `(~0U)` |  |
| `INET_DIAG_REQ_MAX` | `(__INET_DIAG_REQ_MAX - 1)` |  |
| `INET_DIAG_MAX` | `(__INET_DIAG_MAX - 1)` |  |

### INET_ULP (1)

| Name | Value | Comment |
|------|-------|---------|
| `INET_ULP_INFO_MAX` | `(__INET_ULP_INFO_MAX - 1)` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `TCPDIAG_GETSOCK` | `18` |  |
| `DCCPDIAG_GETSOCK` | `19` |  |

## Structs (13)


### `struct inet_diag_sockid`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `idiag_sport` | `-` |
| `__be16` | `idiag_dport` | `-` |
| `__be32` | `idiag_src` | `4` |
| `__be32` | `idiag_dst` | `4` |
| `__u32` | `idiag_if` | `-` |
| `__u32` | `idiag_cookie` | `2` |

### `struct inet_diag_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `idiag_family` | `-` |
| `__u8` | `idiag_src_len` | `-` |
| `__u8` | `idiag_dst_len` | `-` |
| `__u8` | `idiag_ext` | `-` |
| `__u32` | `idiag_states` | `-` |
| `__u32` | `idiag_dbs` | `-` |

### `struct inet_diag_req_v2`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |
| `__u8` | `idiag_ext` | `-` |
| `__u8` | `pad` | `-` |
| `__u32` | `idiag_states` | `-` |

### `struct inet_diag_req_raw`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |
| `__u8` | `idiag_ext` | `-` |
| `__u8` | `sdiag_raw_protocol` | `-` |
| `__u32` | `idiag_states` | `-` |

### `struct inet_diag_bc_op`

| Type | Field | Array |
|------|-------|-------|

### `struct inet_diag_hostcond`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `family` | `-` |
| `__u8` | `prefix_len` | `-` |
| `int` | `port` | `-` |

### `struct inet_diag_markcond`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mark` | `-` |
| `__u32` | `mask` | `-` |

### `struct inet_diag_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `idiag_family` | `-` |
| `__u8` | `idiag_state` | `-` |
| `__u8` | `idiag_timer` | `-` |
| `__u8` | `idiag_retrans` | `-` |
| `__u32` | `idiag_expires` | `-` |
| `__u32` | `idiag_rqueue` | `-` |
| `__u32` | `idiag_wqueue` | `-` |
| `__u32` | `idiag_uid` | `-` |
| `__u32` | `idiag_inode` | `-` |

### `struct inet_diag_meminfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `idiag_rmem` | `-` |
| `__u32` | `idiag_wmem` | `-` |
| `__u32` | `idiag_fmem` | `-` |
| `__u32` | `idiag_tmem` | `-` |

### `struct inet_diag_sockopt`

| Type | Field | Array |
|------|-------|-------|

### `struct tcpvegas_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tcpv_enabled` | `-` |
| `__u32` | `tcpv_rttcnt` | `-` |
| `__u32` | `tcpv_rtt` | `-` |
| `__u32` | `tcpv_minrtt` | `-` |

### `struct tcp_dctcp_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `dctcp_enabled` | `-` |
| `__u16` | `dctcp_ce_state` | `-` |
| `__u32` | `dctcp_alpha` | `-` |
| `__u32` | `dctcp_ab_ecn` | `-` |
| `__u32` | `dctcp_ab_tot` | `-` |

### `struct tcp_bbr_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bbr_bw_lo` | `-` |
| `__u32` | `bbr_bw_hi` | `-` |
| `__u32` | `bbr_min_rtt` | `-` |
| `__u32` | `bbr_pacing_gain` | `-` |
| `__u32` | `bbr_cwnd_gain` | `-` |