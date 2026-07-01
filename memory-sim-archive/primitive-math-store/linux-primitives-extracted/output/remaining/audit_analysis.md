# audit.h

**Source:** `audit.h`


## Includes

- `linux/types.h`
- `linux/elf-em.h`

## Defines (312 total)


### AUDIT_ADD (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_ADD_RULE` | `1011` | Add syscall filtering rule |

### AUDIT_ANOM (4)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_ANOM_PROMISCUOUS` | `1700` | Device changed promiscuous mode |
| `AUDIT_ANOM_ABEND` | `1701` | Process ended abnormally |
| `AUDIT_ANOM_LINK` | `1702` | Suspicious use of file links |
| `AUDIT_ANOM_CREAT` | `1703` | Suspicious file creation |

### AUDIT_ARCH (53)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_ARCH_AARCH64` | `(EM_AARCH64\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_ALPHA` | `(EM_ALPHA\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_ARCOMPACT` | `(EM_ARCOMPACT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_ARCOMPACTBE` | `(EM_ARCOMPACT)` |  |
| `AUDIT_ARCH_ARCV2` | `(EM_ARCV2\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_ARCV2BE` | `(EM_ARCV2)` |  |
| `AUDIT_ARCH_ARM` | `(EM_ARM\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_ARMEB` | `(EM_ARM)` |  |
| `AUDIT_ARCH_C6X` | `(EM_TI_C6000\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_C6XBE` | `(EM_TI_C6000)` |  |
| `AUDIT_ARCH_CRIS` | `(EM_CRIS\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_CSKY` | `(EM_CSKY\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_FRV` | `(EM_FRV)` |  |
| `AUDIT_ARCH_H8300` | `(EM_H8_300)` |  |
| `AUDIT_ARCH_HEXAGON` | `(EM_HEXAGON)` |  |
| `AUDIT_ARCH_I386` | `(EM_386\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_IA64` | `(EM_IA_64\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_M32R` | `(EM_M32R)` |  |
| `AUDIT_ARCH_M68K` | `(EM_68K)` |  |
| `AUDIT_ARCH_MICROBLAZE` | `(EM_MICROBLAZE)` |  |
| `AUDIT_ARCH_MIPS` | `(EM_MIPS)` |  |
| `AUDIT_ARCH_MIPSEL` | `(EM_MIPS\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_MIPS64` | `(EM_MIPS\|__AUDIT_ARCH_64BIT)` |  |
| `AUDIT_ARCH_MIPS64N32` | `(EM_MIPS\|__AUDIT_ARCH_64BIT\|` |  |
| `AUDIT_ARCH_MIPSEL64` | `(EM_MIPS\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_MIPSEL64N32` | `(EM_MIPS\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE\|` |  |
| `AUDIT_ARCH_NDS32` | `(EM_NDS32\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_NDS32BE` | `(EM_NDS32)` |  |
| `AUDIT_ARCH_NIOS2` | `(EM_ALTERA_NIOS2\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_OPENRISC` | `(EM_OPENRISC)` |  |
| `AUDIT_ARCH_PARISC` | `(EM_PARISC)` |  |
| `AUDIT_ARCH_PARISC64` | `(EM_PARISC\|__AUDIT_ARCH_64BIT)` |  |
| `AUDIT_ARCH_PPC` | `(EM_PPC)` |  |
| `AUDIT_ARCH_PPC64` | `(EM_PPC64\|__AUDIT_ARCH_64BIT)` |  |
| `AUDIT_ARCH_PPC64LE` | `(EM_PPC64\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_RISCV32` | `(EM_RISCV\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_RISCV64` | `(EM_RISCV\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_S390` | `(EM_S390)` |  |
| `AUDIT_ARCH_S390X` | `(EM_S390\|__AUDIT_ARCH_64BIT)` |  |
| `AUDIT_ARCH_SH` | `(EM_SH)` |  |
| `AUDIT_ARCH_SHEL` | `(EM_SH\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_SH64` | `(EM_SH\|__AUDIT_ARCH_64BIT)` |  |
| `AUDIT_ARCH_SHEL64` | `(EM_SH\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_SPARC` | `(EM_SPARC)` |  |
| `AUDIT_ARCH_SPARC64` | `(EM_SPARCV9\|__AUDIT_ARCH_64BIT)` |  |
| `AUDIT_ARCH_TILEGX` | `(EM_TILEGX\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_TILEGX32` | `(EM_TILEGX\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_TILEPRO` | `(EM_TILEPRO\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_UNICORE` | `(EM_UNICORE\|__AUDIT_ARCH_LE)` |  |
| `AUDIT_ARCH_X86_64` | `(EM_X86_64\|__AUDIT_ARCH_64BIT\|__AUDIT_ARCH_LE)` |  |

