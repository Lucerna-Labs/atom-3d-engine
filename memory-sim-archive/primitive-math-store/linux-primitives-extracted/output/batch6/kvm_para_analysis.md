# kvm_para.h

**Source:** `kvm_para.h`


## Includes

- `asm/kvm_para.h`

## Defines (18 total)


### KVM_HC (12)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_HC_VAPIC_POLL_IRQ` | `1` |  |
| `KVM_HC_MMU_OP` | `2` |  |
| `KVM_HC_FEATURES` | `3` |  |
| `KVM_HC_PPC_MAP_MAGIC_PAGE` | `4` |  |
| `KVM_HC_KICK_CPU` | `5` |  |
| `KVM_HC_MIPS_GET_CLOCK_FREQ` | `6` |  |
| `KVM_HC_MIPS_EXIT_VM` | `7` |  |
| `KVM_HC_MIPS_CONSOLE_OUTPUT` | `8` |  |
| `KVM_HC_CLOCK_PAIRING` | `9` |  |
| `KVM_HC_SEND_IPI` | `10` |  |
| `KVM_HC_SCHED_YIELD` | `11` |  |
| `KVM_HC_MAP_GPA_RANGE` | `12` |  |

### UNCATEGORIZED (6)

| Name | Value | Comment |
|------|-------|---------|
| `KVM_ENOSYS` | `1000` |  |
| `KVM_EFAULT` | `EFAULT` |  |
| `KVM_EINVAL` | `EINVAL` |  |
| `KVM_E2BIG` | `E2BIG` |  |
| `KVM_EPERM` | `EPERM` |  |
| `KVM_EOPNOTSUPP` | `95` |  |