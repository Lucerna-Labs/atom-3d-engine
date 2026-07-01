# virtio_net.h

**Source:** `virtio_net.h`


## Includes

- `linux/types.h`
- `linux/virtio_ids.h`
- `linux/virtio_config.h`
- `linux/virtio_types.h`
- `linux/if_ether.h`

## Defines (126 total)


### VIRTIO_NET (126)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_NET_F_CSUM` | `0` | Host handles pkts w/ partial csum |
| `VIRTIO_NET_F_GUEST_CSUM` | `1` | Guest handles pkts w/ partial csum |
| `VIRTIO_NET_F_CTRL_GUEST_OFFLOADS` | `2` | Dynamic offload configuration. |
| `VIRTIO_NET_F_MTU` | `3` | Initial MTU advice |
| `VIRTIO_NET_F_MAC` | `5` | Host has given MAC address. |
| `VIRTIO_NET_F_GUEST_TSO4` | `7` | Guest can handle TSOv4 in. |
| `VIRTIO_NET_F_GUEST_TSO6` | `8` | Guest can handle TSOv6 in. |
| `VIRTIO_NET_F_GUEST_ECN` | `9` | Guest can handle TSO[6] w/ ECN in. |
| `VIRTIO_NET_F_GUEST_UFO` | `10` | Guest can handle UFO in. |
| `VIRTIO_NET_F_HOST_TSO4` | `11` | Host can handle TSOv4 in. |
| `VIRTIO_NET_F_HOST_TSO6` | `12` | Host can handle TSOv6 in. |
| `VIRTIO_NET_F_HOST_ECN` | `13` | Host can handle TSO[6] w/ ECN in. |
| `VIRTIO_NET_F_HOST_UFO` | `14` | Host can handle UFO in. |
| `VIRTIO_NET_F_MRG_RXBUF` | `15` | Host can merge receive buffers. |
| `VIRTIO_NET_F_STATUS` | `16` | virtio_net_config.status available |
| `VIRTIO_NET_F_CTRL_VQ` | `17` | Control channel available |
| `VIRTIO_NET_F_CTRL_RX` | `18` | Control channel RX mode support |
| `VIRTIO_NET_F_CTRL_VLAN` | `19` | Control channel VLAN filtering |
| `VIRTIO_NET_F_CTRL_RX_EXTRA` | `20` | Extra RX mode control support |
| `VIRTIO_NET_F_GUEST_ANNOUNCE` | `21	/* Guest can announce device on the` |  |
| `VIRTIO_NET_F_MQ` | `22	/* Device supports Receive Flow` |  |
| `VIRTIO_NET_F_CTRL_MAC_ADDR` | `23` | Set MAC address |
| `VIRTIO_NET_F_DEVICE_STATS` | `50` | Device can provide device-level statistics. |
| `VIRTIO_NET_F_VQ_NOTF_COAL` | `52` | Device supports virtqueue notification coalescing |
| `VIRTIO_NET_F_NOTF_COAL` | `53` | Device supports notifications coalescing |
| `VIRTIO_NET_F_GUEST_USO4` | `54` | Guest can handle USOv4 in. |
| `VIRTIO_NET_F_GUEST_USO6` | `55` | Guest can handle USOv6 in. |
| `VIRTIO_NET_F_HOST_USO` | `56` | Host can handle USO in. |
| `VIRTIO_NET_F_HASH_REPORT` | `57` | Supports hash report |
| `VIRTIO_NET_F_GUEST_HDRLEN` | `59` | Guest provides the exact hdr_len value. |
| `VIRTIO_NET_F_RSS` | `60` | Supports RSS RX steering |
| `VIRTIO_NET_F_RSC_EXT` | `61` | extended coalescing info |
| `VIRTIO_NET_F_STANDBY` | `62	/* Act as standby for another device` |  |
| `VIRTIO_NET_F_SPEED_DUPLEX` | `63` | Device set linkspeed and duplex |
| `VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO` | `65 /* Driver can receive` |  |
| `VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_CSUM` | `66 /* Driver handles` |  |
| `VIRTIO_NET_F_HOST_UDP_TUNNEL_GSO` | `67 /* Device can receive` |  |
| `VIRTIO_NET_F_HOST_UDP_TUNNEL_GSO_CSUM` | `68 /* Device handles` |  |
| `VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_MAPPED` | `46` |  |
| `VIRTIO_NET_F_GUEST_UDP_TUNNEL_GSO_CSUM_MAPPED` | `47` |  |
| `VIRTIO_NET_F_GSO` | `6` | Host handles pkts w/ any GSO type |
| `VIRTIO_NET_S_LINK_UP` | `1` | Link is up |
| `VIRTIO_NET_S_ANNOUNCE` | `2` | Announcement is needed |
| `VIRTIO_NET_RSS_HASH_TYPE_IPv4` | `(1 << 0)` |  |
| `VIRTIO_NET_RSS_HASH_TYPE_TCPv4` | `(1 << 1)` |  |
| `VIRTIO_NET_RSS_HASH_TYPE_UDPv4` | `(1 << 2)` |  |
| `VIRTIO_NET_RSS_HASH_TYPE_IPv6` | `(1 << 3)` |  |
| `VIRTIO_NET_RSS_HASH_TYPE_TCPv6` | `(1 << 4)` |  |
| `VIRTIO_NET_RSS_HASH_TYPE_UDPv6` | `(1 << 5)` |  |
| `VIRTIO_NET_RSS_HASH_TYPE_IP_EX` | `(1 << 6)` |  |

