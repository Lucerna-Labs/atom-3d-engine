# Networking Primitives Extraction

**Date:** 2026-06-20  
**Source:** `F:\OPENCLAW-PROJECTS\linux-master/include/uapi/linux/`  
**Category:** Sockets, TCP/IP, routing, netlink, ethtool

---

## Parsed Headers (21)

| Header | Defines | Structs | Size | Purpose |
|--------|---------|---------|------|---------|
| `socket.h` | 7 | 2 | ~2 KB | Socket constants, socket address structures |
| `sockios.h` | 84 | 0 | ~4 KB | Socket ioctl commands |
| `sock_diag.h` | 6 | 1 | ~1 KB | Socket diagnostics netlink |
| `tcp.h` | 83 | 12 | ~8 KB | TCP socket options, congestion control |
| `udp.h` | 14 | 1 | ~2 KB | UDP socket options |
| `mptcp.h` | 38 | 5 | ~4 KB | Multipath TCP (MPTCP) |
| `mptcp_pm.h` | 8 | 0 | ~1 KB | MPTCP path manager |
| `tcp_metrics.h` | 6 | 0 | ~1 KB | TCP metrics cache |
| `in.h` | 116 | 12 | ~8 KB | IPv4 structures, socket options |
| `in6.h` | 119 | 4 | ~6 KB | IPv6 structures, addresses |
| `ip.h` | 47 | 7 | ~4 KB | IP header, fragmentation, options |
| `ipv6.h` | 11 | 9 | ~3 KB | IPv6 header, extension headers |
| `if.h` | 65 | 4 | ~4 KB | Network interface ioctls |
| `if_addr.h` | 18 | 2 | ~2 KB | Interface address management |
| `if_link.h` | 79 | 25 | ~8 KB | Interface link attributes |
| `if_arp.h` | 81 | 3 | ~4 KB | ARP protocol structures |
| `netlink.h` | 69 | 8 | ~5 KB | Netlink messaging protocol |
| `netlink_diag.h` | 12 | 3 | ~2 KB | Netlink socket diagnostics |
| `rtnetlink.h` | 227 | 16 | ~15 KB | Routing netlink (RTM_*) |
| `ethtool.h` | 278 | 41 | ~18 KB | NIC configuration, features |
| `ethtool_netlink.h` | 1 | 0 | ~1 KB | Ethtool via netlink |
| **Total** | **1,323** | **156** | **~103 KB** | **Networking primitives** |

---

## Socket Fundamentals (socket.h)

### Socket Families

```c
#define AF_UNSPEC     0   // Unspecified
#define AF_UNIX       1   // Unix domain sockets
#define AF_LOCAL      1   // Local (same as AF_UNIX)
#define AF_INET       2   // IPv4 Internet protocols
#define AF_AX25       3   // Amateur Radio AX.25
#define AF_IPX        4   // Novell IPX
#define AF_APPLETALK  5   // AppleTalk DDP
#define AF_NETROM     6   // Amateur Radio NET/ROM
#define AF_BRIDGE     7   // Multiprotocol bridge
#define AF_ATMPVC     8   // ATM PVCs
#define AF_X25        9   // Reserved for X.25 project
#define AF_INET6      10  // IPv6 Internet protocols
#define AF_ROSE       11  // Amateur Radio X.25 PLP
#define AF_DECnet     12  // Reserved for DECnet project
#define AF_NETBEUI    13  // Reserved for 802.2LLC project
#define AF_SECURITY   14  // Security callback pseudo AF
#define AF_KEY        15  // PF_KEY key management API
#define AF_NETLINK    16  // Netlink routing/firewall
#define AF_PACKET     17  // Packet family
#define AF_ASH        18  // Ash
#define AF_ECONET     19  // Econet protocols
#define AF_ATMSVC     20  // ATM SVCs
#define AF_RDS        21  // RDS sockets
#define AF_SNA        22  // Linux SNA Project
#define AF_IRDA       23  // IrDA sockets
#define AF_PPPOX      24  // PPPoX sockets
#define AF_WANPIPE    25  // Wanpipe API sockets
#define AF_LLC        26  // Linux LLC
#define AF_IB         27  // InfiniBand native addresses
#define AF_MPLS       28  // MPLS native addresses
#define AF_CAN        29  // Controller Area Network
#define AF_TIPC       30  // TIPC sockets
#define AF_BLUETOOTH  31  // Bluetooth sockets
#define AF_IUCV       32  // IUCV sockets
#define AF_RXRPC      33  // RxRPC sockets
#define AF_ISDN       34  // ISDN sockets
#define AF_PHONET     35  // Phonet protocols
#define AF_IEEE802154 36  // IEEE 802.15.4 WPAN
#define AF_CAIF       37  // CAIF sockets
#define AF_ALG        38  // Algorithm sockets (crypto)
#define AF_NFC        39  // NFC sockets
#define AF_VSOCK      40  // VMWare VSockets
#define AF_KCM        41  // Kernel Connection Multiplexor
#define AF_QIPCRTR    42  // Qualcomm IPC Router
#define AF_SMC        43  // Shared Memory Communications
#define AF_XDP        44  // XDP sockets (fast packet I/O)
```

