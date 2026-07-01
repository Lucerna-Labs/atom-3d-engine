# tcp.h

**Source:** `tcp.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`
- `linux/socket.h`

## Defines (83 total)


### TCPF_CA (5)

| Name | Value | Comment |
|------|-------|---------|
| `TCPF_CA_Open` | `(1<<TCP_CA_Open)` |  |
| `TCPF_CA_Disorder` | `(1<<TCP_CA_Disorder)` |  |
| `TCPF_CA_CWR` | `(1<<TCP_CA_CWR)` |  |
| `TCPF_CA_Recovery` | `(1<<TCP_CA_Recovery)` |  |
| `TCPF_CA_Loss` | `(1<<TCP_CA_Loss)` |  |

### TCPI_ECN (4)

| Name | Value | Comment |
|------|-------|---------|
| `TCPI_ECN_MODE_DISABLED` | `0x0` |  |
| `TCPI_ECN_MODE_RFC3168` | `0x1` |  |
| `TCPI_ECN_MODE_ACCECN` | `0x2` |  |
| `TCPI_ECN_MODE_PENDING` | `0x3` |  |

### TCPI_OPT (8)

| Name | Value | Comment |
|------|-------|---------|
| `TCPI_OPT_TIMESTAMPS` | `1` |  |
| `TCPI_OPT_SACK` | `2` |  |
| `TCPI_OPT_WSCALE` | `4` |  |
| `TCPI_OPT_ECN` | `8` | ECN was negociated at TCP session init |
| `TCPI_OPT_ECN_SEEN` | `16` | we received at least one packet with ECT |
| `TCPI_OPT_SYN_DATA` | `32` | SYN-ACK acked data in SYN sent or rcvd |
| `TCPI_OPT_USEC_TS` | `64` | usec timestamps |
| `TCPI_OPT_TFO_CHILD` | `128` | child from a Fast Open option on SYN |

### TCP_ACCECN (8)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_ACCECN_OPT_NOT_SEEN` | `0x0` |  |
| `TCP_ACCECN_OPT_EMPTY_SEEN` | `0x1` |  |
| `TCP_ACCECN_OPT_COUNTER_SEEN` | `0x2` |  |
| `TCP_ACCECN_OPT_FAIL_SEEN` | `0x3` |  |
| `TCP_ACCECN_ACE_FAIL_SEND` | `BIT(0)` |  |
| `TCP_ACCECN_ACE_FAIL_RECV` | `BIT(1)` |  |
| `TCP_ACCECN_OPT_FAIL_SEND` | `BIT(2)` |  |
| `TCP_ACCECN_OPT_FAIL_RECV` | `BIT(3)` |  |

### TCP_AO (8)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_AO_ADD_KEY` | `38` | Add/Set MKT |
| `TCP_AO_DEL_KEY` | `39` | Delete MKT |
| `TCP_AO_INFO` | `40` | Set/list TCP-AO per-socket options |
| `TCP_AO_GET_KEYS` | `41` | List MKT(s) |
| `TCP_AO_REPAIR` | `42` | Get/Set SNEs and ISNs |
| `TCP_AO_MAXKEYLEN` | `80` |  |
| `TCP_AO_KEYF_IFINDEX` | `(1 << 0)` | L3 ifindex for VRF |
| `TCP_AO_KEYF_EXCLUDE_OPT` | `(1 << 1)	/* "Indicates whether TCP` |  |

### TCP_CC (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_CC_INFO` | `26` | Get Congestion Control (optional) info |

### TCP_CM (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_CM_INQ` | `TCP_INQ` |  |

### TCP_DEFER (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_DEFER_ACCEPT` | `9` | Wake up listener only when data arrive |

### TCP_DELACK (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_DELACK_MAX_US` | `46` | max delayed ack time in us |

### TCP_FASTOPEN (3)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_FASTOPEN_CONNECT` | `30` | Attempt FastOpen with connect |
| `TCP_FASTOPEN_KEY` | `33` | Set the key for Fast Open (cookie) |
| `TCP_FASTOPEN_NO_COOKIE` | `34` | Enable TFO without a TFO cookie |

