# ipc.h

**Source:** `ipc.h`


## Includes

- `linux/types.h`
- `asm/ipcbuf.h`

## Defines (25 total)


### UNCATEGORIZED (25)

| Name | Value | Comment |
|------|-------|---------|
| `IPC_PRIVATE` | `((__kernel_key_t) 0)` |  |
| `IPC_CREAT` | `00001000` | create if key is nonexistent |
| `IPC_EXCL` | `00002000` | fail if key exists |
| `IPC_NOWAIT` | `00004000` | return error on wait |
| `IPC_DIPC` | `00010000` | make it distributed |
| `IPC_OWN` | `00020000` | this machine is the DIPC owner |
| `IPC_RMID` | `0` | remove resource |
| `IPC_SET` | `1` | set ipc_perm options |
| `IPC_STAT` | `2` | get ipc_perm options |
| `IPC_INFO` | `3` | see ipcs |
| `IPC_OLD` | `0	/* Old version (no 32-bit UID support on many` |  |
| `IPC_64` | `0x0100  /* New version (support 32-bit UIDs, bigger` |  |
| `SEMOP` | `1` |  |
| `SEMGET` | `2` |  |
| `SEMCTL` | `3` |  |
| `SEMTIMEDOP` | `4` |  |
| `MSGSND` | `11` |  |
| `MSGRCV` | `12` |  |
| `MSGGET` | `13` |  |
| `MSGCTL` | `14` |  |
| `SHMAT` | `21` |  |
| `SHMDT` | `22` |  |
| `SHMGET` | `23` |  |
| `SHMCTL` | `24` |  |
| `DIPC` | `25` |  |

## Structs (2)


### `struct ipc_perm`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_key_t` | `key` | `-` |
| `__kernel_uid_t` | `uid` | `-` |
| `__kernel_gid_t` | `gid` | `-` |
| `__kernel_uid_t` | `cuid` | `-` |
| `__kernel_gid_t` | `cgid` | `-` |
| `__kernel_mode_t` | `mode` | `-` |

### `struct ipc_kludge`

| Type | Field | Array |
|------|-------|-------|
| `long` | `msgtyp` | `-` |