*...and 3 more*

### AUDIT_AVC (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_AVC_PATH` | `1402` | dentry, vfsmount pair from avc |

### AUDIT_BIT (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_BIT_MASK` | `0x08000000` |  |
| `AUDIT_BIT_TEST` | `(AUDIT_BIT_MASK\|AUDIT_EQUAL)` |  |

### AUDIT_BITMASK (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_BITMASK_SIZE` | `64` |  |

### AUDIT_BPRM (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_BPRM_FCAPS` | `1321` | Information about fcaps increasing perms |

### AUDIT_CLASS (10)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_CLASS_DIR_WRITE` | `0` |  |
| `AUDIT_CLASS_DIR_WRITE_32` | `1` |  |
| `AUDIT_CLASS_CHATTR` | `2` |  |
| `AUDIT_CLASS_CHATTR_32` | `3` |  |
| `AUDIT_CLASS_READ` | `4` |  |
| `AUDIT_CLASS_READ_32` | `5` |  |
| `AUDIT_CLASS_WRITE` | `6` |  |
| `AUDIT_CLASS_WRITE_32` | `7` |  |
| `AUDIT_CLASS_SIGNAL` | `8` |  |
| `AUDIT_CLASS_SIGNAL_32` | `9` |  |

### AUDIT_COMPARE (25)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_COMPARE_UID_TO_OBJ_UID` | `1` |  |
| `AUDIT_COMPARE_GID_TO_OBJ_GID` | `2` |  |
| `AUDIT_COMPARE_EUID_TO_OBJ_UID` | `3` |  |
| `AUDIT_COMPARE_EGID_TO_OBJ_GID` | `4` |  |
| `AUDIT_COMPARE_AUID_TO_OBJ_UID` | `5` |  |
| `AUDIT_COMPARE_SUID_TO_OBJ_UID` | `6` |  |
| `AUDIT_COMPARE_SGID_TO_OBJ_GID` | `7` |  |
| `AUDIT_COMPARE_FSUID_TO_OBJ_UID` | `8` |  |
| `AUDIT_COMPARE_FSGID_TO_OBJ_GID` | `9` |  |
| `AUDIT_COMPARE_UID_TO_AUID` | `10` |  |
| `AUDIT_COMPARE_UID_TO_EUID` | `11` |  |
| `AUDIT_COMPARE_UID_TO_FSUID` | `12` |  |
| `AUDIT_COMPARE_UID_TO_SUID` | `13` |  |
| `AUDIT_COMPARE_AUID_TO_FSUID` | `14` |  |
| `AUDIT_COMPARE_AUID_TO_SUID` | `15` |  |
| `AUDIT_COMPARE_AUID_TO_EUID` | `16` |  |
| `AUDIT_COMPARE_EUID_TO_SUID` | `17` |  |
| `AUDIT_COMPARE_EUID_TO_FSUID` | `18` |  |
| `AUDIT_COMPARE_SUID_TO_FSUID` | `19` |  |
| `AUDIT_COMPARE_GID_TO_EGID` | `20` |  |
| `AUDIT_COMPARE_GID_TO_FSGID` | `21` |  |
| `AUDIT_COMPARE_GID_TO_SGID` | `22` |  |
| `AUDIT_COMPARE_EGID_TO_FSGID` | `23` |  |
| `AUDIT_COMPARE_EGID_TO_SGID` | `24` |  |
| `AUDIT_COMPARE_SGID_TO_FSGID` | `25` |  |

### AUDIT_CONFIG (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_CONFIG_CHANGE` | `1305` | Audit system configuration change |

### AUDIT_DAEMON (4)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_DAEMON_START` | `1200` | Daemon startup record |
| `AUDIT_DAEMON_END` | `1201` | Daemon normal stop record |
| `AUDIT_DAEMON_ABORT` | `1202` | Daemon error stop record |
| `AUDIT_DAEMON_CONFIG` | `1203` | Daemon config change |

### AUDIT_DEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_DEL_RULE` | `1012` | Delete syscall filtering rule |

### AUDIT_DM (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_DM_CTRL` | `1338` | Device Mapper target control |
| `AUDIT_DM_EVENT` | `1339` | Device Mapper events |

