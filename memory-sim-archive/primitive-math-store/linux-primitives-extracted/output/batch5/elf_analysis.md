# elf.h

**Source:** `elf.h`


## Includes

- `linux/types.h`
- `linux/elf-em.h`

## Defines (319 total)


### DT_FLAGS (1)

| Name | Value | Comment |
|------|-------|---------|
| `DT_FLAGS_1` | `0x6ffffffb` |  |

### DT_GNU (1)

| Name | Value | Comment |
|------|-------|---------|
| `DT_GNU_HASH` | `0x6ffffef5` |  |

### GNU_PROPERTY (2)

| Name | Value | Comment |
|------|-------|---------|
| `GNU_PROPERTY_AARCH64_FEATURE_1_AND` | `0xc0000000` |  |
| `GNU_PROPERTY_AARCH64_FEATURE_1_BTI` | `(1U << 0)` |  |

### NN_ARC (1)

| Name | Value | Comment |
|------|-------|---------|
| `NN_ARC_V2` | `"LINUX"` |  |

### NN_ARM (17)

| Name | Value | Comment |
|------|-------|---------|
| `NN_ARM_VFP` | `"LINUX"` |  |
| `NN_ARM_TLS` | `"LINUX"` |  |
| `NN_ARM_HW_BREAK` | `"LINUX"` |  |
| `NN_ARM_HW_WATCH` | `"LINUX"` |  |
| `NN_ARM_SYSTEM_CALL` | `"LINUX"` |  |
| `NN_ARM_SVE` | `"LINUX"` |  |
| `NN_ARM_PAC_MASK` | `"LINUX"` |  |
| `NN_ARM_PACA_KEYS` | `"LINUX"` |  |
| `NN_ARM_PACG_KEYS` | `"LINUX"` |  |
| `NN_ARM_TAGGED_ADDR_CTRL` | `"LINUX"` |  |
| `NN_ARM_PAC_ENABLED_KEYS` | `"LINUX"` |  |
| `NN_ARM_SSVE` | `"LINUX"` |  |
| `NN_ARM_ZA` | `"LINUX"` |  |
| `NN_ARM_ZT` | `"LINUX"` |  |
| `NN_ARM_FPMR` | `"LINUX"` |  |
| `NN_ARM_POE` | `"LINUX"` |  |
| `NN_ARM_GCS` | `"LINUX"` |  |

### NN_GNU (1)

| Name | Value | Comment |
|------|-------|---------|
| `NN_GNU_PROPERTY_TYPE_0` | `"GNU"` |  |

### NN_LOONGARCH (7)

| Name | Value | Comment |
|------|-------|---------|
| `NN_LOONGARCH_CPUCFG` | `"LINUX"` |  |
| `NN_LOONGARCH_CSR` | `"LINUX"` |  |
| `NN_LOONGARCH_LSX` | `"LINUX"` |  |
| `NN_LOONGARCH_LASX` | `"LINUX"` |  |
| `NN_LOONGARCH_LBT` | `"LINUX"` |  |
| `NN_LOONGARCH_HW_BREAK` | `"LINUX"` |  |
| `NN_LOONGARCH_HW_WATCH` | `"LINUX"` |  |

### NN_MIPS (3)

| Name | Value | Comment |
|------|-------|---------|
| `NN_MIPS_DSP` | `"LINUX"` |  |
| `NN_MIPS_FP_MODE` | `"LINUX"` |  |
| `NN_MIPS_MSA` | `"LINUX"` |  |

### NN_PPC (19)

| Name | Value | Comment |
|------|-------|---------|
| `NN_PPC_VMX` | `"LINUX"` |  |
| `NN_PPC_SPE` | `"LINUX"` |  |
| `NN_PPC_VSX` | `"LINUX"` |  |
| `NN_PPC_TAR` | `"LINUX"` |  |
| `NN_PPC_PPR` | `"LINUX"` |  |
| `NN_PPC_DSCR` | `"LINUX"` |  |
| `NN_PPC_EBB` | `"LINUX"` |  |
| `NN_PPC_PMU` | `"LINUX"` |  |
| `NN_PPC_TM_CGPR` | `"LINUX"` |  |
| `NN_PPC_TM_CFPR` | `"LINUX"` |  |
| `NN_PPC_TM_CVMX` | `"LINUX"` |  |
| `NN_PPC_TM_CVSX` | `"LINUX"` |  |
| `NN_PPC_TM_SPR` | `"LINUX"` |  |
| `NN_PPC_TM_CTAR` | `"LINUX"` |  |
| `NN_PPC_TM_CPPR` | `"LINUX"` |  |
| `NN_PPC_TM_CDSCR` | `"LINUX"` |  |
| `NN_PPC_PKEY` | `"LINUX"` |  |
| `NN_PPC_DEXCR` | `"LINUX"` |  |
| `NN_PPC_HASHKEYR` | `"LINUX"` |  |

