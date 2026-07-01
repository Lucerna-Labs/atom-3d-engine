# romfs_fs.h

**Source:** `romfs_fs.h`


## Includes

- `linux/types.h`
- `linux/fs.h`

## Defines (20 total)


### UNCATEGORIZED (20)

| Name | Value | Comment |
|------|-------|---------|
| `ROMBSIZE` | `BLOCK_SIZE` |  |
| `ROMBSBITS` | `BLOCK_SIZE_BITS` |  |
| `ROMBMASK` | `(ROMBSIZE-1)` |  |
| `ROMFS_MAGIC` | `0x7275` |  |
| `ROMFS_MAXFN` | `128` |  |
| `ROMSB_WORD0` | `__mk4('-','r','o','m')` |  |
| `ROMSB_WORD1` | `__mk4('1','f','s','-')` |  |
| `ROMFH_TYPE` | `7` |  |
| `ROMFH_HRD` | `0` |  |
| `ROMFH_DIR` | `1` |  |
| `ROMFH_REG` | `2` |  |
| `ROMFH_SYM` | `3` |  |
| `ROMFH_BLK` | `4` |  |
| `ROMFH_CHR` | `5` |  |
| `ROMFH_SCK` | `6` |  |
| `ROMFH_FIF` | `7` |  |
| `ROMFH_EXEC` | `8` |  |
| `ROMFH_SIZE` | `16` |  |
| `ROMFH_PAD` | `(ROMFH_SIZE-1)` |  |
| `ROMFH_MASK` | `(~ROMFH_PAD)` |  |

## Structs (2)


### `struct romfs_super_block`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `word0` | `-` |
| `__be32` | `word1` | `-` |
| `__be32` | `size` | `-` |
| `__be32` | `checksum` | `-` |

### `struct romfs_inode`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `next` | `-` |
| `__be32` | `spec` | `-` |
| `__be32` | `size` | `-` |
| `__be32` | `checksum` | `-` |