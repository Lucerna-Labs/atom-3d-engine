# auxvec.h

**Source:** `auxvec.h`


## Includes

- `asm/auxvec.h`

## Defines (28 total)


### AT_BASE (1)

| Name | Value | Comment |
|------|-------|---------|
| `AT_BASE_PLATFORM` | `24	/* string identifying real platform, may` |  |

### AT_RSEQ (2)

| Name | Value | Comment |
|------|-------|---------|
| `AT_RSEQ_FEATURE_SIZE` | `27` | rseq supported feature size |
| `AT_RSEQ_ALIGN` | `28` | rseq allocation alignment |

### UNCATEGORIZED (25)

| Name | Value | Comment |
|------|-------|---------|
| `AT_NULL` | `0` | end of vector |
| `AT_IGNORE` | `1` | entry should be ignored |
| `AT_EXECFD` | `2` | file descriptor of program |
| `AT_PHDR` | `3` | program headers for program |
| `AT_PHENT` | `4` | size of program header entry |
| `AT_PHNUM` | `5` | number of program headers |
| `AT_PAGESZ` | `6` | system page size |
| `AT_BASE` | `7` | base address of interpreter |
| `AT_FLAGS` | `8` | flags |
| `AT_ENTRY` | `9` | entry point of program |
| `AT_NOTELF` | `10` | program is not ELF |
| `AT_UID` | `11` | real uid |
| `AT_EUID` | `12` | effective uid |
| `AT_GID` | `13` | real gid |
| `AT_EGID` | `14` | effective gid |
| `AT_PLATFORM` | `15` | string identifying CPU for optimizations |
| `AT_HWCAP` | `16` | arch dependent hints at CPU capabilities |
| `AT_CLKTCK` | `17` | frequency at which times() increments |
| `AT_SECURE` | `23` | secure mode boolean |
| `AT_RANDOM` | `25` | address of 16 random bytes |
| `AT_HWCAP2` | `26` | extension of AT_HWCAP |
| `AT_HWCAP3` | `29` | extension of AT_HWCAP |
| `AT_HWCAP4` | `30` | extension of AT_HWCAP |
| `AT_EXECFN` | `31` | filename of program |
| `AT_MINSIGSTKSZ` | `51` | minimal stack size for signal delivery |