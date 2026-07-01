# kvm.h

**Source:** `kvm.h`


## Includes

- `linux/const.h`
- `linux/types.h`
- `linux/compiler.h`
- `linux/stddef.h`
- `linux/ioctl.h`
- `asm/kvm.h`
- `linux/kvm_types.h`

## Defines (606 total)


### GUEST_MEMFD (2)

| Name | Value | Comment |
|------|-------|---------|
| `GUEST_MEMFD_FLAG_MMAP` | `(1ULL << 0)` |  |
| `GUEST_MEMFD_FLAG_INIT_SHARED` | `(1ULL << 1)` |  |

### KVM_ALLOCATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_ALLOCATE_RMA` | `_IOR(KVMIO,  0xa9, struct kvm_allocate_rma)` |  |

### KVM_API (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_API_VERSION` | `12` |  |

### KVM_ARM (7)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_ARM_SET_DEVICE_ADDR` | `_IOW(KVMIO,  0xab, struct kvm_arm_device_addr)` |  |
| `KVM_ARM_MTE_COPY_TAGS` | `_IOR(KVMIO,  0xb4, struct kvm_arm_copy_mte_tags)` |  |
| `KVM_ARM_SET_COUNTER_OFFSET` | `_IOW(KVMIO,  0xb5, struct kvm_arm_counter_offset)` |  |
| `KVM_ARM_GET_REG_WRITABLE_MASKS` | `_IOR(KVMIO,  0xb6, struct reg_mask_range)` |  |
| `KVM_ARM_VCPU_INIT` | `_IOW(KVMIO,  0xae, struct kvm_vcpu_init)` |  |
| `KVM_ARM_PREFERRED_TARGET` | `_IOR(KVMIO,  0xaf, struct kvm_vcpu_init)` |  |
| `KVM_ARM_VCPU_FINALIZE` | `_IOW(KVMIO,  0xc2, int)` |  |

### KVM_BUS (2)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_BUS_LOCK_DETECTION_OFF` | `(1 << 0)` |  |
| `KVM_BUS_LOCK_DETECTION_EXIT` | `(1 << 1)` |  |

### KVM_CAP (243)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_CAP_IRQCHIP` | `0` |  |
| `KVM_CAP_HLT` | `1` |  |
| `KVM_CAP_MMU_SHADOW_CACHE_CONTROL` | `2` |  |
| `KVM_CAP_USER_MEMORY` | `3` |  |
| `KVM_CAP_SET_TSS_ADDR` | `4` |  |
| `KVM_CAP_VAPIC` | `6` |  |
| `KVM_CAP_EXT_CPUID` | `7` |  |
| `KVM_CAP_CLOCKSOURCE` | `8` |  |
| `KVM_CAP_NR_VCPUS` | `9` | returns recommended max vcpus per vm |
| `KVM_CAP_NR_MEMSLOTS` | `10` | returns max memory slots per vm |
| `KVM_CAP_PIT` | `11` |  |
| `KVM_CAP_NOP_IO_DELAY` | `12` |  |
| `KVM_CAP_PV_MMU` | `13` |  |
| `KVM_CAP_MP_STATE` | `14` |  |
| `KVM_CAP_COALESCED_MMIO` | `15` |  |
| `KVM_CAP_SYNC_MMU` | `16` | Changes to host mmap are reflected in guest |
| `KVM_CAP_IOMMU` | `18` |  |
| `KVM_CAP_DESTROY_MEMORY_REGION_WORKS` | `21` |  |
| `KVM_CAP_USER_NMI` | `22` |  |
| `KVM_CAP_SET_GUEST_DEBUG` | `23` |  |
| `KVM_CAP_REINJECT_CONTROL` | `24` |  |
| `KVM_CAP_IRQ_ROUTING` | `25` |  |
| `KVM_CAP_IRQ_INJECT_STATUS` | `26` |  |
| `KVM_CAP_ASSIGN_DEV_IRQ` | `29` |  |
| `KVM_CAP_JOIN_MEMORY_REGIONS_WORKS` | `30` |  |
| `KVM_CAP_MCE` | `31` |  |
| `KVM_CAP_IRQFD` | `32` |  |
| `KVM_CAP_PIT2` | `33` |  |
| `KVM_CAP_SET_BOOT_CPU_ID` | `34` |  |
| `KVM_CAP_PIT_STATE2` | `35` |  |
| `KVM_CAP_IOEVENTFD` | `36` |  |
| `KVM_CAP_SET_IDENTITY_MAP_ADDR` | `37` |  |
| `KVM_CAP_XEN_HVM` | `38` |  |
| `KVM_CAP_ADJUST_CLOCK` | `39` |  |
| `KVM_CAP_INTERNAL_ERROR_DATA` | `40` |  |
| `KVM_CAP_VCPU_EVENTS` | `41` |  |
| `KVM_CAP_S390_PSW` | `42` |  |
| `KVM_CAP_PPC_SEGSTATE` | `43` |  |
| `KVM_CAP_HYPERV` | `44` |  |
| `KVM_CAP_HYPERV_VAPIC` | `45` |  |
| `KVM_CAP_HYPERV_SPIN` | `46` |  |
| `KVM_CAP_PCI_SEGMENT` | `47` |  |
| `KVM_CAP_PPC_PAIRED_SINGLES` | `48` |  |
| `KVM_CAP_INTR_SHADOW` | `49` |  |
| `KVM_CAP_DEBUGREGS` | `50` |  |
| `KVM_CAP_X86_ROBUST_SINGLESTEP` | `51` |  |
| `KVM_CAP_PPC_OSI` | `52` |  |
| `KVM_CAP_PPC_UNSET_IRQ` | `53` |  |
| `KVM_CAP_ENABLE_CAP` | `54` |  |
| `KVM_CAP_XSAVE` | `55` |  |

*...and 193 more*

### KVM_CHECK (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_CHECK_EXTENSION` | `_IO(KVMIO,   0x03)` |  |

### KVM_CLEAR (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_CLEAR_DIRTY_LOG` | `_IOWR(KVMIO, 0xc0, struct kvm_clear_dirty_log)` |  |

### KVM_CLOCK (3)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_CLOCK_TSC_STABLE` | `2` |  |
| `KVM_CLOCK_REALTIME` | `(1 << 2)` |  |
| `KVM_CLOCK_HOST_TSC` | `(1 << 3)` |  |

### KVM_COALESCED (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_COALESCED_MMIO_MAX` | `` |  |

### KVM_CREATE (10)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_CREATE_VM` | `_IO(KVMIO,   0x01)` | returns a VM fd |
| `KVM_CREATE_DEVICE_TEST` | `1` |  |
| `KVM_CREATE_VCPU` | `_IO(KVMIO,   0x41)` |  |
| `KVM_CREATE_IRQCHIP` | `_IO(KVMIO,   0x60)` |  |
| `KVM_CREATE_PIT` | `_IO(KVMIO,   0x64)` |  |
| `KVM_CREATE_PIT2` | `_IOW(KVMIO,  0x77, struct kvm_pit_config)` |  |
| `KVM_CREATE_SPAPR_TCE` | `_IOW(KVMIO,  0xa8, struct kvm_create_spapr_tce)` |  |
| `KVM_CREATE_SPAPR_TCE_64` | `_IOW(KVMIO,  0xa8, ` |  |
| `KVM_CREATE_DEVICE` | `_IOWR(KVMIO,  0xe0, struct kvm_create_device)` |  |
| `KVM_CREATE_GUEST_MEMFD` | `_IOWR(KVMIO,  0xd4, struct kvm_create_guest_memfd)` |  |

