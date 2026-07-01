# kexec.h

**Source:** `kexec.h`


## Includes

- `linux/types.h`

## Defines (28 total)


### KEXEC_ARCH (17)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_ARCH_MASK` | `0xffff0000` |  |
| `KEXEC_ARCH_DEFAULT` | `( 0 << 16)` |  |
| `KEXEC_ARCH_386` | `( 3 << 16)` |  |
| `KEXEC_ARCH_68K` | `( 4 << 16)` |  |
| `KEXEC_ARCH_PARISC` | `(15 << 16)` |  |
| `KEXEC_ARCH_X86_64` | `(62 << 16)` |  |
| `KEXEC_ARCH_PPC` | `(20 << 16)` |  |
| `KEXEC_ARCH_PPC64` | `(21 << 16)` |  |
| `KEXEC_ARCH_IA_64` | `(50 << 16)` |  |
| `KEXEC_ARCH_ARM` | `(40 << 16)` |  |
| `KEXEC_ARCH_S390` | `(22 << 16)` |  |
| `KEXEC_ARCH_SH` | `(42 << 16)` |  |
| `KEXEC_ARCH_MIPS_LE` | `(10 << 16)` |  |
| `KEXEC_ARCH_MIPS` | `( 8 << 16)` |  |
| `KEXEC_ARCH_AARCH64` | `(183 << 16)` |  |
| `KEXEC_ARCH_RISCV` | `(243 << 16)` |  |
| `KEXEC_ARCH_LOONGARCH` | `(258 << 16)` |  |

### KEXEC_CRASH (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_CRASH_HOTPLUG_SUPPORT` | `0x00000008` |  |

### KEXEC_FILE (6)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_FILE_UNLOAD` | `0x00000001` |  |
| `KEXEC_FILE_ON_CRASH` | `0x00000002` |  |
| `KEXEC_FILE_NO_INITRAMFS` | `0x00000004` |  |
| `KEXEC_FILE_DEBUG` | `0x00000008` |  |
| `KEXEC_FILE_NO_CMA` | `0x00000010` |  |
| `KEXEC_FILE_FORCE_DTB` | `0x00000020` |  |

### KEXEC_ON (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_ON_CRASH` | `0x00000001` |  |

### KEXEC_PRESERVE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_PRESERVE_CONTEXT` | `0x00000002` |  |

### KEXEC_SEGMENT (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_SEGMENT_MAX` | `16` |  |

### KEXEC_UPDATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KEXEC_UPDATE_ELFCOREHDR` | `0x00000004` |  |

## Structs (1)


### `struct kexec_segment`

| Type | Field | Array |
|------|-------|-------|
| `__kernel_size_t` | `bufsz` | `-` |
| `__kernel_size_t` | `memsz` | `-` |