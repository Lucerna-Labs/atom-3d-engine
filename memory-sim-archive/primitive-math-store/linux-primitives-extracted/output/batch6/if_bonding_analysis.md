# if_bonding.h

**Source:** `if_bonding.h`


## Includes

- `linux/if.h`
- `linux/types.h`
- `linux/if_ether.h`

## Defines (40 total)


### BOND_ABI (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_ABI_VERSION` | `2` |  |

### BOND_CHANGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_CHANGE_ACTIVE_OLD` | `(SIOCDEVPRIVATE + 13)` |  |

### BOND_CHECK (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_CHECK_MII_STATUS` | `(SIOCGMIIPHY)` |  |

### BOND_DEFAULT (3)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_DEFAULT_MAX_BONDS` | `1` | Default maximum number of devices to support |
| `BOND_DEFAULT_TX_QUEUES` | `16` | Default number of tx queues per device |
| `BOND_DEFAULT_RESEND_IGMP` | `1` | Default number of IGMP membership reports |

### BOND_ENSLAVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_ENSLAVE_OLD` | `(SIOCDEVPRIVATE)` |  |

### BOND_INFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_INFO_QUERY_OLD` | `(SIOCDEVPRIVATE + 12)` |  |

### BOND_LINK (4)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_LINK_UP` | `0` | link is up and running |
| `BOND_LINK_FAIL` | `1` | link has just gone down |
| `BOND_LINK_DOWN` | `2` | link has been down for too long time |
| `BOND_LINK_BACK` | `3` | link is going back |

### BOND_MODE (7)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_MODE_ROUNDROBIN` | `0` |  |
| `BOND_MODE_ACTIVEBACKUP` | `1` |  |
| `BOND_MODE_XOR` | `2` |  |
| `BOND_MODE_BROADCAST` | `3` |  |
| `BOND_MODE_8023AD` | `4` |  |
| `BOND_MODE_TLB` | `5` |  |
| `BOND_MODE_ALB` | `6` | TLB + RLB (receive load balancing) |

### BOND_RELEASE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_RELEASE_OLD` | `(SIOCDEVPRIVATE + 1)` |  |

### BOND_SETHWADDR (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_SETHWADDR_OLD` | `(SIOCDEVPRIVATE + 2)` |  |

### BOND_SLAVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_SLAVE_INFO_QUERY_OLD` | `(SIOCDEVPRIVATE + 11)` |  |

### BOND_STATE (2)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_STATE_ACTIVE` | `0` | link is active |
| `BOND_STATE_BACKUP` | `1` | link is backup |

### BOND_XMIT (6)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_XMIT_POLICY_LAYER2` | `0` | layer 2 (MAC only), default |
| `BOND_XMIT_POLICY_LAYER34` | `1` | layer 3+4 (IP ^ (TCP || UDP)) |
| `BOND_XMIT_POLICY_LAYER23` | `2` | layer 2+3 (IP ^ MAC) |
| `BOND_XMIT_POLICY_ENCAP23` | `3` | encapsulated layer 2+3 |
| `BOND_XMIT_POLICY_ENCAP34` | `4` | encapsulated layer 3+4 |
| `BOND_XMIT_POLICY_VLAN_SRCMAC` | `5` | vlan + source MAC |

### BOND_XSTATS (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_XSTATS_MAX` | `(__BOND_XSTATS_MAX - 1)` |  |

### LACP_STATE (8)

| Name | Value | Comment |
|------|-------|---------|
| `LACP_STATE_LACP_ACTIVITY` | `0x1` |  |
| `LACP_STATE_LACP_TIMEOUT` | `0x2` |  |
| `LACP_STATE_AGGREGATION` | `0x4` |  |
| `LACP_STATE_SYNCHRONIZATION` | `0x8` |  |
| `LACP_STATE_COLLECTING` | `0x10` |  |
| `LACP_STATE_DISTRIBUTING` | `0x20` |  |
| `LACP_STATE_DEFAULTED` | `0x40` |  |
| `LACP_STATE_EXPIRED` | `0x80` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `BOND_3AD_STAT_MAX` | `(__BOND_3AD_STAT_MAX - 1)` |  |

## Structs (3)


### `struct ifbond`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `bond_mode` | `-` |
| `__s32` | `num_slaves` | `-` |
| `__s32` | `miimon` | `-` |

### `struct ifslave`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `slave_id` | `-` |
| `char` | `slave_name` | `IFNAMSIZ` |
| `__s8` | `link` | `-` |
| `__s8` | `state` | `-` |
| `__u32` | `link_failure_count` | `-` |

### `struct ad_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `aggregator_id` | `-` |
| `__u16` | `ports` | `-` |
| `__u16` | `actor_key` | `-` |
| `__u16` | `partner_key` | `-` |
| `__u8` | `partner_system` | `ETH_ALEN` |

## Typedefs

- `ifbond`
- `ifslave`