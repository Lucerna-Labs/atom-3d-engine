# udp.h

**Source:** `udp.h`


## Includes

- `linux/types.h`

## Defines (14 total)


### TCP_ENCAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `TCP_ENCAP_ESPINTCP` | `7` | Yikes, this is really xfrm encap types. |

### UDP_ENCAP (7)

| Name | Value | Comment |
|------|-------|---------|
| `UDP_ENCAP_ESPINUDP_NON_IKE` | `1` | unused  draft-ietf-ipsec-nat-t-ike-00/01 |
| `UDP_ENCAP_ESPINUDP` | `2` | draft-ietf-ipsec-udp-encaps-06 |
| `UDP_ENCAP_L2TPINUDP` | `3` | rfc2661 |
| `UDP_ENCAP_GTP0` | `4` | GSM TS 09.60 |
| `UDP_ENCAP_GTP1U` | `5` | 3GPP TS 29.060 |
| `UDP_ENCAP_RXRPC` | `6` |  |
| `UDP_ENCAP_OVPNINUDP` | `8` | OpenVPN traffic |

### UDP_NO (2)

| Name | Value | Comment |
|------|-------|---------|
| `UDP_NO_CHECK6_TX` | `101` | Disable sending checksum for UDP6X |
| `UDP_NO_CHECK6_RX` | `102` | Disable accepting checksum for UDP6 |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `UDP_CORK` | `1` | Never send partially complete segments |
| `UDP_ENCAP` | `100` | Set the socket to accept encapsulated packets |
| `UDP_SEGMENT` | `103` | Set GSO segmentation size |
| `UDP_GRO` | `104` | This socket can receive UDP GRO packets |

## Structs (1)


### `struct udphdr`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `source` | `-` |
| `__be16` | `dest` | `-` |
| `__be16` | `len` | `-` |
| `__sum16` | `check` | `-` |