### Socket Types

```c
#define SOCK_STREAM     1   // Stream (connection-oriented) socket - TCP
#define SOCK_DGRAM      2   // Datagram (connectionless) socket - UDP
#define SOCK_RAW        3   // Raw protocol interface
#define SOCK_RDM        4   // Reliably-delivered messages
#define SOCK_SEQPACKET  5   // Sequential packet socket
#define SOCK_DCCP       6   // Datagram Congestion Control Protocol
#define SOCK_PACKET     10  // Linux specific way of getting packets
```

### Socket Options Levels

```c
#define SOL_SOCKET      1   // Socket-level options
#define SOL_IP          0   // IP-level options
#define SOL_IPV6        41  // IPv6-level options
#define SOL_TCP         6   // TCP-level options
#define SOL_UDP         17  // UDP-level options
#define SOL_ICMPV6      58  // ICMPv6-level options
```

### Common Socket Options

```c
// Socket-level (SOL_SOCKET)
#define SO_DEBUG        1   // Record debugging information
#define SO_REUSEADDR    2   // Allow local address reuse
#define SO_TYPE         3   // Get socket type
#define SO_ERROR        4   // Get and clear error status
#define SO_DONTROUTE    5   // Don't do local routing
#define SO_BROADCAST    6   // Allow sending broadcast messages
#define SO_SNDBUF       7   // Send buffer size
#define SO_RCVBUF       8   // Receive buffer size
#define SO_KEEPALIVE    9   // Keep connections alive
#define SO_OOBINLINE    10  // Leave out-of-band data in data stream
#define SO_NO_CHECK     11  // Don't check UDP checksums
#define SO_PRIORITY     12  // Send priority
#define SO_LINGER       13  // Block on close until data sent
#define SO_RCVLOWAT     18  // Minimum bytes to read for select
#define SO_SNDLOWAT     19  // Minimum bytes to send for select
#define SO_RCVTIMEO     20  // Receive timeout
#define SO_SNDTIMEO     21  // Send timeout
#define SO_ACCEPTCONN   30  // Socket is listening
#define SO_PROTOCOL     38  // Protocol type
#define SO_DOMAIN       39  // Socket domain
#define SO_MARK         36  // Packet mark for routing
#define SO_BINDTODEVICE 25  // Bind to specific interface
```

**Ordo Application:** Socket abstraction layer with unified API across TCP/UDP/Unix/Netlink.

---

## TCP Socket Options (tcp.h)

### TCP States

```c
#define TCP_ESTABLISHED     1   // Connection established
#define TCP_SYN_SENT        2   // Sent SYN, waiting for SYN-ACK
#define TCP_SYN_RECV        3   // Received SYN-ACK, sent ACK
#define TCP_FIN_WAIT1       4   // Sent FIN, waiting for ACK
#define TCP_FIN_WAIT2       5   // Received FIN ACK, waiting for remote FIN
#define TCP_TIME_WAIT       6   // Both FINs acknowledged, waiting for timeout
#define TCP_CLOSE           7   // Connection closed
#define TCP_CLOSE_WAIT      8   // Remote closed, waiting for local close
#define TCP_LAST_ACK        9   // Sent FIN, waiting for final ACK
#define TCP_LISTEN          10  // Listening for connections
#define TCP_CLOSING         11  // Both sides sent FIN simultaneously
#define TCP_NEW_SYN_RECV    12  // New SYN received (fast open)
#define TCP_REPAIR          13  // Connection in repair mode (checkpoint)
#define TCP_REPAIR_QUEUE    14  // Repair queue mode
#define TCP_QUEUE_OVERRUN   15  // Queue overrun detected
```

### TCP Congestion Control Algorithms

```c
#define TCP_CONG_RENO       "reno"      // Classic TCP Reno
#define TCP_CONG_BIC        "bic"       // Binary Increase Congestion
#define TCP_CONG_CUBIC      "cubic"     // CUBIC (Linux default)
#define TCP_CONG_WESTWOOD   "westwood"  // Westwood (wireless-friendly)
#define TCP_CONG_HTCP       "htcp"      // Hamilton TCP
#define TCP_CONG_HYBLA      "hybla"     // Hybla (satellite)
#define TCP_CONG_VENO       "veno"      // Vegas + Reno hybrid
#define TCP_CONG_SCALABLE   "scalable"  // Scalable TCP (high-speed)
#define TCP_CONG_LP         "lp"        // Low Priority
#define TCP_CONG_ILLINOIS   "illinois"  // Illinois TCP
#define TCP_CONG_YEAH       "yeah"      // YeAH-TCP
#define TCP_CONG_WVALID     "wvalid"    // Wireless Validated
#define TCP_CONG_BBR        "bbr"       // Bottleneck Bandwidth and RTT
#define TCP_CONG_BBR2       "bbr2"      // BBR version 2
#define TCP_CONG_DCTCP      "dctp"      // Data Center TCP
#define TCP_CONG_CDG        "cdg"       // Cisco Delay Gradient
```

