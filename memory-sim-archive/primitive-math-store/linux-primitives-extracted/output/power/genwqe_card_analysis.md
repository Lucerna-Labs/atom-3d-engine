# genwqe_card.h

**Source:** `genwqe_card.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`

## Defines (175 total)


### ATS_TYPE (5)

| Name | Value | Comment |
|------|-------|---------|
| `ATS_TYPE_DATA` | `0x0ull` | data |
| `ATS_TYPE_FLAT_RD` | `0x4ull` | flat buffer read only |
| `ATS_TYPE_FLAT_RDWR` | `0x5ull` | flat buffer read/write |
| `ATS_TYPE_SGL_RD` | `0x6ull` | sgl read only |
| `ATS_TYPE_SGL_RDWR` | `0x7ull` | sgl read/write |

### DDCB_ACFUNC (2)

| Name | Value | Comment |
|------|-------|---------|
| `DDCB_ACFUNC_SLU` | `0x00` | chip service layer unit |
| `DDCB_ACFUNC_APP` | `0x01` | chip application |

### DDCB_ASIV (2)

| Name | Value | Comment |
|------|-------|---------|
| `DDCB_ASIV_LENGTH` | `104` | len of the DDCB ASIV array |
| `DDCB_ASIV_LENGTH_ATS` | `96` | ASIV in ATS architecture |

### DDCB_ASV (1)

| Name | Value | Comment |
|------|-------|---------|
| `DDCB_ASV_LENGTH` | `64` | len of the DDCB ASV array |

### DDCB_OPT (10)

| Name | Value | Comment |
|------|-------|---------|
| `DDCB_OPT_ECHO_FORCE_NO` | `0x0000` | ECHO DDCB |
| `DDCB_OPT_ECHO_FORCE_102` | `0x0001` | force return code |
| `DDCB_OPT_ECHO_FORCE_104` | `0x0002` |  |
| `DDCB_OPT_ECHO_FORCE_108` | `0x0003` |  |
| `DDCB_OPT_ECHO_FORCE_110` | `0x0004` | only on PF ! |
| `DDCB_OPT_ECHO_FORCE_120` | `0x0005` |  |
| `DDCB_OPT_ECHO_FORCE_140` | `0x0006` |  |
| `DDCB_OPT_ECHO_FORCE_180` | `0x0007` |  |
| `DDCB_OPT_ECHO_COPY_NONE` | `(0 << 5)` |  |
| `DDCB_OPT_ECHO_COPY_ALL` | `(1 << 5)` |  |

### DDCB_RETC (10)

| Name | Value | Comment |
|------|-------|---------|
| `DDCB_RETC_IDLE` | `0x0000` | Unexecuted/DDCB created |
| `DDCB_RETC_PENDING` | `0x0101` | Pending Execution |
| `DDCB_RETC_COMPLETE` | `0x0102` | Cmd complete. No error |
| `DDCB_RETC_FAULT` | `0x0104` | App Err, recoverable |
| `DDCB_RETC_ERROR` | `0x0108` | App Err, non-recoverable |
| `DDCB_RETC_FORCED_ERROR` | `0x01ff` | overwritten by driver |
| `DDCB_RETC_UNEXEC` | `0x0110` | Unexe/Removed from queue |
| `DDCB_RETC_TERM` | `0x0120` | Terminated |
| `DDCB_RETC_RES0` | `0x0140` | Reserved |
| `DDCB_RETC_RES1` | `0x0180` | Reserved |

### GENWQE_APP (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_APP_OFFS` | `GENWQE_UID_OFFS(2)` |  |

### GENWQE_EXECUTE (2)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_EXECUTE_DDCB` | `` |  |
| `GENWQE_EXECUTE_RAW_DDCB` | `` |  |

### GENWQE_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_GET_CARD_STATE` | `_IOR(GENWQE_IOC_CODE, 36,	enum genwqe_card_state)` |  |

### GENWQE_HSU (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_HSU_OFFS` | `GENWQE_UID_OFFS(1)` |  |