### NN_RISCV (4)

| Name | Value | Comment |
|------|-------|---------|
| `NN_RISCV_CSR` | `"LINUX"` |  |
| `NN_RISCV_VECTOR` | `"LINUX"` |  |
| `NN_RISCV_TAGGED_ADDR_CTRL` | `"LINUX"` |  |
| `NN_RISCV_USER_CFI` | `"LINUX"` |  |

### NT_ARC (1)

| Name | Value | Comment |
|------|-------|---------|
| `NT_ARC_V2` | `0x600` | ARCv2 accumulator/extra registers |

### NT_ARM (17)

| Name | Value | Comment |
|------|-------|---------|
| `NT_ARM_VFP` | `0x400` | ARM VFP/NEON registers |
| `NT_ARM_TLS` | `0x401` | ARM TLS register |
| `NT_ARM_HW_BREAK` | `0x402` | ARM hardware breakpoint registers |
| `NT_ARM_HW_WATCH` | `0x403` | ARM hardware watchpoint registers |
| `NT_ARM_SYSTEM_CALL` | `0x404` | ARM system call number |
| `NT_ARM_SVE` | `0x405` | ARM Scalable Vector Extension registers |
| `NT_ARM_PAC_MASK` | `0x406` | ARM pointer authentication code masks |
| `NT_ARM_PACA_KEYS` | `0x407` | ARM pointer authentication address keys |
| `NT_ARM_PACG_KEYS` | `0x408` | ARM pointer authentication generic key |
| `NT_ARM_TAGGED_ADDR_CTRL` | `0x409` | arm64 tagged address control (prctl()) |
| `NT_ARM_PAC_ENABLED_KEYS` | `0x40a` | arm64 ptr auth enabled keys (prctl()) |
| `NT_ARM_SSVE` | `0x40b` | ARM Streaming SVE registers |
| `NT_ARM_ZA` | `0x40c` | ARM SME ZA registers |
| `NT_ARM_ZT` | `0x40d` | ARM SME ZT registers |
| `NT_ARM_FPMR` | `0x40e` | ARM floating point mode register |
| `NT_ARM_POE` | `0x40f` | ARM POE registers |
| `NT_ARM_GCS` | `0x410` | ARM GCS state |

### NT_GNU (1)

| Name | Value | Comment |
|------|-------|---------|
| `NT_GNU_PROPERTY_TYPE_0` | `5` |  |

### NT_LOONGARCH (7)

| Name | Value | Comment |
|------|-------|---------|
| `NT_LOONGARCH_CPUCFG` | `0xa00` | LoongArch CPU config registers |
| `NT_LOONGARCH_CSR` | `0xa01` | LoongArch control and status registers |
| `NT_LOONGARCH_LSX` | `0xa02` | LoongArch Loongson SIMD Extension registers |
| `NT_LOONGARCH_LASX` | `0xa03` | LoongArch Loongson Advanced SIMD Extension registers |
| `NT_LOONGARCH_LBT` | `0xa04` | LoongArch Loongson Binary Translation registers |
| `NT_LOONGARCH_HW_BREAK` | `0xa05` | LoongArch hardware breakpoint registers |
| `NT_LOONGARCH_HW_WATCH` | `0xa06` | LoongArch hardware watchpoint registers |

### NT_MIPS (3)

| Name | Value | Comment |
|------|-------|---------|
| `NT_MIPS_DSP` | `0x800` | MIPS DSP ASE registers |
| `NT_MIPS_FP_MODE` | `0x801` | MIPS floating-point mode |
| `NT_MIPS_MSA` | `0x802` | MIPS SIMD registers |

### NT_PPC (19)