### KVM_DEV (23)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_DEV_VFIO_FILE` | `1` |  |
| `KVM_DEV_VFIO_FILE_ADD` | `1` |  |
| `KVM_DEV_VFIO_FILE_DEL` | `2` |  |
| `KVM_DEV_VFIO_GROUP` | `KVM_DEV_VFIO_FILE` |  |
| `KVM_DEV_VFIO_GROUP_ADD` | `KVM_DEV_VFIO_FILE_ADD` |  |
| `KVM_DEV_VFIO_GROUP_DEL` | `KVM_DEV_VFIO_FILE_DEL` |  |
| `KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE` | `3` |  |
| `KVM_DEV_TYPE_FSL_MPIC_20` | `KVM_DEV_TYPE_FSL_MPIC_20` |  |
| `KVM_DEV_TYPE_FSL_MPIC_42` | `KVM_DEV_TYPE_FSL_MPIC_42` |  |
| `KVM_DEV_TYPE_XICS` | `KVM_DEV_TYPE_XICS` |  |
| `KVM_DEV_TYPE_VFIO` | `KVM_DEV_TYPE_VFIO` |  |
| `KVM_DEV_TYPE_ARM_VGIC_V2` | `KVM_DEV_TYPE_ARM_VGIC_V2` |  |
| `KVM_DEV_TYPE_FLIC` | `KVM_DEV_TYPE_FLIC` |  |
| `KVM_DEV_TYPE_ARM_VGIC_V3` | `KVM_DEV_TYPE_ARM_VGIC_V3` |  |
| `KVM_DEV_TYPE_ARM_VGIC_ITS` | `KVM_DEV_TYPE_ARM_VGIC_ITS` |  |
| `KVM_DEV_TYPE_XIVE` | `KVM_DEV_TYPE_XIVE` |  |
| `KVM_DEV_TYPE_ARM_PV_TIME` | `KVM_DEV_TYPE_ARM_PV_TIME` |  |
| `KVM_DEV_TYPE_RISCV_AIA` | `KVM_DEV_TYPE_RISCV_AIA` |  |
| `KVM_DEV_TYPE_LOONGARCH_IPI` | `KVM_DEV_TYPE_LOONGARCH_IPI` |  |
| `KVM_DEV_TYPE_LOONGARCH_EIOINTC` | `KVM_DEV_TYPE_LOONGARCH_EIOINTC` |  |
| `KVM_DEV_TYPE_LOONGARCH_PCHPIC` | `KVM_DEV_TYPE_LOONGARCH_PCHPIC` |  |
| `KVM_DEV_TYPE_LOONGARCH_DMSINTC` | `KVM_DEV_TYPE_LOONGARCH_DMSINTC` |  |
| `KVM_DEV_TYPE_ARM_VGIC_V5` | `KVM_DEV_TYPE_ARM_VGIC_V5` |  |

### KVM_DIRTY (7)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_DIRTY_TLB` | `_IOW(KVMIO,  0xaa, struct kvm_dirty_tlb)` |  |
| `KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE` | `(1 << 0)` |  |
| `KVM_DIRTY_LOG_INITIALLY_SET` | `(1 << 1)` |  |
| `KVM_DIRTY_LOG_PAGE_OFFSET` | `0` |  |
| `KVM_DIRTY_GFN_F_DIRTY` | `_BITUL(0)` |  |
| `KVM_DIRTY_GFN_F_RESET` | `_BITUL(1)` |  |
| `KVM_DIRTY_GFN_F_MASK` | `0x3` |  |

### KVM_ENABLE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_ENABLE_CAP` | `_IOW(KVMIO,  0xa3, struct kvm_enable_cap)` |  |

### KVM_EXIT (51)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_EXIT_HYPERV_SYNIC` | `1` |  |
| `KVM_EXIT_HYPERV_HCALL` | `2` |  |
| `KVM_EXIT_HYPERV_SYNDBG` | `3` |  |
| `KVM_EXIT_XEN_HCALL` | `1` |  |
| `KVM_EXIT_UNKNOWN` | `0` |  |
| `KVM_EXIT_EXCEPTION` | `1` |  |
| `KVM_EXIT_IO` | `2` |  |
| `KVM_EXIT_HYPERCALL` | `3` |  |
| `KVM_EXIT_DEBUG` | `4` |  |
| `KVM_EXIT_HLT` | `5` |  |
| `KVM_EXIT_MMIO` | `6` |  |
| `KVM_EXIT_IRQ_WINDOW_OPEN` | `7` |  |
| `KVM_EXIT_SHUTDOWN` | `8` |  |
| `KVM_EXIT_FAIL_ENTRY` | `9` |  |
| `KVM_EXIT_INTR` | `10` |  |
| `KVM_EXIT_SET_TPR` | `11` |  |
| `KVM_EXIT_TPR_ACCESS` | `12` |  |
| `KVM_EXIT_S390_SIEIC` | `13` |  |
| `KVM_EXIT_S390_RESET` | `14` |  |
| `KVM_EXIT_DCR` | `15` | deprecated |
| `KVM_EXIT_NMI` | `16` |  |
| `KVM_EXIT_INTERNAL_ERROR` | `17` |  |
| `KVM_EXIT_OSI` | `18` |  |
| `KVM_EXIT_PAPR_HCALL` | `19` |  |
| `KVM_EXIT_S390_UCONTROL` | `20` |  |
| `KVM_EXIT_WATCHDOG` | `21` |  |
| `KVM_EXIT_S390_TSCH` | `22` |  |
| `KVM_EXIT_EPR` | `23` |  |
| `KVM_EXIT_SYSTEM_EVENT` | `24` |  |
| `KVM_EXIT_S390_STSI` | `25` |  |
| `KVM_EXIT_IOAPIC_EOI` | `26` |  |
| `KVM_EXIT_HYPERV` | `27` |  |
| `KVM_EXIT_ARM_NISV` | `28` |  |
| `KVM_EXIT_X86_RDMSR` | `29` |  |
| `KVM_EXIT_X86_WRMSR` | `30` |  |
| `KVM_EXIT_DIRTY_RING_FULL` | `31` |  |
| `KVM_EXIT_AP_RESET_HOLD` | `32` |  |
| `KVM_EXIT_X86_BUS_LOCK` | `33` |  |
| `KVM_EXIT_XEN` | `34` |  |
| `KVM_EXIT_RISCV_SBI` | `35` |  |
| `KVM_EXIT_RISCV_CSR` | `36` |  |
| `KVM_EXIT_NOTIFY` | `37` |  |
| `KVM_EXIT_LOONGARCH_IOCSR` | `38` |  |
| `KVM_EXIT_MEMORY_FAULT` | `39` |  |
| `KVM_EXIT_TDX` | `40` |  |
| `KVM_EXIT_ARM_SEA` | `41` |  |
| `KVM_EXIT_ARM_LDST64B` | `42` |  |
| `KVM_EXIT_SNP_REQ_CERTS` | `43` |  |
| `KVM_EXIT_IO_IN` | `0` |  |
| `KVM_EXIT_IO_OUT` | `1` |  |

*...and 1 more*