### GENWQE_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_IOC_CODE` | `0xa5` |  |

### GENWQE_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_MAX_UNITS` | `3` |  |

### GENWQE_PIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_PIN_MEM` | `_IOWR(GENWQE_IOC_CODE, 40, struct genwqe_mem)` |  |

### GENWQE_READ (3)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_READ_REG64` | `_IOR(GENWQE_IOC_CODE, 30, struct genwqe_reg_io)` |  |
| `GENWQE_READ_REG32` | `_IOR(GENWQE_IOC_CODE, 32, struct genwqe_reg_io)` |  |
| `GENWQE_READ_REG16` | `_IOR(GENWQE_IOC_CODE, 34, struct genwqe_reg_io)` |  |

### GENWQE_SLU (3)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_SLU_OFFS` | `GENWQE_UID_OFFS(0)` |  |
| `GENWQE_SLU_UPDATE` | `_IOWR(GENWQE_IOC_CODE, 80, struct genwqe_bitstream)` |  |
| `GENWQE_SLU_READ` | `_IOWR(GENWQE_IOC_CODE, 81, struct genwqe_bitstream)` |  |

### GENWQE_TYPE (4)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_TYPE_ALTERA_230` | `0x00` | GenWQE4 Stratix-IV-230 |
| `GENWQE_TYPE_ALTERA_530` | `0x01` | GenWQE4 Stratix-IV-530 |
| `GENWQE_TYPE_ALTERA_A4` | `0x02` | GenWQE5 A4 Stratix-V-A4 |
| `GENWQE_TYPE_ALTERA_A7` | `0x03` | GenWQE5 A7 Stratix-V-A7 |

### GENWQE_UNPIN (1)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_UNPIN_MEM` | `_IOWR(GENWQE_IOC_CODE, 41, struct genwqe_mem)` |  |

### GENWQE_WRITE (3)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_WRITE_REG64` | `_IOW(GENWQE_IOC_CODE, 31, struct genwqe_reg_io)` |  |
| `GENWQE_WRITE_REG32` | `_IOW(GENWQE_IOC_CODE, 33, struct genwqe_reg_io)` |  |
| `GENWQE_WRITE_REG16` | `_IOW(GENWQE_IOC_CODE, 35, struct genwqe_reg_io)` |  |

### GFIR_ERR (1)

| Name | Value | Comment |
|------|-------|---------|
| `GFIR_ERR_TRIGGER` | `0x0000ffff` |  |

### IO_APP (29)

| Name | Value | Comment |
|------|-------|---------|
| `IO_APP_UNITCFG` | `0x02000000` |  |
| `IO_APP_FIR` | `0x02000008` |  |
| `IO_APP_FIR_CLR` | `0x02000010` |  |
| `IO_APP_FEC` | `0x02000018` |  |
| `IO_APP_ERR_ACT_MASK` | `0x02000020` |  |
| `IO_APP_ERR_ATTN_MASK` | `0x02000028` |  |
| `IO_APP_FIRX1_ACT_MASK` | `0x02000030` |  |
| `IO_APP_FIRX0_ACT_MASK` | `0x02000038` |  |
| `IO_APP_SEC_LEM_DEBUG_OVR` | `0x02000040` |  |
| `IO_APP_EXTENDED_ERR_PTR` | `0x02000048` |  |
| `IO_APP_COMMON_CONFIG` | `0x02000060` |  |
| `IO_APP_DEBUG_REG_01` | `0x02010000` |  |
| `IO_APP_DEBUG_REG_02` | `0x02010008` |  |
| `IO_APP_DEBUG_REG_03` | `0x02010010` |  |
| `IO_APP_DEBUG_REG_04` | `0x02010018` |  |
| `IO_APP_DEBUG_REG_05` | `0x02010020` |  |
| `IO_APP_DEBUG_REG_06` | `0x02010028` |  |
| `IO_APP_DEBUG_REG_07` | `0x02010030` |  |
| `IO_APP_DEBUG_REG_08` | `0x02010038` |  |
| `IO_APP_DEBUG_REG_09` | `0x02010040` |  |
| `IO_APP_DEBUG_REG_10` | `0x02010048` |  |
| `IO_APP_DEBUG_REG_11` | `0x02010050` |  |
| `IO_APP_DEBUG_REG_12` | `0x02010058` |  |
| `IO_APP_DEBUG_REG_13` | `0x02010060` |  |
| `IO_APP_DEBUG_REG_14` | `0x02010068` |  |
| `IO_APP_DEBUG_REG_15` | `0x02010070` |  |
| `IO_APP_DEBUG_REG_16` | `0x02010078` |  |
| `IO_APP_DEBUG_REG_17` | `0x02010080` |  |
| `IO_APP_DEBUG_REG_18` | `0x02010088` |  |

