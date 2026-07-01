# coff.h

**Source:** `coff.h`


## Defines (72 total)


### COFF_DEF (4)

| Name | Value | Comment |
|------|-------|---------|
| `COFF_DEF_DATA_SECTION_ALIGNMENT` | `4` |  |
| `COFF_DEF_BSS_SECTION_ALIGNMENT` | `4` |  |
| `COFF_DEF_TEXT_SECTION_ALIGNMENT` | `4` |  |
| `COFF_DEF_SECTION_ALIGNMENT` | `4` |  |

### COFF_E (3)

| Name | Value | Comment |
|------|-------|---------|
| `COFF_E_SYMNMLEN` | `8` | # characters in a short symbol name |
| `COFF_E_FILNMLEN` | `14` | # characters in a file name |
| `COFF_E_DIMNUM` | `4` | # array dimensions in auxiliary entry |

### COFF_F (12)

| Name | Value | Comment |
|------|-------|---------|
| `COFF_F_RELFLG` | `0000001` |  |
| `COFF_F_EXEC` | `0000002` |  |
| `COFF_F_LNNO` | `0000004` |  |
| `COFF_F_LSYMS` | `0000010` |  |
| `COFF_F_MINMAL` | `0000020` |  |
| `COFF_F_UPDATE` | `0000040` |  |
| `COFF_F_SWABD` | `0000100` |  |
| `COFF_F_AR16WR` | `0000200` |  |
| `COFF_F_AR32WR` | `0000400` |  |
| `COFF_F_AR32W` | `0001000` |  |
| `COFF_F_PATCH` | `0002000` |  |
| `COFF_F_NODF` | `0002000` |  |

### COFF_N (4)

| Name | Value | Comment |
|------|-------|---------|
| `COFF_N_BTMASK` | `(0xf)` | Mask for important class bits |
| `COFF_N_TMASK` | `(0x30)` | Mask for important type bits |
| `COFF_N_BTSHFT` | `(4)` | # bits to shift class field |
| `COFF_N_TSHIFT` | `(2)` | # bits to shift type field |

### COFF_SECT (4)

| Name | Value | Comment |
|------|-------|---------|
| `COFF_SECT_TEXT` | `0` | Section for instruction code |
| `COFF_SECT_DATA` | `1` | Section for initialized globals |
| `COFF_SECT_BSS` | `2` | Section for un-initialized globals |
| `COFF_SECT_REQD` | `3` | Minimum number of sections for good file |

### COFF_STYP (12)

| Name | Value | Comment |
|------|-------|---------|
| `COFF_STYP_REG` | `0x00` | regular segment |
| `COFF_STYP_DSECT` | `0x01` | dummy segment |
| `COFF_STYP_NOLOAD` | `0x02` | no-load segment |
| `COFF_STYP_GROUP` | `0x04` | group segment |
| `COFF_STYP_PAD` | `0x08` | .pad segment |
| `COFF_STYP_COPY` | `0x10` | copy section |
| `COFF_STYP_TEXT` | `0x20` | .text segment |
| `COFF_STYP_DATA` | `0x40` | .data segment |
| `COFF_STYP_BSS` | `0x80` | .bss segment |
| `COFF_STYP_INFO` | `0x200` | .comment section |
| `COFF_STYP_OVER` | `0x400` | overlay section |
| `COFF_STYP_LIB` | `0x800` | library section |

### UNCATEGORIZED (33)

| Name | Value | Comment |
|------|-------|---------|
| `E_SYMNMLEN` | `8` | Number of characters in a symbol name |
| `E_FILNMLEN` | `14` | Number of characters in a file name |
| `E_DIMNUM` | `4` | Number of array dimensions in auxiliary entry |
| `COFF_I386MAGIC` | `0x14c` | Linux's system |
| `COFF_I386PTXMAGIC` | `0x154` |  |
| `COFF_I386AIXMAGIC` | `0x175` | IBM's AIX system |
| `COFF_FILHDR` | `struct COFF_filehdr` |  |
| `COFF_FILHSZ` | `sizeof(COFF_FILHDR)` |  |
| `COFF_AOUTSZ` | `(sizeof(COFF_AOUTHDR))` |  |
| `COFF_STMAGIC` | `0401` |  |
| `COFF_OMAGIC` | `0404` |  |
| `COFF_JMAGIC` | `0407` | dirty text and data image, can't share |
| `COFF_DMAGIC` | `0410` | dirty text segment, data aligned |
| `COFF_ZMAGIC` | `0413` | The proper magic number for executables |
| `COFF_SHMAGIC` | `0443` | shared library header |
| `COFF_SCNHDR` | `struct COFF_scnhdr` |  |
| `COFF_SCNHSZ` | `sizeof(COFF_SCNHDR)` |  |
| `COFF_TEXT` | `".text"` |  |
| `COFF_DATA` | `".data"` |  |
| `COFF_BSS` | `".bss"` |  |
| `COFF_COMMENT` | `".comment"` |  |
| `COFF_LIB` | `".lib"` |  |
| `COFF_SLIBHD` | `struct COFF_slib` |  |
| `COFF_SLIBSZ` | `sizeof(COFF_SLIBHD)` |  |
| `COFF_LINENO` | `struct COFF_lineno` |  |
| `COFF_LINESZ` | `6` |  |
| `COFF_SYMENT` | `struct COFF_syment` |  |
| `COFF_SYMESZ` | `18` |  |
| `COFF_AUXENT` | `union COFF_auxent` |  |
| `COFF_AUXESZ` | `18` |  |
| `COFF_ETEXT` | `"etext"` |  |
| `COFF_RELOC` | `struct COFF_reloc` |  |
| `COFF_RELSZ` | `10` |  |

