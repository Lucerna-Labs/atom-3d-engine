# ppp-ioctl.h

**Source:** `ppp-ioctl.h`


## Includes

- `linux/types.h`
- `linux/compiler.h`
- `linux/ppp_defs.h`

## Defines (67 total)


### SC_CCP (2)

| Name | Value | Comment |
|------|-------|---------|
| `SC_CCP_OPEN` | `0x00000040` | Look at CCP packets |
| `SC_CCP_UP` | `0x00000080` | May send/recv compressed packets |

### SC_COMP (4)

| Name | Value | Comment |
|------|-------|---------|
| `SC_COMP_PROT` | `0x00000001` | protocol compression (output) |
| `SC_COMP_AC` | `0x00000002` | header compression (output) |
| `SC_COMP_TCP` | `0x00000004` | TCP (VJ) compression (output) |
| `SC_COMP_RUN` | `0x00001000` | compressor has been inited |

### SC_DC (2)

| Name | Value | Comment |
|------|-------|---------|
| `SC_DC_FERROR` | `0x00800000` | fatal decomp error detected |
| `SC_DC_ERROR` | `0x00400000` | non-fatal decomp error detected |

### SC_DECOMP (1)

| Name | Value | Comment |
|------|-------|---------|
| `SC_DECOMP_RUN` | `0x00002000` | decompressor has been inited |

### SC_ENABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SC_ENABLE_IP` | `0x00000100` | IP packets may be exchanged |

### SC_LOG (4)

| Name | Value | Comment |
|------|-------|---------|
| `SC_LOG_INPKT` | `0x00020000` | log contents of good pkts recvd |
| `SC_LOG_OUTPKT` | `0x00040000` | log contents of pkts sent |
| `SC_LOG_RAWIN` | `0x00080000` | log all chars received |
| `SC_LOG_FLUSH` | `0x00100000` | log all chars flushed |

### SC_LOOP (1)

| Name | Value | Comment |
|------|-------|---------|
| `SC_LOOP_TRAFFIC` | `0x00000200` | send traffic to pppd |

### SC_MP (2)

| Name | Value | Comment |
|------|-------|---------|
| `SC_MP_SHORTSEQ` | `0x00000800` | use short MP sequence numbers |
| `SC_MP_XSHORTSEQ` | `0x00004000` | transmit short MP seq numbers |

### SC_MUST (1)

| Name | Value | Comment |
|------|-------|---------|
| `SC_MUST_COMP` | `0x00400000` | no uncompressed packets may be sent or received |

### SC_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `SC_NO_TCP_CCID` | `0x00000008` | disable VJ connection-id comp. |

### SC_RCV (4)

| Name | Value | Comment |
|------|-------|---------|
| `SC_RCV_ODDP` | `0x08000000` | have rcvd char with odd parity |
| `SC_RCV_EVNP` | `0x04000000` | have rcvd char with even parity |
| `SC_RCV_B7_1` | `0x02000000` | have rcvd char with bit 7 = 1 |
| `SC_RCV_B7_0` | `0x01000000` | have rcvd char with bit 7 = 0 |

### SC_REJ (2)

| Name | Value | Comment |
|------|-------|---------|
| `SC_REJ_COMP_AC` | `0x00000010` | reject adrs/ctrl comp. on input |
| `SC_REJ_COMP_TCP` | `0x00000020` | reject TCP (VJ) comp. on input |

### SC_XMIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `SC_XMIT_BUSY` | `0x10000000` | (used by isdn_ppp?) |

### UNCATEGORIZED (41)