### KVM_GET (32)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_GET_API_VERSION` | `_IO(KVMIO,   0x00)` |  |
| `KVM_GET_MSR_INDEX_LIST` | `_IOWR(KVMIO, 0x02, struct kvm_msr_list)` |  |
| `KVM_GET_VCPU_MMAP_SIZE` | `_IO(KVMIO,   0x04)` | in bytes |
| `KVM_GET_SUPPORTED_CPUID` | `_IOWR(KVMIO, 0x05, struct kvm_cpuid2)` |  |
| `KVM_GET_EMULATED_CPUID` | `_IOWR(KVMIO, 0x09, struct kvm_cpuid2)` |  |
| `KVM_GET_MSR_FEATURE_INDEX_LIST` | `_IOWR(KVMIO, 0x0a, struct kvm_msr_list)` |  |
| `KVM_GET_DIRTY_LOG` | `_IOW(KVMIO,  0x42, struct kvm_dirty_log)` |  |
| `KVM_GET_NR_MMU_PAGES` | `_IO(KVMIO,   0x45)` | deprecated |
| `KVM_GET_IRQCHIP` | `_IOWR(KVMIO, 0x62, struct kvm_irqchip)` |  |
| `KVM_GET_PIT` | `_IOWR(KVMIO, 0x65, struct kvm_pit_state)` |  |
| `KVM_GET_CLOCK` | `_IOR(KVMIO,  0x7c, struct kvm_clock_data)` |  |
| `KVM_GET_PIT2` | `_IOR(KVMIO,  0x9f, struct kvm_pit_state2)` |  |
| `KVM_GET_TSC_KHZ` | `_IO(KVMIO,  0xa3)` |  |
| `KVM_GET_DEVICE_ATTR` | `_IOW(KVMIO,  0xe2, struct kvm_device_attr)` |  |
| `KVM_GET_REGS` | `_IOR(KVMIO,  0x81, struct kvm_regs)` |  |
| `KVM_GET_SREGS` | `_IOR(KVMIO,  0x83, struct kvm_sregs)` |  |
| `KVM_GET_MSRS` | `_IOWR(KVMIO, 0x88, struct kvm_msrs)` |  |
| `KVM_GET_FPU` | `_IOR(KVMIO,  0x8c, struct kvm_fpu)` |  |
| `KVM_GET_LAPIC` | `_IOR(KVMIO,  0x8e, struct kvm_lapic_state)` |  |
| `KVM_GET_CPUID2` | `_IOWR(KVMIO, 0x91, struct kvm_cpuid2)` |  |
| `KVM_GET_MP_STATE` | `_IOR(KVMIO,  0x98, struct kvm_mp_state)` |  |
| `KVM_GET_VCPU_EVENTS` | `_IOR(KVMIO,  0x9f, struct kvm_vcpu_events)` |  |
| `KVM_GET_DEBUGREGS` | `_IOR(KVMIO,  0xa1, struct kvm_debugregs)` |  |
| `KVM_GET_XSAVE` | `_IOR(KVMIO,  0xa4, struct kvm_xsave)` |  |
| `KVM_GET_XCRS` | `_IOR(KVMIO,  0xa6, struct kvm_xcrs)` |  |
| `KVM_GET_ONE_REG` | `_IOW(KVMIO,  0xab, struct kvm_one_reg)` |  |
| `KVM_GET_REG_LIST` | `_IOWR(KVMIO, 0xb0, struct kvm_reg_list)` |  |
| `KVM_GET_NESTED_STATE` | `_IOWR(KVMIO, 0xbe, struct kvm_nested_state)` |  |
| `KVM_GET_SUPPORTED_HV_CPUID` | `_IOWR(KVMIO, 0xc1, struct kvm_cpuid2)` |  |
| `KVM_GET_SREGS2` | `_IOR(KVMIO,  0xcc, struct kvm_sregs2)` |  |
| `KVM_GET_STATS_FD` | `_IO(KVMIO,  0xce)` |  |
| `KVM_GET_XSAVE2` | `_IOR(KVMIO,  0xcf, struct kvm_xsave)` |  |

### KVM_GUESTDBG (2)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_GUESTDBG_ENABLE` | `0x00000001` |  |
| `KVM_GUESTDBG_SINGLESTEP` | `0x00000002` |  |

### KVM_HAS (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_HAS_DEVICE_ATTR` | `_IOW(KVMIO,  0xe3, struct kvm_device_attr)` |  |

### KVM_HYPERV (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_HYPERV_EVENTFD` | `_IOW(KVMIO,  0xbd, struct kvm_hyperv_eventfd)` |  |

### KVM_INTERNAL (5)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_INTERNAL_ERROR_EMULATION` | `1` |  |
| `KVM_INTERNAL_ERROR_SIMUL_EX` | `2` |  |
| `KVM_INTERNAL_ERROR_DELIVERY_EV` | `3` |  |
| `KVM_INTERNAL_ERROR_UNEXPECTED_EXIT_REASON` | `4` |  |
| `KVM_INTERNAL_ERROR_EMULATION_FLAG_INSTRUCTION_BYTES` | `(1ULL << 0)` |  |

### KVM_IOEVENTFD (5)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_IOEVENTFD_FLAG_DATAMATCH` | `(1 << kvm_ioeventfd_flag_nr_datamatch)` |  |
| `KVM_IOEVENTFD_FLAG_PIO` | `(1 << kvm_ioeventfd_flag_nr_pio)` |  |
| `KVM_IOEVENTFD_FLAG_DEASSIGN` | `(1 << kvm_ioeventfd_flag_nr_deassign)` |  |
| `KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY` | `` |  |
| `KVM_IOEVENTFD_VALID_FLAG_MASK` | `((1 << kvm_ioeventfd_flag_nr_max) - 1)` |  |

### KVM_IRQ (8)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_IRQ_ROUTING_XEN_EVTCHN_PRIO_2LEVEL` | `((__u32)(-1))` |  |
| `KVM_IRQ_ROUTING_IRQCHIP` | `1` |  |
| `KVM_IRQ_ROUTING_MSI` | `2` |  |
| `KVM_IRQ_ROUTING_S390_ADAPTER` | `3` |  |
| `KVM_IRQ_ROUTING_HV_SINT` | `4` |  |
| `KVM_IRQ_ROUTING_XEN_EVTCHN` | `5` |  |
| `KVM_IRQ_LINE` | `_IOW(KVMIO,  0x61, struct kvm_irq_level)` |  |
| `KVM_IRQ_LINE_STATUS` | `_IOWR(KVMIO, 0x67, struct kvm_irq_level)` |  |

### KVM_IRQFD (2)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_IRQFD_FLAG_DEASSIGN` | `(1 << 0)` |  |
| `KVM_IRQFD_FLAG_RESAMPLE` | `(1 << 1)` |  |

### KVM_KVMCLOCK (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_KVMCLOCK_CTRL` | `_IO(KVMIO,   0xad)` |  |

### KVM_MEM (3)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_MEM_LOG_DIRTY_PAGES` | `(1UL << 0)` |  |
| `KVM_MEM_READONLY` | `(1UL << 1)` |  |
| `KVM_MEM_GUEST_MEMFD` | `(1UL << 2)` |  |

### KVM_MEMORY (5)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_MEMORY_EXIT_FLAG_PRIVATE` | `(1ULL << 3)` |  |
| `KVM_MEMORY_ENCRYPT_OP` | `_IOWR(KVMIO, 0xba, unsigned long)` |  |
| `KVM_MEMORY_ENCRYPT_REG_REGION` | `_IOR(KVMIO, 0xbb, struct kvm_enc_region)` |  |
| `KVM_MEMORY_ENCRYPT_UNREG_REGION` | `_IOR(KVMIO, 0xbc, struct kvm_enc_region)` |  |
| `KVM_MEMORY_ATTRIBUTE_PRIVATE` | `(1ULL << 3)` |  |

