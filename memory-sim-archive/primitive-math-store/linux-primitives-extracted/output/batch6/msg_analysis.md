# msg.h

**Source:** `msg.h`


## Includes

- `linux/ipc.h`
- `asm/msgbuf.h`

## Defines (15 total)


### MSG_STAT (1)

| Name | Value | Comment |
|------|-------|---------|
| `MSG_STAT_ANY` | `13` |  |

### UNCATEGORIZED (14)

| Name | Value | Comment |
|------|-------|---------|
| `MSG_STAT` | `11` |  |
| `MSG_INFO` | `12` |  |
| `MSG_NOERROR` | `010000` | no error if message is too big |
| `MSG_EXCEPT` | `020000` | recv any msg except of specified type. |
| `MSG_COPY` | `040000` | copy (not remove) all queue messages |
| `MSGMNI` | `32000` | <= IPCMNI */     /* max # of msg queue identifiers |
| `MSGMAX` | `8192` | <= INT_MAX */   /* max size of message (bytes) |
| `MSGMNB` | `16384` | <= INT_MAX */   /* default max size of a message queue |
| `MSGPOOL` | `(MSGMNI * MSGMNB / 1024)` | size in kbytes of message pool |
| `MSGTQL` | `MSGMNB` | number of system message headers |
| `MSGMAP` | `MSGMNB` | number of entries in message map |
| `MSGSSZ` | `16` | message segment size |
| `__MSGSEG` | `((MSGPOOL * 1024) / MSGSSZ)` | max no. of segments |
| `MSGSEG` | `(__MSGSEG <= 0xffff ? __MSGSEG : 0xffff)` |  |

## Structs (3)


### `struct msqid_ds`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_old_time_t` | `msg_stime` | `-` |
| `__kernel_old_time_t` | `msg_rtime` | `-` |
| `__kernel_old_time_t` | `msg_ctime` | `-` |
| `__kernel_ipc_pid_t` | `msg_lspid` | `-` |
| `__kernel_ipc_pid_t` | `msg_lrpid` | `-` |

### `struct msgbuf`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_long_t` | `mtype` | `-` |
| `char` | `mtext` | `1` |

### `struct msginfo`

| Type | Field | Array |
|------|-------|-------|
| `int` | `msgpool` | `-` |
| `int` | `msgmap` | `-` |
| `int` | `msgmax` | `-` |
| `int` | `msgmnb` | `-` |
| `int` | `msgmni` | `-` |
| `int` | `msgssz` | `-` |
| `int` | `msgtql` | `-` |