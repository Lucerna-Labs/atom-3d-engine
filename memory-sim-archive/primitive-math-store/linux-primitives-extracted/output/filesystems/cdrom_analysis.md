# cdrom.h

**Source:** `cdrom.h`


## Includes

- `linux/types.h`
- `asm/byteorder.h`

## Defines (229 total)


### CDC_CD (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_CD_R` | `0x2000` | drive is a CD-R |
| `CDC_CD_RW` | `0x4000` | drive is a CD-RW |

### CDC_CLOSE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_CLOSE_TRAY` | `0x1` | caddy systems _can't_ close |

### CDC_DRIVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_DRIVE_STATUS` | `0x800` | driver implements drive status |

### CDC_DVD (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_DVD_R` | `0x10000` | drive can write DVD-R |
| `CDC_DVD_RAM` | `0x20000` | drive can write DVD-RAM |

### CDC_GENERIC (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_GENERIC_PACKET` | `0x1000` | driver implements generic packets |

### CDC_MEDIA (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_MEDIA_CHANGED` | `0x80` | media changed |

### CDC_MO (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_MO_DRIVE` | `0x40000` | drive is an MO device |

### CDC_MRW (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_MRW_W` | `0x100000` | drive can write MRW |

### CDC_MULTI (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_MULTI_SESSION` | `0x20` | read sessions>1 |

### CDC_OPEN (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_OPEN_TRAY` | `0x2` | but _can_ eject. |

### CDC_PLAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_PLAY_AUDIO` | `0x100` | audio functions |

### CDC_SELECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDC_SELECT_SPEED` | `0x8` | programmable speed |
| `CDC_SELECT_DISC` | `0x10` | select disc from juke-box |

### CDM_MRW (4)

| Name | Value | Comment |
|------|-------|---------|
| `CDM_MRW_NOTMRW` | `0` |  |
| `CDM_MRW_BGFORMAT_INACTIVE` | `1` |  |
| `CDM_MRW_BGFORMAT_ACTIVE` | `2` |  |
| `CDM_MRW_BGFORMAT_COMPLETE` | `3` |  |

### CDO_AUTO (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDO_AUTO_CLOSE` | `0x1` | close tray on first open() |
| `CDO_AUTO_EJECT` | `0x2` | open tray on last release() |

### CDO_CHECK (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDO_CHECK_TYPE` | `0x10` | check type on open for data |

### CDO_USE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDO_USE_FFLAGS` | `0x4` | use O_NONBLOCK information on open |

### CDROM_AUDIO (6)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_AUDIO_INVALID` | `0x00` | audio status not supported |
| `CDROM_AUDIO_PLAY` | `0x11` | audio play operation in progress |
| `CDROM_AUDIO_PAUSED` | `0x12` | audio play operation paused |
| `CDROM_AUDIO_COMPLETED` | `0x13` | audio play successfully completed |
| `CDROM_AUDIO_ERROR` | `0x14` | audio play stopped due to error |
| `CDROM_AUDIO_NO_STATUS` | `0x15` | no current audio status to return |

### CDROM_CHANGER (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_CHANGER_NSLOTS` | `0x5328` | Get number of slots |

### CDROM_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_CLEAR_OPTIONS` | `0x5321` | Clear behavior options |

### CDROM_DATA (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_DATA_TRACK` | `0x04` |  |

### CDROM_DISC (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_DISC_STATUS` | `0x5327` | Get disc type, etc. |

### CDROM_DRIVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_DRIVE_STATUS` | `0x5326` | Get tray position, etc. |

### CDROM_GET (3)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_GET_MCN` | `0x5311 /* Obtain the "Universal Product Code"` |  |
| `CDROM_GET_UPC` | `CDROM_GET_MCN  /* This one is deprecated,` |  |
| `CDROM_GET_CAPABILITY` | `0x5331` | get capabilities |

### CDROM_LAST (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_LAST_WRITTEN` | `0x5395` | get last block written on disc |