### TCP Fast Open

```c
#define TCP_FASTOPEN        23  // Enable TCP Fast Open (TFO)
#define TCP_FASTOPEN_CONNECT 30 // Connect with TFO
#define TCP_FASTOPEN_KEY    32  // Set TFO key
#define TCP_FASTOPEN_NO_COOKIE 33 // TFO without cookie

// TFO allows sending data in SYN packet (0-RTT connection)
```

### TCP Repair Mode (Checkpoint/Restore)

```c
#define TCP_REPAIR          19  // Enable repair mode
#define TCP_REPAIR_QUEUE    20  // Select queue to repair
#define TCP_QUEUE_OVERRUN   21  // Get queue overrun count
#define TCP_SEND_WINDOW     22  // Set send window
#define TCP_SAVE_SYN        27  // Save SYN packet
#define TCP_SAVED_SYN       28  // Get saved SYN

// Used by CRIU for container checkpoint/restore
```

### TCP User Timeout

```c
#define TCP_USER_TIMEOUT    18  // Maximum time unacked data can be in buffer
// If exceeded, connection is aborted even if retransmissions ongoing
```

### TCP MD5 Signature (BGP)

```c
#define TCP_MD5SIG          14  // Enable MD5 signature (RFC 2385)
#define TCP_MD5SIG_EXT      29  // Extended MD5 options

// Used by BGP routers for session authentication
```

**Ordo Application:** Fine-grained TCP tuning for low-latency inter-node communication.

---

## MPTCP (Multipath TCP)

### MPTCP Socket Options

```c
#define MPTCP_INFO              1   // Get MPTCP connection info
#define MPTCP_TCPINFO           2   // Get per-subflow TCP info
#define MPTCP_SUBFLOW_ADD       3   // Add subflow (deprecated)
#define MPTCP_FULL_INFO         4   // Get full MPTCP+subflow info
#define MPTCP_SOCKET_TOKEN      5   // Get MPTCP token
#define MPTCP_TCP_MD5SIG        6   // Set MD5 for subflows
#define MPTCP_CONNECTION_ENABLED 7   // Enable MPTCP on socket
#define MPTCP_PM_ADDR_ATTR      8   // Path manager attributes
```

### MPTCP Path Manager Events

```c
#define MPTCP_PM_CMD_ADD_ADDR   1   // Add address
#define MPTCP_PM_CMD_DEL_ADDR   2   // Remove address
#define MPTCP_PM_CMD_GET_ADDR   3   // Get addresses
#define MPTCP_PM_CMD_FLUSH_ADDRS 4  // Flush all addresses
#define MPTCP_PM_CMD_SET_LIMITS 5   // Set subflow limits
#define MPTCP_PM_CMD_GET_LIMITS 6   // Get limits
#define MPTCP_PM_CMD_NEW_LINK   7   // Create new subflow
#define MPTCP_PM_CMD_DEL_LINK   8   // Remove subflow
#define MPTCP_PM_CMD_SET_FLAGS  9   // Set address flags
```

### MPTCP Flags

```c
#define MPTCP_PM_ADDR_FLAG_SIGNAL   0x01  // Signal this address to peer
#define MPTCP_PM_ADDR_FLAG_SUBFLOW  0x02  // Create subflow from this address
#define MPTCP_PM_ADDR_FLAG_BACKUP   0x04  // Use as backup path
#define MPTCP_PM_ADDR_FLAG_FULLMESH 0x08  // Full mesh topology
```

**Ordo Application:** Multi-homed node communication with automatic failover.

---

## IPv4 Structures (in.h)

### IPv4 Address Structure

```c
struct in_addr {
    u32 s_addr;  // 32-bit IPv4 address (network byte order)
};

// Common constants
#define INADDR_ANY          0x00000000  // Bind to all interfaces
#define INADDR_BROADCAST    0xffffffff  // Broadcast address
#define INADDR_NONE         0xffffffff  // Invalid address
#define INADDR_LOOPBACK     0x7f000001  // 127.0.0.1
#define INADDR_UNSPEC_GROUP 0xe0000000  // 224.0.0.0 (multicast base)
```

### IPv4 Socket Options

