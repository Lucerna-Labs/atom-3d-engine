# fd.h

**Source:** `fd.h`


## Includes

- `linux/ioctl.h`
- `linux/compiler.h`

## Defines (66 total)


### FD_AUTODETECT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_AUTODETECT_SIZE` | `8` |  |

### FD_BROKEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_BROKEN_DCL` | `0x20` |  |

### FD_DISK (3)

| Name | Value | Comment |
|------|-------|---------|
| `FD_DISK_NEWCHANGE` | `(1 << FD_DISK_NEWCHANGE_BIT)` |  |
| `FD_DISK_CHANGED` | `(1 << FD_DISK_CHANGED_BIT)` |  |
| `FD_DISK_WRITABLE` | `(1 << FD_DISK_WRITABLE_BIT)` |  |

### FD_DRIVER (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_DRIVER_VERSION` | `0x100` |  |

### FD_FILL (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_FILL_BYTE` | `0xF6` | format fill byte. |

### FD_INVERTED (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_INVERTED_DCL` | `0x80 /* must be 0x80, because of hardware` |  |

### FD_NEED (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_NEED_TWADDLE` | `(1 << FD_NEED_TWADDLE_BIT)` |  |

### FD_RAW (18)

| Name | Value | Comment |
|------|-------|---------|
| `FD_RAW_READ` | `1` |  |
| `FD_RAW_WRITE` | `2` |  |
| `FD_RAW_NO_MOTOR` | `4` |  |
| `FD_RAW_DISK_CHANGE` | `4` | out: disk change flag was set |
| `FD_RAW_INTR` | `8` | wait for an interrupt |
| `FD_RAW_SPIN` | `0x10` | spin up the disk for this command |
| `FD_RAW_NO_MOTOR_AFTER` | `0x20 /* switch the motor off after command` |  |
| `FD_RAW_NEED_DISK` | `0x40` | this command needs a disk to be present |
| `FD_RAW_NEED_SEEK` | `0x80` | this command uses an implied seek (soft) |
| `FD_RAW_MORE` | `0x100` | more records follow |
| `FD_RAW_STOP_IF_FAILURE` | `0x200` | stop if we encounter a failure |
| `FD_RAW_STOP_IF_SUCCESS` | `0x400` | stop if command successful |
| `FD_RAW_SOFTFAILURE` | `0x800 /* consider the return value for failure` |  |
| `FD_RAW_FAILURE` | `0x10000` | command sent to fdc, fdc returned error |
| `FD_RAW_HARDFAILURE` | `0x20000` | fdc had to be reset, or timed out |
| `FD_RAW_CMD_SIZE` | `16` |  |
| `FD_RAW_REPLY_SIZE` | `16` |  |
| `FD_RAW_CMD_FULLSIZE` | `(FD_RAW_CMD_SIZE + 1 + FD_RAW_REPLY_SIZE)` |  |

### FD_SILENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `FD_SILENT_DCL_CLEAR` | `0x4` |  |

### UNCATEGORIZED (38)