### CDROM_MEDIA (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_MEDIA_CHANGED` | `0x5325` | Check is media changed |

### CDROM_NEXT (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_NEXT_WRITABLE` | `0x5394` | get next writable block |

### CDROM_PACKET (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_PACKET_SIZE` | `12` |  |

### CDROM_SELECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_SELECT_SPEED` | `0x5322` | Set the CD-ROM speed |
| `CDROM_SELECT_DISC` | `0x5323` | Select disc (for juke-boxes) |

### CDROM_SEND (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_SEND_PACKET` | `0x5393` | send a packet to the drive |

### CDROM_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_SET_OPTIONS` | `0x5320` | Set behavior options |

### CDROM_TIMED (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDROM_TIMED_MEDIA_CHANGE` | `0x5396` | get the timestamp of the last media change |

### CDS_DATA (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDS_DATA_1` | `101` |  |
| `CDS_DATA_2` | `102` |  |

### CDS_DISC (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDS_DISC_OK` | `4` |  |

### CDS_DRIVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDS_DRIVE_NOT_READY` | `3` |  |

### CDS_NO (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDS_NO_INFO` | `0` | if not implemented |
| `CDS_NO_DISC` | `1` |  |

### CDS_TRAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `CDS_TRAY_OPEN` | `2` |  |

### CDS_XA (2)

| Name | Value | Comment |
|------|-------|---------|
| `CDS_XA_2_1` | `103` |  |
| `CDS_XA_2_2` | `104` |  |

### CD_CHUNK (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_CHUNK_SIZE` | `24` | lowest-level "data bytes piece" |

### CD_ECC (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_ECC_SIZE` | `276` | bytes ECC per most raw data frame types |

### CD_EDC (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_EDC_SIZE` | `4` | bytes EDC per most raw data frame types |

### CD_FRAMESIZE (5)

| Name | Value | Comment |
|------|-------|---------|
| `CD_FRAMESIZE_SUB` | `96` | subchannel data "frame" size |
| `CD_FRAMESIZE_RAW` | `2352` | bytes per frame, "raw" mode |
| `CD_FRAMESIZE_RAWER` | `2646` | The maximum possible returned bytes |
| `CD_FRAMESIZE_RAW1` | `(CD_FRAMESIZE_RAW-CD_SYNC_SIZE)` | 2340 |
| `CD_FRAMESIZE_RAW0` | `(CD_FRAMESIZE_RAW-CD_SYNC_SIZE-CD_HEAD_SIZE)` | 2336 |

### CD_HEAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_HEAD_SIZE` | `4` | header (address) bytes per raw data frame |

### CD_MSF (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_MSF_OFFSET` | `150` | MSF numbering offset of first frame |

### CD_NUM (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_NUM_OF_CHUNKS` | `98` | chunks per frame |

### CD_PART (2)

| Name | Value | Comment |
|------|-------|---------|
| `CD_PART_MAX` | `64` |  |
| `CD_PART_MASK` | `(CD_PART_MAX - 1)` |  |

### CD_SUBHEAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_SUBHEAD_SIZE` | `8` | subheader bytes per raw XA data frame |

### CD_SYNC (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_SYNC_SIZE` | `12` | 12 sync bytes per raw data frame |

### CD_XA (3)

| Name | Value | Comment |
|------|-------|---------|
| `CD_XA_HEAD` | `(CD_HEAD_SIZE+CD_SUBHEAD_SIZE)` | "before data" part of raw XA frame |
| `CD_XA_TAIL` | `(CD_EDC_SIZE+CD_ECC_SIZE)` | "after data" part of raw XA frame |
| `CD_XA_SYNC_HEAD` | `(CD_SYNC_SIZE+CD_XA_HEAD)` | sync bytes + header of XA frame |

### CD_ZERO (1)

| Name | Value | Comment |
|------|-------|---------|
| `CD_ZERO_SIZE` | `8` | bytes zero per yellow book mode 1 frame |

