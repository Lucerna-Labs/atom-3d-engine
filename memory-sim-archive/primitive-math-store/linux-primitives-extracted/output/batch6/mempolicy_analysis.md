# mempolicy.h

**Source:** `mempolicy.h`


## Includes

- `linux/errno.h`

## Defines (20 total)


### MPOL_F (9)

| Name | Value | Comment |
|------|-------|---------|
| `MPOL_F_STATIC_NODES` | `(1 << 15)` |  |
| `MPOL_F_RELATIVE_NODES` | `(1 << 14)` |  |
| `MPOL_F_NUMA_BALANCING` | `(1 << 13)` | Optimize with NUMA balancing if possible |
| `MPOL_F_NODE` | `(1<<0)` | return next IL mode instead of node mask |
| `MPOL_F_ADDR` | `(1<<1)` | look up vma using address |
| `MPOL_F_MEMS_ALLOWED` | `(1<<2)` | return allowed memories |
| `MPOL_F_SHARED` | `(1 << 0)` | identify shared policies |
| `MPOL_F_MOF` | `(1 << 3)` | this policy wants migrate on fault |
| `MPOL_F_MORON` | `(1 << 4)` | Migrate On protnone Reference On Node |

### MPOL_MF (6)

| Name | Value | Comment |
|------|-------|---------|
| `MPOL_MF_STRICT` | `(1<<0)` | Verify existing pages in the mapping |
| `MPOL_MF_MOVE` | `(1<<1)	/* Move pages owned by this process to conform` |  |
| `MPOL_MF_MOVE_ALL` | `(1<<2)` | Move every page to conform to policy |
| `MPOL_MF_LAZY` | `(1<<3)` | UNSUPPORTED FLAG: Lazy migrate on fault |
| `MPOL_MF_INTERNAL` | `(1<<4)` | Internal flags start here |
| `MPOL_MF_VALID` | `(MPOL_MF_STRICT   \| 	` |  |

### MPOL_MODE (1)

| Name | Value | Comment |
|------|-------|---------|
| `MPOL_MODE_FLAGS` | `` |  |

### MPOL_USER (1)

| Name | Value | Comment |
|------|-------|---------|
| `MPOL_USER_NODEMASK_FLAGS` | `(MPOL_F_STATIC_NODES \| MPOL_F_RELATIVE_NODES)` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `RECLAIM_ZONE` | `(1<<0)` | Enable zone reclaim |
| `RECLAIM_WRITE` | `(1<<1)` | Writeout pages during reclaim |
| `RECLAIM_UNMAP` | `(1<<2)` | Unmap pages during reclaim |