# mtio.h

**Source:** `mtio.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (95 total)


### MT_ISARCHIVE (4)

| Name | Value | Comment |
|------|-------|---------|
| `MT_ISARCHIVE_5945L2` | `0x04` | Archive 5945L-2, QIC-24, QIC-02? |
| `MT_ISARCHIVE_VP60I` | `0x07` | Archive VP60i, QIC-02 |
| `MT_ISARCHIVE_2150L` | `0x08` | Archive Viper 2150L |
| `MT_ISARCHIVE_2060L` | `0x09` | Archive Viper 2060L |

### MT_ISEVEREX (1)

| Name | Value | Comment |
|------|-------|---------|
| `MT_ISEVEREX_FT40A` | `0x32` | Everex FT40A (QIC-40) |

### MT_ISFTAPE (2)

| Name | Value | Comment |
|------|-------|---------|
| `MT_ISFTAPE_UNKNOWN` | `0x800000` | obsolete |
| `MT_ISFTAPE_FLAG` | `0x800000` |  |

### MT_ISONSTREAM (1)

| Name | Value | Comment |
|------|-------|---------|
| `MT_ISONSTREAM_SC` | `0x61   /* OnStream SCSI tape drives (SC-x0)` |  |

### MT_ISTEAC (1)

| Name | Value | Comment |
|------|-------|---------|
| `MT_ISTEAC_MT2ST` | `0x12` | Teac MT-2ST 155mb drive, Teac DC-1 card (Wangtek type) |

### MT_ST (38)

| Name | Value | Comment |
|------|-------|---------|
| `MT_ST_BLKSIZE_SHIFT` | `0` |  |
| `MT_ST_BLKSIZE_MASK` | `0xffffff` |  |
| `MT_ST_DENSITY_SHIFT` | `24` |  |
| `MT_ST_DENSITY_MASK` | `0xff000000` |  |
| `MT_ST_SOFTERR_SHIFT` | `0` |  |
| `MT_ST_SOFTERR_MASK` | `0xffff` |  |
| `MT_ST_OPTIONS` | `0xf0000000` |  |
| `MT_ST_BOOLEANS` | `0x10000000` |  |
| `MT_ST_SETBOOLEANS` | `0x30000000` |  |
| `MT_ST_CLEARBOOLEANS` | `0x40000000` |  |
| `MT_ST_WRITE_THRESHOLD` | `0x20000000` |  |
| `MT_ST_DEF_BLKSIZE` | `0x50000000` |  |
| `MT_ST_DEF_OPTIONS` | `0x60000000` |  |
| `MT_ST_TIMEOUTS` | `0x70000000` |  |
| `MT_ST_SET_TIMEOUT` | `(MT_ST_TIMEOUTS \| 0x000000)` |  |
| `MT_ST_SET_LONG_TIMEOUT` | `(MT_ST_TIMEOUTS \| 0x100000)` |  |
| `MT_ST_SET_CLN` | `0x80000000` |  |
| `MT_ST_BUFFER_WRITES` | `0x1` |  |
| `MT_ST_ASYNC_WRITES` | `0x2` |  |
| `MT_ST_READ_AHEAD` | `0x4` |  |
| `MT_ST_DEBUGGING` | `0x8` |  |
| `MT_ST_TWO_FM` | `0x10` |  |
| `MT_ST_FAST_MTEOM` | `0x20` |  |
| `MT_ST_AUTO_LOCK` | `0x40` |  |
| `MT_ST_DEF_WRITES` | `0x80` |  |
| `MT_ST_CAN_BSR` | `0x100` |  |
| `MT_ST_NO_BLKLIMS` | `0x200` |  |
| `MT_ST_CAN_PARTITIONS` | `0x400` |  |
| `MT_ST_SCSI2LOGICAL` | `0x800` |  |
| `MT_ST_SYSV` | `0x1000` |  |
| `MT_ST_NOWAIT` | `0x2000` |  |
| `MT_ST_SILI` | `0x4000` |  |
| `MT_ST_NOWAIT_EOF` | `0x8000` |  |
| `MT_ST_CLEAR_DEFAULT` | `0xfffff` |  |
| `MT_ST_DEF_DENSITY` | `(MT_ST_DEF_OPTIONS \| 0x100000)` |  |
| `MT_ST_DEF_COMPRESSION` | `(MT_ST_DEF_OPTIONS \| 0x200000)` |  |
| `MT_ST_DEF_DRVBUFFER` | `(MT_ST_DEF_OPTIONS \| 0x300000)` |  |
| `MT_ST_HPLOADER_OFFSET` | `10000` |  |

### UNCATEGORIZED (48)

| Name | Value | Comment |
|------|-------|---------|
| `MTRESET` | `0` | +reset drive in case of problems |
| `MTFSF` | `1	/* forward space over FileMark,` |  |
| `MTBSF` | `2` | backward space FileMark (position before FM) |
| `MTFSR` | `3` | forward space record |
| `MTBSR` | `4` | backward space record |
| `MTWEOF` | `5` | write an end-of-file record (mark) |
| `MTREW` | `6` | rewind |
| `MTOFFL` | `7` | rewind and put the drive offline (eject?) |
| `MTNOP` | `8` | no op, set status only (read with MTIOCGET) |
| `MTRETEN` | `9` | retension tape |
| `MTBSFM` | `10` | +backward space FileMark, position at FM |
| `MTFSFM` | `11` | +forward space FileMark, position at FM |
| `MTEOM` | `12	/* goto end of recorded media (for appending files).` |  |
| `MTERASE` | `13` | erase tape -- be careful! |
| `MTRAS1` | `14` | run self test 1 (nondestructive) |
| `MTRAS2` | `15` | run self test 2 (destructive) |
| `MTRAS3` | `16` | reserved for self test 3 |
| `MTSETBLK` | `20` | set block length (SCSI) |
| `MTSETDENSITY` | `21` | set tape density (SCSI) |
| `MTSEEK` | `22` | seek to block (Tandberg, etc.) |
| `MTTELL` | `23` | tell block (Tandberg, etc.) |
| `MTSETDRVBUFFER` | `24` | set the drive buffering according to SCSI-2 |
| `MTFSS` | `25` | space forward over setmarks |
| `MTBSS` | `26` | space backward over setmarks |
| `MTWSM` | `27` | write setmarks |
| `MTLOCK` | `28` | lock the drive door |
| `MTUNLOCK` | `29` | unlock the drive door |
| `MTLOAD` | `30` | execute the SCSI load command |
| `MTUNLOAD` | `31` | execute the SCSI unload command |
| `MTCOMPRESSION` | `32` | control compression with SCSI mode page 15 |
| `MTSETPART` | `33` | Change the active tape partition |
| `MTMKPART` | `34` | Format the tape with one or two partitions |
| `MTWEOFI` | `35` | write an end-of-file record (mark) in immediate mode |
| `MT_ISUNKNOWN` | `0x01` |  |
| `MT_ISQIC02` | `0x02` | Generic QIC-02 tape streamer |
| `MT_ISWT5150` | `0x03` | Wangtek 5150EQ, QIC-150, QIC-02 |
| `MT_ISCMSJ500` | `0x05` | CMS Jumbo 500 (QIC-02?) |
| `MT_ISTDC3610` | `0x06` | Tandberg 6310, QIC-24 |
| `MT_ISARCHIVESC499` | `0x0A` | Archive SC-499 QIC-36 controller |
| `MT_ISQIC02_ALL_FEATURES` | `0x0F` | Generic QIC-02 with all features |
| `MT_ISWT5099EEN24` | `0x11` | Wangtek 5099-een24, 60MB, QIC-24 |
| `MT_ISDDS1` | `0x51` | DDS device without partitions |
| `MT_ISDDS2` | `0x52` | DDS device with partitions |
| `MT_ISSCSI1` | `0x71` | Generic ANSI SCSI-1 tape unit |
| `MT_ISSCSI2` | `0x72` | Generic ANSI SCSI-2 tape unit |
| `MTIOCTOP` | `_IOW('m', 1, struct mtop)` | do a mag tape op |
| `MTIOCGET` | `_IOR('m', 2, struct mtget)` | get tape status |
| `MTIOCPOS` | `_IOR('m', 3, struct mtpos)` | get tape position |

## Structs (3)


### `struct mtop`

| Type | Field | Array |
|------|-------|-------|
| `short` | `mt_op` | `-` |
| `int` | `mt_count` | `-` |

### `struct mtget`

| Type | Field | Array |
|------|-------|-------|
| `long` | `mt_type` | `-` |
| `long` | `mt_resid` | `-` |
| `long` | `mt_dsreg` | `-` |
| `long` | `mt_gstat` | `-` |
| `long` | `mt_erreg` | `-` |
| `__kernel_daddr_t` | `mt_fileno` | `-` |
| `__kernel_daddr_t` | `mt_blkno` | `-` |

### `struct mtpos`

| Type | Field | Array |
|------|-------|-------|
| `long` | `mt_blkno` | `-` |