### TCP_IS (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_IS_MPTCP` | `43` | Is MPTCP being used? |

### TCP_MSS (2)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_MSS_DEFAULT` | `536U` | IPv4 (RFC1122, RFC2581) |
| `TCP_MSS_DESIRED` | `1220U` | IPv6 (tunneled), EDNS0 (RFC3226) |

### TCP_NOTSENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_NOTSENT_LOWAT` | `25` | limit number of unsent bytes in write queue |

### TCP_QUEUE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_QUEUE_SEQ` | `21` |  |

### TCP_RECEIVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_RECEIVE_ZEROCOPY_FLAG_TLB_CLEAN_HINT` | `0x1` |  |

### TCP_REPAIR (6)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_REPAIR_QUEUE` | `20` |  |
| `TCP_REPAIR_OPTIONS` | `22` |  |
| `TCP_REPAIR_WINDOW` | `29` | Get/set window parameters |
| `TCP_REPAIR_ON` | `1` |  |
| `TCP_REPAIR_OFF` | `0` |  |
| `TCP_REPAIR_OFF_NO_WP` | `-1` | Turn off without window probes |

### TCP_RTO (2)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_RTO_MAX_MS` | `44` | max rto time in ms |
| `TCP_RTO_MIN_US` | `45` | min rto time in us |

### TCP_SAVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_SAVE_SYN` | `27` | Record SYN headers for new connections |

### TCP_SAVED (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_SAVED_SYN` | `28` | Get SYN headers recorded for connection |

### TCP_THIN (2)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_THIN_LINEAR_TIMEOUTS` | `16` | Use linear timeouts for thin streams |
| `TCP_THIN_DUPACK` | `17` | Fast retrans. after 1 dupack |

### TCP_TX (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_TX_DELAY` | `37` | delay outgoing packets by XX usec |

### TCP_USER (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_USER_TIMEOUT` | `18` | How long for loss retry before timeout |

### TCP_WINDOW (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_WINDOW_CLAMP` | `10` | Bound advertised window |

### TCP_ZEROCOPY (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_ZEROCOPY_RECEIVE` | `35` |  |

### UNCATEGORIZED (21)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_NODELAY` | `1` | Turn off Nagle's algorithm. |
| `TCP_MAXSEG` | `2` | Limit MSS |
| `TCP_CORK` | `3` | Never send partially complete segments |
| `TCP_KEEPIDLE` | `4` | Start keeplives after this period |
| `TCP_KEEPINTVL` | `5` | Interval between keepalives |
| `TCP_KEEPCNT` | `6` | Number of keepalives before death |
| `TCP_SYNCNT` | `7` | Number of SYN retransmits |
| `TCP_LINGER2` | `8` | Life time of orphaned FIN-WAIT-2 state |
| `TCP_INFO` | `11` | Information about this connection. |
| `TCP_QUICKACK` | `12` | Block/reenable quick acks |
| `TCP_CONGESTION` | `13` | Congestion control algorithm |
| `TCP_MD5SIG` | `14` | TCP MD5 Signature (RFC2385) |
| `TCP_REPAIR` | `19` | TCP sock is under repair right now |
| `TCP_FASTOPEN` | `23` | Enable FastOpen on listeners |
| `TCP_TIMESTAMP` | `24` |  |
| `TCP_ULP` | `31` | Attach a ULP to a TCP connection |
| `TCP_MD5SIG_EXT` | `32` | TCP MD5 Signature with extensions |
| `TCP_INQ` | `36` | Notify bytes available to read as a cmsg on read |
| `TCP_MD5SIG_MAXKEYLEN` | `80` |  |
| `TCP_MD5SIG_FLAG_PREFIX` | `0x1` | address prefix length |
| `TCP_MD5SIG_FLAG_IFINDEX` | `0x2` | ifindex set |

## Structs (12)


### `struct tcphdr`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `source` | `-` |
| `__be16` | `dest` | `-` |
| `__be32` | `seq` | `-` |
| `__be32` | `ack_seq` | `-` |
| `__be16` | `window` | `-` |
| `__sum16` | `check` | `-` |
| `__be16` | `urg_ptr` | `-` |

