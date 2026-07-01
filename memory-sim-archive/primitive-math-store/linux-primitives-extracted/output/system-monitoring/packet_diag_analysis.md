# packet_diag.h

**Source:** `packet_diag.h`


## Includes

- `linux/types.h`

## Defines (12 total)


### PACKET_DIAG (1)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_DIAG_MAX` | `(__PACKET_DIAG_MAX - 1)` |  |

### PACKET_SHOW (6)

| Name | Value | Comment |
|------|-------|---------|
| `PACKET_SHOW_INFO` | `0x00000001` | Basic packet_sk information |
| `PACKET_SHOW_MCLIST` | `0x00000002` | A set of packet_diag_mclist-s |
| `PACKET_SHOW_RING_CFG` | `0x00000004` | Rings configuration parameters |
| `PACKET_SHOW_FANOUT` | `0x00000008` |  |
| `PACKET_SHOW_MEMINFO` | `0x00000010` |  |
| `PACKET_SHOW_FILTER` | `0x00000020` |  |

### UNCATEGORIZED (5)

| Name | Value | Comment |
|------|-------|---------|
| `PDI_RUNNING` | `0x1` |  |
| `PDI_AUXDATA` | `0x2` |  |
| `PDI_ORIGDEV` | `0x4` |  |
| `PDI_VNETHDR` | `0x8` |  |
| `PDI_LOSS` | `0x10` |  |

## Structs (5)


### `struct packet_diag_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `sdiag_family` | `-` |
| `__u8` | `sdiag_protocol` | `-` |
| `__u16` | `pad` | `-` |
| `__u32` | `pdiag_ino` | `-` |
| `__u32` | `pdiag_show` | `-` |
| `__u32` | `pdiag_cookie` | `2` |

### `struct packet_diag_msg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `pdiag_family` | `-` |
| `__u8` | `pdiag_type` | `-` |
| `__u16` | `pdiag_num` | `-` |
| `__u32` | `pdiag_ino` | `-` |
| `__u32` | `pdiag_cookie` | `2` |

### `struct packet_diag_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pdi_index` | `-` |
| `__u32` | `pdi_version` | `-` |
| `__u32` | `pdi_reserve` | `-` |
| `__u32` | `pdi_copy_thresh` | `-` |
| `__u32` | `pdi_tstamp` | `-` |
| `__u32` | `pdi_flags` | `-` |

### `struct packet_diag_mclist`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pdmc_index` | `-` |
| `__u32` | `pdmc_count` | `-` |
| `__u16` | `pdmc_type` | `-` |
| `__u16` | `pdmc_alen` | `-` |
| `__u8` | `pdmc_addr` | `32` |

### `struct packet_diag_ring`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pdr_block_size` | `-` |
| `__u32` | `pdr_block_nr` | `-` |
| `__u32` | `pdr_frame_size` | `-` |
| `__u32` | `pdr_frame_nr` | `-` |
| `__u32` | `pdr_retire_tmo` | `-` |
| `__u32` | `pdr_sizeof_priv` | `-` |
| `__u32` | `pdr_features` | `-` |