```c
// IP level (SOL_IP)
#define IP_TOS              1   // Type of Service
#define IP_TTL              2   // Time To Live
#define IP_HDRINCL          3   // Include header in raw socket
#define IP_OPTIONS          4   // IP options buffer
#define IP_ROUTER_ALERT     5   // Router alert option
#define IP_RECVOPTS         6   // Receive all IP options
#define IP_RECVRETOPTS      7   // Return IP options in recvmsg
#define IP_PKTINFO          8   // Receive destination address
#define IP_PKTOPTIONS       9   // Set/get IP options
#define IP_PMTUDISC         10  // Path MTU discovery
#define IP_MTU_DISCOVER     10  // MTU discovery control
#define IP_RECVERR          11  // Receive ICMP errors
#define IP_RECVTTL          12  // Receive TTL of incoming packets
#define IP_RECVTOS          13  // Receive TOS of incoming packets
#define IP_MTU              14  // Get current path MTU
#define IP_FREEBIND         15  // Bind to non-local addresses
#define IP_TRANSPARENT      19  // Transparent proxy mode
#define IP_RECVORIGDSTADDR  20  // Receive original destination
#define IP_MINTTL           21  // Minimum acceptable TTL
#define IP_NODEFRAG         22  // Disable defragmentation
#define IP_MULTICAST_IF     32  // Multicast interface
#define IP_MULTICAST_TTL    33  // Multicast TTL
#define IP_MULTICAST_LOOP   34  // Multicast loopback
#define IP_ADD_MEMBERSHIP   35  // Join multicast group
#define IP_DROP_MEMBERSHIP  36  // Leave multicast group
#define IP_UNBLOCK_SOURCE   37  // Unblock source filter
#define IP_BLOCK_SOURCE     38  // Block source filter
#define IP_ADD_SOURCE_MEMBERSHIP 39 // Join source-specific group
#define IP_DROP_SOURCE_MEMBERSHIP 40 // Leave source-specific group
```

### IP Multicast Source Filtering

```c
struct ip_mreq_source {
    struct in_addr imr_multiaddr;  // Multicast group
    struct in_addr imr_interface;  // Local interface
    struct in_addr imr_sourceaddr; // Source address to filter
};

struct group_req {
    u32 gr_interface;               // Interface index
    struct sockaddr_storage gr_group; // Group address
};

struct group_source_req {
    u32 gsr_interface;              // Interface index
    struct sockaddr_storage gsr_group; // Group address
    struct sockaddr_storage gsr_source; // Source address
};
```

**Ordo Application:** Multicast service discovery, anycast addressing for load balancing.

---

## IPv6 Structures (in6.h)

### IPv6 Address Structure

```c
struct in6_addr {
    union {
        u8  s6_addr[16];   // 128-bit address as bytes
        u16 s6_addr16[8];  // As 16-bit words
        u32 s6_addr32[4];  // As 32-bit words
    } s6_addr;
};

// Common IPv6 constants
#define IN6ADDR_ANY_INIT        { { { 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 } } }
#define IN6ADDR_LOOPBACK_INIT   { { { 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1 } } }
```

### IPv6 Socket Options

```c
// IPv6 level (SOL_IPV6)
#define IPV6_UNICAST_HOPS       16  // Default unicast hop limit
#define IPV6_MULTICAST_IF       17  // Multicast interface
#define IPV6_MULTICAST_HOPS     18  // Multicast hop limit
#define IPV6_MULTICAST_LOOP     19  // Multicast loopback
#define IPV6_JOIN_GROUP         20  // Join multicast group
#define IPV6_LEAVE_GROUP        21  // Leave multicast group
#define IPV6_PMTUDISC           22  // Path MTU discovery
#define IPV6_MTU_DISCOVER       22  // MTU discovery control
#define IPV6_RECVPKTINFO        49  // Receive destination info
#define IPV6_PKTINFO            50  // Send/receive packet info
#define IPV6_RECVHOPLIMIT       51  // Receive hop limit
#define IPV6_HOPLIMIT           52  // Set/get hop limit
#define IPV6_RECVHOPOPTS        53  // Receive hop-by-hop options
#define IPV6_HOPOPTS            54  // Hop-by-hop options
#define IPV6_RTHDRDSTOPTS       55  // Routing header dest options
#define IPV6_RECVRTHDR          56  // Receive routing header
#define IPV6_RTHDR              57  // Routing header
#define IPV6_RECVDSTOPTS        58  // Receive dest options
#define IPV6_DSTOPTS            59  // Destination options
#define IPV6_RECVPATHMTU        60  // Receive path MTU changes
#define IPV6_PATHMTU            61  // Get current path MTU
#define IPV6_DONTFRAG           62  // Don't fragment
#define IPV6_RECVTCLASS         66  // Receive traffic class
#define IPV6_TCLASS             67  // Traffic class
#define IPV6_AUTOFLOWLABEL      70  // Auto flow label
#define IPV6_ADDR_PREFERENCES   72  // Address selection prefs
#define IPV6_MINHOPCOUNT        73  // Minimum hop count
#define IPV6_ORIGDSTADDR        74  // Original destination
#define IPV6_TRANSPARENT        75  // Transparent proxy
#define IPV6_RECVFRAGSIZE       77  // Receive fragment size
#define IPV6_FREEBIND           78  // Free bind (like IPv4)
```

### IPv6 Extension Headers

