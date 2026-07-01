# nfnetlink.h

**Source:** `nfnetlink.h`


## Includes

- `linux/types.h`
- `linux/netfilter/nfnetlink_compat.h`

## Defines (29 total)


### NFNLGRP_ACCT (1)

| Name | Value | Comment |
|------|-------|---------|
| `NFNLGRP_ACCT_QUOTA` | `NFNLGRP_ACCT_QUOTA` |  |

### NFNLGRP_CONNTRACK (6)

| Name | Value | Comment |
|------|-------|---------|
| `NFNLGRP_CONNTRACK_NEW` | `NFNLGRP_CONNTRACK_NEW` |  |
| `NFNLGRP_CONNTRACK_UPDATE` | `NFNLGRP_CONNTRACK_UPDATE` |  |
| `NFNLGRP_CONNTRACK_DESTROY` | `NFNLGRP_CONNTRACK_DESTROY` |  |
| `NFNLGRP_CONNTRACK_EXP_NEW` | `NFNLGRP_CONNTRACK_EXP_NEW` |  |
| `NFNLGRP_CONNTRACK_EXP_UPDATE` | `NFNLGRP_CONNTRACK_EXP_UPDATE` |  |
| `NFNLGRP_CONNTRACK_EXP_DESTROY` | `NFNLGRP_CONNTRACK_EXP_DESTROY` |  |

### NFNL_BATCH (1)

| Name | Value | Comment |
|------|-------|---------|
| `NFNL_BATCH_MAX` | `(__NFNL_BATCH_MAX - 1)` |  |

### NFNL_MSG (2)

| Name | Value | Comment |
|------|-------|---------|
| `NFNL_MSG_BATCH_BEGIN` | `NLMSG_MIN_TYPE` |  |
| `NFNL_MSG_BATCH_END` | `NLMSG_MIN_TYPE+1` |  |

### NFNL_SUBSYS (14)

| Name | Value | Comment |
|------|-------|---------|
| `NFNL_SUBSYS_NONE` | `0` |  |
| `NFNL_SUBSYS_CTNETLINK` | `1` |  |
| `NFNL_SUBSYS_CTNETLINK_EXP` | `2` |  |
| `NFNL_SUBSYS_QUEUE` | `3` |  |
| `NFNL_SUBSYS_ULOG` | `4` |  |
| `NFNL_SUBSYS_OSF` | `5` |  |
| `NFNL_SUBSYS_IPSET` | `6` |  |
| `NFNL_SUBSYS_ACCT` | `7` |  |
| `NFNL_SUBSYS_CTNETLINK_TIMEOUT` | `8` |  |
| `NFNL_SUBSYS_CTHELPER` | `9` |  |
| `NFNL_SUBSYS_NFTABLES` | `10` |  |
| `NFNL_SUBSYS_NFT_COMPAT` | `11` |  |
| `NFNL_SUBSYS_HOOK` | `12` |  |
| `NFNL_SUBSYS_COUNT` | `13` |  |

### UNCATEGORIZED (5)

| Name | Value | Comment |
|------|-------|---------|
| `NFNLGRP_NONE` | `NFNLGRP_NONE` |  |
| `NFNLGRP_NFTABLES` | `NFNLGRP_NFTABLES` |  |
| `NFNLGRP_NFTRACE` | `NFNLGRP_NFTRACE` |  |
| `NFNLGRP_MAX` | `(__NFNLGRP_MAX - 1)` |  |
| `NFNETLINK_V0` | `0` |  |

## Structs (1)


### `struct nfgenmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `nfgen_family` | `-` |
| `__u8` | `version` | `-` |
| `__be16` | `res_id` | `-` |