### KVM_MMU (2)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_MMU_FSL_BOOKE_NOHV` | `0` |  |
| `KVM_MMU_FSL_BOOKE_HV` | `1` |  |

### KVM_MP (11)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_MP_STATE_RUNNABLE` | `0` |  |
| `KVM_MP_STATE_UNINITIALIZED` | `1` |  |
| `KVM_MP_STATE_INIT_RECEIVED` | `2` |  |
| `KVM_MP_STATE_HALTED` | `3` |  |
| `KVM_MP_STATE_SIPI_RECEIVED` | `4` |  |
| `KVM_MP_STATE_STOPPED` | `5` |  |
| `KVM_MP_STATE_CHECK_STOP` | `6` |  |
| `KVM_MP_STATE_OPERATING` | `7` |  |
| `KVM_MP_STATE_LOAD` | `8` |  |
| `KVM_MP_STATE_AP_RESET_HOLD` | `9` |  |
| `KVM_MP_STATE_SUSPENDED` | `10` |  |

### KVM_MSI (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_MSI_VALID_DEVID` | `(1U << 0)` |  |

### KVM_MSR (4)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_MSR_EXIT_REASON_INVAL` | `(1 << 0)` |  |
| `KVM_MSR_EXIT_REASON_UNKNOWN` | `(1 << 1)` |  |
| `KVM_MSR_EXIT_REASON_FILTER` | `(1 << 2)` |  |
| `KVM_MSR_EXIT_REASON_VALID_MASK` | `(KVM_MSR_EXIT_REASON_INVAL   \|	` |  |

### KVM_NOTIFY (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_NOTIFY_CONTEXT_INVALID` | `(1 << 0)` |  |

### KVM_PIT (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_PIT_SPEAKER_DUMMY` | `1` |  |

### KVM_PMU (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_PMU_CAP_DISABLE` | `(1 << 0)` |  |

### KVM_PPC (11)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_PPC_GET_PVINFO` | `_IOW(KVMIO,  0xa1, struct kvm_ppc_pvinfo)` |  |
| `KVM_PPC_GET_SMMU_INFO` | `_IOR(KVMIO,  0xa6, struct kvm_ppc_smmu_info)` |  |
| `KVM_PPC_ALLOCATE_HTAB` | `_IOWR(KVMIO, 0xa7, __u32)` |  |
| `KVM_PPC_GET_HTAB_FD` | `_IOW(KVMIO,  0xaa, struct kvm_get_htab_fd)` |  |
| `KVM_PPC_RTAS_DEFINE_TOKEN` | `_IOW(KVMIO,  0xac, struct kvm_rtas_token_args)` |  |
| `KVM_PPC_RESIZE_HPT_PREPARE` | `_IOR(KVMIO, 0xad, struct kvm_ppc_resize_hpt)` |  |
| `KVM_PPC_RESIZE_HPT_COMMIT` | `_IOR(KVMIO, 0xae, struct kvm_ppc_resize_hpt)` |  |
| `KVM_PPC_CONFIGURE_V3_MMU` | `_IOW(KVMIO,  0xaf, struct kvm_ppc_mmuv3_cfg)` |  |
| `KVM_PPC_GET_RMMU_INFO` | `_IOW(KVMIO,  0xb0, struct kvm_ppc_rmmu_info)` |  |
| `KVM_PPC_GET_CPU_CHAR` | `_IOR(KVMIO,  0xb1, struct kvm_ppc_cpu_char)` |  |
| `KVM_PPC_SVM_OFF` | `_IO(KVMIO,  0xb3)` |  |

### KVM_PRE (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_PRE_FAULT_MEMORY` | `_IOWR(KVMIO, 0xd5, struct kvm_pre_fault_memory)` |  |

### KVM_REG (22)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_REG_ARCH_MASK` | `0xff00000000000000ULL` |  |
| `KVM_REG_GENERIC` | `0x0000000000000000ULL` |  |
| `KVM_REG_PPC` | `0x1000000000000000ULL` |  |
| `KVM_REG_X86` | `0x2000000000000000ULL` |  |
| `KVM_REG_IA64` | `0x3000000000000000ULL` |  |
| `KVM_REG_ARM` | `0x4000000000000000ULL` |  |
| `KVM_REG_S390` | `0x5000000000000000ULL` |  |
| `KVM_REG_ARM64` | `0x6000000000000000ULL` |  |
| `KVM_REG_MIPS` | `0x7000000000000000ULL` |  |
| `KVM_REG_RISCV` | `0x8000000000000000ULL` |  |
| `KVM_REG_LOONGARCH` | `0x9000000000000000ULL` |  |
| `KVM_REG_SIZE_SHIFT` | `52` |  |
| `KVM_REG_SIZE_MASK` | `0x00f0000000000000ULL` |  |
| `KVM_REG_SIZE_U8` | `0x0000000000000000ULL` |  |
| `KVM_REG_SIZE_U16` | `0x0010000000000000ULL` |  |
| `KVM_REG_SIZE_U32` | `0x0020000000000000ULL` |  |
| `KVM_REG_SIZE_U64` | `0x0030000000000000ULL` |  |
| `KVM_REG_SIZE_U128` | `0x0040000000000000ULL` |  |
| `KVM_REG_SIZE_U256` | `0x0050000000000000ULL` |  |
| `KVM_REG_SIZE_U512` | `0x0060000000000000ULL` |  |
| `KVM_REG_SIZE_U1024` | `0x0070000000000000ULL` |  |
| `KVM_REG_SIZE_U2048` | `0x0080000000000000ULL` |  |

### KVM_REGISTER (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_REGISTER_COALESCED_MMIO` | `` |  |

### KVM_REINJECT (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_REINJECT_CONTROL` | `_IO(KVMIO,   0x71)` |  |

### KVM_RESET (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_RESET_DIRTY_RINGS` | `_IO(KVMIO, 0xc7)` |  |