### IO_ERROR (1)

| Name | Value | Comment |
|------|-------|---------|
| `IO_ERROR_INJECT_SELECTOR` | `0x00000060` |  |

### IO_EXTENDED (3)

| Name | Value | Comment |
|------|-------|---------|
| `IO_EXTENDED_ERROR_POINTER` | `0x00000048` |  |
| `IO_EXTENDED_DIAG_SELECTOR` | `0x00000070` |  |
| `IO_EXTENDED_DIAG_READ_MBX` | `0x00000078` |  |

### IO_HSU (12)

| Name | Value | Comment |
|------|-------|---------|
| `IO_HSU_ERR_BEHAVIOR` | `0x01001010` |  |
| `IO_HSU_UNITCFG` | `0x01000000` |  |
| `IO_HSU_FIR` | `0x01000008` |  |
| `IO_HSU_FIR_CLR` | `0x01000010` |  |
| `IO_HSU_FEC` | `0x01000018` |  |
| `IO_HSU_ERR_ACT_MASK` | `0x01000020` |  |
| `IO_HSU_ERR_ATTN_MASK` | `0x01000028` |  |
| `IO_HSU_FIRX1_ACT_MASK` | `0x01000030` |  |
| `IO_HSU_FIRX0_ACT_MASK` | `0x01000038` |  |
| `IO_HSU_SEC_LEM_DEBUG_OVR` | `0x01000040` |  |
| `IO_HSU_EXTENDED_ERR_PTR` | `0x01000048` |  |
| `IO_HSU_COMMON_CONFIG` | `0x01000060` |  |

### IO_ILLEGAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `IO_ILLEGAL_VALUE` | `0xffffffffffffffffull` |  |

### IO_PF (2)

| Name | Value | Comment |
|------|-------|---------|
| `IO_PF_SLC_VIRTUAL_REGION` | `0x00050000` |  |
| `IO_PF_SLC_VIRTUAL_WINDOW` | `0x00060000` |  |

### IO_SLC (27)

