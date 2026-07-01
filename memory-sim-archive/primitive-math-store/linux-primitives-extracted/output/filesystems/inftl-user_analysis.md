# inftl-user.h

**Source:** `inftl-user.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### UNCATEGORIZED (6)

| Name | Value | Comment |
|------|-------|---------|
| `OSAK_VERSION` | `0x5120` |  |
| `PERCENTUSED` | `98` |  |
| `SECTORSIZE` | `512` |  |
| `INFTL_BINARY` | `0x20000000` |  |
| `INFTL_BDTL` | `0x40000000` |  |
| `INFTL_LAST` | `0x80000000` |  |

## Structs (7)


### `struct inftl_bci`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `ECCsig` | `6` |
| `__u8` | `Status` | `-` |
| `__u8` | `Status1` | `-` |

### `struct inftl_unithead1`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `virtualUnitNo` | `-` |
| `__u16` | `prevUnitNo` | `-` |
| `__u8` | `ANAC` | `-` |
| `__u8` | `NACs` | `-` |
| `__u8` | `parityPerField` | `-` |
| `__u8` | `discarded` | `-` |

### `struct inftl_unithead2`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `parityPerField` | `-` |
| `__u8` | `ANAC` | `-` |
| `__u16` | `prevUnitNo` | `-` |
| `__u16` | `virtualUnitNo` | `-` |
| `__u8` | `NACs` | `-` |
| `__u8` | `discarded` | `-` |

### `struct inftl_unittail`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `Reserved` | `4` |
| `__u16` | `EraseMark` | `-` |
| `__u16` | `EraseMark1` | `-` |

### `struct inftl_oob`

| Type | Field | Array |
|------|-------|-------|

### `struct INFTLPartition`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `virtualUnits` | `-` |
| `__u32` | `firstUnit` | `-` |
| `__u32` | `lastUnit` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `spareUnits` | `-` |
| `__u32` | `Reserved0` | `-` |
| `__u32` | `Reserved1` | `-` |

### `struct INFTLMediaHeader`

| Type | Field | Array |
|------|-------|-------|
| `char` | `bootRecordID` | `8` |
| `__u32` | `NoOfBootImageBlocks` | `-` |
| `__u32` | `NoOfBinaryPartitions` | `-` |
| `__u32` | `NoOfBDTLPartitions` | `-` |
| `__u32` | `BlockMultiplierBits` | `-` |
| `__u32` | `FormatFlags` | `-` |
| `__u32` | `OsakVersion` | `-` |
| `__u32` | `PercentUsed` | `-` |