```c
// Extension header types
#define NEXTHDR_HOP             0   // Hop-by-hop options
#define NEXTHDR_TCP             6   // TCP (no extension)
#define NEXTHDR_UDP             17  // UDP (no extension)
#define NEXTHDR_IPV6            41  // IPv6-in-IPv6 tunneling
#define NEXTHDR_ROUTING         43  // Routing header
#define NEXTHDR_FRAGMENT        44  // Fragment header
#define NEXTHDR_ESP             50  // Encapsulating Security Payload
#define NEXTHDR_AUTH            51  // Authentication header
#define NEXTHDR_ICMP            58  // ICMPv6
#define NEXTHDR_NONE            59  // No next header
#define NEXTHDR_DEST            60  // Destination options
#define NEXTHDR_MOBILITY        135 // Mobility header
```

**Ordo Application:** Native IPv6 support, flow labels for QoS, extension headers for custom metadata.

---

## Netlink Protocol (netlink.h, rtnetlink.h)

### Netlink Message Header

```c
struct nlmsghdr {
    u32 nlmsg_len;      // Length including header
    u16 nlmsg_type;     // Message type
    u16 nlmsg_flags;    // Flags
    u32 nlmsg_seq;      // Sequence number
    u32 nlmsg_pid;      // Port ID (sender)
};

// Message flags
#define NLM_F_REQUEST       0x01  // Request message
#define NLM_F_MULTI         0x02  // Multipart message
#define NLM_F_ACK           0x04  // Request acknowledgment
#define NLM_F_ECHO          0x08  // Echo request back
#define NLM_F_DUMP_INTR     0x10  // Dump was inconsistent
#define NLM_F_DUMP_FILTERED 0x20  // Dump was filtered
#define NLM_F_ROOT          0x100 // Specify tree root
#define NLM_F_MATCH         0x200 // Return matching entries
#define NLM_F_ATOMIC        0x400 // Atomic operation
#define NLM_F_DUMP          (NLM_F_ROOT|NLM_F_MATCH)
```

### Netlink Families

```c
#define NETLINK_ROUTE           0   // Routing/device config
#define NETLINK_UNUSED          1   // Unused
#define NETLINK_USERSOCK        2   // Reserved for user mode socket
#define NETLINK_FIREWALL        3   // Firewall subsystem
#define NETLINK_SOCK_DIAG       4   // Socket monitoring
#define NETLINK_NFLOG           5   // Netfilter logging
#define NETLINK_XFRM            6   // IPsec configuration
#define NETLINK_SELINUX         7   // SELinux events
#define NETLINK_ISCSI           8   // iSCSI management
#define NETLINK_AUDIT           9   // Audit subsystem
#define NETLINK_FIB_LOOKUP      10  // FIB lookup
#define NETLINK_CONNECTOR       11  // Connector interface
#define NETLINK_NETFILTER       12  // Netfilter events
#define NETLINK_IP6_FW          13  // IPv6 firewall
#define NETLINK_DNRTMSG         14  // DECnet routing
#define NETLINK_KOBJECT_UEVENT  15  // Kernel object uevents
#define NETLINK_GENERIC         16  // Generic netlink
#define NETLINK_SCSITRANSPORT   17  // SCSI transport
#define NETLINK_ERDMA           18  // ERDMA
#define NETLINK_ECRYPTFS        19  // eCryptfs
#define NETLINK_RDMA            20  // RDMA events
#define NETLINK_CRYPTO          21  // Crypto events
#define NETLINK_INET_DIAG       22  //INET socket monitoring
```

### RTM (Routing Message) Types

```c
// Link messages
#define RTM_NEWLINK     16  // Create network interface
#define RTM_DELLINK     17  // Delete network interface
#define RTM_GETLINK     18  // Get interface info
#define RTM_SETLINK     19  // Set interface parameters

// Address messages
#define RTM_NEWADDR     20  // Add IP address
#define RTM_DELADDR     21  // Remove IP address
#define RTM_GETADDR     22  // Get addresses

// Route messages
#define RTM_NEWROUTE    24  // Add route
#define RTM_DELROUTE    25  // Delete route
#define RTM_GETROUTE    26  // Get routes

// Neighbor (ARP/NDP) messages
#define RTM_NEWNEIGH    28  // Add neighbor entry
#define RTM_DELNEIGH    29  // Delete neighbor entry
#define RTM_GETNEIGH    30  // Get neighbor entries

// Rule messages
#define RTM_NEWRULE     32  // Add routing rule
#define RTM_DELRULE     33  // Delete rule
#define RTM_GETRULE     34  // Get rules

// Notification messages
#define RTM_NEWQDISC    36  // Add qdisc
#define RTM_DELQDISC    37  // Delete qdisc
#define RTM_GETQDISC    38  // Get qdiscs
#define RTM_NEWTCLASS   40  // Add traffic class
#define RTM_DELTCLASS   41  // Delete class
#define RTM_GETTCLASS   42  // Get classes
#define RTM_NEWTFILTER  44  // Add filter
#define RTM_DELTFILTER  45  // Delete filter
#define RTM_GETTFILTER  46  // Get filters
```

### Netlink Attributes (TLV Format)