### CGC_DATA (4)

| Name | Value | Comment |
|------|-------|---------|
| `CGC_DATA_UNKNOWN` | `0` |  |
| `CGC_DATA_WRITE` | `1` |  |
| `CGC_DATA_READ` | `2` |  |
| `CGC_DATA_NONE` | `3` |  |

### DVD_AUTH (2)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_AUTH_ESTABLISHED` | `5` |  |
| `DVD_AUTH_FAILURE` | `6` |  |

### DVD_CGMS (3)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_CGMS_UNRESTRICTED` | `0` |  |
| `DVD_CGMS_SINGLE` | `2` |  |
| `DVD_CGMS_RESTRICTED` | `3` |  |

### DVD_CP (2)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_CP_SEC_NONE` | `0` |  |
| `DVD_CP_SEC_EXIST` | `1` |  |

### DVD_CPM (2)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_CPM_NO_COPYRIGHT` | `0` |  |
| `DVD_CPM_COPYRIGHTED` | `1` |  |

### DVD_HOST (3)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_HOST_SEND_CHALLENGE` | `1` |  |
| `DVD_HOST_SEND_KEY2` | `4` |  |
| `DVD_HOST_SEND_RPC_STATE` | `11` |  |

### DVD_INVALIDATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_INVALIDATE_AGID` | `9` |  |

### DVD_LU (6)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_LU_SEND_AGID` | `0` |  |
| `DVD_LU_SEND_KEY1` | `2` |  |
| `DVD_LU_SEND_CHALLENGE` | `3` |  |
| `DVD_LU_SEND_TITLE_KEY` | `7` |  |
| `DVD_LU_SEND_ASF` | `8` |  |
| `DVD_LU_SEND_RPC_STATE` | `10` |  |

### DVD_READ (1)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_READ_STRUCT` | `0x5390` | Read structure |

### DVD_STRUCT (5)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_STRUCT_PHYSICAL` | `0x00` |  |
| `DVD_STRUCT_COPYRIGHT` | `0x01` |  |
| `DVD_STRUCT_DISCKEY` | `0x02` |  |
| `DVD_STRUCT_BCA` | `0x03` |  |
| `DVD_STRUCT_MANUFACT` | `0x04` |  |

### DVD_WRITE (1)

| Name | Value | Comment |
|------|-------|---------|
| `DVD_WRITE_STRUCT` | `0x5391` | Write structure |

### EDRIVE_CANT (1)

| Name | Value | Comment |
|------|-------|---------|
| `EDRIVE_CANT_DO_THIS` | `EOPNOTSUPP` |  |

### GPCMD_CLOSE (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_CLOSE_TRACK` | `0x5b` |  |

### GPCMD_FLUSH (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_FLUSH_CACHE` | `0x35` |  |

### GPCMD_FORMAT (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_FORMAT_UNIT` | `0x04` |  |

### GPCMD_GET (4)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_GET_CONFIGURATION` | `0x46` |  |
| `GPCMD_GET_EVENT_STATUS_NOTIFICATION` | `0x4a` |  |
| `GPCMD_GET_PERFORMANCE` | `0xac` |  |
| `GPCMD_GET_MEDIA_STATUS` | `0xda` |  |

### GPCMD_LOAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_LOAD_UNLOAD` | `0xa6` |  |

### GPCMD_MECHANISM (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_MECHANISM_STATUS` | `0xbd` |  |

### GPCMD_MODE (2)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_MODE_SELECT_10` | `0x55` |  |
| `GPCMD_MODE_SENSE_10` | `0x5a` |  |

### GPCMD_PAUSE (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_PAUSE_RESUME` | `0x4b` |  |

### GPCMD_PLAY (4)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_PLAY_AUDIO_10` | `0x45` |  |
| `GPCMD_PLAY_AUDIO_MSF` | `0x47` |  |
| `GPCMD_PLAY_AUDIO_TI` | `0x48` |  |
| `GPCMD_PLAY_CD` | `0xbc` |  |