### KVM_SET (33)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_SET_NR_MMU_PAGES` | `_IO(KVMIO,   0x44)` |  |
| `KVM_SET_USER_MEMORY_REGION` | `_IOW(KVMIO, 0x46, ` |  |
| `KVM_SET_TSS_ADDR` | `_IO(KVMIO,   0x47)` |  |
| `KVM_SET_IDENTITY_MAP_ADDR` | `_IOW(KVMIO,  0x48, __u64)` |  |
| `KVM_SET_USER_MEMORY_REGION2` | `_IOW(KVMIO, 0x49, ` |  |
| `KVM_SET_IRQCHIP` | `_IOR(KVMIO,  0x63, struct kvm_irqchip)` |  |
| `KVM_SET_PIT` | `_IOR(KVMIO,  0x66, struct kvm_pit_state)` |  |
| `KVM_SET_GSI_ROUTING` | `_IOW(KVMIO,  0x6a, struct kvm_irq_routing)` |  |
| `KVM_SET_BOOT_CPU_ID` | `_IO(KVMIO,   0x78)` |  |
| `KVM_SET_CLOCK` | `_IOW(KVMIO,  0x7b, struct kvm_clock_data)` |  |
| `KVM_SET_PIT2` | `_IOW(KVMIO,  0xa0, struct kvm_pit_state2)` |  |
| `KVM_SET_TSC_KHZ` | `_IO(KVMIO,  0xa2)` |  |
| `KVM_SET_PMU_EVENT_FILTER` | `_IOW(KVMIO,  0xb2, struct kvm_pmu_event_filter)` |  |
| `KVM_SET_DEVICE_ATTR` | `_IOW(KVMIO,  0xe1, struct kvm_device_attr)` |  |
| `KVM_SET_REGS` | `_IOW(KVMIO,  0x82, struct kvm_regs)` |  |
| `KVM_SET_SREGS` | `_IOW(KVMIO,  0x84, struct kvm_sregs)` |  |
| `KVM_SET_MSRS` | `_IOW(KVMIO,  0x89, struct kvm_msrs)` |  |
| `KVM_SET_CPUID` | `_IOW(KVMIO,  0x8a, struct kvm_cpuid)` |  |
| `KVM_SET_SIGNAL_MASK` | `_IOW(KVMIO,  0x8b, struct kvm_signal_mask)` |  |
| `KVM_SET_FPU` | `_IOW(KVMIO,  0x8d, struct kvm_fpu)` |  |
| `KVM_SET_LAPIC` | `_IOW(KVMIO,  0x8f, struct kvm_lapic_state)` |  |
| `KVM_SET_CPUID2` | `_IOW(KVMIO,  0x90, struct kvm_cpuid2)` |  |
| `KVM_SET_VAPIC_ADDR` | `_IOW(KVMIO,  0x93, struct kvm_vapic_addr)` |  |
| `KVM_SET_MP_STATE` | `_IOW(KVMIO,  0x99, struct kvm_mp_state)` |  |
| `KVM_SET_GUEST_DEBUG` | `_IOW(KVMIO,  0x9b, struct kvm_guest_debug)` |  |
| `KVM_SET_VCPU_EVENTS` | `_IOW(KVMIO,  0xa0, struct kvm_vcpu_events)` |  |
| `KVM_SET_DEBUGREGS` | `_IOW(KVMIO,  0xa2, struct kvm_debugregs)` |  |
| `KVM_SET_XSAVE` | `_IOW(KVMIO,  0xa5, struct kvm_xsave)` |  |
| `KVM_SET_XCRS` | `_IOW(KVMIO,  0xa7, struct kvm_xcrs)` |  |
| `KVM_SET_ONE_REG` | `_IOW(KVMIO,  0xac, struct kvm_one_reg)` |  |
| `KVM_SET_NESTED_STATE` | `_IOW(KVMIO,  0xbf, struct kvm_nested_state)` |  |
| `KVM_SET_SREGS2` | `_IOW(KVMIO,  0xcd, struct kvm_sregs2)` |  |
| `KVM_SET_MEMORY_ATTRIBUTES` | `_IOW(KVMIO,  0xd2, struct kvm_memory_attributes)` |  |

### KVM_SIGNAL (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_SIGNAL_MSI` | `_IOW(KVMIO,  0xa5, struct kvm_msi)` |  |

### KVM_STATS (21)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_STATS_TYPE_SHIFT` | `0` |  |
| `KVM_STATS_TYPE_MASK` | `(0xF << KVM_STATS_TYPE_SHIFT)` |  |
| `KVM_STATS_TYPE_CUMULATIVE` | `(0x0 << KVM_STATS_TYPE_SHIFT)` |  |
| `KVM_STATS_TYPE_INSTANT` | `(0x1 << KVM_STATS_TYPE_SHIFT)` |  |
| `KVM_STATS_TYPE_PEAK` | `(0x2 << KVM_STATS_TYPE_SHIFT)` |  |
| `KVM_STATS_TYPE_LINEAR_HIST` | `(0x3 << KVM_STATS_TYPE_SHIFT)` |  |
| `KVM_STATS_TYPE_LOG_HIST` | `(0x4 << KVM_STATS_TYPE_SHIFT)` |  |
| `KVM_STATS_TYPE_MAX` | `KVM_STATS_TYPE_LOG_HIST` |  |
| `KVM_STATS_UNIT_SHIFT` | `4` |  |
| `KVM_STATS_UNIT_MASK` | `(0xF << KVM_STATS_UNIT_SHIFT)` |  |
| `KVM_STATS_UNIT_NONE` | `(0x0 << KVM_STATS_UNIT_SHIFT)` |  |
| `KVM_STATS_UNIT_BYTES` | `(0x1 << KVM_STATS_UNIT_SHIFT)` |  |
| `KVM_STATS_UNIT_SECONDS` | `(0x2 << KVM_STATS_UNIT_SHIFT)` |  |
| `KVM_STATS_UNIT_CYCLES` | `(0x3 << KVM_STATS_UNIT_SHIFT)` |  |
| `KVM_STATS_UNIT_BOOLEAN` | `(0x4 << KVM_STATS_UNIT_SHIFT)` |  |
| `KVM_STATS_UNIT_MAX` | `KVM_STATS_UNIT_BOOLEAN` |  |
| `KVM_STATS_BASE_SHIFT` | `8` |  |
| `KVM_STATS_BASE_MASK` | `(0xF << KVM_STATS_BASE_SHIFT)` |  |
| `KVM_STATS_BASE_POW10` | `(0x0 << KVM_STATS_BASE_SHIFT)` |  |
| `KVM_STATS_BASE_POW2` | `(0x1 << KVM_STATS_BASE_SHIFT)` |  |
| `KVM_STATS_BASE_MAX` | `KVM_STATS_BASE_POW2` |  |

### KVM_SYSTEM (7)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_SYSTEM_EVENT_SHUTDOWN` | `1` |  |
| `KVM_SYSTEM_EVENT_RESET` | `2` |  |
| `KVM_SYSTEM_EVENT_CRASH` | `3` |  |
| `KVM_SYSTEM_EVENT_WAKEUP` | `4` |  |
| `KVM_SYSTEM_EVENT_SUSPEND` | `5` |  |
| `KVM_SYSTEM_EVENT_SEV_TERM` | `6` |  |
| `KVM_SYSTEM_EVENT_TDX_FATAL` | `7` |  |

### KVM_TPR (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_TPR_ACCESS_REPORTING` | `_IOWR(KVMIO, 0x92, struct kvm_tpr_access_ctl)` |  |

### KVM_UNREGISTER (1)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_UNREGISTER_COALESCED_MMIO` | `` |  |

### KVM_VM (9)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_VM_S390_UCONTROL` | `1` |  |
| `KVM_VM_PPC_HV` | `1` |  |
| `KVM_VM_PPC_PR` | `2` |  |
| `KVM_VM_MIPS_AUTO` | `0` |  |
| `KVM_VM_MIPS_VZ` | `1` |  |
| `KVM_VM_MIPS_TE` | `2` |  |
| `KVM_VM_TYPE_ARM_IPA_SIZE_MASK` | `0xffULL` |  |
| `KVM_VM_TYPE_ARM_PROTECTED` | `(1UL << 31)` |  |
| `KVM_VM_TYPE_ARM_MASK` | `(KVM_VM_TYPE_ARM_IPA_SIZE_MASK \| ` |  |

### KVM_XEN (6)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_XEN_HVM_CONFIG` | `_IOW(KVMIO,  0x7a, struct kvm_xen_hvm_config)` |  |
| `KVM_XEN_HVM_GET_ATTR` | `_IOWR(KVMIO, 0xc8, struct kvm_xen_hvm_attr)` |  |
| `KVM_XEN_HVM_SET_ATTR` | `_IOW(KVMIO,  0xc9, struct kvm_xen_hvm_attr)` |  |
| `KVM_XEN_VCPU_GET_ATTR` | `_IOWR(KVMIO, 0xca, struct kvm_xen_vcpu_attr)` |  |
| `KVM_XEN_VCPU_SET_ATTR` | `_IOW(KVMIO,  0xcb, struct kvm_xen_vcpu_attr)` |  |
| `KVM_XEN_HVM_EVTCHN_SEND` | `_IOW(KVMIO,  0xd0, struct kvm_irq_routing_xen_evtchn)` |  |