### AUDIT_EVENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_EVENT_LISTENER` | `1335` | Task joined multicast read socket |

### AUDIT_FAIL (3)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_FAIL_SILENT` | `0` |  |
| `AUDIT_FAIL_PRINTK` | `1` |  |
| `AUDIT_FAIL_PANIC` | `2` |  |

### AUDIT_FD (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_FD_PAIR` | `1317` | audit record for pipe/socketpair |

### AUDIT_FEATURE (12)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_FEATURE_CHANGE` | `1328` | audit log listing feature changes |
| `AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT` | `0x00000001` |  |
| `AUDIT_FEATURE_BITMAP_BACKLOG_WAIT_TIME` | `0x00000002` |  |
| `AUDIT_FEATURE_BITMAP_EXECUTABLE_PATH` | `0x00000004` |  |
| `AUDIT_FEATURE_BITMAP_EXCLUDE_EXTEND` | `0x00000008` |  |
| `AUDIT_FEATURE_BITMAP_SESSIONID_FILTER` | `0x00000010` |  |
| `AUDIT_FEATURE_BITMAP_LOST_RESET` | `0x00000020` |  |
| `AUDIT_FEATURE_BITMAP_FILTER_FS` | `0x00000040` |  |
| `AUDIT_FEATURE_BITMAP_ALL` | `(AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT \| ` |  |
| `AUDIT_FEATURE_VERSION` | `1` |  |
| `AUDIT_FEATURE_ONLY_UNSET_LOGINUID` | `0` |  |
| `AUDIT_FEATURE_LOGINUID_IMMUTABLE` | `1` |  |

### AUDIT_FIELD (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_FIELD_COMPARE` | `111` |  |

### AUDIT_FILTER (10)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_FILTER_USER` | `0x00` | Apply rule to user-generated messages |
| `AUDIT_FILTER_TASK` | `0x01` | Apply rule at task creation (not syscall) |
| `AUDIT_FILTER_ENTRY` | `0x02` | Apply rule at syscall entry |
| `AUDIT_FILTER_WATCH` | `0x03` | Apply rule to file system watches |
| `AUDIT_FILTER_EXIT` | `0x04` | Apply rule at syscall exit |
| `AUDIT_FILTER_EXCLUDE` | `0x05` | Apply rule before record creation |
| `AUDIT_FILTER_TYPE` | `AUDIT_FILTER_EXCLUDE` | obsolete misleading naming |
| `AUDIT_FILTER_FS` | `0x06` | Apply rule at __audit_inode_child |
| `AUDIT_FILTER_URING_EXIT` | `0x07` | Apply rule at io_uring op exit |
| `AUDIT_FILTER_PREPEND` | `0x10` | Prepend to front of list |

### AUDIT_FIRST (3)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_FIRST_USER_MSG` | `1100` | Userspace messages mostly uninteresting to kernel |
| `AUDIT_FIRST_USER_MSG2` | `2100` | More user space messages |
| `AUDIT_FIRST_KERN_ANOM_MSG` | `1700` |  |

### AUDIT_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_GET_FEATURE` | `1019` | Get which features are enabled |

### AUDIT_GREATER (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_GREATER_THAN` | `0x20000000` |  |
| `AUDIT_GREATER_THAN_OR_EQUAL` | `(AUDIT_GREATER_THAN\|AUDIT_EQUAL)` |  |

### AUDIT_INTEGRITY (9)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_INTEGRITY_DATA` | `1800` | Data integrity verification |
| `AUDIT_INTEGRITY_METADATA` | `1801` | Metadata integrity verification |
| `AUDIT_INTEGRITY_STATUS` | `1802` | Integrity enable status |
| `AUDIT_INTEGRITY_HASH` | `1803` | Integrity HASH type |
| `AUDIT_INTEGRITY_PCR` | `1804` | PCR invalidation msgs |
| `AUDIT_INTEGRITY_RULE` | `1805` | policy rule |
| `AUDIT_INTEGRITY_EVM_XATTR` | `1806` | New EVM-covered xattr |
| `AUDIT_INTEGRITY_POLICY_RULE` | `1807` | IMA policy rules |
| `AUDIT_INTEGRITY_USERSPACE` | `1808` | Userspace enforced data integrity |

### AUDIT_IPC (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_IPC_SET_PERM` | `1311` | IPC new permissions record type |

### AUDIT_IPE (3)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_IPE_ACCESS` | `1420` | IPE denial or grant |
| `AUDIT_IPE_CONFIG_CHANGE` | `1421` | IPE config change |
| `AUDIT_IPE_POLICY_LOAD` | `1422` | IPE policy load |