*...and 76 more*

## Structs (33)


### `struct virtio_net_config`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `mac` | `ETH_ALEN` |
| `__virtio16` | `status` | `-` |
| `__virtio16` | `max_virtqueue_pairs` | `-` |
| `__virtio16` | `mtu` | `-` |
| `__le32` | `speed` | `-` |
| `__u8` | `duplex` | `-` |
| `__u8` | `rss_max_key_size` | `-` |
| `__le16` | `rss_max_indirection_table_length` | `-` |
| `__le32` | `supported_hash_types` | `-` |

### `struct virtio_net_hdr_v1`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `gso_type` | `-` |
| `__virtio16` | `hdr_len` | `-` |
| `__virtio16` | `gso_size` | `-` |
| `__virtio16` | `csum_start` | `-` |
| `__virtio16` | `csum_offset` | `-` |
| `__virtio16` | `start` | `-` |
| `__virtio16` | `offset` | `-` |
| `__le16` | `segments` | `-` |
| `__le16` | `dup_acks` | `-` |
| `__virtio16` | `num_buffers` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `csum_start` | `-` |
| `__virtio16` | `csum_offset` | `-` |

### `struct anonymous_3`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `start` | `-` |
| `__virtio16` | `offset` | `-` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `segments` | `-` |
| `__le16` | `dup_acks` | `-` |

### `struct virtio_net_hdr_v1_hash`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `hash_value_lo` | `-` |
| `__le16` | `hash_value_hi` | `-` |
| `__le16` | `hash_report` | `-` |
| `__le16` | `padding` | `-` |

### `struct virtio_net_hdr_v1_hash_tunnel`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `outer_th_offset` | `-` |
| `__le16` | `inner_nh_offset` | `-` |

### `struct virtio_net_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `gso_type` | `-` |
| `__virtio16` | `hdr_len` | `-` |
| `__virtio16` | `gso_size` | `-` |
| `__virtio16` | `csum_start` | `-` |
| `__virtio16` | `csum_offset` | `-` |

### `struct virtio_net_hdr_mrg_rxbuf`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `num_buffers` | `-` |

### `struct virtio_net_ctrl_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `class` | `-` |
| `__u8` | `cmd` | `-` |

### `struct virtio_net_ctrl_mac`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `entries` | `-` |
| `__u8` | `macs` | `][ETH_ALEN` |

### `struct virtio_net_ctrl_mq`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `virtqueue_pairs` | `-` |

### `struct virtio_net_rss_config`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `hash_types` | `-` |
| `__le16` | `indirection_table_mask` | `-` |
| `__le16` | `unclassified_queue` | `-` |
| `__le16` | `max_tx_vq` | `-` |
| `__u8` | `hash_key_length` | `-` |

