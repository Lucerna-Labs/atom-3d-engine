# memfd.h

**Source:** `memfd.h`


## Includes

- `asm-generic/hugetlb_encode.h`

## Defines (19 total)


### MFD_ALLOW (1)

| Name | Value | Comment |
|------|-------|---------|
| `MFD_ALLOW_SEALING` | `0x0002U` |  |

### MFD_HUGE (14)

| Name | Value | Comment |
|------|-------|---------|
| `MFD_HUGE_SHIFT` | `HUGETLB_FLAG_ENCODE_SHIFT` |  |
| `MFD_HUGE_MASK` | `HUGETLB_FLAG_ENCODE_MASK` |  |
| `MFD_HUGE_64KB` | `HUGETLB_FLAG_ENCODE_64KB` |  |
| `MFD_HUGE_512KB` | `HUGETLB_FLAG_ENCODE_512KB` |  |
| `MFD_HUGE_1MB` | `HUGETLB_FLAG_ENCODE_1MB` |  |
| `MFD_HUGE_2MB` | `HUGETLB_FLAG_ENCODE_2MB` |  |
| `MFD_HUGE_8MB` | `HUGETLB_FLAG_ENCODE_8MB` |  |
| `MFD_HUGE_16MB` | `HUGETLB_FLAG_ENCODE_16MB` |  |
| `MFD_HUGE_32MB` | `HUGETLB_FLAG_ENCODE_32MB` |  |
| `MFD_HUGE_256MB` | `HUGETLB_FLAG_ENCODE_256MB` |  |
| `MFD_HUGE_512MB` | `HUGETLB_FLAG_ENCODE_512MB` |  |
| `MFD_HUGE_1GB` | `HUGETLB_FLAG_ENCODE_1GB` |  |
| `MFD_HUGE_2GB` | `HUGETLB_FLAG_ENCODE_2GB` |  |
| `MFD_HUGE_16GB` | `HUGETLB_FLAG_ENCODE_16GB` |  |

### MFD_NOEXEC (1)

| Name | Value | Comment |
|------|-------|---------|
| `MFD_NOEXEC_SEAL` | `0x0008U` |  |

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `MFD_CLOEXEC` | `0x0001U` |  |
| `MFD_HUGETLB` | `0x0004U` |  |
| `MFD_EXEC` | `0x0010U` |  |