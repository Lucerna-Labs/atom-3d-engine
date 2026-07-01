# hdreg.h

**Source:** `hdreg.h`


## Includes

- `linux/types.h`

## Defines (211 total)


### CFA_ERASE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CFA_ERASE_SECTORS` | `0xC0` |  |

### CFA_REQ (1)

| Name | Value | Comment |
|------|-------|---------|
| `CFA_REQ_EXT_ERROR_CODE` | `0x03` | CFA Request Extended Error Code |

### CFA_TRANSLATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CFA_TRANSLATE_SECTOR` | `0x87` | CFA Translate Sector |

### CFA_WRITE (2)

| Name | Value | Comment |
|------|-------|---------|
| `CFA_WRITE_SECT_WO_ERASE` | `0x38` | CFA Write Sectors without erase |
| `CFA_WRITE_MULTI_WO_ERASE` | `0xCD` | CFA Write multiple without erase |

### EXABYTE_ENABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `EXABYTE_ENABLE_NEST` | `0xF0` |  |

### HDIO_DRIVE (8)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_DRIVE_CMD_HDR_SIZE` | `(4 * sizeof(__u8))` |  |
| `HDIO_DRIVE_HOB_HDR_SIZE` | `(8 * sizeof(__u8))` |  |
| `HDIO_DRIVE_TASK_HDR_SIZE` | `(8 * sizeof(__u8))` |  |
| `HDIO_DRIVE_RESET` | `0x031c` | execute a device reset |
| `HDIO_DRIVE_TASKFILE` | `0x031d` | execute raw taskfile |
| `HDIO_DRIVE_TASK` | `0x031e` | execute task and special drive command |
| `HDIO_DRIVE_CMD` | `0x031f` | execute a special drive command |
| `HDIO_DRIVE_CMD_AEB` | `HDIO_DRIVE_TASK` |  |

### HDIO_GET (13)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_GET_UNMASKINTR` | `0x0302` | get current unmask setting |
| `HDIO_GET_MULTCOUNT` | `0x0304` | get current IDE blockmode setting |
| `HDIO_GET_QDMA` | `0x0305` | get use-qdma flag |
| `HDIO_GET_KEEPSETTINGS` | `0x0308` | get keep-settings-on-reset flag |
| `HDIO_GET_32BIT` | `0x0309` | get current io_32bit setting |
| `HDIO_GET_NOWERR` | `0x030a` | get ignore-write-error flag |
| `HDIO_GET_DMA` | `0x030b` | get use-dma flag |
| `HDIO_GET_NICE` | `0x030c` | get nice flags |
| `HDIO_GET_IDENTITY` | `0x030d` | get IDE identification info |
| `HDIO_GET_WCACHE` | `0x030e` | get write cache mode on|off |
| `HDIO_GET_ACOUSTIC` | `0x030f` | get acoustic value |
| `HDIO_GET_ADDRESS` | `0x0310` |   |
| `HDIO_GET_BUSSTATE` | `0x031a` | get the bus state of the hwif |

### HDIO_OBSOLETE (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_OBSOLETE_IDENTITY` | `0x0307` | OBSOLETE, DO NOT USE: returns 142 bytes |

### HDIO_SCAN (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_SCAN_HWIF` | `0x0328` | register and (re)scan interface |

### HDIO_SET (14)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_SET_XFER` | `0x0306` | set transfer rate via proc |
| `HDIO_SET_MULTCOUNT` | `0x0321` | change IDE blockmode |
| `HDIO_SET_UNMASKINTR` | `0x0322` | permit other irqs during I/O |
| `HDIO_SET_KEEPSETTINGS` | `0x0323` | keep ioctl settings on reset |
| `HDIO_SET_32BIT` | `0x0324` | change io_32bit flags |
| `HDIO_SET_NOWERR` | `0x0325` | change ignore-write-error flag |
| `HDIO_SET_DMA` | `0x0326` | change use-dma flag |
| `HDIO_SET_PIO_MODE` | `0x0327` | reconfig interface to new speed |
| `HDIO_SET_NICE` | `0x0329` | set nice flags |
| `HDIO_SET_WCACHE` | `0x032b` | change write cache enable-disable |
| `HDIO_SET_ACOUSTIC` | `0x032c` | change acoustic behavior |
| `HDIO_SET_BUSSTATE` | `0x032d` | set the bus state of the hwif |
| `HDIO_SET_QDMA` | `0x032e` | change use-qdma flag |
| `HDIO_SET_ADDRESS` | `0x032f` | change lba addressing modes |

