# if_ether.h

**Source:** `if_ether.h`


## Includes

- `linux/types.h`

## Defines (113 total)


### ETH_DATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_DATA_LEN` | `1500` | Max. octets in payload |

### ETH_FCS (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_FCS_LEN` | `4` | Octets in the FCS |

### ETH_FRAME (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_FRAME_LEN` | `1514` | Max. octets in frame sans FCS |

### ETH_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_MAX_MTU` | `0xFFFFU` | 65535, same as IP_MAX_MTU |

### ETH_MIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_MIN_MTU` | `68` | Min IPv4 MTU per RFC791 |

### ETH_P (103)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_P_LOOP` | `0x0060` | Ethernet Loopback packet |
| `ETH_P_PUP` | `0x0200` | Xerox PUP packet |
| `ETH_P_PUPAT` | `0x0201` | Xerox PUP Addr Trans packet |
| `ETH_P_TSN` | `0x22F0` | TSN (IEEE 1722) packet |
| `ETH_P_ERSPAN2` | `0x22EB` | ERSPAN version 2 (type III) |
| `ETH_P_IP` | `0x0800` | Internet Protocol packet |
| `ETH_P_X25` | `0x0805` | CCITT X.25 |
| `ETH_P_ARP` | `0x0806` | Address Resolution packet |
| `ETH_P_BPQ` | `0x08FF` | G8BPQ AX.25 Ethernet Packet	[ NOT AN OFFICIALLY REGISTERED ID ] |
| `ETH_P_IEEEPUP` | `0x0a00` | Xerox IEEE802.3 PUP packet |
| `ETH_P_IEEEPUPAT` | `0x0a01` | Xerox IEEE802.3 PUP Addr Trans packet |
| `ETH_P_BATMAN` | `0x4305` | B.A.T.M.A.N.-Advanced packet [ NOT AN OFFICIALLY REGISTERED ID ] |
| `ETH_P_DEC` | `0x6000` | DEC Assigned proto |
| `ETH_P_DNA_DL` | `0x6001` | DEC DNA Dump/Load |
| `ETH_P_DNA_RC` | `0x6002` | DEC DNA Remote Console |
| `ETH_P_DNA_RT` | `0x6003` | DEC DNA Routing |
| `ETH_P_LAT` | `0x6004` | DEC LAT |
| `ETH_P_DIAG` | `0x6005` | DEC Diagnostics |
| `ETH_P_CUST` | `0x6006` | DEC Customer use |
| `ETH_P_SCA` | `0x6007` | DEC Systems Comms Arch |
| `ETH_P_TEB` | `0x6558` | Trans Ether Bridging |
| `ETH_P_RARP` | `0x8035` | Reverse Addr Res packet |
| `ETH_P_ATALK` | `0x809B` | Appletalk DDP |
| `ETH_P_AARP` | `0x80F3` | Appletalk AARP |
| `ETH_P_8021Q` | `0x8100` | 802.1Q VLAN Extended Header |
| `ETH_P_ERSPAN` | `0x88BE` | ERSPAN type II |
| `ETH_P_IPX` | `0x8137` | IPX over DIX |
| `ETH_P_IPV6` | `0x86DD` | IPv6 over bluebook |
| `ETH_P_PAUSE` | `0x8808` | IEEE Pause frames. See 802.3 31B |
| `ETH_P_SLOW` | `0x8809` | Slow Protocol. See 802.3ad 43B |
| `ETH_P_WCCP` | `0x883E		/* Web-cache coordination protocol` |  |
| `ETH_P_MPLS_UC` | `0x8847` | MPLS Unicast traffic |
| `ETH_P_MPLS_MC` | `0x8848` | MPLS Multicast traffic |
| `ETH_P_ATMMPOA` | `0x884c` | MultiProtocol Over ATM |
| `ETH_P_PPP_DISC` | `0x8863` | PPPoE discovery messages |
| `ETH_P_PPP_SES` | `0x8864` | PPPoE session messages |
| `ETH_P_LINK_CTL` | `0x886c` | HPNA, wlan link local tunnel |
| `ETH_P_ATMFATE` | `0x8884		/* Frame-based ATM Transport` |  |
| `ETH_P_PAE` | `0x888E` | Port Access Entity (IEEE 802.1X) |
| `ETH_P_PROFINET` | `0x8892` | PROFINET |
| `ETH_P_REALTEK` | `0x8899` | Multiple proprietary protocols |
| `ETH_P_AOE` | `0x88A2` | ATA over Ethernet |
| `ETH_P_ETHERCAT` | `0x88A4` | EtherCAT |
| `ETH_P_8021AD` | `0x88A8` | 802.1ad Service VLAN |
| `ETH_P_802_EX1` | `0x88B5` | 802.1 Local Experimental 1. |
| `ETH_P_MXLGSW` | `0x88C3		/* Infineon Technologies Corporate Research ST` |  |
| `ETH_P_PREAUTH` | `0x88C7` | 802.11 Preauthentication |
| `ETH_P_TIPC` | `0x88CA` | TIPC |
| `ETH_P_LLDP` | `0x88CC` | Link Layer Discovery Protocol |
| `ETH_P_MRP` | `0x88E3` | Media Redundancy Protocol |

*...and 53 more*

### UNCATEGORIZED (5)

| Name | Value | Comment |
|------|-------|---------|
| `ETH_ALEN` | `6` | Octets in one ethernet addr |
| `ETH_TLEN` | `2` | Octets in ethernet type field |
| `ETH_HLEN` | `14` | Total octets in header. |
| `ETH_ZLEN` | `60` | Min. octets in frame sans FCS |
| `__UAPI_DEF_ETHHDR` | `1` |  |

## Structs (1)


### `struct ethhdr`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `h_proto` | `-` |