| Name | Value | Comment |
|------|-------|---------|
| `NT_PPC_VMX` | `0x100` | PowerPC Altivec/VMX registers |
| `NT_PPC_SPE` | `0x101` | PowerPC SPE/EVR registers |
| `NT_PPC_VSX` | `0x102` | PowerPC VSX registers |
| `NT_PPC_TAR` | `0x103` | Target Address Register |
| `NT_PPC_PPR` | `0x104` | Program Priority Register |
| `NT_PPC_DSCR` | `0x105` | Data Stream Control Register |
| `NT_PPC_EBB` | `0x106` | Event Based Branch Registers |
| `NT_PPC_PMU` | `0x107` | Performance Monitor Registers |
| `NT_PPC_TM_CGPR` | `0x108` | TM checkpointed GPR Registers |
| `NT_PPC_TM_CFPR` | `0x109` | TM checkpointed FPR Registers |
| `NT_PPC_TM_CVMX` | `0x10a` | TM checkpointed VMX Registers |
| `NT_PPC_TM_CVSX` | `0x10b` | TM checkpointed VSX Registers |
| `NT_PPC_TM_SPR` | `0x10c` | TM Special Purpose Registers |
| `NT_PPC_TM_CTAR` | `0x10d` | TM checkpointed Target Address Register |
| `NT_PPC_TM_CPPR` | `0x10e` | TM checkpointed Program Priority Register |
| `NT_PPC_TM_CDSCR` | `0x10f` | TM checkpointed Data Stream Control Register |
| `NT_PPC_PKEY` | `0x110` | Memory Protection Keys registers |
| `NT_PPC_DEXCR` | `0x111` | PowerPC DEXCR registers |
| `NT_PPC_HASHKEYR` | `0x112` | PowerPC HASHKEYR register |

### NT_RISCV (4)

| Name | Value | Comment |
|------|-------|---------|
| `NT_RISCV_CSR` | `0x900` | RISC-V Control and Status Registers |
| `NT_RISCV_VECTOR` | `0x901` | RISC-V vector registers |
| `NT_RISCV_TAGGED_ADDR_CTRL` | `0x902` | RISC-V tagged address control (prctl()) |
| `NT_RISCV_USER_CFI` | `0x903` | RISC-V shadow stack state |

### OLD_DT (2)

| Name | Value | Comment |
|------|-------|---------|
| `OLD_DT_LOOS` | `0x60000000` |  |
| `OLD_DT_HIOS` | `0x6fffffff` |  |

### PT_GNU (4)

| Name | Value | Comment |
|------|-------|---------|
| `PT_GNU_EH_FRAME` | `(PT_LOOS + 0x474e550)` |  |
| `PT_GNU_STACK` | `(PT_LOOS + 0x474e551)` |  |
| `PT_GNU_RELRO` | `(PT_LOOS + 0x474e552)` |  |
| `PT_GNU_PROPERTY` | `(PT_LOOS + 0x474e553)` |  |

### SHF_INFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `SHF_INFO_LINK` | `0x40` |  |

### SHF_LINK (1)

| Name | Value | Comment |
|------|-------|---------|
| `SHF_LINK_ORDER` | `0x80` |  |

### SHF_OS (1)

| Name | Value | Comment |
|------|-------|---------|
| `SHF_OS_NONCONFORMING` | `0x100` |  |

### SHF_RELA (1)

| Name | Value | Comment |
|------|-------|---------|
| `SHF_RELA_LIVEPATCH` | `0x00100000` |  |

### SHF_RO (1)

| Name | Value | Comment |
|------|-------|---------|
| `SHF_RO_AFTER_INIT` | `0x00200000` |  |

### UNCATEGORIZED (198)