| Name | Value | Comment |
|------|-------|---------|
| `IO_SLC_QUEUE_SEGMENT` | `0x00010000` |  |
| `IO_SLC_VF_QUEUE_SEGMENT` | `0x00050000` |  |
| `IO_SLC_QUEUE_OFFSET` | `0x00010008` |  |
| `IO_SLC_VF_QUEUE_OFFSET` | `0x00050008` |  |
| `IO_SLC_QUEUE_CONFIG` | `0x00010010` |  |
| `IO_SLC_VF_QUEUE_CONFIG` | `0x00050010` |  |
| `IO_SLC_APPJOB_TIMEOUT` | `0x00010018` |  |
| `IO_SLC_VF_APPJOB_TIMEOUT` | `0x00050018` |  |
| `IO_SLC_QUEUE_INITSQN` | `0x00010020` |  |
| `IO_SLC_VF_QUEUE_INITSQN` | `0x00050020` |  |
| `IO_SLC_QUEUE_WRAP` | `0x00010028` |  |
| `IO_SLC_VF_QUEUE_WRAP` | `0x00050028` |  |
| `IO_SLC_QUEUE_STATUS` | `0x00010100` |  |
| `IO_SLC_VF_QUEUE_STATUS` | `0x00050100` |  |
| `IO_SLC_QUEUE_WTIME` | `0x00010030` |  |
| `IO_SLC_VF_QUEUE_WTIME` | `0x00050030` |  |
| `IO_SLC_QUEUE_ERRCNTS` | `0x00010038` |  |
| `IO_SLC_VF_QUEUE_ERRCNTS` | `0x00050038` |  |
| `IO_SLC_QUEUE_LRW` | `0x00010040` |  |
| `IO_SLC_VF_QUEUE_LRW` | `0x00050040` |  |
| `IO_SLC_FREE_RUNNING_TIMER` | `0x00010108` |  |
| `IO_SLC_VF_FREE_RUNNING_TIMER` | `0x00050108` |  |
| `IO_SLC_CFGREG_GFIR` | `0x00020000` |  |
| `IO_SLC_CFGREG_SOFTRESET` | `0x00020018` |  |
| `IO_SLC_MISC_DEBUG` | `0x00020060` |  |
| `IO_SLC_MISC_DEBUG_CLR` | `0x00020068` |  |
| `IO_SLC_MISC_DEBUG_SET` | `0x00020070` |  |

### IO_SLU (28)

| Name | Value | Comment |
|------|-------|---------|
| `IO_SLU_UNITCFG` | `0x00000000` |  |
| `IO_SLU_UNITCFG_TYPE_MASK` | `0x000000000ff00000` | 27:20 |
| `IO_SLU_FIR` | `0x00000008` | read only, wr direct |
| `IO_SLU_FIR_CLR` | `0x00000010` | read and clear |
| `IO_SLU_FEC` | `0x00000018` |  |
| `IO_SLU_ERR_ACT_MASK` | `0x00000020` |  |
| `IO_SLU_ERR_ATTN_MASK` | `0x00000028` |  |
| `IO_SLU_FIRX1_ACT_MASK` | `0x00000030` |  |
| `IO_SLU_FIRX0_ACT_MASK` | `0x00000038` |  |
| `IO_SLU_SEC_LEM_DEBUG_OVR` | `0x00000040` |  |
| `IO_SLU_EXTENDED_ERR_PTR` | `0x00000048` |  |
| `IO_SLU_COMMON_CONFIG` | `0x00000060` |  |
| `IO_SLU_FLASH_FIR` | `0x00000108` |  |
| `IO_SLU_SLC_FIR` | `0x00000110` |  |
| `IO_SLU_RIU_TRAP` | `0x00000280` |  |
| `IO_SLU_FLASH_FEC` | `0x00000308` |  |
| `IO_SLU_SLC_FEC` | `0x00000310` |  |
| `IO_SLU_TEMPERATURE_SENSOR` | `0x00030000` |  |
| `IO_SLU_TEMPERATURE_CONFIG` | `0x00030008` |  |
| `IO_SLU_VOLTAGE_CONTROL` | `0x00030080` |  |
| `IO_SLU_VOLTAGE_NOMINAL` | `0x00000000` |  |
| `IO_SLU_VOLTAGE_DOWN5` | `0x00000006` |  |
| `IO_SLU_VOLTAGE_UP5` | `0x00000007` |  |
| `IO_SLU_LEDCONTROL` | `0x00030100` |  |
| `IO_SLU_FLASH_DIRECTACCESS` | `0x00040010` |  |
| `IO_SLU_FLASH_DIRECTACCESS2` | `0x00040020` |  |
| `IO_SLU_FLASH_CMDINTF` | `0x00040030` |  |
| `IO_SLU_BITSTREAM` | `0x00040040` |  |

### SLCMD_ECHO (1)

| Name | Value | Comment |
|------|-------|---------|
| `SLCMD_ECHO_SYNC` | `0x00` | PF/VF |

### SLCMD_MOVE (10)