### HDIO_TRISTATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_TRISTATE_HWIF` | `0x031b` | execute a channel tristate |

### HDIO_UNREGISTER (1)

| Name | Value | Comment |
|------|-------|---------|
| `HDIO_UNREGISTER_HWIF` | `0x032a` | unregister interface |

### IDE_DRIVE (6)

| Name | Value | Comment |
|------|-------|---------|
| `IDE_DRIVE_TASK_NO_DATA` | `0` |  |
| `IDE_DRIVE_TASK_INVALID` | `-1` |  |
| `IDE_DRIVE_TASK_SET_XFER` | `1` |  |
| `IDE_DRIVE_TASK_IN` | `2` |  |
| `IDE_DRIVE_TASK_OUT` | `3` |  |
| `IDE_DRIVE_TASK_RAW_WRITE` | `4` |  |

### IDE_HOB (2)

| Name | Value | Comment |
|------|-------|---------|
| `IDE_HOB_STD_IN_FLAGS` | `0x3C` |  |
| `IDE_HOB_STD_OUT_FLAGS` | `0x3C` |  |

### IDE_NICE (5)

| Name | Value | Comment |
|------|-------|---------|
| `IDE_NICE_DSC_OVERLAP` | `(0)` | per the DSC overlap protocol |
| `IDE_NICE_ATAPI_OVERLAP` | `(1)` | not supported yet |
| `IDE_NICE_1` | `(3)` | when probably won't affect us much |
| `IDE_NICE_0` | `(2)` | when sure that it won't affect us |
| `IDE_NICE_2` | `(4)` | when we know it's on our expense |

### IDE_TASKFILE (2)

| Name | Value | Comment |
|------|-------|---------|
| `IDE_TASKFILE_STD_IN_FLAGS` | `0xFE` |  |
| `IDE_TASKFILE_STD_OUT_FLAGS` | `0xFE` |  |

### SECURITY_DISABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SECURITY_DISABLE_PASSWORD` | `0xBF` |  |

### SECURITY_ERASE (2)

| Name | Value | Comment |
|------|-------|---------|
| `SECURITY_ERASE_PREPARE` | `0xBC` |  |
| `SECURITY_ERASE_UNIT` | `0xBD` |  |

### SECURITY_FREEZE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SECURITY_FREEZE_LOCK` | `0xBE` |  |

### SECURITY_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `SECURITY_SET_PASSWORD` | `0xBA` |  |

### SETFEATURES_DIS (12)

| Name | Value | Comment |
|------|-------|---------|
| `SETFEATURES_DIS_DEFECT` | `0x04` | Disable Defect Management |
| `SETFEATURES_DIS_MSN` | `0x31` | Disable Media Status Notification |
| `SETFEATURES_DIS_RETRY` | `0x33` | Disable Retry |
| `SETFEATURES_DIS_RLA` | `0x55` | Disable read look-ahead feature |
| `SETFEATURES_DIS_RPOD` | `0x66` | Disable reverting to power on defaults |
| `SETFEATURES_DIS_ECC` | `0x77` | Disable ECC byte count |
| `SETFEATURES_DIS_8BIT` | `0x81` | Disable 8-Bit Transfers |
| `SETFEATURES_DIS_WCACHE` | `0x82` | Disable write cache |
| `SETFEATURES_DIS_APM` | `0x85` | Disable advanced power management |
| `SETFEATURES_DIS_AAM` | `0xC2` | Disable Automatic Acoustic Management |
| `SETFEATURES_DIS_RI` | `0xDD` | Disable release interrupt ATAPI |
| `SETFEATURES_DIS_SI` | `0xDE` | Disable SERVICE interrupt ATAPI |

### SETFEATURES_EN (15)