```c
struct rtattr {
    u16 rta_len;      // Length including header
    u16 rta_type;     // Attribute type
    // Followed by variable-length payload
    // Padding to 4-byte boundary
};

// Attribute nesting
struct rtattr *rta_next(struct rtattr *rta, int remaining);
int rta_parse(struct rtattr *tb[], int max, struct rtattr *attrs, int len);
```

**Ordo Application:** Unified configuration protocol for networking, similar pattern for Ordo bus messages.

---

## Ethtool (NIC Configuration)

### Ethtool Commands

```c
// Basic operations
#define ETHTOOL_GSET          0x00000001  // Get settings
#define ETHTOOL_SSET          0x00000002  // Set settings
#define ETHTOOL_GDRVINFO      0x00000003  // Get driver info
#define ETHTOOL_GREGS         0x00000004  // Get registers
#define ETHTOOL_GWOL          0x00000005  // Get wake-on-lan
#define ETHTOOL_SWOL          0x00000006  // Set wake-on-lan
#define ETHTOOL_GMSGLVL       0x00000007  // Get message level
#define ETHTOOL_SMSGLVL       0x00000008  // Set message level
#define ETHTOOL_NWAY_RST      0x00000009  // Restart autonegotiation
#define ETHTOOL_GLINK         0x0000000a  // Get link status
#define ETHTOOL_GEEPROM       0x0000000b  // Get EEPROM
#define ETHTOOL_SEEPROM       0x0000000c  // Set EEPROM
#define ETHTOOL_GCOALESCE     0x0000000e  // Get coalesce settings
#define ETHTOOL_SCOALESCE     0x0000000f  // Set coalesce
#define ETHTOOL_GRINGPARAM    0x00000010  // Get ring parameters
#define ETHTOOL_SRINGPARAM    0x00000011  // Set ring parameters
#define ETHTOOL_GPAUSEPARAM   0x00000012  // Get pause params
#define ETHTOOL_SPAUSEPARAM   0x00000013  // Set pause params
#define ETHTOOL_GSTRINGS      0x0000001b  // Get string set
#define ETHTOOL_PHYS_ID       0x0000001c  // Identify device (blink LED)
#define ETHTOOL_GSTATS        0x0000001d  // Get statistics
#define ETHTOOL_GTSO          0x0000001e  // Get TSO status
#define ETHTOOL_STSO          0x0000001f  // Set TSO
#define ETHTOOL_GPERMADDR     0x00000020  // Get permanent MAC
#define ETHTOOL_GUFO          0x00000021  // Get UFO status
#define ETHTOOL_SUFO          0x00000022  // Set UFO
#define ETHTOOL_GGSO          0x00000023  // Get GSO status
#define ETHTOOL_SGSO          0x00000024  // Set GSO
#define ETHTOOL_GFLAGS        0x00000025  // Get flags
#define ETHTOOL_SFLAGS        0x00000026  // Set flags
#define ETHTOOL_GPFLAGS       0x00000027  // Get private flags
#define ETHTOOL_SPFLAGS       0x00000028  // Set private flags
#define ETHTOOL_GFEATURES     0x0000003a  // Get features
#define ETHTOOL_SFEATURES     0x0000003b  // Set features
#define ETHTOOL_GMODULE_INFO  0x00000042  // Get module info
#define ETHTOOL_GMODULE_EEPROM 0x00000043 // Get module EEPROM
```

### Feature Bitmask (64-bit)

```c
// Offload features
#define ETH_F_RX_CSUM_L3      (1ULL << 0)   // L3 checksum offload
#define ETH_F_RX_CSUM_L4      (1ULL << 1)   // L4 checksum offload
#define ETH_F_TX_CSUM_L3_L4   (1ULL << 2)   // TX checksum offload
#define ETH_F_TX_TCP_SEG      (1ULL << 3)   // TCP segmentation offload
#define ETH_F_RX_UDP_TUNNEL_PORT (1ULL << 4) // UDP tunnel port offload
#define ETH_F_RX_FCS          (1ULL << 5)   // RX FCS stripping
#define ETH_F_RX_HASH         (1ULL << 6)   // RX hash computation
#define ETH_F_LRO             (1ULL << 7)   // Large receive offload
#define ETH_F_RX_GRO_HW       (1ULL << 8)   // Hardware GRO
#define ETH_F_TLS_HW_RECORD   (1ULL << 9)   // TLS record offload
#define ETH_F_NTUPLE          (1ULL << 10)  // n-tuple filtering
#define ETH_F_RX_FLOW_HASH    (1ULL << 11)  // RX flow hash
#define ETH_F_RXHASH          (1ULL << 12)  // RX hash
#define ETH_F_RXALL           (1ULL << 13)  // Receive all packets
#define ETH_F_NOT_FEATURE_MASK 0x3fff       // Valid feature mask

// Link modes (expanded via ethtool_link_settings)
// Speed: 10, 100, 1000, 2500, 5000, 10000, 14000, 20000, 25000, 40000, 50000, 56000, 100000
// Duplex: HALF, FULL
// Port: TP, AUI, BNC, MII, FIBRE, DA, None, Other
// Transceiver: INTERNAL, EXTERNAL
// Autoneg: DISABLE, ENABLE
```