### GPCMD_PLAYAUDIO (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_PLAYAUDIO_TI` | `0x48` |  |

### GPCMD_PREVENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_PREVENT_ALLOW_MEDIUM_REMOVAL` | `0x1e` |  |

### GPCMD_READ (14)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_READ_10` | `0x28` |  |
| `GPCMD_READ_12` | `0xa8` |  |
| `GPCMD_READ_BUFFER` | `0x3c` |  |
| `GPCMD_READ_BUFFER_CAPACITY` | `0x5c` |  |
| `GPCMD_READ_CDVD_CAPACITY` | `0x25` |  |
| `GPCMD_READ_CD` | `0xbe` |  |
| `GPCMD_READ_CD_MSF` | `0xb9` |  |
| `GPCMD_READ_DISC_INFO` | `0x51` |  |
| `GPCMD_READ_DVD_STRUCTURE` | `0xad` |  |
| `GPCMD_READ_FORMAT_CAPACITIES` | `0x23` |  |
| `GPCMD_READ_HEADER` | `0x44` |  |
| `GPCMD_READ_TRACK_RZONE_INFO` | `0x52` |  |
| `GPCMD_READ_SUBCHANNEL` | `0x42` |  |
| `GPCMD_READ_TOC_PMA_ATIP` | `0x43` |  |

### GPCMD_REPAIR (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_REPAIR_RZONE_TRACK` | `0x58` |  |

### GPCMD_REPORT (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_REPORT_KEY` | `0xa4` |  |

### GPCMD_REQUEST (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_REQUEST_SENSE` | `0x03` |  |

### GPCMD_RESERVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_RESERVE_RZONE_TRACK` | `0x53` |  |

### GPCMD_SEND (5)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_SEND_CUE_SHEET` | `0x5d` |  |
| `GPCMD_SEND_DVD_STRUCTURE` | `0xbf` |  |
| `GPCMD_SEND_EVENT` | `0xa2` |  |
| `GPCMD_SEND_KEY` | `0xa3` |  |
| `GPCMD_SEND_OPC` | `0x54` |  |

### GPCMD_SET (3)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_SET_READ_AHEAD` | `0xa7` |  |
| `GPCMD_SET_STREAMING` | `0xb6` |  |
| `GPCMD_SET_SPEED` | `0xbb` |  |

### GPCMD_START (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_START_STOP_UNIT` | `0x1b` |  |

### GPCMD_STOP (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_STOP_PLAY_SCAN` | `0x4e` |  |

### GPCMD_TEST (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_TEST_UNIT_READY` | `0x00` |  |

### GPCMD_VERIFY (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_VERIFY_10` | `0x2f` |  |

### GPCMD_WRITE (4)

| Name | Value | Comment |
|------|-------|---------|
| `GPCMD_WRITE_10` | `0x2a` |  |
| `GPCMD_WRITE_12` | `0xaa` |  |
| `GPCMD_WRITE_AND_VERIFY_10` | `0x2e` |  |
| `GPCMD_WRITE_BUFFER` | `0x3b` |  |

### GPMODE_ALL (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_ALL_PAGES` | `0x3f` |  |

### GPMODE_AUDIO (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_AUDIO_CTL_PAGE` | `0x0e` |  |

### GPMODE_CAPABILITIES (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_CAPABILITIES_PAGE` | `0x2a` |  |

### GPMODE_CDROM (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_CDROM_PAGE` | `0x0d` |  |

### GPMODE_FAULT (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_FAULT_FAIL_PAGE` | `0x1c` |  |

### GPMODE_POWER (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_POWER_PAGE` | `0x1a` |  |

### GPMODE_R (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_R_W_ERROR_PAGE` | `0x01` |  |

### GPMODE_TO (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_TO_PROTECT_PAGE` | `0x1d` |  |

### GPMODE_VENDOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_VENDOR_PAGE` | `0x00` |  |

