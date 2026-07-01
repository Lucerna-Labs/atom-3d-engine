# filter.h

**Source:** `filter.h`


## Includes

- `linux/compiler.h`
- `linux/types.h`
- `linux/bpf_common.h`

## Defines (28 total)


### BPF_LL (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_LL_OFF` | `SKF_LL_OFF` |  |

### BPF_MAJOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_MAJOR_VERSION` | `1` |  |

### BPF_MINOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_MINOR_VERSION` | `1` |  |

### BPF_NET (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_NET_OFF` | `SKF_NET_OFF` |  |

### SKF_AD (18)

| Name | Value | Comment |
|------|-------|---------|
| `SKF_AD_OFF` | `(-0x1000)` |  |
| `SKF_AD_PROTOCOL` | `0` |  |
| `SKF_AD_PKTTYPE` | `4` |  |
| `SKF_AD_IFINDEX` | `8` |  |
| `SKF_AD_NLATTR` | `12` |  |
| `SKF_AD_NLATTR_NEST` | `16` |  |
| `SKF_AD_MARK` | `20` |  |
| `SKF_AD_QUEUE` | `24` |  |
| `SKF_AD_HATYPE` | `28` |  |
| `SKF_AD_RXHASH` | `32` |  |
| `SKF_AD_CPU` | `36` |  |
| `SKF_AD_ALU_XOR_X` | `40` |  |
| `SKF_AD_VLAN_TAG` | `44` |  |
| `SKF_AD_VLAN_TAG_PRESENT` | `48` |  |
| `SKF_AD_PAY_OFFSET` | `52` |  |
| `SKF_AD_RANDOM` | `56` |  |
| `SKF_AD_VLAN_TPID` | `60` |  |
| `SKF_AD_MAX` | `64` |  |

### SKF_LL (1)

| Name | Value | Comment |
|------|-------|---------|
| `SKF_LL_OFF` | `(-0x200000)` |  |

### SKF_NET (1)

| Name | Value | Comment |
|------|-------|---------|
| `SKF_NET_OFF` | `(-0x100000)` |  |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_A` | `0x10` |  |
| `BPF_TAX` | `0x00` |  |
| `BPF_TXA` | `0x80` |  |
| `BPF_MEMWORDS` | `16` |  |

## Structs (2)


### `struct sock_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `code` | `-` |
| `__u8` | `jt` | `-` |
| `__u8` | `jf` | `-` |
| `__u32` | `k` | `-` |

### `struct sock_fprog`

| Type | Field | Array |
|------|-------|-------|