| Name | Value | Comment |
|------|-------|---------|
| `SETFEATURES_EN_8BIT` | `0x01` | Enable 8-Bit Transfers |
| `SETFEATURES_EN_WCACHE` | `0x02` | Enable write cache |
| `SETFEATURES_EN_APM` | `0x05` | Enable advanced power management |
| `SETFEATURES_EN_SAME_R` | `0x22` | for a region ATA-1 |
| `SETFEATURES_EN_AAM` | `0x42` | Enable Automatic Acoustic Management |
| `SETFEATURES_EN_RI` | `0x5D` | Enable release interrupt |
| `SETFEATURES_EN_SI` | `0x5E` | Enable SERVICE interrupt |
| `SETFEATURES_EN_DEFECT` | `0x84` | Enable Defect Management |
| `SETFEATURES_EN_ECC` | `0x88` | Enable ECC byte count |
| `SETFEATURES_EN_MSN` | `0x95` | Enable Media Status Notification |
| `SETFEATURES_EN_RETRY` | `0x99` | Enable Retry |
| `SETFEATURES_EN_RLA` | `0xAA` | Enable read look-ahead feature |
| `SETFEATURES_EN_REST` | `0xAC` | ATA-1 |
| `SETFEATURES_EN_RPOD` | `0xCC` | Enable reverting to power on defaults |
| `SETFEATURES_EN_SAME_M` | `0xDD` | for a entire device ATA-1 |

### SETFEATURES_RW (1)

| Name | Value | Comment |
|------|-------|---------|
| `SETFEATURES_RW_LONG` | `0x44` | Set Length of VS bytes |

### SETFEATURES_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `SETFEATURES_SET_CACHE` | `0x54` | Set Cache segments to SC Reg. Val |

### SMART_AUTO (1)

| Name | Value | Comment |
|------|-------|---------|
| `SMART_AUTO_OFFLINE` | `0xDB` |  |

### SMART_HCYL (1)

| Name | Value | Comment |
|------|-------|---------|
| `SMART_HCYL_PASS` | `0xC2` |  |

### SMART_IMMEDIATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `SMART_IMMEDIATE_OFFLINE` | `0xD4` |  |

### SMART_LCYL (1)

| Name | Value | Comment |
|------|-------|---------|
| `SMART_LCYL_PASS` | `0x4F` |  |

### SMART_READ (3)

| Name | Value | Comment |
|------|-------|---------|
| `SMART_READ_VALUES` | `0xD0` |  |
| `SMART_READ_THRESHOLDS` | `0xD1` |  |
| `SMART_READ_LOG_SECTOR` | `0xD5` |  |

### SMART_WRITE (2)

| Name | Value | Comment |
|------|-------|---------|
| `SMART_WRITE_LOG_SECTOR` | `0xD6` |  |
| `SMART_WRITE_THRESHOLDS` | `0xD7` |  |

### TASKFILE_IN (3)

| Name | Value | Comment |
|------|-------|---------|
| `TASKFILE_IN_OUT` | `0x0010` |  |
| `TASKFILE_IN_DMA` | `0x0020` |  |
| `TASKFILE_IN_DMAQ` | `0x0080` |  |

### TASKFILE_MULTI (2)

| Name | Value | Comment |
|------|-------|---------|
| `TASKFILE_MULTI_IN` | `0x0002` |  |
| `TASKFILE_MULTI_OUT` | `0x0008` |  |

### TASKFILE_NO (1)

| Name | Value | Comment |
|------|-------|---------|
| `TASKFILE_NO_DATA` | `0x0000` |  |

### TASKFILE_OUT (2)

| Name | Value | Comment |
|------|-------|---------|
| `TASKFILE_OUT_DMA` | `0x0040` |  |
| `TASKFILE_OUT_DMAQ` | `0x0100` |  |

### TASKFILE_P (6)

| Name | Value | Comment |
|------|-------|---------|
| `TASKFILE_P_IN` | `0x0200` |  |
| `TASKFILE_P_OUT` | `0x0400` |  |
| `TASKFILE_P_IN_DMA` | `0x0800` |  |
| `TASKFILE_P_OUT_DMA` | `0x1000` |  |
| `TASKFILE_P_IN_DMAQ` | `0x2000` |  |
| `TASKFILE_P_OUT_DMAQ` | `0x4000` |  |

### UNCATEGORIZED (54)

