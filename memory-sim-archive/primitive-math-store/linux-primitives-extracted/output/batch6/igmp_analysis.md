# igmp.h

**Source:** `igmp.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`

## Defines (31 total)


### IGMP_AGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_AGE_THRESHOLD` | `400` | If this host don't hear any IGMP V1 |

### IGMP_ALL (2)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_ALL_HOSTS` | `htonl(0xE0000001L)` |  |
| `IGMP_ALL_ROUTER` | `htonl(0xE0000002L)` |  |

### IGMP_AWAKENING (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_AWAKENING_MEMBER` | `0x05` |  |

### IGMP_DELAYING (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_DELAYING_MEMBER` | `0x01` |  |

### IGMP_HOST (3)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_HOST_MEMBERSHIP_QUERY` | `0x11` | From RFC1112 |
| `IGMP_HOST_MEMBERSHIP_REPORT` | `0x12` | Ditto |
| `IGMP_HOST_LEAVE_MESSAGE` | `0x17` |  |

### IGMP_IDLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_IDLE_MEMBER` | `0x02` |  |

### IGMP_LAZY (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_LAZY_MEMBER` | `0x03` |  |

### IGMP_LOCAL (2)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_LOCAL_GROUP` | `htonl(0xE0000000L)` |  |
| `IGMP_LOCAL_GROUP_MASK` | `htonl(0xFFFFFF00L)` |  |

### IGMP_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_MAX_HOST_REPORT_DELAY` | `10` | max delay for response to |

### IGMP_MRDISC (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_MRDISC_ADV` | `0x30` | From RFC4286 |

### IGMP_MTRACE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_MTRACE_RESP` | `0x1e` |  |

### IGMP_SLEEPING (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_SLEEPING_MEMBER` | `0x04` |  |

### IGMP_TIMER (1)

| Name | Value | Comment |
|------|-------|---------|
| `IGMP_TIMER_SCALE` | `10` | denotes that the igmphdr->timer field |

### UNCATEGORIZED (14)

| Name | Value | Comment |
|------|-------|---------|
| `IGMPV3_MODE_IS_INCLUDE` | `1` |  |
| `IGMPV3_MODE_IS_EXCLUDE` | `2` |  |
| `IGMPV3_CHANGE_TO_INCLUDE` | `3` |  |
| `IGMPV3_CHANGE_TO_EXCLUDE` | `4` |  |
| `IGMPV3_ALLOW_NEW_SOURCES` | `5` |  |
| `IGMPV3_BLOCK_OLD_SOURCES` | `6` |  |
| `IGMP_DVMRP` | `0x13` | DVMRP routing |
| `IGMP_PIM` | `0x14` | PIM routing |
| `IGMP_TRACE` | `0x15` |  |
| `IGMPV2_HOST_MEMBERSHIP_REPORT` | `0x16` | V2 version of 0x12 |
| `IGMPV3_HOST_MEMBERSHIP_REPORT` | `0x22` | V3 version of 0x12 |
| `IGMP_MTRACE` | `0x1f` |  |
| `IGMP_MINLEN` | `8` |  |
| `IGMPV3_ALL_MCR` | `htonl(0xE0000016L)` |  |

## Structs (4)


### `struct igmphdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `code` | `-` |
| `__sum16` | `csum` | `-` |
| `__be32` | `group` | `-` |

### `struct igmpv3_grec`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `grec_type` | `-` |
| `__u8` | `grec_auxwords` | `-` |
| `__be16` | `grec_nsrcs` | `-` |
| `__be32` | `grec_mca` | `-` |

### `struct igmpv3_report`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `resv1` | `-` |
| `__sum16` | `csum` | `-` |
| `__be16` | `resv2` | `-` |
| `__be16` | `ngrec` | `-` |

### `struct igmpv3_query`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `code` | `-` |
| `__sum16` | `csum` | `-` |
| `__be32` | `group` | `-` |
| `__u8` | `qqic` | `-` |
| `__be16` | `nsrcs` | `-` |