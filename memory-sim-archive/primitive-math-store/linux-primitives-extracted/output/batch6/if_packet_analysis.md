# if_packet.h

**Source:** `if_packet.h`


## Includes

- `asm/byteorder.h`
- `linux/types.h`

## Defines (70 total)


### PACKET_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_ADD_MEMBERSHIP` | `1` |  |

### PACKET_COPY (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_COPY_THRESH` | `7` |  |

### PACKET_DROP (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_DROP_MEMBERSHIP` | `2` |  |

### PACKET_FANOUT (13)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_FANOUT_DATA` | `22` |  |
| `PACKET_FANOUT_HASH` | `0` |  |
| `PACKET_FANOUT_LB` | `1` |  |
| `PACKET_FANOUT_CPU` | `2` |  |
| `PACKET_FANOUT_ROLLOVER` | `3` |  |
| `PACKET_FANOUT_RND` | `4` |  |
| `PACKET_FANOUT_QM` | `5` |  |
| `PACKET_FANOUT_CBPF` | `6` |  |
| `PACKET_FANOUT_EBPF` | `7` |  |
| `PACKET_FANOUT_FLAG_ROLLOVER` | `0x1000` |  |
| `PACKET_FANOUT_FLAG_UNIQUEID` | `0x2000` |  |
| `PACKET_FANOUT_FLAG_IGNORE_OUTGOING` | `0x4000` |  |
| `PACKET_FANOUT_FLAG_DEFRAG` | `0x8000` |  |

### PACKET_IGNORE (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_IGNORE_OUTGOING` | `23` |  |

### PACKET_MR (4)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_MR_MULTICAST` | `0` |  |
| `PACKET_MR_PROMISC` | `1` |  |
| `PACKET_MR_ALLMULTI` | `2` |  |
| `PACKET_MR_UNICAST` | `3` |  |

### PACKET_QDISC (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_QDISC_BYPASS` | `20` |  |

### PACKET_RECV (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_RECV_OUTPUT` | `3` |  |

### PACKET_ROLLOVER (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_ROLLOVER_STATS` | `21` |  |

### PACKET_RX (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_RX_RING` | `5` |  |

### PACKET_TX (3)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_TX_RING` | `13` |  |
| `PACKET_TX_TIMESTAMP` | `16` |  |
| `PACKET_TX_HAS_OFF` | `19` |  |

### PACKET_VNET (2)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_VNET_HDR` | `15` |  |
| `PACKET_VNET_HDR_SZ` | `24` |  |

### TP_FT (1)

| Name | Value | Comment |
|------|-------|---------|
| `TP_FT_REQ_FILL_RXHASH` | `0x1` |  |

### TP_STATUS (17)

| Name | Value | Comment |
|------|-------|---------|
| `TP_STATUS_KERNEL` | `0` |  |
| `TP_STATUS_USER` | `(1 << 0)` |  |
| `TP_STATUS_COPY` | `(1 << 1)` |  |
| `TP_STATUS_LOSING` | `(1 << 2)` |  |
| `TP_STATUS_CSUMNOTREADY` | `(1 << 3)` |  |
| `TP_STATUS_VLAN_VALID` | `(1 << 4)` | auxdata has valid tp_vlan_tci |
| `TP_STATUS_BLK_TMO` | `(1 << 5)` |  |
| `TP_STATUS_VLAN_TPID_VALID` | `(1 << 6)` | auxdata has valid tp_vlan_tpid |
| `TP_STATUS_CSUM_VALID` | `(1 << 7)` |  |
| `TP_STATUS_GSO_TCP` | `(1 << 8)` |  |
| `TP_STATUS_AVAILABLE` | `0` |  |
| `TP_STATUS_SEND_REQUEST` | `(1 << 0)` |  |
| `TP_STATUS_SENDING` | `(1 << 1)` |  |
| `TP_STATUS_WRONG_FORMAT` | `(1 << 2)` |  |
| `TP_STATUS_TS_SOFTWARE` | `(1 << 29)` |  |
| `TP_STATUS_TS_SYS_HARDWARE` | `(1 << 30)` | deprecated, never set |
| `TP_STATUS_TS_RAW_HARDWARE` | `(1U << 31)` |  |