### SYNC_REGS (1)

| Name | Value | Comment |
|------|-------|---------|
| `SYNC_REGS_SIZE_BYTES` | `2048` |  |

### UNCATEGORIZED (49)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_S390_GET_SKEYS_NONE` | `1` |  |
| `KVM_S390_SKEYS_MAX` | `1048576` |  |
| `KVM_X86_DISABLE_EXITS_MWAIT` | `(1 << 0)` |  |
| `KVM_X86_DISABLE_EXITS_HLT` | `(1 << 1)` |  |
| `KVM_X86_DISABLE_EXITS_PAUSE` | `(1 << 2)` |  |
| `KVM_X86_DISABLE_EXITS_CSTATE` | `(1 << 3)` |  |
| `KVM_X86_DISABLE_EXITS_APERFMPERF` | `(1 << 4)` |  |
| `KVMIO` | `0xAE` |  |
| `KVM_S390_SIE_PAGE_OFFSET` | `1` |  |
| `KVM_S390_ENABLE_SIE` | `_IO(KVMIO,   0x06)` |  |
| `KVM_S390_KEYOP_ISKE` | `0x01` |  |
| `KVM_S390_KEYOP_RRBE` | `0x02` |  |
| `KVM_S390_KEYOP_SSKE` | `0x03` |  |
| `KVM_S390_UCAS_MAP` | `_IOW(KVMIO, 0x50, struct kvm_s390_ucas_mapping)` |  |
| `KVM_S390_UCAS_UNMAP` | `_IOW(KVMIO, 0x51, struct kvm_s390_ucas_mapping)` |  |
| `KVM_S390_VCPU_FAULT` | `_IOW(KVMIO, 0x52, unsigned long)` |  |
| `KVM_S390_KEYOP` | `_IOWR(KVMIO, 0x53, struct kvm_s390_keyop)` |  |
| `KVM_IRQFD` | `_IOW(KVMIO,  0x76, struct kvm_irqfd)` |  |
| `KVM_IOEVENTFD` | `_IOW(KVMIO,  0x79, struct kvm_ioeventfd)` |  |
| `KVM_RUN` | `_IO(KVMIO,   0x80)` |  |
| `KVM_TRANSLATE` | `_IOWR(KVMIO, 0x85, struct kvm_translation)` |  |
| `KVM_INTERRUPT` | `_IOW(KVMIO,  0x86, struct kvm_interrupt)` |  |
| `KVM_S390_INTERRUPT` | `_IOW(KVMIO,  0x94, struct kvm_s390_interrupt)` |  |
| `KVM_S390_STORE_STATUS_NOADDR` | `(-1ul)` |  |
| `KVM_S390_STORE_STATUS_PREFIXED` | `(-2ul)` |  |
| `KVM_S390_STORE_STATUS` | `_IOW(KVMIO,  0x95, unsigned long)` |  |
| `KVM_S390_SET_INITIAL_PSW` | `_IOW(KVMIO,  0x96, struct kvm_s390_psw)` |  |
| `KVM_S390_INITIAL_RESET` | `_IO(KVMIO,   0x97)` |  |
| `KVM_NMI` | `_IO(KVMIO,   0x9a)` |  |
| `KVM_X86_SETUP_MCE` | `_IOW(KVMIO,  0x9c, __u64)` |  |
| `KVM_X86_GET_MCE_CAP_SUPPORTED` | `_IOR(KVMIO,  0x9d, __u64)` |  |
| `KVM_X86_SET_MCE` | `_IOW(KVMIO,  0x9e, struct kvm_x86_mce)` |  |
| `KVM_S390_MEM_OP` | `_IOW(KVMIO,  0xb1, struct kvm_s390_mem_op)` |  |
| `KVM_S390_GET_SKEYS` | `_IOW(KVMIO, 0xb2, struct kvm_s390_skeys)` |  |
| `KVM_S390_SET_SKEYS` | `_IOW(KVMIO, 0xb3, struct kvm_s390_skeys)` |  |
| `KVM_S390_IRQ` | `_IOW(KVMIO,  0xb4, struct kvm_s390_irq)` |  |
| `KVM_S390_SET_IRQ_STATE` | `_IOW(KVMIO, 0xb5, struct kvm_s390_irq_state)` |  |
| `KVM_S390_GET_IRQ_STATE` | `_IOW(KVMIO, 0xb6, struct kvm_s390_irq_state)` |  |
| `KVM_SMI` | `_IO(KVMIO,   0xb7)` |  |
| `KVM_S390_GET_CMMA_BITS` | `_IOWR(KVMIO, 0xb8, struct kvm_s390_cmma_log)` |  |
| `KVM_S390_SET_CMMA_BITS` | `_IOW(KVMIO, 0xb9, struct kvm_s390_cmma_log)` |  |
| `KVM_S390_NORMAL_RESET` | `_IO(KVMIO,   0xc3)` |  |
| `KVM_S390_CLEAR_RESET` | `_IO(KVMIO,   0xc4)` |  |
| `KVM_S390_PV_COMMAND` | `_IOWR(KVMIO, 0xc5, struct kvm_pv_cmd)` |  |
| `KVM_X86_SET_MSR_FILTER` | `_IOW(KVMIO,  0xc6, struct kvm_msr_filter)` |  |
| `KVM_S390_PV_CPU_COMMAND` | `_IOWR(KVMIO, 0xd0, struct kvm_pv_cmd)` |  |
| `KVM_X86_NOTIFY_VMEXIT_ENABLED` | `(1ULL << 0)` |  |
| `KVM_X86_NOTIFY_VMEXIT_USER` | `(1ULL << 1)` |  |
| `KVM_S390_ZPCI_OP` | `_IOW(KVMIO,  0xd1, struct kvm_s390_zpci_op)` |  |

## Structs (87)


### `struct kvm_userspace_memory_region`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `slot` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `guest_phys_addr` | `-` |
| `__u64` | `memory_size` | `-` |
| `__u64` | `userspace_addr` | `-` |

### `struct kvm_userspace_memory_region2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `slot` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `guest_phys_addr` | `-` |
| `__u64` | `memory_size` | `-` |
| `__u64` | `userspace_addr` | `-` |
| `__u64` | `guest_memfd_offset` | `-` |
| `__u32` | `guest_memfd` | `-` |
| `__u32` | `pad1` | `-` |
| `__u64` | `pad2` | `14` |

### `struct kvm_irq_level`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `irq` | `-` |
| `__s32` | `status` | `-` |
| `__u32` | `level` | `-` |

### `struct kvm_irqchip`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `chip_id` | `-` |
| `__u32` | `pad` | `-` |
| `char` | `dummy` | `512` |

### `struct kvm_pit_config`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `15` |

### `struct kvm_hyperv_exit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `pad1` | `-` |
| `__u32` | `msr` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `control` | `-` |
| `__u64` | `evt_page` | `-` |
| `__u64` | `msg_page` | `-` |
| `__u64` | `input` | `-` |
| `__u64` | `result` | `-` |
| `__u64` | `params` | `2` |
| `__u32` | `msr` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `control` | `-` |
| `__u64` | `status` | `-` |
| `__u64` | `send_page` | `-` |
| `__u64` | `recv_page` | `-` |
| `__u64` | `pending_page` | `-` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `msr` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `control` | `-` |
| `__u64` | `evt_page` | `-` |
| `__u64` | `msg_page` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `input` | `-` |
| `__u64` | `result` | `-` |
| `__u64` | `params` | `2` |