| Name | Value | Comment |
|------|-------|---------|
| `SC_MULTILINK` | `0x00000400` | do multilink encapsulation |
| `SC_DEBUG` | `0x00010000` | enable debug messages |
| `SC_SYNC` | `0x00200000` | synchronous serial mode |
| `SC_MASK` | `0x0f600fff` | bits that user can change |
| `PPPIOCGFLAGS` | `_IOR('t', 90, int)` | get configuration flags |
| `PPPIOCSFLAGS` | `_IOW('t', 89, int)` | set configuration flags |
| `PPPIOCGASYNCMAP` | `_IOR('t', 88, int)` | get async map |
| `PPPIOCSASYNCMAP` | `_IOW('t', 87, int)` | set async map |
| `PPPIOCGUNIT` | `_IOR('t', 86, int)` | get ppp unit number |
| `PPPIOCGRASYNCMAP` | `_IOR('t', 85, int)` | get receive async map |
| `PPPIOCSRASYNCMAP` | `_IOW('t', 84, int)` | set receive async map |
| `PPPIOCGMRU` | `_IOR('t', 83, int)` | get max receive unit |
| `PPPIOCSMRU` | `_IOW('t', 82, int)` | set max receive unit |
| `PPPIOCSMAXCID` | `_IOW('t', 81, int)` | set VJ max slot ID |
| `PPPIOCGXASYNCMAP` | `_IOR('t', 80, ext_accm)` | get extended ACCM |
| `PPPIOCSXASYNCMAP` | `_IOW('t', 79, ext_accm)` | set extended ACCM |
| `PPPIOCXFERUNIT` | `_IO('t', 78)` | transfer PPP unit |
| `PPPIOCSCOMPRESS` | `_IOW('t', 77, struct ppp_option_data)` |  |
| `PPPIOCGNPMODE` | `_IOWR('t', 76, struct npioctl)` | get NP mode |
| `PPPIOCSNPMODE` | `_IOW('t', 75, struct npioctl)` | set NP mode |
| `PPPIOCSPASS` | `_IOW('t', 71, struct sock_fprog)` | set pass filter |
| `PPPIOCSACTIVE` | `_IOW('t', 70, struct sock_fprog)` | set active filt |
| `PPPIOCGDEBUG` | `_IOR('t', 65, int)` | Read debug level |
| `PPPIOCSDEBUG` | `_IOW('t', 64, int)` | Set debug level |
| `PPPIOCGIDLE` | `_IOR('t', 63, struct ppp_idle)` | get idle time |
| `PPPIOCGIDLE32` | `_IOR('t', 63, struct ppp_idle32)` | 32-bit times |
| `PPPIOCGIDLE64` | `_IOR('t', 63, struct ppp_idle64)` | 64-bit times |
| `PPPIOCNEWUNIT` | `_IOWR('t', 62, int)` | create new ppp unit |
| `PPPIOCATTACH` | `_IOW('t', 61, int)` | attach to ppp unit |
| `PPPIOCDETACH` | `_IOW('t', 60, int)` | obsolete, do not use |
| `PPPIOCSMRRU` | `_IOW('t', 59, int)` | set multilink MRU |
| `PPPIOCCONNECT` | `_IOW('t', 58, int)` | connect channel to unit |
| `PPPIOCDISCONN` | `_IO('t', 57)` | disconnect channel |
| `PPPIOCATTCHAN` | `_IOW('t', 56, int)` | attach to ppp channel |
| `PPPIOCGCHAN` | `_IOR('t', 55, int)` | get ppp channel number |
| `PPPIOCGL2TPSTATS` | `_IOR('t', 54, struct pppol2tp_ioc_stats)` |  |
| `PPPIOCBRIDGECHAN` | `_IOW('t', 53, int)` | bridge one channel to another |
| `PPPIOCUNBRIDGECHAN` | `_IO('t', 52)` | unbridge channel |
| `SIOCGPPPSTATS` | `(SIOCDEVPRIVATE + 0)` |  |
| `SIOCGPPPVER` | `(SIOCDEVPRIVATE + 1)` | NEVER change this!! |
| `SIOCGPPPCSTATS` | `(SIOCDEVPRIVATE + 2)` |  |

## Structs (3)


### `struct npioctl`

| Type | Field | Array |
|------|-------|-------|
| `int` | `protocol` | `-` |

### `struct ppp_option_data`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `length` | `-` |
| `int` | `transmit` | `-` |

### `struct pppol2tp_ioc_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `tunnel_id` | `-` |
| `__u16` | `session_id` | `-` |
| `__aligned_u64` | `tx_packets` | `-` |
| `__aligned_u64` | `tx_bytes` | `-` |
| `__aligned_u64` | `tx_errors` | `-` |
| `__aligned_u64` | `rx_packets` | `-` |
| `__aligned_u64` | `rx_bytes` | `-` |
| `__aligned_u64` | `rx_seq_discards` | `-` |
| `__aligned_u64` | `rx_oos_packets` | `-` |
| `__aligned_u64` | `rx_errors` | `-` |