## Structs (15)


### `struct COFF_filehdr`

| Type | Field | Array |
|------|-------|-------|
| `char` | `f_magic` | `2` |
| `char` | `f_nscns` | `2` |
| `char` | `f_timdat` | `4` |
| `char` | `f_symptr` | `4` |
| `char` | `f_nsyms` | `4` |
| `char` | `f_opthdr` | `2` |
| `char` | `f_flags` | `2` |

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|
| `char` | `magic` | `2` |
| `char` | `vstamp` | `2` |
| `char` | `tsize` | `4` |
| `char` | `dsize` | `4` |
| `char` | `bsize` | `4` |
| `char` | `entry` | `4` |
| `char` | `text_start` | `4` |
| `char` | `data_start` | `4` |

### `struct COFF_scnhdr`

| Type | Field | Array |
|------|-------|-------|
| `char` | `s_name` | `8` |
| `char` | `s_paddr` | `4` |
| `char` | `s_vaddr` | `4` |
| `char` | `s_size` | `4` |
| `char` | `s_scnptr` | `4` |
| `char` | `s_relptr` | `4` |
| `char` | `s_lnnoptr` | `4` |
| `char` | `s_nreloc` | `2` |
| `char` | `s_nlnno` | `2` |
| `char` | `s_flags` | `4` |

### `struct COFF_slib`

| Type | Field | Array |
|------|-------|-------|
| `char` | `sl_entsz` | `4` |
| `char` | `sl_pathndx` | `4` |

### `struct COFF_lineno`

| Type | Field | Array |
|------|-------|-------|
| `char` | `l_symndx` | `4` |
| `char` | `l_paddr` | `4` |
| `char` | `l_lnno` | `2` |

### `struct COFF_syment`

| Type | Field | Array |
|------|-------|-------|
| `char` | `e_name` | `E_SYMNMLEN` |
| `char` | `e_zeroes` | `4` |
| `char` | `e_offset` | `4` |
| `char` | `e_value` | `4` |
| `char` | `e_scnum` | `2` |
| `char` | `e_type` | `2` |
| `char` | `e_sclass` | `1` |
| `char` | `e_numaux` | `1` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `char` | `e_zeroes` | `4` |
| `char` | `e_offset` | `4` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_tagndx` | `4` |
| `char` | `x_lnno` | `2` |
| `char` | `x_size` | `2` |
| `char` | `x_fsize` | `4` |
| `char` | `x_lnnoptr` | `4` |
| `char` | `x_endndx` | `4` |
| `char` | `x_dimen` | `E_DIMNUM][2` |
| `char` | `x_tvndx` | `2` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_lnno` | `2` |
| `char` | `x_size` | `2` |

### `struct anonymous_9`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_lnnoptr` | `4` |
| `char` | `x_endndx` | `4` |

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_dimen` | `E_DIMNUM][2` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_zeroes` | `4` |
| `char` | `x_offset` | `4` |

### `struct anonymous_12`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_scnlen` | `4` |
| `char` | `x_nreloc` | `2` |
| `char` | `x_nlinno` | `2` |

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|
| `char` | `x_tvfill` | `4` |
| `char` | `x_tvlen` | `2` |
| `char` | `x_tvran` | `2][2` |

### `struct COFF_reloc`

| Type | Field | Array |
|------|-------|-------|
| `char` | `r_vaddr` | `4` |
| `char` | `r_symndx` | `4` |
| `char` | `r_type` | `2` |