### GPMODE_WCACHING (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_WCACHING_PAGE` | `0x08` |  |

### GPMODE_WRITE (1)

| Name | Value | Comment |
|------|-------|---------|
| `GPMODE_WRITE_PARMS_PAGE` | `0x05` |  |

### MEDIA_CHANGED (1)

| Name | Value | Comment |
|------|-------|---------|
| `MEDIA_CHANGED_FLAG` | `0x1	/* Last detected media change was more` |  |

### MRW_LBA (2)

| Name | Value | Comment |
|------|-------|---------|
| `MRW_LBA_DMA` | `0` |  |
| `MRW_LBA_GAA` | `1` |  |

### MRW_MODE (2)

| Name | Value | Comment |
|------|-------|---------|
| `MRW_MODE_PC_PRE1` | `0x2c` |  |
| `MRW_MODE_PC` | `0x03` |  |

### UNCATEGORIZED (56)

| Name | Value | Comment |
|------|-------|---------|
| `CDROMPAUSE` | `0x5301` | Pause Audio Operation |
| `CDROMRESUME` | `0x5302` | Resume paused Audio Operation |
| `CDROMPLAYMSF` | `0x5303` | Play Audio MSF (struct cdrom_msf) |
| `CDROMPLAYTRKIND` | `0x5304 /* Play Audio Track/index` |  |
| `CDROMREADTOCHDR` | `0x5305 /* Read TOC header` |  |
| `CDROMREADTOCENTRY` | `0x5306 /* Read TOC entry` |  |
| `CDROMSTOP` | `0x5307` | Stop the cdrom drive |
| `CDROMSTART` | `0x5308` | Start the cdrom drive |
| `CDROMEJECT` | `0x5309` | Ejects the cdrom media |
| `CDROMVOLCTRL` | `0x530a /* Control output volume` |  |
| `CDROMSUBCHNL` | `0x530b /* Read subchannel data` |  |
| `CDROMREADMODE2` | `0x530c /* Read CDROM mode 2 data (2336 Bytes)` |  |
| `CDROMREADMODE1` | `0x530d /* Read CDROM mode 1 data (2048 Bytes)` |  |
| `CDROMREADAUDIO` | `0x530e` | (struct cdrom_read_audio) |
| `CDROMEJECT_SW` | `0x530f` | enable(1)/disable(0) auto-ejecting |
| `CDROMMULTISESSION` | `0x5310 /* Obtain the start-of-last-session` |  |
| `CDROMRESET` | `0x5312` | hard-reset the drive |
| `CDROMVOLREAD` | `0x5313 /* Get the drive's volume setting` |  |
| `CDROMREADRAW` | `0x5314	/* read data in raw mode (2352 Bytes)` |  |
| `CDROMREADCOOKED` | `0x5315` | read data in cooked mode |
| `CDROMSEEK` | `0x5316` | seek msf address |
| `CDROMPLAYBLK` | `0x5317` | (struct cdrom_blk) |
| `CDROMREADALL` | `0x5318` | read all 2646 bytes |
| `CDROMGETSPINDOWN` | `0x531d` |  |
| `CDROMSETSPINDOWN` | `0x531e` |  |
| `CDROMCLOSETRAY` | `0x5319` | pendant of CDROMEJECT |
| `CDROM_LOCKDOOR` | `0x5329` | lock or unlock door |
| `CDROM_DEBUG` | `0x5330` | Turn debug messages on/off |
| `CDROMAUDIOBUFSIZ` | `0x5382` | set the audio buffer size |
| `DVD_AUTH` | `0x5392` | Authentication |
| `CD_MINS` | `74` | max. minutes per CD, not really a limit |
| `CD_SECS` | `60` | seconds per minute |
| `CD_FRAMES` | `75` | frames per second |
| `CD_FRAMESIZE` | `2048` | bytes per frame, "cooked" mode |
| `CDROM_LBA` | `0x01` | "logical block": first frame is #0 |
| `CDROM_MSF` | `0x02` | "minute-second-frame": binary, not bcd here! |
| `CDROM_LEADOUT` | `0xAA` |  |
| `CDC_LOCK` | `0x4` | disable manual eject |
| `CDC_MCN` | `0x40` | Medium Catalog Number |
| `CDC_RESET` | `0x200` | hard reset device |
| `CDC_DVD` | `0x8000` | drive is a DVD |
| `CDC_MRW` | `0x80000` | drive can read MRW |
| `CDC_RAM` | `0x200000` | ok to open for WRITE |
| `CDS_AUDIO` | `100` |  |
| `CDS_MIXED` | `105` |  |
| `CDO_LOCK` | `0x8` | lock tray on open files |
| `CDSL_NONE` | `(INT_MAX-1)` |  |
| `CDSL_CURRENT` | `INT_MAX` |  |
| `GPCMD_BLANK` | `0xa1` |  |
| `GPCMD_INQUIRY` | `0x12` |  |