### UNCATEGORIZED (22)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_HOST` | `0` | To us |
| `PACKET_BROADCAST` | `1` | To all |
| `PACKET_MULTICAST` | `2` | To group |
| `PACKET_OTHERHOST` | `3` | To someone else |
| `PACKET_OUTGOING` | `4` | Outgoing of any type |
| `PACKET_LOOPBACK` | `5` | MC/BRD frame looped back |
| `PACKET_USER` | `6` | To user space |
| `PACKET_KERNEL` | `7` | To kernel space |
| `PACKET_FASTROUTE` | `6` | Fastrouted frame |
| `PACKET_STATISTICS` | `6` |  |
| `PACKET_AUXDATA` | `8` |  |
| `PACKET_ORIGDEV` | `9` |  |
| `PACKET_VERSION` | `10` |  |
| `PACKET_HDRLEN` | `11` |  |
| `PACKET_RESERVE` | `12` |  |
| `PACKET_LOSS` | `14` |  |
| `PACKET_TIMESTAMP` | `17` |  |
| `PACKET_FANOUT` | `18` |  |
| `TPACKET_ALIGNMENT` | `16` |  |
| `TPACKET_HDRLEN` | `(TPACKET_ALIGN(sizeof(struct tpacket_hdr)) + sizeof(struct s` |  |
| `TPACKET2_HDRLEN` | `(TPACKET_ALIGN(sizeof(struct tpacket2_hdr)) + sizeof(struct ` |  |
| `TPACKET3_HDRLEN` | `(TPACKET_ALIGN(sizeof(struct tpacket3_hdr)) + sizeof(struct ` |  |

## Structs (17)


### `struct sockaddr_pkt`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `spkt_protocol` | `-` |

### `struct sockaddr_ll`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `sll_protocol` | `-` |
| `int` | `sll_ifindex` | `-` |

### `struct tpacket_stats`

| Type | Field | Array |
|------|-------|-------|

### `struct tpacket_stats_v3`

| Type | Field | Array |
|------|-------|-------|

### `struct tpacket_rollover_stats`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `tp_all` | `-` |
| `__aligned_u64` | `tp_huge` | `-` |
| `__aligned_u64` | `tp_failed` | `-` |

### `struct tpacket_auxdata`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tp_status` | `-` |
| `__u32` | `tp_len` | `-` |
| `__u32` | `tp_snaplen` | `-` |
| `__u16` | `tp_mac` | `-` |
| `__u16` | `tp_net` | `-` |
| `__u16` | `tp_vlan_tci` | `-` |
| `__u16` | `tp_vlan_tpid` | `-` |

### `struct tpacket_hdr`

| Type | Field | Array |
|------|-------|-------|

### `struct tpacket2_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tp_status` | `-` |
| `__u32` | `tp_len` | `-` |
| `__u32` | `tp_snaplen` | `-` |
| `__u16` | `tp_mac` | `-` |
| `__u16` | `tp_net` | `-` |
| `__u32` | `tp_sec` | `-` |
| `__u32` | `tp_nsec` | `-` |
| `__u16` | `tp_vlan_tci` | `-` |
| `__u16` | `tp_vlan_tpid` | `-` |
| `__u8` | `tp_padding` | `4` |

### `struct tpacket_hdr_variant1`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tp_rxhash` | `-` |
| `__u32` | `tp_vlan_tci` | `-` |
| `__u16` | `tp_vlan_tpid` | `-` |
| `__u16` | `tp_padding` | `-` |

### `struct tpacket3_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tp_next_offset` | `-` |
| `__u32` | `tp_sec` | `-` |
| `__u32` | `tp_nsec` | `-` |
| `__u32` | `tp_snaplen` | `-` |
| `__u32` | `tp_len` | `-` |
| `__u32` | `tp_status` | `-` |
| `__u16` | `tp_mac` | `-` |
| `__u16` | `tp_net` | `-` |
| `__u8` | `tp_padding` | `8` |

### `struct tpacket_bd_ts`

| Type | Field | Array |
|------|-------|-------|

### `struct tpacket_hdr_v1`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `block_status` | `-` |
| `__u32` | `num_pkts` | `-` |
| `__u32` | `offset_to_first_pkt` | `-` |
| `__u32` | `blk_len` | `-` |
| `__aligned_u64` | `seq_num` | `-` |

### `struct tpacket_block_desc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `-` |
| `__u32` | `offset_to_priv` | `-` |

### `struct tpacket_req`

| Type | Field | Array |
|------|-------|-------|

### `struct tpacket_req3`

| Type | Field | Array |
|------|-------|-------|

### `struct packet_mreq`

| Type | Field | Array |
|------|-------|-------|
| `int` | `mr_ifindex` | `-` |

### `struct fanout_args`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `id` | `-` |
| `__u16` | `type_flags` | `-` |
| `__u16` | `type_flags` | `-` |
| `__u16` | `id` | `-` |
| `__u32` | `max_num_members` | `-` |