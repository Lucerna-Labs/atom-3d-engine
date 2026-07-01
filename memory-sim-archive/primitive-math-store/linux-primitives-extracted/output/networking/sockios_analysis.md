# sockios.h

**Source:** `sockios.h`


## Includes

- `asm/bitsperlong.h`
- `asm/sockios.h`

## Defines (84 total)


### SOCK_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `SOCK_IOC_TYPE` | `0x89` |  |

### UNCATEGORIZED (83)

| Name | Value | Comment |
|------|-------|---------|
| `SIOCINQ` | `FIONREAD` |  |
| `SIOCOUTQ` | `TIOCOUTQ` | output queue size (not sent + not acked) |
| `SIOCGSTAMP_NEW` | `_IOR(SOCK_IOC_TYPE, 0x06, long long[2])` |  |
| `SIOCGSTAMPNS_NEW` | `_IOR(SOCK_IOC_TYPE, 0x07, long long[2])` |  |
| `SIOCGSTAMP` | `SIOCGSTAMP_OLD` |  |
| `SIOCGSTAMPNS` | `SIOCGSTAMPNS_OLD` |  |
| `SIOCGSTAMP` | `((sizeof(struct timeval))  == 8 ? ` |  |
| `SIOCGSTAMPNS` | `((sizeof(struct timespec)) == 8 ? ` |  |
| `SIOCADDRT` | `0x890B` | add routing table entry |
| `SIOCDELRT` | `0x890C` | delete routing table entry |
| `SIOCRTMSG` | `0x890D` | unused |
| `SIOCGIFNAME` | `0x8910` | get iface name |
| `SIOCSIFLINK` | `0x8911` | set iface channel |
| `SIOCGIFCONF` | `0x8912` | get iface list |
| `SIOCGIFFLAGS` | `0x8913` | get flags |
| `SIOCSIFFLAGS` | `0x8914` | set flags |
| `SIOCGIFADDR` | `0x8915` | get PA address |
| `SIOCSIFADDR` | `0x8916` | set PA address |
| `SIOCGIFDSTADDR` | `0x8917` | get remote PA address |
| `SIOCSIFDSTADDR` | `0x8918` | set remote PA address |
| `SIOCGIFBRDADDR` | `0x8919` | get broadcast PA address |
| `SIOCSIFBRDADDR` | `0x891a` | set broadcast PA address |
| `SIOCGIFNETMASK` | `0x891b` | get network PA mask |
| `SIOCSIFNETMASK` | `0x891c` | set network PA mask |
| `SIOCGIFMETRIC` | `0x891d` | get metric |
| `SIOCSIFMETRIC` | `0x891e` | set metric |
| `SIOCGIFMEM` | `0x891f` | get memory address (BSD) |
| `SIOCSIFMEM` | `0x8920` | set memory address (BSD) |
| `SIOCGIFMTU` | `0x8921` | get MTU size |
| `SIOCSIFMTU` | `0x8922` | set MTU size |
| `SIOCSIFNAME` | `0x8923` | set interface name |
| `SIOCSIFHWADDR` | `0x8924` | set hardware address |
| `SIOCGIFENCAP` | `0x8925` | get/set encapsulations |
| `SIOCSIFENCAP` | `0x8926` |  |
| `SIOCGIFHWADDR` | `0x8927` | Get hardware address |
| `SIOCGIFSLAVE` | `0x8929` | Driver slaving support |
| `SIOCSIFSLAVE` | `0x8930` |  |
| `SIOCADDMULTI` | `0x8931` | Multicast address lists |
| `SIOCDELMULTI` | `0x8932` |  |
| `SIOCGIFINDEX` | `0x8933` | name -> if_index mapping |
| `SIOGIFINDEX` | `SIOCGIFINDEX` | misprint compatibility :-) |
| `SIOCSIFPFLAGS` | `0x8934` | set/get extended flags set |
| `SIOCGIFPFLAGS` | `0x8935` |  |
| `SIOCDIFADDR` | `0x8936` | delete PA address |
| `SIOCSIFHWBROADCAST` | `0x8937` | set hardware broadcast addr |
| `SIOCGIFCOUNT` | `0x8938` | get number of devices |
| `SIOCGIFBR` | `0x8940` | Bridging support |
| `SIOCSIFBR` | `0x8941` | Set bridging options |
| `SIOCGIFTXQLEN` | `0x8942` | Get the tx queue length |
| `SIOCSIFTXQLEN` | `0x8943` | Set the tx queue length |

*...and 33 more*