*...and 6 more*

## Structs (36)


### `struct cdrom_msf0`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `minute` | `-` |
| `__u8` | `second` | `-` |
| `__u8` | `frame` | `-` |

### `struct cdrom_msf`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cdmsf_min0` | `-` |
| `__u8` | `cdmsf_sec0` | `-` |
| `__u8` | `cdmsf_frame0` | `-` |
| `__u8` | `cdmsf_min1` | `-` |
| `__u8` | `cdmsf_sec1` | `-` |
| `__u8` | `cdmsf_frame1` | `-` |

### `struct cdrom_ti`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cdti_trk0` | `-` |
| `__u8` | `cdti_ind0` | `-` |
| `__u8` | `cdti_trk1` | `-` |
| `__u8` | `cdti_ind1` | `-` |

### `struct cdrom_tochdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cdth_trk0` | `-` |
| `__u8` | `cdth_trk1` | `-` |

### `struct cdrom_volctrl`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `channel0` | `-` |
| `__u8` | `channel1` | `-` |
| `__u8` | `channel2` | `-` |
| `__u8` | `channel3` | `-` |

### `struct cdrom_subchnl`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cdsc_format` | `-` |
| `__u8` | `cdsc_audiostatus` | `-` |
| `__u8` | `cdsc_trk` | `-` |
| `__u8` | `cdsc_ind` | `-` |

### `struct cdrom_tocentry`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cdte_track` | `-` |
| `__u8` | `cdte_format` | `-` |
| `__u8` | `cdte_datamode` | `-` |

### `struct cdrom_read`

| Type | Field | Array |
|------|-------|-------|
| `int` | `cdread_lba` | `-` |
| `int` | `cdread_buflen` | `-` |

### `struct cdrom_read_audio`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `addr_format` | `-` |
| `int` | `nframes` | `-` |

### `struct cdrom_multisession`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `xa_flag` | `-` |
| `__u8` | `addr_format` | `-` |

### `struct cdrom_mcn`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `medium_catalog_number` | `14` |

### `struct cdrom_blk`

| Type | Field | Array |
|------|-------|-------|
| `unsigned` | `from` | `-` |

### `struct cdrom_generic_command`

| Type | Field | Array |
|------|-------|-------|
| `int` | `stat` | `-` |
| `int` | `quiet` | `-` |
| `int` | `timeout` | `-` |

### `struct cdrom_timed_media_change_info`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `last_media_change` | `-` |
| `__u64` | `media_flags` | `-` |

### `struct dvd_layer`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `start_sector` | `-` |
| `__u32` | `end_sector` | `-` |
| `__u32` | `end_sector_l0` | `-` |

### `struct dvd_physical`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `layer_num` | `-` |

### `struct dvd_copyright`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `layer_num` | `-` |
| `__u8` | `cpst` | `-` |
| `__u8` | `rmi` | `-` |

### `struct dvd_disckey`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `value` | `2048` |

