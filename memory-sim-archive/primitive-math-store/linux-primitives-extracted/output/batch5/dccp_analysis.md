# dccp.h

**Source:** `dccp.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`

## Defines (19 total)


### DCCP_NR (1)

| Name | Value | Comment |
|------|-------|---------|
| `DCCP_NR_PKT_TYPES` | `DCCP_PKT_INVALID` |  |

### DCCP_SERVICE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DCCP_SERVICE_LIST_MAX_LEN` | `32` |  |

### DCCP_SINGLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DCCP_SINGLE_OPT_MAXLEN` | `253` |  |

### DCCP_SOCKOPT (16)

| Name | Value | Comment |
|------|-------|---------|
| `DCCP_SOCKOPT_PACKET_SIZE` | `1` | XXX deprecated, without effect |
| `DCCP_SOCKOPT_SERVICE` | `2` |  |
| `DCCP_SOCKOPT_CHANGE_L` | `3` |  |
| `DCCP_SOCKOPT_CHANGE_R` | `4` |  |
| `DCCP_SOCKOPT_GET_CUR_MPS` | `5` |  |
| `DCCP_SOCKOPT_SERVER_TIMEWAIT` | `6` |  |
| `DCCP_SOCKOPT_SEND_CSCOV` | `10` |  |
| `DCCP_SOCKOPT_RECV_CSCOV` | `11` |  |
| `DCCP_SOCKOPT_AVAILABLE_CCIDS` | `12` |  |
| `DCCP_SOCKOPT_CCID` | `13` |  |
| `DCCP_SOCKOPT_TX_CCID` | `14` |  |
| `DCCP_SOCKOPT_RX_CCID` | `15` |  |
| `DCCP_SOCKOPT_QPOLICY_ID` | `16` |  |
| `DCCP_SOCKOPT_QPOLICY_TXQLEN` | `17` |  |
| `DCCP_SOCKOPT_CCID_RX_INFO` | `128` |  |
| `DCCP_SOCKOPT_CCID_TX_INFO` | `192` |  |

## Structs (6)


### `struct dccp_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `dccph_doff` | `-` |
| `__sum16` | `dccph_checksum` | `-` |
| `__u8` | `dccph_seq2` | `-` |
| `__be16` | `dccph_seq` | `-` |

### `struct dccp_hdr_ext`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `dccph_seq_low` | `-` |

### `struct dccp_hdr_request`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `dccph_req_service` | `-` |

### `struct dccp_hdr_ack_bits`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `dccph_reserved1` | `-` |
| `__be16` | `dccph_ack_nr_high` | `-` |
| `__be32` | `dccph_ack_nr_low` | `-` |

### `struct dccp_hdr_response`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `dccph_resp_service` | `-` |

### `struct dccp_hdr_reset`

| Type | Field | Array |
|------|-------|-------|