### `struct virtio_net_rss_config_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `hash_types` | `-` |
| `__le16` | `indirection_table_mask` | `-` |
| `__le16` | `unclassified_queue` | `-` |

### `struct virtio_net_rss_config_trailer`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `max_tx_vq` | `-` |
| `__u8` | `hash_key_length` | `-` |

### `struct virtio_net_hash_config`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `hash_types` | `-` |
| `__le16` | `reserved` | `4` |
| `__u8` | `hash_key_length` | `-` |

### `struct virtio_net_ctrl_coal_tx`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `tx_max_packets` | `-` |
| `__le32` | `tx_usecs` | `-` |

### `struct virtio_net_ctrl_coal_rx`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `rx_max_packets` | `-` |
| `__le32` | `rx_usecs` | `-` |

### `struct virtio_net_ctrl_coal`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `max_packets` | `-` |
| `__le32` | `max_usecs` | `-` |

### `struct virtio_net_ctrl_coal_vq`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `vqn` | `-` |
| `__le16` | `reserved` | `-` |

### `struct virtio_net_stats_capabilities`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `supported_stats_types` | `1` |

### `struct virtio_net_ctrl_queue_stats`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `vq_index` | `-` |
| `__le16` | `reserved` | `3` |
| `__le64` | `types_bitmap` | `1` |

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `vq_index` | `-` |
| `__le16` | `reserved` | `3` |
| `__le64` | `types_bitmap` | `1` |

### `struct virtio_net_stats_reply_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `reserved` | `-` |
| `__le16` | `vq_index` | `-` |
| `__le16` | `reserved1` | `-` |
| `__le16` | `size` | `-` |

### `struct virtio_net_stats_cvq`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `command_num` | `-` |
| `__le64` | `ok_num` | `-` |

### `struct virtio_net_stats_rx_basic`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `rx_notifications` | `-` |
| `__le64` | `rx_packets` | `-` |
| `__le64` | `rx_bytes` | `-` |
| `__le64` | `rx_interrupts` | `-` |
| `__le64` | `rx_drops` | `-` |
| `__le64` | `rx_drop_overruns` | `-` |

### `struct virtio_net_stats_tx_basic`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `tx_notifications` | `-` |
| `__le64` | `tx_packets` | `-` |
| `__le64` | `tx_bytes` | `-` |
| `__le64` | `tx_interrupts` | `-` |
| `__le64` | `tx_drops` | `-` |
| `__le64` | `tx_drop_malformed` | `-` |

### `struct virtio_net_stats_rx_csum`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `rx_csum_valid` | `-` |
| `__le64` | `rx_needs_csum` | `-` |
| `__le64` | `rx_csum_none` | `-` |
| `__le64` | `rx_csum_bad` | `-` |

### `struct virtio_net_stats_tx_csum`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `tx_csum_none` | `-` |
| `__le64` | `tx_needs_csum` | `-` |

### `struct virtio_net_stats_rx_gso`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `rx_gso_packets` | `-` |
| `__le64` | `rx_gso_bytes` | `-` |
| `__le64` | `rx_gso_packets_coalesced` | `-` |
| `__le64` | `rx_gso_bytes_coalesced` | `-` |

### `struct virtio_net_stats_tx_gso`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `tx_gso_packets` | `-` |
| `__le64` | `tx_gso_bytes` | `-` |
| `__le64` | `tx_gso_segments` | `-` |
| `__le64` | `tx_gso_segments_bytes` | `-` |
| `__le64` | `tx_gso_packets_noseg` | `-` |
| `__le64` | `tx_gso_bytes_noseg` | `-` |

### `struct virtio_net_stats_rx_speed`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `rx_ratelimit_packets` | `-` |
| `__le64` | `rx_ratelimit_bytes` | `-` |

### `struct virtio_net_stats_tx_speed`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `tx_ratelimit_packets` | `-` |
| `__le64` | `tx_ratelimit_bytes` | `-` |