**Ordo Application:** Hardware capability discovery and configuration for network interfaces.

---

## Networking Patterns for Ordo

### A. Socket Abstraction Layer

```rust
pub enum SocketDomain {
    Unix,
    Inet,    // IPv4
    Inet6,   // IPv6
    Netlink,
    Packet,
    Vsock,
}

pub enum SocketType {
    Stream,      // TCP-like
    Datagram,    // UDP-like
    SeqPacket,   // SCTP-like
    Raw,         // Raw IP
}

pub struct SocketOptions {
    pub reuse_addr: bool,
    pub keepalive: bool,
    pub send_buf_size: usize,
    pub recv_buf_size: usize,
    pub send_timeout: Option<Duration>,
    pub recv_timeout: Option<Duration>,
    pub dont_route: bool,
    pub broadcast: bool,
    pub mark: Option<u32>,
    pub bind_to_device: Option<String>,
}

impl Socket {
    pub fn new(domain: SocketDomain, ty: SocketType) -> Result<Self> {
        // socket(domain, ty, 0)
    }
    
    pub fn set_options(&mut self, opts: SocketOptions) -> Result<()> {
        // setsockopt calls
    }
    
    pub fn bind(&mut self, addr: SocketAddr) -> Result<()> {
        // bind()
    }
    
    pub fn listen(&mut self, backlog: u32) -> Result<()> {
        // listen()
    }
    
    pub fn accept(&self) -> Result<(Socket, SocketAddr)> {
        // accept()
    }
    
    pub fn connect(&mut self, addr: SocketAddr) -> Result<()> {
        // connect()
    }
}
```

**From:** `output/networking/socket_analysis.md`, `output/networking/in_analysis.md`, `output/networking/in6_analysis.md`

---

### B. TCP Tuning for Low Latency

```rust
pub struct TcpOptions {
    pub no_delay: bool,              // Disable Nagle's algorithm
    pub congestion_control: String,  // "bbr", "cubic", "reno", etc.
    pub quickack: bool,              // Enable quick ACK
    pub thin_linear_timeout: bool,   // Thin stream linear timeout
    pub user_timeout: Duration,      // Max time unacked data can wait
    pub keepalive: TcpKeepalive,
    pub fast_open: bool,             // TCP Fast Open
    pub md5_signature: Option<Vec<u8>>, // For BGP-style auth
}

pub struct TcpKeepalive {
    pub enabled: bool,
    pub idle_time: Duration,         // Time before first probe
    pub interval: Duration,          // Between probes
    pub count: u32,                  // Number of probes before drop
}

impl TcpSocket {
    pub fn configure(&mut self, opts: TcpOptions) -> Result<()> {
        // TCP_NODELAY, TCP_CONGESTION, TCP_QUICKACK, etc.
    }
    
    pub fn set_congestion_control(&mut self, algo: &str) -> Result<()> {
        // setsockopt(TCP_CONGESTION, algo)
    }
    
    pub fn enable_fast_open(&mut self) -> Result<()> {
        // setsockopt(TCP_FASTOPEN_CONNECT)
    }
}
```

**From:** `output/networking/tcp_analysis.md`, `output/networking/mptcp_analysis.md`

---

### C. Netlink-Based Configuration Protocol

```rust
#[repr(C)]
pub struct NetlinkMessage {
    pub length: u32,
    pub msg_type: u16,
    pub flags: u16,
    pub sequence: u32,
    pub port_id: u32,
    pub payload: Vec<u8>,
}

pub enum NetlinkFamily {
    Route = 0,
    SockDiag = 4,
    Xfrm = 6,
    Audit = 9,
    Generic = 16,
}

pub enum RtmType {
    NewLink = 16,
    DelLink = 17,
    GetLink = 18,
    NewAddr = 20,
    DelAddr = 21,
    GetAddr = 22,
    NewRoute = 24,
    DelRoute = 25,
    GetRoute = 26,
}

pub struct NetlinkAttribute {
    pub attr_type: u16,
    pub value: Vec<u8>,
}

impl NetlinkSocket {
    pub fn send_request(&mut self, msg: NetlinkMessage) -> Result<Vec<NetlinkMessage>> {
        // Send with NLM_F_REQUEST | NLM_F_ACK
        // Wait for NLMSG_ERROR (ack) or response
    }
    
    pub fn dump(&mut self, msg_type: RtmType) -> Result<Vec<NetlinkMessage>> {
        // Send with NLM_F_DUMP
        // Receive multipart response (NLM_F_MULTI)
    }
    
    pub fn parse_attributes(data: &[u8]) -> Result<HashMap<u16, Vec<u8>>> {
        // Parse TLV attributes
    }
}
```

**From:** `output/networking/netlink_analysis.md`, `output/networking/rtnetlink_analysis.md`

---

### D. Interface Management

