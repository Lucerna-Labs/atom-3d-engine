# ppp_defs.h

**Source:** `ppp_defs.h`


## Includes

- `linux/types.h`

## Defines (33 total)


### PPP_MPLS (2)

| Name | Value | Comment |
|------|-------|---------|
| `PPP_MPLS_UC` | `0x0281` | Multi Protocol Label Switching - Unicast |
| `PPP_MPLS_MC` | `0x0283` | Multi Protocol Label Switching - Multicast |

### PPP_VJC (2)

| Name | Value | Comment |
|------|-------|---------|
| `PPP_VJC_COMP` | `0x2d` | VJ compressed TCP |
| `PPP_VJC_UNCOMP` | `0x2f` | VJ uncompressed TCP |

### UNCATEGORIZED (29)

| Name | Value | Comment |
|------|-------|---------|
| `PPP_HDRLEN` | `4` | octets for standard ppp header |
| `PPP_FCSLEN` | `2` | octets for FCS |
| `PPP_MRU` | `1500` | default MRU = max length of info field |
| `PPP_ALLSTATIONS` | `0xff` | All-Stations broadcast address |
| `PPP_UI` | `0x03` | Unnumbered Information |
| `PPP_FLAG` | `0x7e` | Flag Sequence |
| `PPP_ESCAPE` | `0x7d` | Asynchronous Control Escape |
| `PPP_TRANS` | `0x20` | Asynchronous transparency modifier |
| `PPP_IP` | `0x21` | Internet Protocol |
| `PPP_AT` | `0x29` | AppleTalk Protocol |
| `PPP_IPX` | `0x2b` | IPX protocol |
| `PPP_MP` | `0x3d` | Multilink protocol |
| `PPP_IPV6` | `0x57` | Internet Protocol Version 6 |
| `PPP_COMPFRAG` | `0xfb` | fragment compressed below bundle |
| `PPP_COMP` | `0xfd` | compressed packet |
| `PPP_IPCP` | `0x8021` | IP Control Protocol |
| `PPP_ATCP` | `0x8029` | AppleTalk Control Protocol |
| `PPP_IPXCP` | `0x802b` | IPX Control Protocol |
| `PPP_IPV6CP` | `0x8057` | IPv6 Control Protocol |
| `PPP_CCPFRAG` | `0x80fb` | CCP at link level (below MP bundle) |
| `PPP_CCP` | `0x80fd` | Compression Control Protocol |
| `PPP_MPLSCP` | `0x80fd` | MPLS Control Protocol |
| `PPP_LCP` | `0xc021` | Link Control Protocol |
| `PPP_PAP` | `0xc023` | Password Authentication Protocol |
| `PPP_LQR` | `0xc025` | Link Quality Report protocol |
| `PPP_CHAP` | `0xc223` | Cryptographic Handshake Auth. Protocol |
| `PPP_CBCP` | `0xc029` | Callback Control Protocol |
| `PPP_INITFCS` | `0xffff` | Initial FCS value |
| `PPP_GOODFCS` | `0xf0b8` | Good final FCS value |

## Structs (8)


### `struct pppstat`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ppp_discards` | `-` |
| `__u32` | `ppp_ibytes` | `-` |
| `__u32` | `ppp_ioctects` | `-` |
| `__u32` | `ppp_ipackets` | `-` |
| `__u32` | `ppp_ierrors` | `-` |
| `__u32` | `ppp_ilqrs` | `-` |
| `__u32` | `ppp_obytes` | `-` |
| `__u32` | `ppp_ooctects` | `-` |
| `__u32` | `ppp_opackets` | `-` |
| `__u32` | `ppp_oerrors` | `-` |
| `__u32` | `ppp_olqrs` | `-` |

### `struct vjstat`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vjs_packets` | `-` |
| `__u32` | `vjs_compressed` | `-` |
| `__u32` | `vjs_searches` | `-` |
| `__u32` | `vjs_misses` | `-` |
| `__u32` | `vjs_uncompressedin` | `-` |
| `__u32` | `vjs_compressedin` | `-` |
| `__u32` | `vjs_errorin` | `-` |
| `__u32` | `vjs_tossed` | `-` |

### `struct compstat`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `unc_bytes` | `-` |
| `__u32` | `unc_packets` | `-` |
| `__u32` | `comp_bytes` | `-` |
| `__u32` | `comp_packets` | `-` |
| `__u32` | `inc_bytes` | `-` |
| `__u32` | `inc_packets` | `-` |
| `__u32` | `in_count` | `-` |
| `__u32` | `bytes_out` | `-` |
| `double` | `ratio` | `-` |

### `struct ppp_stats`

| Type | Field | Array |
|------|-------|-------|

### `struct ppp_comp_stats`

| Type | Field | Array |
|------|-------|-------|

### `struct ppp_idle`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_old_time_t` | `xmit_idle` | `-` |
| `__kernel_old_time_t` | `recv_idle` | `-` |

### `struct ppp_idle32`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `xmit_idle` | `-` |
| `__s32` | `recv_idle` | `-` |

### `struct ppp_idle64`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `xmit_idle` | `-` |
| `__s64` | `recv_idle` | `-` |