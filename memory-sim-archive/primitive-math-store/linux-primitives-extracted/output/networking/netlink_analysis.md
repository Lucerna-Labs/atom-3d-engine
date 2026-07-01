# netlink.h

**Source:** `netlink.h`


## Includes

- `linux/const.h`
- `linux/socket.h`
- `linux/types.h`

## Defines (69 total)


### NETLINK_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_ADD_MEMBERSHIP` | `1` |  |

### NETLINK_BROADCAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_BROADCAST_ERROR` | `4` |  |

### NETLINK_CAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_CAP_ACK` | `10` |  |

### NETLINK_DROP (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_DROP_MEMBERSHIP` | `2` |  |

### NETLINK_EXT (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_EXT_ACK` | `11` |  |

### NETLINK_FIB (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_FIB_LOOKUP` | `10` |  |

### NETLINK_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_GET_STRICT_CHK` | `12` |  |

### NETLINK_INET (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_INET_DIAG` | `NETLINK_SOCK_DIAG` |  |

### NETLINK_KOBJECT (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_KOBJECT_UEVENT` | `15` | Kernel messages to userspace |

### NETLINK_LIST (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_LIST_MEMBERSHIPS` | `9` |  |

### NETLINK_LISTEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_LISTEN_ALL_NSID` | `8` |  |

### NETLINK_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_NO_ENOBUFS` | `5` |  |

### NETLINK_RX (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_RX_RING` | `6` |  |

### NETLINK_SOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_SOCK_DIAG` | `4` | socket monitoring |

### NETLINK_TX (1)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_TX_RING` | `7` |  |

### NLA_F (2)

| Name | Value | Comment |
|------|-------|---------|
| `NLA_F_NESTED` | `(1 << 15)` |  |
| `NLA_F_NET_BYTEORDER` | `(1 << 14)` |  |

### NLA_TYPE (1)

| Name | Value | Comment |
|------|-------|---------|
| `NLA_TYPE_MASK` | `~(NLA_F_NESTED \| NLA_F_NET_BYTEORDER)` |  |

### NLMSG_MIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `NLMSG_MIN_TYPE` | `0x10` | < 0x10: reserved control messages |

### NLM_F (18)

| Name | Value | Comment |
|------|-------|---------|
| `NLM_F_REQUEST` | `0x01` | It is request message. |
| `NLM_F_MULTI` | `0x02` | Multipart message, terminated by NLMSG_DONE |
| `NLM_F_ACK` | `0x04` | Reply with ack, with zero or error code |
| `NLM_F_ECHO` | `0x08` | Receive resulting notifications |
| `NLM_F_DUMP_INTR` | `0x10` | Dump was inconsistent due to sequence change |
| `NLM_F_DUMP_FILTERED` | `0x20` | Dump was filtered as requested |
| `NLM_F_ROOT` | `0x100` | specify tree	root |
| `NLM_F_MATCH` | `0x200` | return all matching |
| `NLM_F_ATOMIC` | `0x400` | atomic GET |
| `NLM_F_DUMP` | `(NLM_F_ROOT\|NLM_F_MATCH)` |  |
| `NLM_F_REPLACE` | `0x100` | Override existing |
| `NLM_F_EXCL` | `0x200` | Do not touch, if it exists |
| `NLM_F_CREATE` | `0x400` | Create, if it does not exist |
| `NLM_F_APPEND` | `0x800` | Add to end of list |
| `NLM_F_NONREC` | `0x100` | Do not delete recursively |
| `NLM_F_BULK` | `0x200` | Delete multiple objects |
| `NLM_F_CAPPED` | `0x100` | request was capped |
| `NLM_F_ACK_TLVS` | `0x200` | extended ACK TVLs were included |

### NL_MMAP (2)

| Name | Value | Comment |
|------|-------|---------|
| `NL_MMAP_MSG_ALIGNMENT` | `NLMSG_ALIGNTO` |  |
| `NL_MMAP_HDRLEN` | `NL_MMAP_MSG_ALIGN(sizeof(struct nl_mmap_hdr))` |  |

### UNCATEGORIZED (30)

| Name | Value | Comment |
|------|-------|---------|
| `NETLINK_ROUTE` | `0` | Routing/device hook |
| `NETLINK_UNUSED` | `1` | Unused number |
| `NETLINK_USERSOCK` | `2` | Reserved for user mode socket protocols |
| `NETLINK_FIREWALL` | `3` | Unused number, formerly ip_queue |
| `NETLINK_NFLOG` | `5` | netfilter/iptables ULOG |
| `NETLINK_XFRM` | `6` | ipsec |
| `NETLINK_SELINUX` | `7` | SELinux event notifications |
| `NETLINK_ISCSI` | `8` | Open-iSCSI |
| `NETLINK_AUDIT` | `9` | auditing |
| `NETLINK_CONNECTOR` | `11` |  |
| `NETLINK_NETFILTER` | `12` | netfilter subsystem |
| `NETLINK_IP6_FW` | `13` |  |
| `NETLINK_DNRTMSG` | `14` | DECnet routing messages (obsolete) |
| `NETLINK_GENERIC` | `16` |  |
| `NETLINK_SCSITRANSPORT` | `18` | SCSI Transports |
| `NETLINK_ECRYPTFS` | `19` |  |
| `NETLINK_RDMA` | `20` |  |
| `NETLINK_CRYPTO` | `21` | Crypto layer |
| `NETLINK_SMC` | `22` | SMC monitoring |
| `MAX_LINKS` | `32` |  |
| `NLMSG_ALIGNTO` | `4U` |  |
| `NLMSG_HDRLEN` | `((int) NLMSG_ALIGN(sizeof(struct nlmsghdr)))` |  |
| `NLMSG_NOOP` | `0x1` | Nothing. |
| `NLMSG_ERROR` | `0x2` | Error |
| `NLMSG_DONE` | `0x3` | End of a dump |
| `NLMSG_OVERRUN` | `0x4` | Data lost |
| `NETLINK_PKTINFO` | `3` |  |
| `NET_MAJOR` | `36` | Major 36 is reserved for networking |
| `NLA_ALIGNTO` | `4` |  |
| `NLA_HDRLEN` | `((int) NLA_ALIGN(sizeof(struct nlattr)))` |  |

## Structs (8)


### `struct sockaddr_nl`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_sa_family_t` | `nl_family` | `-` |
| `__u32` | `nl_pid` | `-` |
| `__u32` | `nl_groups` | `-` |

### `struct nlmsghdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nlmsg_len` | `-` |
| `__u16` | `nlmsg_type` | `-` |
| `__u16` | `nlmsg_flags` | `-` |
| `__u32` | `nlmsg_seq` | `-` |
| `__u32` | `nlmsg_pid` | `-` |

### `struct nlmsgerr`

| Type | Field | Array |
|------|-------|-------|
| `int` | `error` | `-` |

### `struct nl_pktinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `group` | `-` |

### `struct nl_mmap_req`

| Type | Field | Array |
|------|-------|-------|

### `struct nl_mmap_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nm_group` | `-` |
| `__u32` | `nm_pid` | `-` |
| `__u32` | `nm_uid` | `-` |
| `__u32` | `nm_gid` | `-` |

### `struct nlattr`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `nla_len` | `-` |
| `__u16` | `nla_type` | `-` |

### `struct nla_bitfield32`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `value` | `-` |
| `__u32` | `selector` | `-` |