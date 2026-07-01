# pci.h

**Source:** `pci.h`


## Includes

- `linux/pci_regs.h`

## Defines (5 total)


### PCIIOC_MMAP (2)

| Name | Value | Comment |
|------|-------|---------|
| `PCIIOC_MMAP_IS_IO` | `(PCIIOC_BASE \| 0x01)` | Set mmap state to I/O space. |
| `PCIIOC_MMAP_IS_MEM` | `(PCIIOC_BASE \| 0x02)` | Set mmap state to MEM space. |

### PCIIOC_WRITE (1)

| Name | Value | Comment |
|------|-------|---------|
| `PCIIOC_WRITE_COMBINE` | `(PCIIOC_BASE \| 0x03)` | Enable/disable write-combining. |

### UNCATEGORIZED (2)

| Name | Value | Comment |
|------|-------|---------|
| `PCIIOC_BASE` | `('P' << 24 \| 'C' << 16 \| 'I' << 8)` |  |
| `PCIIOC_CONTROLLER` | `(PCIIOC_BASE \| 0x00)` | Get controller for PCI device. |