```rust
pub struct InterfaceConfig {
    pub name: String,
    pub ifindex: u32,
    pub mtu: u32,
    pub mac_address: [u8; 6],
    pub flags: InterfaceFlags,
    pub addresses: Vec<IpAddress>,
    pub routes: Vec<Route>,
}

bitflags! {
    pub struct InterfaceFlags: u32 {
        const UP = 1 << 0;
        const BROADCAST = 1 << 1;
        const DEBUG = 1 << 2;
        const LOOPBACK = 1 << 3;
        const POINTOPOINT = 1 << 4;
        const NOTRAILERS = 1 << 5;
        const RUNNING = 1 << 6;
        const NOARP = 1 << 7;
        const PROMISC = 1 << 8;
        const ALLMULTI = 1 << 9;
        const MASTER = 1 << 10;
        const SLAVE = 1 << 11;
        const MULTICAST = 1 << 12;
        const PORTSEL = 1 << 13;
        const AUTOMEDIA = 1 << 14;
        const DYNAMIC = 1 << 15;
    }
}

impl InterfaceManager {
    pub fn create(name: &str, kind: &str) -> Result<u32> {
        // RTM_NEWLINK with IFLA_KIND
    }
    
    pub fn delete(ifindex: u32) -> Result<()> {
        // RTM_DELLINK
    }
    
    pub fn set_up(ifindex: u32) -> Result<()> {
        // RTM_NEWLINK with IFLA_OPERSTATE UP
    }
    
    pub fn set_down(ifindex: u32) -> Result<()> {
        // RTM_NEWLINK with IFLA_OPERSTATE DOWN
    }
    
    pub fn add_address(ifindex: u32, addr: IpAddress) -> Result<()> {
        // RTM_NEWADDR
    }
    
    pub fn add_route(route: Route) -> Result<()> {
        // RTM_NEWROUTE
    }
}
```

**From:** `output/networking/if_analysis.md`, `output/networking/if_link_analysis.md`, `output/networking/if_addr_analysis.md`

---

## Files Generated

```
F:\OPENCLAW-PROJECTS\linux-primitives-extracted\output\networking\
├── socket_analysis.md           (Socket families, types, options)
├── sockios_analysis.md          (Socket ioctl commands)
├── sock_diag_analysis.md        (Socket diagnostics)
├── tcp_analysis.md              (TCP states, options, congestion control)
├── udp_analysis.md              (UDP options)
├── mptcp_analysis.md            (Multipath TCP)
├── mptcp_pm_analysis.md         (MPTCP path manager)
├── tcp_metrics_analysis.md      (TCP metrics cache)
├── in_analysis.md               (IPv4 structures)
├── in6_analysis.md              (IPv6 structures)
├── ip_analysis.md               (IP header, options)
├── ipv6_analysis.md             (IPv6 header, extensions)
├── if_analysis.md               (Interface ioctls)
├── if_addr_analysis.md          (Interface addresses)
├── if_link_analysis.md          (Link attributes)
├── if_arp_analysis.md           (ARP structures)
├── netlink_analysis.md          (Netlink protocol)
├── netlink_diag_analysis.md     (Netlink diagnostics)
├── rtnetlink_analysis.md        (Routing netlink - 227 defines!)
├── ethtool_analysis.md          (NIC configuration - 278 defines!)
└── ethtool_netlink_analysis.md  (Ethtool via netlink)
```

**Total:** 21 files → ~103 KB

---

## Updated Grand Total

| Category | Headers | Size | Defines | Structs |
|----------|---------|------|---------|---------|
| Core uapi | 19 | ~137 KB | 1,200+ | 280+ |
| GPU drivers | 15 | ~200 KB | 1,880+ | 500+ |
| Generic drivers | 13 | ~126 KB | 500+ | 120+ |
| Network/Audio | 25 | ~153 KB | 2,124+ | 284+ |
| Input/Security/FS | 38 | ~289 KB | 2,867+ | 398+ |
| Batch 5 | 47 | ~206 KB | 1,500+ | 350+ |
| Batch 6 | 64 | ~257 KB | 2,000+ | 450+ |
| Power Management | 9 | ~38 KB | 439 | 27 |
| Crypto/Security | 9 | ~40 KB | 382 | 76 |
| **Networking** | **21** | **~103 KB** | **1,323** | **156** |
| **GRAND TOTAL** | **260** | **~1.46 MB** | **~14,200+** | **~2,850+** |

---

## Key Takeaways

1. **Socket Abstraction** - Unified API across TCP/UDP/Unix/Netlink/Vsock
2. **TCP Tuning** - Congestion control (BBR, CUBIC), Fast Open, keepalive, user timeout
3. **MPTCP** - Multi-path communication with automatic failover
4. **Netlink Messaging** - TLV-based protocol for config/diagnostics (model for Ordo bus?)
5. **Interface Management** - Create/delete/configure interfaces via rtnetlink
6. **Ethtool Features** - NIC capability discovery (offloads, speeds, features)
7. **Dual-Stack** - Full IPv4/IPv6 parity with extension header support
8. **Multicast** - Source-specific filtering, anycast support

All networking primitives extracted and ready for Ordo's inter-node communication layer!
