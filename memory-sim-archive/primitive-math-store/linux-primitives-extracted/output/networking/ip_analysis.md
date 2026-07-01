# ip.h

**Source:** `ip.h`


## Includes

- `linux/types.h`
- `linux/stddef.h`
- `asm/byteorder.h`

## Defines (47 total)


### IPOPT_CLASS (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPOPT_CLASS_MASK` | `0x60` |  |

### IPOPT_NUMBER (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPOPT_NUMBER_MASK` | `0x1f` |  |

### IPOPT_TS (3)

| Name | Value | Comment |
|------|-------|---------|
| `IPOPT_TS_TSONLY` | `0` | timestamps only |
| `IPOPT_TS_TSANDADDR` | `1` | timestamps and addresses |
| `IPOPT_TS_PRESPEC` | `3` | specified modules only |

### IPTOS_PREC (9)

| Name | Value | Comment |
|------|-------|---------|
| `IPTOS_PREC_MASK` | `0xE0` |  |
| `IPTOS_PREC_NETCONTROL` | `0xe0` |  |
| `IPTOS_PREC_INTERNETCONTROL` | `0xc0` |  |
| `IPTOS_PREC_CRITIC_ECP` | `0xa0` |  |
| `IPTOS_PREC_FLASHOVERRIDE` | `0x80` |  |
| `IPTOS_PREC_FLASH` | `0x60` |  |
| `IPTOS_PREC_IMMEDIATE` | `0x40` |  |
| `IPTOS_PREC_PRIORITY` | `0x20` |  |
| `IPTOS_PREC_ROUTINE` | `0x00` |  |

### IPTOS_TOS (1)

| Name | Value | Comment |
|------|-------|---------|
| `IPTOS_TOS_MASK` | `0x1E` |  |

### UNCATEGORIZED (32)

| Name | Value | Comment |
|------|-------|---------|
| `IPTOS_LOWDELAY` | `0x10` |  |
| `IPTOS_THROUGHPUT` | `0x08` |  |
| `IPTOS_RELIABILITY` | `0x04` |  |
| `IPTOS_MINCOST` | `0x02` |  |
| `IPOPT_COPY` | `0x80` |  |
| `IPOPT_CONTROL` | `0x00` |  |
| `IPOPT_RESERVED1` | `0x20` |  |
| `IPOPT_MEASUREMENT` | `0x40` |  |
| `IPOPT_RESERVED2` | `0x60` |  |
| `IPOPT_END` | `(0 \|IPOPT_CONTROL)` |  |
| `IPOPT_NOOP` | `(1 \|IPOPT_CONTROL)` |  |
| `IPOPT_SEC` | `(2 \|IPOPT_CONTROL\|IPOPT_COPY)` |  |
| `IPOPT_LSRR` | `(3 \|IPOPT_CONTROL\|IPOPT_COPY)` |  |
| `IPOPT_TIMESTAMP` | `(4 \|IPOPT_MEASUREMENT)` |  |
| `IPOPT_CIPSO` | `(6 \|IPOPT_CONTROL\|IPOPT_COPY)` |  |
| `IPOPT_RR` | `(7 \|IPOPT_CONTROL)` |  |
| `IPOPT_SID` | `(8 \|IPOPT_CONTROL\|IPOPT_COPY)` |  |
| `IPOPT_SSRR` | `(9 \|IPOPT_CONTROL\|IPOPT_COPY)` |  |
| `IPOPT_RA` | `(20\|IPOPT_CONTROL\|IPOPT_COPY)` |  |
| `IPVERSION` | `4` |  |
| `MAXTTL` | `255` |  |
| `IPDEFTTL` | `64` |  |
| `IPOPT_OPTVAL` | `0` |  |
| `IPOPT_OLEN` | `1` |  |
| `IPOPT_OFFSET` | `2` |  |
| `IPOPT_MINOFF` | `4` |  |
| `MAX_IPOPTLEN` | `40` |  |
| `IPOPT_NOP` | `IPOPT_NOOP` |  |
| `IPOPT_EOL` | `IPOPT_END` |  |
| `IPOPT_TS` | `IPOPT_TIMESTAMP` |  |
| `IPV4_BEET_PHMAXLEN` | `8` |  |
| `IPV4_DEVCONF_MAX` | `(__IPV4_DEVCONF_MAX - 1)` |  |

## Structs (7)


### `struct iphdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `tos` | `-` |
| `__be16` | `tot_len` | `-` |
| `__be16` | `id` | `-` |
| `__be16` | `frag_off` | `-` |
| `__u8` | `ttl` | `-` |
| `__u8` | `protocol` | `-` |
| `__sum16` | `check` | `-` |
| `__be32` | `saddr` | `-` |
| `__be32` | `daddr` | `-` |

### `struct ip_auth_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `nexthdr` | `-` |
| `__u8` | `hdrlen` | `-` |
| `__be16` | `reserved` | `-` |
| `__be32` | `spi` | `-` |
| `__be32` | `seq_no` | `-` |

### `struct ip_esp_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `spi` | `-` |
| `__be32` | `seq_no` | `-` |

### `struct ip_comp_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `nexthdr` | `-` |
| `__u8` | `flags` | `-` |
| `__be16` | `cpi` | `-` |

### `struct ip_beet_phdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `nexthdr` | `-` |
| `__u8` | `hdrlen` | `-` |
| `__u8` | `padlen` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ip_iptfs_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `subtype` | `-` |
| `__u8` | `flags` | `-` |
| `__be16` | `block_offset` | `-` |

### `struct ip_iptfs_cc_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `subtype` | `-` |
| `__u8` | `flags` | `-` |
| `__be16` | `block_offset` | `-` |
| `__be32` | `loss_rate` | `-` |
| `__be64` | `rtt_adelay_xdelay` | `-` |
| `__be32` | `tval` | `-` |
| `__be32` | `techo` | `-` |