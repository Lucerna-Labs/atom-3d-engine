# osd.h

**Source:** `osd.h`


## Includes

- `linux/compiler.h`

## Defines (3 total)


### OSD_CAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `OSD_CAP_MEMSIZE` | `1` | memory size |

### OSD_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `OSD_GET_CAPABILITY` | `_IOR('o', 161, osd_cap_t)` |  |

### OSD_SEND (1)

| Name | Value | Comment |
|------|-------|---------|
| `OSD_SEND_CMD` | `_IOW('o', 160, osd_cmd_t)` |  |

## Structs (2)


### `struct osd_cmd_s`

| Type | Field | Array |
|------|-------|-------|
| `OSD_Command` | `cmd` | `-` |
| `int` | `x0` | `-` |
| `int` | `y0` | `-` |
| `int` | `x1` | `-` |
| `int` | `y1` | `-` |
| `int` | `color` | `-` |

### `struct osd_cap_s`

| Type | Field | Array |
|------|-------|-------|
| `int` | `cmd` | `-` |
| `long` | `val` | `-` |

## Typedefs

- `osd_cmd_s`
- `osd_cap_s`

## Enums


### `anonymous_enum_0`

- `All` 
- `functions` 
- `return` 
- `2` 
- `on` 
- `not` 
- `open` 
- `OSD_Close` = 1
- `Disables` 
- `OSD` 
- `and` 
- `releases` 
- `the` 
- `buffers` 
- `returns` 
- `0` 
- `on` 
- `success` 
- `OSD_Open` 
- `x0` 

*...and 385 more*

### `anonymous_enum_1`

- `OSD_BITMAP1` 
- `1` 
- `bit` 
- `bitmap` 
- `OSD_BITMAP2` 
- `2` 
- `bit` 
- `bitmap` 
- `OSD_BITMAP4` 
- `4` 
- `bit` 
- `bitmap` 
- `OSD_BITMAP8` 
- `8` 
- `bit` 
- `bitmap` 
- `OSD_BITMAP1HR` 
- `1` 
- `Bit` 
- `bitmap` 

*...and 99 more*