| Name | Value | Comment |
|------|-------|---------|
| `FD_STRETCH` | `1` |  |
| `FD_SWAPSIDES` | `2` |  |
| `FD_ZEROBASED` | `4` |  |
| `FD_SECTBASEMASK` | `0x3FC` |  |
| `FD_2M` | `0x4` |  |
| `FD_SIZECODEMASK` | `0x38` |  |
| `FD_PERP` | `0x40` |  |
| `FDCLRPRM` | `_IO(2, 0x41)` |  |
| `FDSETPRM` | `_IOW(2, 0x42, struct floppy_struct)` |  |
| `FDSETMEDIAPRM` | `FDSETPRM` |  |
| `FDDEFPRM` | `_IOW(2, 0x43, struct floppy_struct)` |  |
| `FDGETPRM` | `_IOR(2, 0x04, struct floppy_struct)` |  |
| `FDDEFMEDIAPRM` | `FDDEFPRM` |  |
| `FDGETMEDIAPRM` | `FDGETPRM` |  |
| `FDMSGON` | `_IO(2,0x45)` |  |
| `FDMSGOFF` | `_IO(2,0x46)` |  |
| `FDFMTBEG` | `_IO(2,0x47)` |  |
| `FDFMTTRK` | `_IOW(2,0x48, struct format_descr)` |  |
| `FDFMTEND` | `_IO(2,0x49)` |  |
| `FDSETEMSGTRESH` | `_IO(2,0x4a)` |  |
| `FDFLUSH` | `_IO(2,0x4b)` |  |
| `FDSETMAXERRS` | `_IOW(2, 0x4c, struct floppy_max_errors)` |  |
| `FDGETMAXERRS` | `_IOR(2, 0x0e, struct floppy_max_errors)` |  |
| `FDGETDRVTYP` | `_IOR(2, 0x0f, floppy_drive_name)` |  |
| `FTD_MSG` | `0x10` |  |
| `FD_DEBUG` | `0x02` |  |
| `FDSETDRVPRM` | `_IOW(2, 0x90, struct floppy_drive_params)` |  |
| `FDGETDRVPRM` | `_IOR(2, 0x11, struct floppy_drive_params)` |  |
| `FD_VERIFY` | `(1 << FD_VERIFY_BIT)` |  |
| `FDGETDRVSTAT` | `_IOR(2, 0x12, struct floppy_drive_struct)` |  |
| `FDPOLLDRVSTAT` | `_IOR(2, 0x13, struct floppy_drive_struct)` |  |
| `FDRESET` | `_IO(2, 0x54)` |  |
| `FDGETFDCSTAT` | `_IOR(2, 0x15, struct floppy_fdc_state)` |  |
| `FDWERRORCLR` | `_IO(2, 0x56)` |  |
| `FDWERRORGET` | `_IOR(2, 0x17, struct floppy_write_errors)` |  |
| `FDRAWCMD` | `_IO(2, 0x58)` |  |
| `FDTWADDLE` | `_IO(2, 0x59)` |  |
| `FDEJECT` | `_IO(2, 0x5a)` |  |

## Structs (9)


### `struct floppy_struct`

| Type | Field | Array |
|------|-------|-------|

### `struct format_descr`

| Type | Field | Array |
|------|-------|-------|

### `struct floppy_max_errors`

| Type | Field | Array |
|------|-------|-------|

### `struct floppy_drive_params`

| Type | Field | Array |
|------|-------|-------|
| `char` | `flags` | `-` |
| `char` | `read_track` | `-` |
| `short` | `autodetect` | `FD_AUTODETECT_SIZE` |
| `int` | `checkfreq` | `-` |
| `int` | `native_format` | `-` |

### `struct floppy_drive_struct`

| Type | Field | Array |
|------|-------|-------|
| `short` | `probed_format` | `-` |
| `short` | `track` | `-` |
| `short` | `maxblock` | `-` |
| `short` | `maxtrack` | `-` |
| `int` | `generation` | `-` |
| `int` | `keep_data` | `-` |
| `int` | `fd_ref` | `-` |
| `int` | `fd_device` | `-` |
| `int` | `bufblocks` | `-` |

### `struct floppy_fdc_state`

| Type | Field | Array |
|------|-------|-------|
| `int` | `spec1` | `-` |
| `int` | `spec2` | `-` |
| `int` | `dtr` | `-` |

### `struct floppy_write_errors`

| Type | Field | Array |
|------|-------|-------|
| `int` | `first_error_generation` | `-` |
| `int` | `last_error_generation` | `-` |

### `struct floppy_raw_cmd`

| Type | Field | Array |
|------|-------|-------|
| `long` | `length` | `-` |
| `long` | `phys_length` | `-` |
| `int` | `buffer_length` | `-` |
| `int` | `track` | `-` |
| `int` | `resultcode` | `-` |
| `int` | `reserved1` | `-` |
| `int` | `reserved2` | `-` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|