### `struct tcp_repair_opt`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `opt_code` | `-` |
| `__u32` | `opt_val` | `-` |

### `struct tcp_repair_window`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `snd_wl1` | `-` |
| `__u32` | `snd_wnd` | `-` |
| `__u32` | `max_window` | `-` |
| `__u32` | `rcv_wnd` | `-` |
| `__u32` | `rcv_wup` | `-` |

### `struct tcp_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `tcpi_state` | `-` |
| `__u8` | `tcpi_ca_state` | `-` |
| `__u8` | `tcpi_retransmits` | `-` |
| `__u8` | `tcpi_probes` | `-` |
| `__u8` | `tcpi_backoff` | `-` |
| `__u8` | `tcpi_options` | `-` |
| `__u32` | `tcpi_rto` | `-` |
| `__u32` | `tcpi_ato` | `-` |
| `__u32` | `tcpi_snd_mss` | `-` |
| `__u32` | `tcpi_rcv_mss` | `-` |
| `__u32` | `tcpi_unacked` | `-` |
| `__u32` | `tcpi_sacked` | `-` |
| `__u32` | `tcpi_lost` | `-` |
| `__u32` | `tcpi_retrans` | `-` |
| `__u32` | `tcpi_fackets` | `-` |
| `__u32` | `tcpi_last_data_sent` | `-` |
| `__u32` | `tcpi_last_ack_sent` | `-` |
| `__u32` | `tcpi_last_data_recv` | `-` |
| `__u32` | `tcpi_last_ack_recv` | `-` |
| `__u32` | `tcpi_pmtu` | `-` |
| `__u32` | `tcpi_rcv_ssthresh` | `-` |
| `__u32` | `tcpi_rtt` | `-` |
| `__u32` | `tcpi_rttvar` | `-` |
| `__u32` | `tcpi_snd_ssthresh` | `-` |
| `__u32` | `tcpi_snd_cwnd` | `-` |
| `__u32` | `tcpi_advmss` | `-` |
| `__u32` | `tcpi_reordering` | `-` |
| `__u32` | `tcpi_rcv_rtt` | `-` |
| `__u32` | `tcpi_rcv_space` | `-` |
| `__u32` | `tcpi_total_retrans` | `-` |
| `__u64` | `tcpi_pacing_rate` | `-` |
| `__u64` | `tcpi_max_pacing_rate` | `-` |
| `__u64` | `tcpi_bytes_acked` | `-` |
| `__u64` | `tcpi_bytes_received` | `-` |
| `__u32` | `tcpi_segs_out` | `-` |
| `__u32` | `tcpi_segs_in` | `-` |
| `__u32` | `tcpi_notsent_bytes` | `-` |
| `__u32` | `tcpi_min_rtt` | `-` |
| `__u32` | `tcpi_data_segs_in` | `-` |
| `__u32` | `tcpi_data_segs_out` | `-` |
| `__u64` | `tcpi_delivery_rate` | `-` |
| `__u64` | `tcpi_busy_time` | `-` |
| `__u64` | `tcpi_rwnd_limited` | `-` |
| `__u64` | `tcpi_sndbuf_limited` | `-` |
| `__u32` | `tcpi_delivered` | `-` |
| `__u32` | `tcpi_delivered_ce` | `-` |
| `__u64` | `tcpi_bytes_sent` | `-` |
| `__u64` | `tcpi_bytes_retrans` | `-` |
| `__u32` | `tcpi_dsack_dups` | `-` |
| `__u32` | `tcpi_reord_seen` | `-` |
| `__u32` | `tcpi_rcv_ooopack` | `-` |
| `__u32` | `tcpi_snd_wnd` | `-` |
| `__u32` | `tcpi_rcv_wnd` | `-` |
| `__u32` | `tcpi_rehash` | `-` |
| `__u16` | `tcpi_total_rto` | `-` |
| `__u16` | `tcpi_total_rto_recoveries` | `-` |
| `__u32` | `tcpi_total_rto_time` | `-` |
| `__u32` | `tcpi_received_ce` | `-` |
| `__u32` | `tcpi_delivered_e1_bytes` | `-` |
| `__u32` | `tcpi_delivered_e0_bytes` | `-` |
| `__u32` | `tcpi_delivered_ce_bytes` | `-` |
| `__u32` | `tcpi_received_e1_bytes` | `-` |
| `__u32` | `tcpi_received_e0_bytes` | `-` |
| `__u32` | `tcpi_received_ce_bytes` | `-` |

