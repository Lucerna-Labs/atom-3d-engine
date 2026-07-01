# mman.h

**Source:** `mman.h`


## Includes

- `asm/mman.h`
- `asm-generic/hugetlb_encode.h`
- `linux/types.h`

## Defines (25 total)


### MAP_HUGE (15)

| Name | Value | Comment |
|------|-------|---------|
| `MAP_HUGE_SHIFT` | `HUGETLB_FLAG_ENCODE_SHIFT` |  |
| `MAP_HUGE_MASK` | `HUGETLB_FLAG_ENCODE_MASK` |  |
| `MAP_HUGE_16KB` | `HUGETLB_FLAG_ENCODE_16KB` |  |
| `MAP_HUGE_64KB` | `HUGETLB_FLAG_ENCODE_64KB` |  |
| `MAP_HUGE_512KB` | `HUGETLB_FLAG_ENCODE_512KB` |  |
| `MAP_HUGE_1MB` | `HUGETLB_FLAG_ENCODE_1MB` |  |
| `MAP_HUGE_2MB` | `HUGETLB_FLAG_ENCODE_2MB` |  |
| `MAP_HUGE_8MB` | `HUGETLB_FLAG_ENCODE_8MB` |  |
| `MAP_HUGE_16MB` | `HUGETLB_FLAG_ENCODE_16MB` |  |
| `MAP_HUGE_32MB` | `HUGETLB_FLAG_ENCODE_32MB` |  |
| `MAP_HUGE_256MB` | `HUGETLB_FLAG_ENCODE_256MB` |  |
| `MAP_HUGE_512MB` | `HUGETLB_FLAG_ENCODE_512MB` |  |
| `MAP_HUGE_1GB` | `HUGETLB_FLAG_ENCODE_1GB` |  |
| `MAP_HUGE_2GB` | `HUGETLB_FLAG_ENCODE_2GB` |  |
| `MAP_HUGE_16GB` | `HUGETLB_FLAG_ENCODE_16GB` |  |

### MAP_SHARED (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAP_SHARED_VALIDATE` | `0x03` | share + validate extension flags |

### UNCATEGORIZED (9)

| Name | Value | Comment |
|------|-------|---------|
| `MREMAP_MAYMOVE` | `1` |  |
| `MREMAP_FIXED` | `2` |  |
| `MREMAP_DONTUNMAP` | `4` |  |
| `OVERCOMMIT_GUESS` | `0` |  |
| `OVERCOMMIT_ALWAYS` | `1` |  |
| `OVERCOMMIT_NEVER` | `2` |  |
| `MAP_SHARED` | `0x01` | Share changes |
| `MAP_PRIVATE` | `0x02` | Changes are private |
| `MAP_DROPPABLE` | `0x08` | Zero memory under memory pressure. |

## Structs (2)


### `struct cachestat_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `off` | `-` |
| `__u64` | `len` | `-` |

### `struct cachestat`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `nr_cache` | `-` |
| `__u64` | `nr_dirty` | `-` |
| `__u64` | `nr_writeback` | `-` |
| `__u64` | `nr_evicted` | `-` |
| `__u64` | `nr_recently_evicted` | `-` |