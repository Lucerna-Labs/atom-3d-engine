# kd.h

**Source:** `kd.h`


## Includes

- `linux/types.h`
- `linux/compiler.h`

## Defines (80 total)


### KD_FONT (7)

| Name | Value | Comment |
|------|-------|---------|
| `KD_FONT_OP_SET` | `0` | Set font |
| `KD_FONT_OP_GET` | `1` | Get font |
| `KD_FONT_OP_SET_DEFAULT` | `2` | Set font to default, data points to name / NULL |
| `KD_FONT_OP_COPY` | `3` | Obsolete, do not use |
| `KD_FONT_OP_SET_TALL` | `4` | Set font with vpitch = height |
| `KD_FONT_OP_GET_TALL` | `5` | Get font with vpitch = height |
| `KD_FONT_FLAG_DONT_RECALC` | `1` | Don't recalculate hw charcell size [compat] |

### UNCATEGORIZED (71)

| Name | Value | Comment |
|------|-------|---------|
| `GIO_FONT` | `0x4B60` | gets font in expanded form |
| `PIO_FONT` | `0x4B61` | use font in expanded form |
| `GIO_FONTX` | `0x4B6B` | get font using struct consolefontdesc |
| `PIO_FONTX` | `0x4B6C` | set font using struct consolefontdesc |
| `PIO_FONTRESET` | `0x4B6D` | reset to default font |
| `GIO_CMAP` | `0x4B70` | gets colour palette on VGA+ |
| `PIO_CMAP` | `0x4B71` | sets colour palette on VGA+ |
| `KIOCSOUND` | `0x4B2F` | start sound generation (0 for off) |
| `KDMKTONE` | `0x4B30` | generate tone |
| `KDGETLED` | `0x4B31` | return current led state |
| `KDSETLED` | `0x4B32` | set led state [lights, not flags] |
| `LED_SCR` | `0x01` | scroll lock led |
| `LED_NUM` | `0x02` | num lock led |
| `LED_CAP` | `0x04` | caps lock led |
| `KDGKBTYPE` | `0x4B33` | get keyboard type |
| `KB_84` | `0x01` |  |
| `KB_101` | `0x02` | this is what we always answer |
| `KB_OTHER` | `0x03` |  |
| `KDADDIO` | `0x4B34` | add i/o port as valid |
| `KDDELIO` | `0x4B35` | del i/o port as valid |
| `KDENABIO` | `0x4B36` | enable i/o to video board |
| `KDDISABIO` | `0x4B37` | disable i/o to video board |
| `KDSETMODE` | `0x4B3A` | set text/graphics mode |
| `KD_TEXT` | `0x00` |  |
| `KD_GRAPHICS` | `0x01` |  |
| `KD_TEXT0` | `0x02` | obsolete |
| `KD_TEXT1` | `0x03` | obsolete |
| `KDGETMODE` | `0x4B3B` | get current mode |
| `KDMAPDISP` | `0x4B3C` | map display into address space |
| `KDUNMAPDISP` | `0x4B3D` | unmap display from address space |
| `E_TABSZ` | `256` |  |
| `GIO_SCRNMAP` | `0x4B40` | get screen mapping from kernel |
| `PIO_SCRNMAP` | `0x4B41` | put screen mapping table in kernel |
| `GIO_UNISCRNMAP` | `0x4B69` | get full Unicode screen mapping |
| `PIO_UNISCRNMAP` | `0x4B6A` | set full Unicode screen mapping |
| `GIO_UNIMAP` | `0x4B66` | get unicode-to-font mapping from kernel |
| `PIO_UNIMAP` | `0x4B67` | put unicode-to-font mapping in kernel |
| `PIO_UNIMAPCLR` | `0x4B68` | clear table, possibly advise hash algorithm |
| `K_RAW` | `0x00` |  |
| `K_XLATE` | `0x01` |  |
| `K_MEDIUMRAW` | `0x02` |  |
| `K_UNICODE` | `0x03` |  |
| `K_OFF` | `0x04` |  |
| `KDGKBMODE` | `0x4B44` | gets current keyboard mode |
| `KDSKBMODE` | `0x4B45` | sets current keyboard mode |
| `K_METABIT` | `0x03` |  |
| `K_ESCPREFIX` | `0x04` |  |
| `KDGKBMETA` | `0x4B62` | gets meta key handling mode |
| `KDSKBMETA` | `0x4B63` | sets meta key handling mode |
| `K_SCROLLLOCK` | `0x01` |  |

*...and 21 more*

### UNI_DIRECT (2)

| Name | Value | Comment |
|------|-------|---------|
| `UNI_DIRECT_BASE` | `0xF000` | start of Direct Font Region |
| `UNI_DIRECT_MASK` | `0x01FF` | Direct Font Region bitmask |

## Structs (14)


### `struct consolefontdesc`

| Type | Field | Array |
|------|-------|-------|

### `struct unipair`

| Type | Field | Array |
|------|-------|-------|

### `struct unimapdesc`

| Type | Field | Array |
|------|-------|-------|

### `struct unimapinit`

| Type | Field | Array |
|------|-------|-------|

### `struct kbentry`

| Type | Field | Array |
|------|-------|-------|

### `struct kbsentry`

| Type | Field | Array |
|------|-------|-------|

### `struct kbdiacr`

| Type | Field | Array |
|------|-------|-------|

### `struct kbdiacrs`

| Type | Field | Array |
|------|-------|-------|

### `struct kbdiacruc`

| Type | Field | Array |
|------|-------|-------|

### `struct kbdiacrsuc`

| Type | Field | Array |
|------|-------|-------|

### `struct kbkeycode`

| Type | Field | Array |
|------|-------|-------|

### `struct kbd_repeat`

| Type | Field | Array |
|------|-------|-------|
| `int` | `delay` | `-` |
| `int` | `period` | `-` |

### `struct console_font_op`

| Type | Field | Array |
|------|-------|-------|

### `struct console_font`

| Type | Field | Array |
|------|-------|-------|