### `struct dvd_bca`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `int` | `len` | `-` |
| `__u8` | `value` | `188` |

### `struct dvd_manufact`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `layer_num` | `-` |
| `int` | `len` | `-` |
| `__u8` | `value` | `2048` |

### `struct dvd_lu_send_agid`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |

### `struct dvd_host_send_challenge`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `dvd_challenge` | `chal` | `-` |

### `struct dvd_send_key`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `dvd_key` | `key` | `-` |

### `struct dvd_lu_send_challenge`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `dvd_challenge` | `chal` | `-` |

### `struct dvd_lu_send_title_key`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `dvd_key` | `title_key` | `-` |
| `int` | `lba` | `-` |

### `struct dvd_lu_send_asf`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |

### `struct dvd_host_send_rpcstate`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `pdrc` | `-` |

### `struct dvd_lu_send_rpcstate`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `region_mask` | `-` |
| `__u8` | `rpc_scheme` | `-` |

### `struct request_sense`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `segment_number` | `-` |
| `__u8` | `information` | `4` |
| `__u8` | `add_sense_len` | `-` |
| `__u8` | `command_info` | `4` |
| `__u8` | `asc` | `-` |
| `__u8` | `ascq` | `-` |
| `__u8` | `fruc` | `-` |
| `__u8` | `sks` | `3` |
| `__u8` | `asb` | `46` |

### `struct mrw_feature_desc`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `feature_code` | `-` |
| `__u8` | `add_len` | `-` |
| `__u8` | `reserved3` | `-` |
| `__u8` | `reserved4` | `-` |
| `__u8` | `reserved5` | `-` |

### `struct rwrt_feature_desc`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `feature_code` | `-` |
| `__u8` | `add_len` | `-` |
| `__u32` | `last_lba` | `-` |
| `__u32` | `block_size` | `-` |
| `__u16` | `blocking` | `-` |
| `__u8` | `reserved3` | `-` |

### `struct anonymous_31`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `disc_information_length` | `-` |
| `__u8` | `n_first_track` | `-` |
| `__u8` | `n_sessions_lsb` | `-` |
| `__u8` | `first_track_lsb` | `-` |
| `__u8` | `last_track_lsb` | `-` |
| `__u8` | `disc_type` | `-` |
| `__u8` | `n_sessions_msb` | `-` |
| `__u8` | `first_track_msb` | `-` |
| `__u8` | `last_track_msb` | `-` |
| `__u32` | `disc_id` | `-` |
| `__u32` | `lead_in` | `-` |
| `__u32` | `lead_out` | `-` |
| `__u8` | `disc_bar_code` | `8` |
| `__u8` | `reserved3` | `-` |
| `__u8` | `n_opc` | `-` |

### `struct anonymous_32`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `track_information_length` | `-` |
| `__u8` | `track_lsb` | `-` |
| `__u8` | `session_lsb` | `-` |
| `__u8` | `reserved1` | `-` |
| `__be32` | `track_start` | `-` |
| `__be32` | `next_writable` | `-` |
| `__be32` | `free_blocks` | `-` |
| `__be32` | `fixed_packet_size` | `-` |
| `__be32` | `track_size` | `-` |
| `__be32` | `last_rec_address` | `-` |

### `struct feature_header`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `data_len` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u8` | `reserved2` | `-` |
| `__u16` | `curr_profile` | `-` |

### `struct mode_page_header`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `mode_data_length` | `-` |
| `__u8` | `medium_type` | `-` |
| `__u8` | `reserved1` | `-` |
| `__u8` | `reserved2` | `-` |
| `__u8` | `reserved3` | `-` |
| `__be16` | `desc_length` | `-` |

### `struct rm_feature_desc`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `feature_code` | `-` |
| `__u8` | `add_len` | `-` |
| `__u8` | `reserved2` | `-` |
| `__u8` | `reserved3` | `-` |
| `__u8` | `reserved4` | `-` |