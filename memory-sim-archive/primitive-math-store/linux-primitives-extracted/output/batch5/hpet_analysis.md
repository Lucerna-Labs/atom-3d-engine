# hpet.h

**Source:** `hpet.h`


## Includes

- `linux/compiler.h`

## Defines (8 total)


### HPET_IE (2)

| Name | Value | Comment |
|------|-------|---------|
| `HPET_IE_ON` | `_IO('h', 0x01)` | interrupt on |
| `HPET_IE_OFF` | `_IO('h', 0x02)` | interrupt off |

### HPET_INFO (1)

| Name | Value | Comment |
|------|-------|---------|
| `HPET_INFO_PERIODIC` | `0x0010` | periodic-capable comparator |

### MAX_HPET (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_HPET_TBS` | `8` | maximum hpet timer blocks |

### UNCATEGORIZED (4)

| Name | Value | Comment |
|------|-------|---------|
| `HPET_INFO` | `_IOR('h', 0x03, struct hpet_info)` |  |
| `HPET_EPI` | `_IO('h', 0x04)` | enable periodic |
| `HPET_DPI` | `_IO('h', 0x05)` | disable periodic |
| `HPET_IRQFREQ` | `_IOW('h', 0x6, unsigned long)` | IRQFREQ usec |

## Structs (1)


### `struct hpet_info`

| Type | Field | Array |
|------|-------|-------|