### `struct tcp_md5sig`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `tcpm_flags` | `-` |
| `__u8` | `tcpm_prefixlen` | `-` |
| `__u16` | `tcpm_keylen` | `-` |
| `int` | `tcpm_ifindex` | `-` |
| `__u8` | `tcpm_key` | `TCP_MD5SIG_MAXKEYLEN` |

### `struct tcp_diag_md5sig`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `tcpm_family` | `-` |
| `__u8` | `tcpm_prefixlen` | `-` |
| `__u16` | `tcpm_keylen` | `-` |
| `__be32` | `tcpm_addr` | `4` |
| `__u8` | `tcpm_key` | `TCP_MD5SIG_MAXKEYLEN` |

### `struct tcp_ao_add`

| Type | Field | Array |
|------|-------|-------|
| `char` | `alg_name` | `64` |
| `__s32` | `ifindex` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u8` | `prefix` | `-` |
| `__u8` | `sndid` | `-` |
| `__u8` | `rcvid` | `-` |
| `__u8` | `maclen` | `-` |
| `__u8` | `keyflags` | `-` |
| `__u8` | `keylen` | `-` |
| `__u8` | `key` | `TCP_AO_MAXKEYLEN` |

### `struct tcp_ao_del`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `ifindex` | `-` |
| `__u16` | `reserved2` | `-` |
| `__u8` | `prefix` | `-` |
| `__u8` | `sndid` | `-` |
| `__u8` | `rcvid` | `-` |
| `__u8` | `current_key` | `-` |
| `__u8` | `rnext` | `-` |
| `__u8` | `keyflags` | `-` |

### `struct tcp_ao_info_opt`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `reserved2` | `-` |
| `__u8` | `current_key` | `-` |
| `__u8` | `rnext` | `-` |
| `__u64` | `pkt_good` | `-` |
| `__u64` | `pkt_bad` | `-` |
| `__u64` | `pkt_key_not_found` | `-` |
| `__u64` | `pkt_ao_required` | `-` |
| `__u64` | `pkt_dropped_icmp` | `-` |

### `struct tcp_ao_getsockopt`

| Type | Field | Array |
|------|-------|-------|
| `char` | `alg_name` | `64` |
| `__u8` | `key` | `TCP_AO_MAXKEYLEN` |
| `__u32` | `nkeys` | `-` |
| `__u8` | `sndid` | `-` |
| `__u8` | `rcvid` | `-` |
| `__u8` | `prefix` | `-` |
| `__u8` | `maclen` | `-` |
| `__u8` | `keyflags` | `-` |
| `__u8` | `keylen` | `-` |
| `__s32` | `ifindex` | `-` |
| `__u64` | `pkt_good` | `-` |
| `__u64` | `pkt_bad` | `-` |

### `struct tcp_ao_repair`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `snt_isn` | `-` |
| `__be32` | `rcv_isn` | `-` |
| `__u32` | `snd_sne` | `-` |
| `__u32` | `rcv_sne` | `-` |

### `struct tcp_zerocopy_receive`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `address` | `-` |
| `__u32` | `length` | `-` |
| `__u32` | `recv_skip_hint` | `-` |
| `__u32` | `inq` | `-` |
| `__s32` | `err` | `-` |
| `__u64` | `copybuf_address` | `-` |
| `__s32` | `copybuf_len` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `msg_control` | `-` |
| `__u64` | `msg_controllen` | `-` |
| `__u32` | `msg_flags` | `-` |
| `__u32` | `reserved` | `-` |