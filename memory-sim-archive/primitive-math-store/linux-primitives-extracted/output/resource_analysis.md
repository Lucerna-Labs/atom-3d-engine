# resource.h

**Source:** `resource.h`


## Includes

- `linux/time_types.h`
- `linux/types.h`
- `asm/resource.h`

## Defines (12 total)


### UNCATEGORIZED (12)

| Name | Value | Comment |
|------|-------|---------|
| `RUSAGE_SELF` | `0` |  |
| `RUSAGE_CHILDREN` | `(-1)` |  |
| `RUSAGE_BOTH` | `(-2)` | sys_wait4() uses this |
| `RUSAGE_THREAD` | `1` | only the calling thread |
| `RLIM64_INFINITY` | `(~0ULL)` |  |
| `PRIO_MIN` | `(-20)` |  |
| `PRIO_MAX` | `20` |  |
| `PRIO_PROCESS` | `0` |  |
| `PRIO_PGRP` | `1` |  |
| `PRIO_USER` | `2` |  |
| `_STK_LIM` | `(8*1024*1024)` |  |
| `MLOCK_LIMIT` | `(8*1024*1024)` |  |

## Structs (3)


### `struct rusage`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_long_t` | `ru_maxrss` | `-` |
| `__kernel_long_t` | `ru_ixrss` | `-` |
| `__kernel_long_t` | `ru_idrss` | `-` |
| `__kernel_long_t` | `ru_isrss` | `-` |
| `__kernel_long_t` | `ru_minflt` | `-` |
| `__kernel_long_t` | `ru_majflt` | `-` |
| `__kernel_long_t` | `ru_nswap` | `-` |
| `__kernel_long_t` | `ru_inblock` | `-` |
| `__kernel_long_t` | `ru_oublock` | `-` |
| `__kernel_long_t` | `ru_msgsnd` | `-` |
| `__kernel_long_t` | `ru_msgrcv` | `-` |
| `__kernel_long_t` | `ru_nsignals` | `-` |
| `__kernel_long_t` | `ru_nvcsw` | `-` |
| `__kernel_long_t` | `ru_nivcsw` | `-` |

### `struct rlimit`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_ulong_t` | `rlim_cur` | `-` |
| `__kernel_ulong_t` | `rlim_max` | `-` |

### `struct rlimit64`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rlim_cur` | `-` |
| `__u64` | `rlim_max` | `-` |