### AUDIT_KERN (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_KERN_MODULE` | `1330` | Kernel Module events |

### AUDIT_KERNEL (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_KERNEL_OTHER` | `1316` | For use by 3rd party modules |

### AUDIT_LANDLOCK (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_LANDLOCK_ACCESS` | `1423` | Landlock denial |
| `AUDIT_LANDLOCK_DOMAIN` | `1424` | Landlock domain status |

### AUDIT_LAST (4)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_LAST_USER_MSG` | `1199` |  |
| `AUDIT_LAST_USER_MSG2` | `2999` |  |
| `AUDIT_LAST_KERN_ANOM_MSG` | `1799` |  |
| `AUDIT_LAST_FEATURE` | `AUDIT_FEATURE_LOGINUID_IMMUTABLE` |  |

### AUDIT_LESS (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_LESS_THAN` | `0x10000000` |  |
| `AUDIT_LESS_THAN_OR_EQUAL` | `(AUDIT_LESS_THAN\|AUDIT_EQUAL)` |  |

### AUDIT_LIST (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_LIST_RULES` | `1013` | List syscall filtering rules |

### AUDIT_LOGINUID (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_LOGINUID_SET` | `24` |  |

### AUDIT_MAC (19)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_MAC_POLICY_LOAD` | `1403` | Policy file load |
| `AUDIT_MAC_STATUS` | `1404` | Changed enforcing,permissive,off |
| `AUDIT_MAC_CONFIG_CHANGE` | `1405` | Changes to booleans |
| `AUDIT_MAC_UNLBL_ALLOW` | `1406` | NetLabel: allow unlabeled traffic |
| `AUDIT_MAC_CIPSOV4_ADD` | `1407` | NetLabel: add CIPSOv4 DOI entry |
| `AUDIT_MAC_CIPSOV4_DEL` | `1408` | NetLabel: del CIPSOv4 DOI entry |
| `AUDIT_MAC_MAP_ADD` | `1409` | NetLabel: add LSM domain mapping |
| `AUDIT_MAC_MAP_DEL` | `1410` | NetLabel: del LSM domain mapping |
| `AUDIT_MAC_IPSEC_ADDSA` | `1411` | Not used |
| `AUDIT_MAC_IPSEC_DELSA` | `1412` | Not used |
| `AUDIT_MAC_IPSEC_ADDSPD` | `1413` | Not used |
| `AUDIT_MAC_IPSEC_DELSPD` | `1414` | Not used |
| `AUDIT_MAC_IPSEC_EVENT` | `1415` | Audit an IPSec event |
| `AUDIT_MAC_UNLBL_STCADD` | `1416` | NetLabel: add a static label |
| `AUDIT_MAC_UNLBL_STCDEL` | `1417` | NetLabel: del a static label |
| `AUDIT_MAC_CALIPSO_ADD` | `1418` | NetLabel: add CALIPSO DOI entry |
| `AUDIT_MAC_CALIPSO_DEL` | `1419` | NetLabel: del CALIPSO DOI entry |
| `AUDIT_MAC_TASK_CONTEXTS` | `1425` | Multiple LSM task contexts |
| `AUDIT_MAC_OBJ_CONTEXTS` | `1426` | Multiple LSM objext contexts |

### AUDIT_MAKE (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_MAKE_EQUIV` | `1015` | Append to watched tree |

### AUDIT_MAX (3)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_MAX_FIELDS` | `64` |  |
| `AUDIT_MAX_KEY_LEN` | `256` |  |
| `AUDIT_MAX_FIELD_COMPARE` | `AUDIT_COMPARE_SGID_TO_FSGID` |  |

### AUDIT_MESSAGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_MESSAGE_TEXT_MAX` | `8560` |  |

### AUDIT_MQ (4)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_MQ_OPEN` | `1312` | POSIX MQ open record type |
| `AUDIT_MQ_SENDRECV` | `1313` | POSIX MQ send/receive record type |
| `AUDIT_MQ_NOTIFY` | `1314` | POSIX MQ notify record type |
| `AUDIT_MQ_GETSETATTR` | `1315` | POSIX MQ get/set attribute record type |

### AUDIT_NETFILTER (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_NETFILTER_PKT` | `1324` | Packets traversing netfilter chains |
| `AUDIT_NETFILTER_CFG` | `1325` | Netfilter chain modifications |