| Name | Value | Comment |
|------|-------|---------|
| `SLCMD_MOVE_FLASH` | `0x06` | PF only |
| `SLCMD_MOVE_FLASH_FLAGS_MODE` | `0x03` | bit 0 and 1 used for mode |
| `SLCMD_MOVE_FLASH_FLAGS_DLOAD` | `0` | mode: download |
| `SLCMD_MOVE_FLASH_FLAGS_EMUL` | `1` | mode: emulation |
| `SLCMD_MOVE_FLASH_FLAGS_UPLOAD` | `2` | mode: upload |
| `SLCMD_MOVE_FLASH_FLAGS_VERIFY` | `3` | mode: verify |
| `SLCMD_MOVE_FLASH_FLAG_NOTAP` | `(1 << 2)` | just dump DDCB and exit |
| `SLCMD_MOVE_FLASH_FLAG_POLL` | `(1 << 3)` | wait for RETC >= 0102 |
| `SLCMD_MOVE_FLASH_FLAG_PARTITION` | `(1 << 4)` |  |
| `SLCMD_MOVE_FLASH_FLAG_ERASE` | `(1 << 5)` |  |

### UNCATEGORIZED (8)

| Name | Value | Comment |
|------|-------|---------|
| `GENWQE_DEVNAME` | `"genwqe"` |  |
| `TIMEOUT_250MS` | `0x0000000f` |  |
| `HEARTBEAT_DISABLE` | `0x0000ff00` |  |
| `IO_SLC2_SQB_TRAP` | `0x00062000` |  |
| `IO_SLC2_QUEUE_MANAGER_TRAP` | `0x00062008` |  |
| `IO_SLC2_FLS_MASTER_TRAP` | `0x00062010` |  |
| `DDCB_LENGTH` | `256` | for debug data |
| `DDCB_FIXUPS` | `12` | maximum number of fixups |

## Structs (6)


### `struct genwqe_reg_io`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `num` | `-` |
| `__u64` | `val64` | `-` |

### `struct genwqe_bitstream`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data_addr` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `crc` | `-` |
| `__u64` | `target_addr` | `-` |
| `__u32` | `partition` | `-` |
| `__u32` | `uid` | `-` |
| `__u64` | `slu_id` | `-` |
| `__u64` | `app_id` | `-` |
| `__u16` | `retc` | `-` |
| `__u16` | `attn` | `-` |
| `__u32` | `progress` | `-` |

### `struct genwqe_debug_data`

| Type | Field | Array |
|------|-------|-------|
| `char` | `driver_version` | `64` |
| `__u64` | `slu_unitcfg` | `-` |
| `__u64` | `app_unitcfg` | `-` |
| `__u8` | `ddcb_before` | `DDCB_LENGTH` |
| `__u8` | `ddcb_prev` | `DDCB_LENGTH` |
| `__u8` | `ddcb_finished` | `DDCB_LENGTH` |

### `struct genwqe_ddcb_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `next_addr` | `-` |
| `__u64` | `flags` | `-` |
| `__u8` | `acfunc` | `-` |
| `__u8` | `cmd` | `-` |
| `__u8` | `asiv_length` | `-` |
| `__u8` | `asv_length` | `-` |
| `__u16` | `cmdopts` | `-` |
| `__u16` | `retc` | `-` |
| `__u16` | `attn` | `-` |
| `__u16` | `vcrc` | `-` |
| `__u32` | `progress` | `-` |
| `__u64` | `deque_ts` | `-` |
| `__u64` | `cmplt_ts` | `-` |
| `__u64` | `disp_ts` | `-` |
| `__u64` | `ddata_addr` | `-` |
| `__u8` | `asv` | `DDCB_ASV_LENGTH` |
| `__u64` | `ats` | `-` |
| `__u8` | `asiv` | `DDCB_ASIV_LENGTH_ATS` |
| `__u8` | `__asiv` | `DDCB_ASIV_LENGTH` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ats` | `-` |
| `__u8` | `asiv` | `DDCB_ASIV_LENGTH_ATS` |

### `struct genwqe_mem`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `direction` | `-` |
| `__u64` | `flags` | `-` |