| Name | Value | Comment |
|------|-------|---------|
| `TASKFILE_IN` | `0x0001` |  |
| `TASKFILE_OUT` | `0x0004` |  |
| `TASKFILE_48` | `0x8000` |  |
| `TASKFILE_INVALID` | `0x7fff` |  |
| `WIN_NOP` | `0x00` |  |
| `WIN_SRST` | `0x08` | ATAPI soft reset command |
| `WIN_RECAL` | `0x10` |  |
| `WIN_RESTORE` | `WIN_RECAL` |  |
| `WIN_READ` | `0x20` | 28-Bit |
| `WIN_WRITE` | `0x30` | 28-Bit |
| `WIN_VERIFY` | `0x40` | 28-Bit - Read Verify Sectors |
| `WIN_FORMAT` | `0x50` |  |
| `WIN_INIT` | `0x60` |  |
| `WIN_SEEK` | `0x70` | 0x70-0x7F Reserved |
| `WIN_DIAGNOSE` | `0x90` |  |
| `WIN_SPECIFY` | `0x91` | set drive geometry translation |
| `WIN_STANDBYNOW2` | `0x94` |  |
| `WIN_STANDBY2` | `0x96` |  |
| `WIN_SETIDLE2` | `0x97` |  |
| `WIN_CHECKPOWERMODE2` | `0x98` |  |
| `WIN_SLEEPNOW2` | `0x99` |  |
| `WIN_PACKETCMD` | `0xA0` | Send a packet command. |
| `WIN_PIDENTIFY` | `0xA1` | identify ATAPI device |
| `WIN_SMART` | `0xB0` | self-monitoring and reporting |
| `WIN_MULTREAD` | `0xC4` | read sectors using multiple mode |
| `WIN_MULTWRITE` | `0xC5` | write sectors using multiple mode |
| `WIN_SETMULT` | `0xC6` | enable/disable multiple mode |
| `WIN_READDMA` | `0xC8` | read sectors using DMA transfers |
| `WIN_WRITEDMA` | `0xCA` | write sectors using DMA transfers |
| `WIN_GETMEDIASTATUS` | `0xDA` |  |
| `WIN_ACKMEDIACHANGE` | `0xDB` | ATA-1, ATA-2 vendor |
| `WIN_POSTBOOT` | `0xDC` |  |
| `WIN_PREBOOT` | `0xDD` |  |
| `WIN_DOORLOCK` | `0xDE` | lock door on removable drives |
| `WIN_DOORUNLOCK` | `0xDF` | unlock door on removable drives |
| `WIN_STANDBYNOW1` | `0xE0` |  |
| `WIN_IDLEIMMEDIATE` | `0xE1` | force drive to become "ready" |
| `WIN_STANDBY` | `0xE2` | Set device in Standby Mode |
| `WIN_SETIDLE1` | `0xE3` |  |
| `WIN_CHECKPOWERMODE1` | `0xE5` |  |
| `WIN_SLEEPNOW1` | `0xE6` |  |
| `WIN_IDENTIFY` | `0xEC` | ask drive to identify itself |
| `WIN_MEDIAEJECT` | `0xED` |  |
| `WIN_SETFEATURES` | `0xEF` | set special drive features |
| `DISABLE_SEAGATE` | `0xFB` |  |
| `SMART_AUTOSAVE` | `0xD2` |  |
| `SMART_SAVE` | `0xD3` |  |
| `SMART_ENABLE` | `0xD8` |  |
| `SMART_DISABLE` | `0xD9` |  |
| `SMART_STATUS` | `0xDA` |  |

*...and 4 more*

### WIN_DEVICE (1)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_DEVICE_RESET` | `0x08` |  |

### WIN_DOWNLOAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_DOWNLOAD_MICROCODE` | `0x92` |  |

### WIN_FLUSH (2)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_FLUSH_CACHE` | `0xE7` |  |
| `WIN_FLUSH_CACHE_EXT` | `0xEA` | 48-Bit |

### WIN_IDENTIFY (1)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_IDENTIFY_DMA` | `0xEE` | same as WIN_IDENTIFY, but DMA |

### WIN_MULTREAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_MULTREAD_EXT` | `0x29` | 48-Bit |

### WIN_MULTWRITE (1)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_MULTWRITE_EXT` | `0x39` | 48-Bit |

### WIN_QUEUED (1)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_QUEUED_SERVICE` | `0xA2` |  |

### WIN_READ (7)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_READ_ONCE` | `0x21` | 28-Bit without retries |
| `WIN_READ_LONG` | `0x22` | 28-Bit |
| `WIN_READ_LONG_ONCE` | `0x23` | 28-Bit without retries |
| `WIN_READ_EXT` | `0x24` | 48-Bit |
| `WIN_READ_NATIVE_MAX_EXT` | `0x27` | 48-Bit |
| `WIN_READ_BUFFER` | `0xE4` | force read only 1 sector |
| `WIN_READ_NATIVE_MAX` | `0xF8` | return the native maximum address |