### `struct anonymous_8`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `msr` | `-` |
| `__u32` | `pad2` | `-` |
| `__u64` | `control` | `-` |
| `__u64` | `status` | `-` |
| `__u64` | `send_page` | `-` |
| `__u64` | `recv_page` | `-` |
| `__u64` | `pending_page` | `-` |

### `struct kvm_xen_exit`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `longmode` | `-` |
| `__u32` | `cpl` | `-` |
| `__u64` | `input` | `-` |
| `__u64` | `result` | `-` |
| `__u64` | `params` | `6` |

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `longmode` | `-` |
| `__u32` | `cpl` | `-` |
| `__u64` | `input` | `-` |
| `__u64` | `result` | `-` |
| `__u64` | `params` | `6` |

### `struct kvm_exit_snp_req_certs`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `gpa` | `-` |
| `__u64` | `npages` | `-` |
| `__u64` | `ret` | `-` |

### `struct kvm_run`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `request_interrupt_window` | `-` |
| `__u8` | `padding1` | `6` |
| `__u32` | `exit_reason` | `-` |
| `__u8` | `ready_for_interrupt_injection` | `-` |
| `__u8` | `if_flag` | `-` |
| `__u16` | `flags` | `-` |
| `__u64` | `cr8` | `-` |
| `__u64` | `apic_base` | `-` |
| `__u64` | `psw_mask` | `-` |
| `__u64` | `psw_addr` | `-` |
| `__u64` | `hardware_exit_reason` | `-` |
| `__u64` | `hardware_entry_failure_reason` | `-` |
| `__u32` | `cpu` | `-` |
| `__u32` | `exception` | `-` |
| `__u32` | `error_code` | `-` |
| `__u8` | `direction` | `-` |
| `__u8` | `size` | `-` |
| `__u16` | `port` | `-` |
| `__u32` | `count` | `-` |
| `__u64` | `data_offset` | `-` |
| `__u64` | `phys_addr` | `-` |
| `__u8` | `data` | `8` |
| `__u32` | `len` | `-` |
| `__u8` | `is_write` | `-` |
| `__u64` | `phys_addr` | `-` |
| `__u8` | `data` | `8` |
| `__u32` | `len` | `-` |
| `__u8` | `is_write` | `-` |
| `__u64` | `nr` | `-` |
| `__u64` | `args` | `6` |
| `__u64` | `ret` | `-` |
| `__u32` | `longmode` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `rip` | `-` |
| `__u32` | `is_write` | `-` |
| `__u32` | `pad` | `-` |
| `__u8` | `icptcode` | `-` |
| `__u16` | `ipa` | `-` |
| `__u32` | `ipb` | `-` |
| `__u64` | `s390_reset_flags` | `-` |
| `__u64` | `trans_exc_code` | `-` |
| `__u32` | `pgm_code` | `-` |
| `__u32` | `dcrn` | `-` |
| `__u32` | `data` | `-` |
| `__u8` | `is_write` | `-` |
| `__u32` | `suberror` | `-` |
| `__u32` | `ndata` | `-` |
| `__u64` | `data` | `16` |
| `__u32` | `suberror` | `-` |
| `__u32` | `ndata` | `-` |
| `__u64` | `flags` | `-` |
| `__u8` | `insn_size` | `-` |
| `__u8` | `insn_bytes` | `15` |
| `__u64` | `gprs` | `32` |
| `__u64` | `nr` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `args` | `9` |
| `__u16` | `subchannel_id` | `-` |
| `__u16` | `subchannel_nr` | `-` |
| `__u32` | `io_int_parm` | `-` |
| `__u32` | `io_int_word` | `-` |
| `__u32` | `ipb` | `-` |
| `__u8` | `dequeued` | `-` |
| `__u32` | `epr` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `ndata` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `data` | `16` |
| `__u64` | `addr` | `-` |
| `__u8` | `ar` | `-` |
| `__u8` | `reserved` | `-` |
| `__u8` | `fc` | `-` |
| `__u8` | `sel1` | `-` |
| `__u16` | `sel2` | `-` |
| `__u8` | `vector` | `-` |
| `__u64` | `esr_iss` | `-` |
| `__u64` | `fault_ipa` | `-` |
| `__u8` | `error` | `-` |
| `__u8` | `pad` | `7` |
| `__u32` | `reason` | `-` |
| `__u32` | `index` | `-` |
| `__u64` | `data` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `gpa` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `nr` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `data` | `5` |
| `__u64` | `ret` | `-` |
| `__u64` | `gpa` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `leaf` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `vector` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `esr` | `-` |
| `__u64` | `gva` | `-` |
| `__u64` | `gpa` | `-` |
| `char` | `padding` | `256` |
| `__u64` | `kvm_valid_regs` | `-` |
| `__u64` | `kvm_dirty_regs` | `-` |
| `char` | `padding` | `SYNC_REGS_SIZE_BYTES` |

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `hardware_exit_reason` | `-` |

### `struct anonymous_14`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `hardware_entry_failure_reason` | `-` |
| `__u32` | `cpu` | `-` |

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `exception` | `-` |
| `__u32` | `error_code` | `-` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `direction` | `-` |
| `__u8` | `size` | `-` |
| `__u16` | `port` | `-` |
| `__u32` | `count` | `-` |
| `__u64` | `data_offset` | `-` |

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `phys_addr` | `-` |
| `__u8` | `data` | `8` |
| `__u32` | `len` | `-` |
| `__u8` | `is_write` | `-` |

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `phys_addr` | `-` |
| `__u8` | `data` | `8` |
| `__u32` | `len` | `-` |
| `__u8` | `is_write` | `-` |

### `struct anonymous_20`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `nr` | `-` |
| `__u64` | `args` | `6` |
| `__u64` | `ret` | `-` |
| `__u32` | `longmode` | `-` |
| `__u64` | `flags` | `-` |

### `struct anonymous_21`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `rip` | `-` |
| `__u32` | `is_write` | `-` |
| `__u32` | `pad` | `-` |

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `icptcode` | `-` |
| `__u16` | `ipa` | `-` |
| `__u32` | `ipb` | `-` |

### `struct anonymous_23`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `trans_exc_code` | `-` |
| `__u32` | `pgm_code` | `-` |

### `struct anonymous_24`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dcrn` | `-` |
| `__u32` | `data` | `-` |
| `__u8` | `is_write` | `-` |

### `struct anonymous_25`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `suberror` | `-` |
| `__u32` | `ndata` | `-` |
| `__u64` | `data` | `16` |

### `struct anonymous_26`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `suberror` | `-` |
| `__u32` | `ndata` | `-` |
| `__u64` | `flags` | `-` |
| `__u8` | `insn_size` | `-` |
| `__u8` | `insn_bytes` | `15` |

### `struct anonymous_27`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `insn_size` | `-` |
| `__u8` | `insn_bytes` | `15` |

### `struct anonymous_28`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `gprs` | `32` |

### `struct anonymous_29`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `nr` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `args` | `9` |

### `struct anonymous_30`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `subchannel_id` | `-` |
| `__u16` | `subchannel_nr` | `-` |
| `__u32` | `io_int_parm` | `-` |
| `__u32` | `io_int_word` | `-` |
| `__u32` | `ipb` | `-` |
| `__u8` | `dequeued` | `-` |