| Name | Value | Comment |
|------|-------|---------|
| `PT_NULL` | `0` |  |
| `PT_LOAD` | `1` |  |
| `PT_DYNAMIC` | `2` |  |
| `PT_INTERP` | `3` |  |
| `PT_NOTE` | `4` |  |
| `PT_SHLIB` | `5` |  |
| `PT_PHDR` | `6` |  |
| `PT_TLS` | `7` | Thread local storage segment |
| `PT_LOOS` | `0x60000000` | OS-specific |
| `PT_HIOS` | `0x6fffffff` | OS-specific |
| `PT_LOPROC` | `0x70000000` |  |
| `PT_HIPROC` | `0x7fffffff` |  |
| `PT_AARCH64_MEMTAG_MTE` | `(PT_LOPROC + 0x2)` |  |
| `PN_XNUM` | `0xffff` |  |
| `ET_NONE` | `0` |  |
| `ET_REL` | `1` |  |
| `ET_EXEC` | `2` |  |
| `ET_DYN` | `3` |  |
| `ET_CORE` | `4` |  |
| `ET_LOPROC` | `0xff00` |  |
| `ET_HIPROC` | `0xffff` |  |
| `DT_NULL` | `0` |  |
| `DT_NEEDED` | `1` |  |
| `DT_PLTRELSZ` | `2` |  |
| `DT_PLTGOT` | `3` |  |
| `DT_HASH` | `4` |  |
| `DT_STRTAB` | `5` |  |
| `DT_SYMTAB` | `6` |  |
| `DT_RELA` | `7` |  |
| `DT_RELASZ` | `8` |  |
| `DT_RELAENT` | `9` |  |
| `DT_STRSZ` | `10` |  |
| `DT_SYMENT` | `11` |  |
| `DT_INIT` | `12` |  |
| `DT_FINI` | `13` |  |
| `DT_SONAME` | `14` |  |
| `DT_RPATH` | `15` |  |
| `DT_SYMBOLIC` | `16` |  |
| `DT_REL` | `17` |  |
| `DT_RELSZ` | `18` |  |
| `DT_RELENT` | `19` |  |
| `DT_PLTREL` | `20` |  |
| `DT_DEBUG` | `21` |  |
| `DT_TEXTREL` | `22` |  |
| `DT_JMPREL` | `23` |  |
| `DT_ENCODING` | `32` |  |
| `DT_LOOS` | `0x6000000d` |  |
| `DT_HIOS` | `0x6ffff000` |  |
| `DT_VALRNGLO` | `0x6ffffd00` |  |
| `DT_VALRNGHI` | `0x6ffffdff` |  |

*...and 148 more*

### VER_FLG (2)

| Name | Value | Comment |
|------|-------|---------|
| `VER_FLG_BASE` | `0x1` |  |
| `VER_FLG_WEAK` | `0x2` |  |

## Structs (20)


### `struct anonymous_0`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Sword` | `d_tag` | `-` |
| `Elf32_Sword` | `d_val` | `-` |
| `Elf32_Addr` | `d_ptr` | `-` |

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Sxword` | `d_tag` | `-` |
| `Elf64_Xword` | `d_val` | `-` |
| `Elf64_Addr` | `d_ptr` | `-` |

### `struct elf32_rel`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Addr` | `r_offset` | `-` |
| `Elf32_Word` | `r_info` | `-` |

### `struct elf64_rel`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Addr` | `r_offset` | `-` |
| `Elf64_Xword` | `r_info` | `-` |

### `struct elf32_rela`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Addr` | `r_offset` | `-` |
| `Elf32_Word` | `r_info` | `-` |
| `Elf32_Sword` | `r_addend` | `-` |

### `struct elf64_rela`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Addr` | `r_offset` | `-` |
| `Elf64_Xword` | `r_info` | `-` |
| `Elf64_Sxword` | `r_addend` | `-` |

### `struct elf32_sym`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Word` | `st_name` | `-` |
| `Elf32_Addr` | `st_value` | `-` |
| `Elf32_Word` | `st_size` | `-` |
| `Elf32_Half` | `st_shndx` | `-` |

### `struct elf64_sym`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Word` | `st_name` | `-` |
| `Elf64_Half` | `st_shndx` | `-` |
| `Elf64_Addr` | `st_value` | `-` |
| `Elf64_Xword` | `st_size` | `-` |

### `struct elf32_hdr`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Half` | `e_type` | `-` |
| `Elf32_Half` | `e_machine` | `-` |
| `Elf32_Word` | `e_version` | `-` |
| `Elf32_Addr` | `e_entry` | `-` |
| `Elf32_Off` | `e_phoff` | `-` |
| `Elf32_Off` | `e_shoff` | `-` |
| `Elf32_Word` | `e_flags` | `-` |
| `Elf32_Half` | `e_ehsize` | `-` |
| `Elf32_Half` | `e_phentsize` | `-` |
| `Elf32_Half` | `e_phnum` | `-` |
| `Elf32_Half` | `e_shentsize` | `-` |
| `Elf32_Half` | `e_shnum` | `-` |
| `Elf32_Half` | `e_shstrndx` | `-` |