### AUDIT_NLGRP (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_NLGRP_MAX` | `(__AUDIT_NLGRP_MAX - 1)` |  |

### AUDIT_NOT (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_NOT_EQUAL` | `0x30000000` |  |

### AUDIT_NR (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_NR_FILTERS` | `8` |  |

### AUDIT_OBJ (8)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_OBJ_PID` | `1318` | ptrace target |
| `AUDIT_OBJ_USER` | `19` |  |
| `AUDIT_OBJ_ROLE` | `20` |  |
| `AUDIT_OBJ_TYPE` | `21` |  |
| `AUDIT_OBJ_LEV_LOW` | `22` |  |
| `AUDIT_OBJ_LEV_HIGH` | `23` |  |
| `AUDIT_OBJ_UID` | `109` |  |
| `AUDIT_OBJ_GID` | `110` |  |

### AUDIT_PERM (4)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_PERM_EXEC` | `1` |  |
| `AUDIT_PERM_WRITE` | `2` |  |
| `AUDIT_PERM_READ` | `4` |  |
| `AUDIT_PERM_ATTR` | `8` |  |

### AUDIT_SADDR (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SADDR_FAM` | `113` |  |

### AUDIT_SELINUX (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SELINUX_ERR` | `1401` | Internal SE Linux Errors |

### AUDIT_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SET_FEATURE` | `1018` | Turn an audit feature on or off |

### AUDIT_SID (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SID_UNSET` | `((unsigned int)-1)` |  |

### AUDIT_SIGNAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SIGNAL_INFO` | `1010` | Get info about sender of signal to auditd |

### AUDIT_STATUS (8)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_STATUS_ENABLED` | `0x0001` |  |
| `AUDIT_STATUS_FAILURE` | `0x0002` |  |
| `AUDIT_STATUS_PID` | `0x0004` |  |
| `AUDIT_STATUS_RATE_LIMIT` | `0x0008` |  |
| `AUDIT_STATUS_BACKLOG_LIMIT` | `0x0010` |  |
| `AUDIT_STATUS_BACKLOG_WAIT_TIME` | `0x0020` |  |
| `AUDIT_STATUS_LOST` | `0x0040` |  |
| `AUDIT_STATUS_BACKLOG_WAIT_TIME_ACTUAL` | `0x0080` |  |

### AUDIT_SUBJ (5)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SUBJ_USER` | `13` | security label user |
| `AUDIT_SUBJ_ROLE` | `14` | security label role |
| `AUDIT_SUBJ_TYPE` | `15` | security label type |
| `AUDIT_SUBJ_SEN` | `16` | security label sensitivity label |
| `AUDIT_SUBJ_CLR` | `17` | security label clearance label |

### AUDIT_SYSCALL (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_SYSCALL_CLASSES` | `16` |  |

### AUDIT_TIME (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_TIME_INJOFFSET` | `1332` | Timekeeping offset injected |
| `AUDIT_TIME_ADJNTPVAL` | `1333` | NTP value adjustment |

### AUDIT_TTY (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_TTY_GET` | `1016` | Get TTY auditing status |
| `AUDIT_TTY_SET` | `1017` | Set TTY auditing status |

### AUDIT_UID (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_UID_UNSET` | `((unsigned int)-1)` |  |

### AUDIT_UNUSED (1)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_UNUSED_BITS` | `0x07FFFC00` |  |

### AUDIT_USER (2)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_USER_AVC` | `1107` | We filter this differently |
| `AUDIT_USER_TTY` | `1124` | Non-ICANON TTY input meaning |

### AUDIT_VERSION (3)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_VERSION_LATEST` | `AUDIT_FEATURE_BITMAP_ALL` |  |
| `AUDIT_VERSION_BACKLOG_LIMIT` | `AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT` |  |
| `AUDIT_VERSION_BACKLOG_WAIT_TIME` | `AUDIT_FEATURE_BITMAP_BACKLOG_WAIT_TIME` |  |

### AUDIT_WATCH (3)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_WATCH_INS` | `1007` | Insert file/dir watch entry |
| `AUDIT_WATCH_REM` | `1008` | Remove file/dir watch entry |
| `AUDIT_WATCH_LIST` | `1009` | List all file/dir watches |

### UNCATEGORIZED (69)