### WIN_READDMA (4)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_READDMA_EXT` | `0x25` | 48-Bit |
| `WIN_READDMA_QUEUED_EXT` | `0x26` | 48-Bit |
| `WIN_READDMA_QUEUED` | `0xC7` | read sectors using Queued DMA transfers |
| `WIN_READDMA_ONCE` | `0xC9` | 28-Bit - without retries |

### WIN_SECURITY (6)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_SECURITY_SET_PASS` | `0xF1` |  |
| `WIN_SECURITY_UNLOCK` | `0xF2` |  |
| `WIN_SECURITY_ERASE_PREPARE` | `0xF3` |  |
| `WIN_SECURITY_ERASE_UNIT` | `0xF4` |  |
| `WIN_SECURITY_FREEZE_LOCK` | `0xF5` |  |
| `WIN_SECURITY_DISABLE` | `0xF6` |  |

### WIN_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_SET_MAX_EXT` | `0x37` | 48-Bit |
| `WIN_SET_MAX` | `0xF9` |  |

### WIN_VERIFY (2)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_VERIFY_ONCE` | `0x41` | 28-Bit - without retries |
| `WIN_VERIFY_EXT` | `0x42` | 48-Bit |

### WIN_WRITE (7)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_WRITE_ONCE` | `0x31` | 28-Bit without retries |
| `WIN_WRITE_LONG` | `0x32` | 28-Bit |
| `WIN_WRITE_LONG_ONCE` | `0x33` | 28-Bit without retries |
| `WIN_WRITE_EXT` | `0x34` | 48-Bit |
| `WIN_WRITE_VERIFY` | `0x3C` | 28-Bit |
| `WIN_WRITE_BUFFER` | `0xE8` | force write only 1 sector |
| `WIN_WRITE_SAME` | `0xE9` | read ata-2 to use |

### WIN_WRITEDMA (4)

| Name | Value | Comment |
|------|-------|---------|
| `WIN_WRITEDMA_EXT` | `0x35` | 48-Bit |
| `WIN_WRITEDMA_QUEUED_EXT` | `0x36` | 48-Bit |
| `WIN_WRITEDMA_ONCE` | `0xCB` | 28-Bit - without retries |
| `WIN_WRITEDMA_QUEUED` | `0xCC` | write sectors using Queued DMA transfers |

## Structs (8)


### `struct anonymous_0`

| Type | Field | Array |
|------|-------|-------|

### `struct ide_task_request_s`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `io_ports` | `8` |
| `__u8` | `hob_ports` | `8` |
| `ide_reg_valid_t` | `out_flags` | `-` |
| `ide_reg_valid_t` | `in_flags` | `-` |
| `int` | `data_phase` | `-` |
| `int` | `req_cmd` | `-` |

### `struct ide_ioctl_request_s`

| Type | Field | Array |
|------|-------|-------|

### `struct hd_drive_cmd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `command` | `-` |
| `__u8` | `sector_number` | `-` |
| `__u8` | `feature` | `-` |
| `__u8` | `sector_count` | `-` |

### `struct hd_drive_task_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `data` | `-` |
| `__u8` | `feature` | `-` |
| `__u8` | `sector_count` | `-` |
| `__u8` | `sector_number` | `-` |
| `__u8` | `low_cylinder` | `-` |
| `__u8` | `high_cylinder` | `-` |
| `__u8` | `device_head` | `-` |
| `__u8` | `command` | `-` |

### `struct hd_drive_hob_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `data` | `-` |
| `__u8` | `feature` | `-` |
| `__u8` | `sector_count` | `-` |
| `__u8` | `sector_number` | `-` |
| `__u8` | `low_cylinder` | `-` |
| `__u8` | `high_cylinder` | `-` |
| `__u8` | `device_head` | `-` |
| `__u8` | `control` | `-` |

### `struct hd_geometry`

| Type | Field | Array |
|------|-------|-------|

### `struct hd_driveid`

| Type | Field | Array |
|------|-------|-------|

## Typedefs

- `ide_reg_valid_s`
- `ide_task_request_s`
- `ide_ioctl_request_s`
- `hd_drive_task_hdr`
- `hd_drive_hob_hdr`