### `struct elf64_hdr`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Half` | `e_type` | `-` |
| `Elf64_Half` | `e_machine` | `-` |
| `Elf64_Word` | `e_version` | `-` |
| `Elf64_Addr` | `e_entry` | `-` |
| `Elf64_Off` | `e_phoff` | `-` |
| `Elf64_Off` | `e_shoff` | `-` |
| `Elf64_Word` | `e_flags` | `-` |
| `Elf64_Half` | `e_ehsize` | `-` |
| `Elf64_Half` | `e_phentsize` | `-` |
| `Elf64_Half` | `e_phnum` | `-` |
| `Elf64_Half` | `e_shentsize` | `-` |
| `Elf64_Half` | `e_shnum` | `-` |
| `Elf64_Half` | `e_shstrndx` | `-` |

### `struct elf32_phdr`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Word` | `p_type` | `-` |
| `Elf32_Off` | `p_offset` | `-` |
| `Elf32_Addr` | `p_vaddr` | `-` |
| `Elf32_Addr` | `p_paddr` | `-` |
| `Elf32_Word` | `p_filesz` | `-` |
| `Elf32_Word` | `p_memsz` | `-` |
| `Elf32_Word` | `p_flags` | `-` |
| `Elf32_Word` | `p_align` | `-` |

### `struct elf64_phdr`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Word` | `p_type` | `-` |
| `Elf64_Word` | `p_flags` | `-` |
| `Elf64_Off` | `p_offset` | `-` |
| `Elf64_Addr` | `p_vaddr` | `-` |
| `Elf64_Addr` | `p_paddr` | `-` |
| `Elf64_Xword` | `p_filesz` | `-` |
| `Elf64_Xword` | `p_memsz` | `-` |
| `Elf64_Xword` | `p_align` | `-` |

### `struct elf32_shdr`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Word` | `sh_name` | `-` |
| `Elf32_Word` | `sh_type` | `-` |
| `Elf32_Word` | `sh_flags` | `-` |
| `Elf32_Addr` | `sh_addr` | `-` |
| `Elf32_Off` | `sh_offset` | `-` |
| `Elf32_Word` | `sh_size` | `-` |
| `Elf32_Word` | `sh_link` | `-` |
| `Elf32_Word` | `sh_info` | `-` |
| `Elf32_Word` | `sh_addralign` | `-` |
| `Elf32_Word` | `sh_entsize` | `-` |

### `struct elf64_shdr`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Word` | `sh_name` | `-` |
| `Elf64_Word` | `sh_type` | `-` |
| `Elf64_Xword` | `sh_flags` | `-` |
| `Elf64_Addr` | `sh_addr` | `-` |
| `Elf64_Off` | `sh_offset` | `-` |
| `Elf64_Xword` | `sh_size` | `-` |
| `Elf64_Word` | `sh_link` | `-` |
| `Elf64_Word` | `sh_info` | `-` |
| `Elf64_Xword` | `sh_addralign` | `-` |
| `Elf64_Xword` | `sh_entsize` | `-` |

### `struct elf32_note`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Word` | `n_namesz` | `-` |
| `Elf32_Word` | `n_descsz` | `-` |
| `Elf32_Word` | `n_type` | `-` |

### `struct elf64_note`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Word` | `n_namesz` | `-` |
| `Elf64_Word` | `n_descsz` | `-` |
| `Elf64_Word` | `n_type` | `-` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Half` | `vd_version` | `-` |
| `Elf32_Half` | `vd_flags` | `-` |
| `Elf32_Half` | `vd_ndx` | `-` |
| `Elf32_Half` | `vd_cnt` | `-` |
| `Elf32_Word` | `vd_hash` | `-` |
| `Elf32_Word` | `vd_aux` | `-` |
| `Elf32_Word` | `vd_next` | `-` |

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Half` | `vd_version` | `-` |
| `Elf64_Half` | `vd_flags` | `-` |
| `Elf64_Half` | `vd_ndx` | `-` |
| `Elf64_Half` | `vd_cnt` | `-` |
| `Elf64_Word` | `vd_hash` | `-` |
| `Elf64_Word` | `vd_aux` | `-` |
| `Elf64_Word` | `vd_next` | `-` |

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|
| `Elf32_Word` | `vda_name` | `-` |
| `Elf32_Word` | `vda_next` | `-` |

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|
| `Elf64_Word` | `vda_name` | `-` |
| `Elf64_Word` | `vda_next` | `-` |

## Typedefs

- `elf32_rel`
- `elf64_rel`
- `elf32_rela`
- `elf64_rela`
- `elf32_sym`
- `elf64_sym`
- `elf32_hdr`
- `elf64_hdr`
- `elf32_phdr`
- `elf64_phdr`
- `elf32_shdr`
- `elf64_shdr`
- `elf32_note`
- `elf64_note`