# adfs_fs.h

**Source:** `adfs_fs.h`


## Includes

- `linux/types.h`
- `linux/magic.h`

## Defines (4 total)


### ADFS_DR (3)

| Name | Value | Comment |
|------|-------|---------|
| `ADFS_DR_OFFSET` | `(0x1c0)` |  |
| `ADFS_DR_SIZE` | `60` |  |
| `ADFS_DR_SIZE_BITS` | `(ADFS_DR_SIZE << 3)` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `ADFS_DISCRECORD` | `(0xc00)` |  |

## Structs (1)


### `struct adfs_discrecord`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `log2secsize` | `-` |
| `__u8` | `secspertrack` | `-` |
| `__u8` | `heads` | `-` |
| `__u8` | `density` | `-` |
| `__u8` | `idlen` | `-` |
| `__u8` | `log2bpmb` | `-` |
| `__u8` | `skew` | `-` |
| `__u8` | `bootoption` | `-` |
| `__u8` | `lowsector` | `-` |
| `__u8` | `nzones` | `-` |
| `__le16` | `zone_spare` | `-` |
| `__le32` | `root` | `-` |
| `__le32` | `disc_size` | `-` |
| `__le16` | `disc_id` | `-` |
| `__u8` | `disc_name` | `10` |
| `__le32` | `disc_type` | `-` |
| `__le32` | `disc_size_high` | `-` |
| `__u8` | `nzones_high` | `-` |
| `__u8` | `reserved43` | `-` |
| `__le32` | `format_version` | `-` |
| `__le32` | `root_size` | `-` |
| `__u8` | `unused52` | `60 - 52` |