### `struct anonymous_31`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `epr` | `-` |

### `struct anonymous_32`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `ndata` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `data` | `16` |

### `struct anonymous_33`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u8` | `ar` | `-` |
| `__u8` | `reserved` | `-` |
| `__u8` | `fc` | `-` |
| `__u8` | `sel1` | `-` |
| `__u16` | `sel2` | `-` |

### `struct anonymous_34`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `vector` | `-` |

### `struct anonymous_35`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `esr_iss` | `-` |
| `__u64` | `fault_ipa` | `-` |

### `struct anonymous_36`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `error` | `-` |
| `__u8` | `pad` | `7` |
| `__u32` | `reason` | `-` |
| `__u32` | `index` | `-` |
| `__u64` | `data` | `-` |

### `struct anonymous_37`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_38`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_39`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |

### `struct anonymous_40`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `gpa` | `-` |
| `__u64` | `size` | `-` |

### `struct anonymous_41`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `nr` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `data` | `5` |
| `__u64` | `ret` | `-` |
| `__u64` | `gpa` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `leaf` | `-` |
| `__u64` | `ret` | `-` |
| `__u64` | `vector` | `-` |

### `struct anonymous_42`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ret` | `-` |
| `__u64` | `data` | `5` |

### `struct anonymous_43`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ret` | `-` |
| `__u64` | `gpa` | `-` |
| `__u64` | `size` | `-` |

### `struct anonymous_44`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ret` | `-` |
| `__u64` | `leaf` | `-` |

### `struct anonymous_45`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ret` | `-` |
| `__u64` | `vector` | `-` |

### `struct anonymous_46`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `flags` | `-` |
| `__u64` | `esr` | `-` |
| `__u64` | `gva` | `-` |
| `__u64` | `gpa` | `-` |

### `struct kvm_coalesced_mmio_zone`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `pio` | `-` |

### `struct kvm_coalesced_mmio`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `phys_addr` | `-` |
| `__u32` | `len` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `pio` | `-` |
| `__u8` | `data` | `8` |

### `struct kvm_coalesced_mmio_ring`

| Type | Field | Array |
|------|-------|-------|

### `struct kvm_translation`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `linear_address` | `-` |
| `__u64` | `physical_address` | `-` |
| `__u8` | `valid` | `-` |
| `__u8` | `writeable` | `-` |
| `__u8` | `usermode` | `-` |
| `__u8` | `pad` | `5` |

### `struct kvm_interrupt`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `irq` | `-` |

### `struct kvm_dirty_log`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `slot` | `-` |
| `__u32` | `padding1` | `-` |
| `__u64` | `padding2` | `-` |

### `struct kvm_clear_dirty_log`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `slot` | `-` |
| `__u32` | `num_pages` | `-` |
| `__u64` | `first_page` | `-` |
| `__u64` | `padding2` | `-` |

### `struct kvm_signal_mask`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `len` | `-` |

### `struct kvm_tpr_access_ctl`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `enabled` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `8` |

### `struct kvm_vapic_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `vapic_addr` | `-` |

### `struct kvm_mp_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mp_state` | `-` |

### `struct kvm_guest_debug`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `control` | `-` |
| `__u32` | `pad` | `-` |

### `struct kvm_ioeventfd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `datamatch` | `-` |
| `__u64` | `addr` | `-` |
| `__u32` | `len` | `-` |
| `__s32` | `fd` | `-` |
| `__u32` | `flags` | `-` |
| `__u8` | `pad` | `36` |

### `struct kvm_enable_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cap` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `args` | `4` |
| `__u8` | `pad` | `64` |

### `struct kvm_irq_routing_irqchip`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `irqchip` | `-` |
| `__u32` | `pin` | `-` |

### `struct kvm_irq_routing_msi`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `address_lo` | `-` |
| `__u32` | `address_hi` | `-` |
| `__u32` | `data` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `devid` | `-` |

### `struct kvm_irq_routing_s390_adapter`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ind_addr` | `-` |
| `__u64` | `summary_addr` | `-` |
| `__u64` | `ind_offset` | `-` |
| `__u32` | `summary_offset` | `-` |
| `__u32` | `adapter_id` | `-` |

### `struct kvm_irq_routing_hv_sint`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vcpu` | `-` |
| `__u32` | `sint` | `-` |

### `struct kvm_irq_routing_xen_evtchn`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `port` | `-` |
| `__u32` | `vcpu` | `-` |
| `__u32` | `priority` | `-` |

### `struct kvm_irq_routing_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `gsi` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad` | `-` |
| `__u32` | `pad` | `8` |

### `struct kvm_irq_routing`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nr` | `-` |
| `__u32` | `flags` | `-` |

### `struct kvm_irqfd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fd` | `-` |
| `__u32` | `gsi` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `resamplefd` | `-` |
| `__u8` | `pad` | `16` |

### `struct kvm_clock_data`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `clock` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pad0` | `-` |
| `__u64` | `realtime` | `-` |
| `__u64` | `host_tsc` | `-` |
| `__u32` | `pad` | `4` |

### `struct kvm_config_tlb`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `params` | `-` |
| `__u64` | `array` | `-` |
| `__u32` | `mmu_type` | `-` |
| `__u32` | `array_len` | `-` |

### `struct kvm_dirty_tlb`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bitmap` | `-` |
| `__u32` | `num_dirty` | `-` |

### `struct kvm_reg_list`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `n` | `-` |

### `struct kvm_one_reg`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |
| `__u64` | `addr` | `-` |

### `struct kvm_msi`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `address_lo` | `-` |
| `__u32` | `address_hi` | `-` |
| `__u32` | `data` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `devid` | `-` |
| `__u8` | `pad` | `12` |

### `struct kvm_arm_device_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `id` | `-` |
| `__u64` | `addr` | `-` |

### `struct kvm_create_device`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `fd` | `-` |
| `__u32` | `flags` | `-` |

### `struct kvm_device_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `group` | `-` |
| `__u64` | `attr` | `-` |
| `__u64` | `addr` | `-` |

### `struct kvm_vfio_spapr_tce`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `groupfd` | `-` |
| `__s32` | `tablefd` | `-` |

### `struct kvm_s390_keyop`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `guest_addr` | `-` |
| `__u8` | `key` | `-` |
| `__u8` | `operation` | `-` |
| `__u8` | `pad` | `6` |

### `struct kvm_enc_region`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `addr` | `-` |
| `__u64` | `size` | `-` |

### `struct kvm_dirty_gfn`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `slot` | `-` |
| `__u64` | `offset` | `-` |

### `struct kvm_stats_header`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `name_size` | `-` |
| `__u32` | `num_desc` | `-` |
| `__u32` | `id_offset` | `-` |
| `__u32` | `desc_offset` | `-` |
| `__u32` | `data_offset` | `-` |

### `struct kvm_stats_desc`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__s16` | `exponent` | `-` |
| `__u16` | `size` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `bucket_size` | `-` |
| `char` | `name` | `KVM_STATS_NAME_SIZE` |

### `struct kvm_memory_attributes`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `address` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `attributes` | `-` |
| `__u64` | `flags` | `-` |

### `struct kvm_create_guest_memfd`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `size` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `reserved` | `6` |

### `struct kvm_pre_fault_memory`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `gpa` | `-` |
| `__u64` | `size` | `-` |
| `__u64` | `flags` | `-` |
| `__u64` | `padding` | `5` |