| Name | Value | Comment |
|------|-------|---------|
| `AUDIT_GET` | `1000` | Get status |
| `AUDIT_SET` | `1001` | Set status (enable/disable/auditd) |
| `AUDIT_LIST` | `1002` | List syscall rules -- deprecated |
| `AUDIT_ADD` | `1003` | Add syscall rule -- deprecated |
| `AUDIT_DEL` | `1004` | Delete syscall rule -- deprecated |
| `AUDIT_USER` | `1005` | Message from userspace -- deprecated |
| `AUDIT_LOGIN` | `1006` | Define the login id and information |
| `AUDIT_TRIM` | `1014` | Trim junk from watched tree |
| `AUDIT_SYSCALL` | `1300` | Syscall event |
| `AUDIT_PATH` | `1302` | Filename path information |
| `AUDIT_IPC` | `1303` | IPC record |
| `AUDIT_SOCKETCALL` | `1304` | sys_socketcall arguments |
| `AUDIT_SOCKADDR` | `1306` | sockaddr copied as syscall arg |
| `AUDIT_CWD` | `1307` | Current working directory |
| `AUDIT_EXECVE` | `1309` | execve arguments |
| `AUDIT_TTY` | `1319` | Input on an administrative TTY |
| `AUDIT_EOE` | `1320` | End of multi-record event |
| `AUDIT_CAPSET` | `1322` | Record showing argument to sys_capset |
| `AUDIT_MMAP` | `1323` | Record showing descriptor and flags in mmap |
| `AUDIT_SECCOMP` | `1326` | Secure Computing event |
| `AUDIT_PROCTITLE` | `1327` | Proctitle emit event |
| `AUDIT_REPLACE` | `1329` | Replace auditd if this packet unanswerd |
| `AUDIT_FANOTIFY` | `1331` | Fanotify access decision |
| `AUDIT_BPF` | `1334` | BPF subsystem |
| `AUDIT_URINGOP` | `1336` | io_uring operation |
| `AUDIT_OPENAT2` | `1337` | Record showing openat2 how args |
| `AUDIT_AVC` | `1400` | SE Linux avc denial or grant |
| `AUDIT_KERNEL` | `2000` | Asynchronous audit record. NOT A REQUEST. |
| `AUDIT_NEVER` | `0` | Do not build context if rule matches |
| `AUDIT_POSSIBLE` | `1` | Build context if rule matches |
| `AUDIT_ALWAYS` | `2` | Generate audit record if rule matches |
| `AUDIT_PID` | `0` |  |
| `AUDIT_UID` | `1` |  |
| `AUDIT_EUID` | `2` |  |
| `AUDIT_SUID` | `3` |  |
| `AUDIT_FSUID` | `4` |  |
| `AUDIT_GID` | `5` |  |
| `AUDIT_EGID` | `6` |  |
| `AUDIT_SGID` | `7` |  |
| `AUDIT_FSGID` | `8` |  |
| `AUDIT_LOGINUID` | `9` |  |
| `AUDIT_PERS` | `10` |  |
| `AUDIT_ARCH` | `11` |  |
| `AUDIT_MSGTYPE` | `12` |  |
| `AUDIT_PPID` | `18` |  |
| `AUDIT_SESSIONID` | `25` | Session ID |
| `AUDIT_FSTYPE` | `26` | FileSystem Type |
| `AUDIT_DEVMAJOR` | `100` |  |
| `AUDIT_DEVMINOR` | `101` |  |
| `AUDIT_INODE` | `102` |  |

*...and 19 more*

## Structs (4)


### `struct audit_status`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mask` | `-` |
| `__u32` | `enabled` | `-` |
| `__u32` | `failure` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `rate_limit` | `-` |
| `__u32` | `backlog_limit` | `-` |
| `__u32` | `lost` | `-` |
| `__u32` | `backlog` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `feature_bitmap` | `-` |
| `__u32` | `backlog_wait_time` | `-` |
| `__u32` | `backlog_wait_time_actual` | `-` |

### `struct audit_features`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vers` | `-` |
| `__u32` | `mask` | `-` |
| `__u32` | `features` | `-` |
| `__u32` | `lock` | `-` |

### `struct audit_tty_status`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `enabled` | `-` |
| `__u32` | `log_passwd` | `-` |

### `struct audit_rule_data`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `action` | `-` |
| `__u32` | `field_count` | `-` |
| `__u32` | `mask` | `AUDIT_BITMASK_SIZE` |
| `__u32` | `fields` | `AUDIT_MAX_FIELDS` |
| `__u32` | `values` | `AUDIT_MAX_FIELDS` |
| `__u32` | `fieldflags` | `AUDIT_MAX_FIELDS` |
| `__u32` | `buflen` | `-` |