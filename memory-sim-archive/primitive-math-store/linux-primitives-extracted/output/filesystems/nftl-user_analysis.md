# nftl-user.h

**Source:** `nftl-user.h`


## Includes

- `linux/types.h`

## Defines (10 total)


### FOLD_MARK (1)

| Name | Value | Comment |
|------|-------|---------|
| `FOLD_MARK_IN_PROGRESS` | `0x5555` |  |

### MAX_ERASE (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_ERASE_ZONES` | `(8192 - 512)` |  |

### UNCATEGORIZED (6)

| Name | Value | Comment |
|------|-------|---------|
| `ERASE_MARK` | `0x3c69` |  |
| `SECTOR_FREE` | `0xff` |  |
| `SECTOR_USED` | `0x55` |  |
| `SECTOR_IGNORE` | `0x11` |  |
| `SECTOR_DELETED` | `0x00` |  |
| `ZONE_GOOD` | `0xff` |  |

### ZONE_BAD (2)

| Name | Value | Comment |
|------|-------|---------|
| `ZONE_BAD_ORIGINAL` | `0` |  |
| `ZONE_BAD_MARKED` | `7` |  |

## Structs (6)


### `struct nftl_bci`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `Status` | `-` |
| `__u8` | `Status1` | `-` |

### `struct nftl_uci0`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `VirtUnitNum` | `-` |
| `__u16` | `ReplUnitNum` | `-` |
| `__u16` | `SpareVirtUnitNum` | `-` |
| `__u16` | `SpareReplUnitNum` | `-` |

### `struct nftl_uci1`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `WearInfo` | `-` |
| `__u16` | `EraseMark` | `-` |
| `__u16` | `EraseMark1` | `-` |

### `struct nftl_uci2`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `FoldMark` | `-` |
| `__u16` | `FoldMark1` | `-` |
| `__u32` | `unused` | `-` |

### `struct nftl_oob`

| Type | Field | Array |
|------|-------|-------|

### `struct NFTLMediaHeader`

| Type | Field | Array |
|------|-------|-------|
| `char` | `DataOrgID` | `6` |
| `__u16` | `NumEraseUnits` | `-` |
| `__u16` | `FirstPhysicalEUN` | `-` |
| `__u32` | `FormattedSize` | `-` |