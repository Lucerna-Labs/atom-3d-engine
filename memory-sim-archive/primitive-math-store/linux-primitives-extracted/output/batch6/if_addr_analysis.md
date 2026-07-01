# if_addr.h

**Source:** `if_addr.h`


## Includes

- `linux/types.h`
- `linux/netlink.h`

## Defines (18 total)


### IFAPROT_KERNEL (3)

| Name | Value | Comment |
|------|-------|---------|
| `IFAPROT_KERNEL_LO` | `1` | loopback |
| `IFAPROT_KERNEL_RA` | `2` | set by kernel from router announcement |
| `IFAPROT_KERNEL_LL` | `3` | link-local set by kernel |

### IFA_F (13)

| Name | Value | Comment |
|------|-------|---------|
| `IFA_F_SECONDARY` | `0x01` |  |
| `IFA_F_TEMPORARY` | `IFA_F_SECONDARY` |  |
| `IFA_F_NODAD` | `0x02` |  |
| `IFA_F_OPTIMISTIC` | `0x04` |  |
| `IFA_F_DADFAILED` | `0x08` |  |
| `IFA_F_HOMEADDRESS` | `0x10` |  |
| `IFA_F_DEPRECATED` | `0x20` |  |
| `IFA_F_TENTATIVE` | `0x40` |  |
| `IFA_F_PERMANENT` | `0x80` |  |
| `IFA_F_MANAGETEMPADDR` | `0x100` |  |
| `IFA_F_NOPREFIXROUTE` | `0x200` |  |
| `IFA_F_MCAUTOJOIN` | `0x400` |  |
| `IFA_F_STABLE_PRIVACY` | `0x800` |  |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `IFA_MAX` | `(__IFA_MAX - 1)` |  |
| `IFAPROT_UNSPEC` | `0` |  |

## Structs (2)


### `struct ifaddrmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `ifa_family` | `-` |
| `__u8` | `ifa_prefixlen` | `-` |
| `__u8` | `ifa_flags` | `-` |
| `__u8` | `ifa_scope` | `-` |
| `__u32` | `ifa_index` | `-` |

### `struct ifa_cacheinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifa_prefered` | `-` |
| `__u32` | `ifa_valid` | `-` |
| `__u32` | `cstamp` | `-` |
| `__u32` | `tstamp` | `-` |