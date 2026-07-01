# fb.h

**Source:** `fb.h`


## Includes

- `linux/types.h`
- `linux/i2c.h`
- `linux/vesa.h`

## Defines (180 total)


### FB_ACCEL (81)

| Name | Value | Comment |
|------|-------|---------|
| `FB_ACCEL_NONE` | `0` | no hardware accelerator |
| `FB_ACCEL_ATARIBLITT` | `1` | Atari Blitter |
| `FB_ACCEL_AMIGABLITT` | `2` | Amiga Blitter |
| `FB_ACCEL_S3_TRIO64` | `3` | Cybervision64 (S3 Trio64) |
| `FB_ACCEL_NCR_77C32BLT` | `4` | RetinaZ3 (NCR 77C32BLT) |
| `FB_ACCEL_S3_VIRGE` | `5` | Cybervision64/3D (S3 ViRGE) |
| `FB_ACCEL_ATI_MACH64GX` | `6` | ATI Mach 64GX family |
| `FB_ACCEL_DEC_TGA` | `7` | DEC 21030 TGA |
| `FB_ACCEL_ATI_MACH64CT` | `8` | ATI Mach 64CT family |
| `FB_ACCEL_ATI_MACH64VT` | `9` | ATI Mach 64CT family VT class |
| `FB_ACCEL_ATI_MACH64GT` | `10` | ATI Mach 64CT family GT class |
| `FB_ACCEL_SUN_CREATOR` | `11` | Sun Creator/Creator3D |
| `FB_ACCEL_SUN_CGSIX` | `12` | Sun cg6 |
| `FB_ACCEL_SUN_LEO` | `13` | Sun leo/zx |
| `FB_ACCEL_IMS_TWINTURBO` | `14` | IMS Twin Turbo |
| `FB_ACCEL_3DLABS_PERMEDIA2` | `15` | 3Dlabs Permedia 2 |
| `FB_ACCEL_MATROX_MGA2064W` | `16` | Matrox MGA2064W (Millenium) |
| `FB_ACCEL_MATROX_MGA1064SG` | `17` | Matrox MGA1064SG (Mystique) |
| `FB_ACCEL_MATROX_MGA2164W` | `18` | Matrox MGA2164W (Millenium II) |
| `FB_ACCEL_MATROX_MGA2164W_AGP` | `19` | Matrox MGA2164W (Millenium II) |
| `FB_ACCEL_MATROX_MGAG100` | `20` | Matrox G100 (Productiva G100) |
| `FB_ACCEL_MATROX_MGAG200` | `21` | Matrox G200 (Myst, Mill, ...) |
| `FB_ACCEL_SUN_CG14` | `22` | Sun cgfourteen |
| `FB_ACCEL_SUN_BWTWO` | `23` | Sun bwtwo |
| `FB_ACCEL_SUN_CGTHREE` | `24` | Sun cgthree |
| `FB_ACCEL_SUN_TCX` | `25` | Sun tcx |
| `FB_ACCEL_MATROX_MGAG400` | `26` | Matrox G400 |
| `FB_ACCEL_NV3` | `27` | nVidia RIVA 128 |
| `FB_ACCEL_NV4` | `28` | nVidia RIVA TNT |
| `FB_ACCEL_NV5` | `29` | nVidia RIVA TNT2 |
| `FB_ACCEL_CT_6555x` | `30` | C&T 6555x |
| `FB_ACCEL_3DFX_BANSHEE` | `31` | 3Dfx Banshee |
| `FB_ACCEL_ATI_RAGE128` | `32` | ATI Rage128 family |
| `FB_ACCEL_IGS_CYBER2000` | `33` | CyberPro 2000 |
| `FB_ACCEL_IGS_CYBER2010` | `34` | CyberPro 2010 |
| `FB_ACCEL_IGS_CYBER5000` | `35` | CyberPro 5000 |
| `FB_ACCEL_SIS_GLAMOUR` | `36` | SiS 300/630/540 |
| `FB_ACCEL_3DLABS_PERMEDIA3` | `37` | 3Dlabs Permedia 3 |
| `FB_ACCEL_ATI_RADEON` | `38` | ATI Radeon family |
| `FB_ACCEL_I810` | `39` | Intel 810/815 |
| `FB_ACCEL_SIS_GLAMOUR_2` | `40` | SiS 315, 650, 740 |
| `FB_ACCEL_SIS_XABRE` | `41` | SiS 330 ("Xabre") |
| `FB_ACCEL_I830` | `42` | Intel 830M/845G/85x/865G |
| `FB_ACCEL_NV_10` | `43` | nVidia Arch 10 |
| `FB_ACCEL_NV_20` | `44` | nVidia Arch 20 |
| `FB_ACCEL_NV_30` | `45` | nVidia Arch 30 |
| `FB_ACCEL_NV_40` | `46` | nVidia Arch 40 |
| `FB_ACCEL_XGI_VOLARI_V` | `47` | XGI Volari V3XT, V5, V8 |
| `FB_ACCEL_XGI_VOLARI_Z` | `48` | XGI Volari Z7 |
| `FB_ACCEL_OMAP1610` | `49` | TI OMAP16xx |

*...and 31 more*

### FB_ACCELF (1)

| Name | Value | Comment |
|------|-------|---------|
| `FB_ACCELF_TEXT` | `1` | (OBSOLETE) see fb_info.flags and vc_mode |

### FB_ACTIVATE (9)

| Name | Value | Comment |
|------|-------|---------|
| `FB_ACTIVATE_NOW` | `0` | set values immediately (or vbl) |
| `FB_ACTIVATE_NXTOPEN` | `1` | activate on next open |
| `FB_ACTIVATE_TEST` | `2` | don't set, round up impossible |
| `FB_ACTIVATE_MASK` | `15` |  |
| `FB_ACTIVATE_VBL` | `16` | activate values on next vbl |
| `FB_ACTIVATE_ALL` | `64` | change all VCs on this fb |
| `FB_ACTIVATE_FORCE` | `128` | force apply even when no change |
| `FB_ACTIVATE_INV_MODE` | `256` | invalidate videomode |
| `FB_ACTIVATE_KD_TEXT` | `512` | for KDSET vt ioctl |

### FB_AUX (15)

| Name | Value | Comment |
|------|-------|---------|
| `FB_AUX_TEXT_MDA` | `0` | Monochrome text |
| `FB_AUX_TEXT_CGA` | `1` | CGA/EGA/VGA Color text |
| `FB_AUX_TEXT_S3_MMIO` | `2` | S3 MMIO fasttext |
| `FB_AUX_TEXT_MGA_STEP16` | `3` | MGA Millenium I: text, attr, 14 reserved bytes |
| `FB_AUX_TEXT_MGA_STEP8` | `4` | other MGAs:      text, attr,  6 reserved bytes |
| `FB_AUX_TEXT_SVGA_GROUP` | `8` | 8-15: SVGA tileblit compatible modes |
| `FB_AUX_TEXT_SVGA_MASK` | `7` | lower three bits says step |
| `FB_AUX_TEXT_SVGA_STEP2` | `8` | SVGA text mode:  text, attr |
| `FB_AUX_TEXT_SVGA_STEP4` | `9` | SVGA text mode:  text, attr,  2 reserved bytes |
| `FB_AUX_TEXT_SVGA_STEP8` | `10` | SVGA text mode:  text, attr,  6 reserved bytes |
| `FB_AUX_TEXT_SVGA_STEP16` | `11` | SVGA text mode:  text, attr, 14 reserved bytes |
| `FB_AUX_TEXT_SVGA_LAST` | `15` | reserved up to 15 |
| `FB_AUX_VGA_PLANES_VGA4` | `0` | 16 color planes (EGA/VGA) |
| `FB_AUX_VGA_PLANES_CFB4` | `1` | CFB4 in planes (VGA) |
| `FB_AUX_VGA_PLANES_CFB8` | `2` | CFB8 in planes (VGA) |

### FB_BACKLIGHT (2)

| Name | Value | Comment |
|------|-------|---------|
| `FB_BACKLIGHT_LEVELS` | `128` |  |
| `FB_BACKLIGHT_MAX` | `0xFF` |  |

### FB_CAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `FB_CAP_FOURCC` | `1` | Device supports FOURCC-based formats |

### FB_CHANGE (1)

| Name | Value | Comment |
|------|-------|---------|
| `FB_CHANGE_CMAP_VBL` | `32` | change colormap on vbl |

### FB_CUR (7)

| Name | Value | Comment |
|------|-------|---------|
| `FB_CUR_SETIMAGE` | `0x01` |  |
| `FB_CUR_SETPOS` | `0x02` |  |
| `FB_CUR_SETHOT` | `0x04` |  |
| `FB_CUR_SETCMAP` | `0x08` |  |
| `FB_CUR_SETSHAPE` | `0x10` |  |
| `FB_CUR_SETSIZE` | `0x20` |  |
| `FB_CUR_SETALL` | `0xFF` |  |

### FB_NONSTD (2)

| Name | Value | Comment |
|------|-------|---------|
| `FB_NONSTD_HAM` | `1` | Hold-And-Modify (HAM) |
| `FB_NONSTD_REV_PIX_IN_B` | `2` | order of pixels in each byte is reversed |

### FB_ROTATE (4)

| Name | Value | Comment |
|------|-------|---------|
| `FB_ROTATE_UR` | `0` |  |
| `FB_ROTATE_CW` | `1` |  |
| `FB_ROTATE_UD` | `2` |  |
| `FB_ROTATE_CCW` | `3` |  |

### FB_SYNC (6)

| Name | Value | Comment |
|------|-------|---------|
| `FB_SYNC_HOR_HIGH_ACT` | `1` | horizontal sync high active |
| `FB_SYNC_VERT_HIGH_ACT` | `2` | vertical sync high active |
| `FB_SYNC_EXT` | `4` | external sync |
| `FB_SYNC_COMP_HIGH_ACT` | `8` | composite sync high active |
| `FB_SYNC_BROADCAST` | `16` | broadcast video timings |
| `FB_SYNC_ON_GREEN` | `32` | sync on green |

### FB_TYPE (6)

| Name | Value | Comment |
|------|-------|---------|
| `FB_TYPE_PACKED_PIXELS` | `0` | Packed Pixels |
| `FB_TYPE_PLANES` | `1` | Non interleaved planes |
| `FB_TYPE_INTERLEAVED_PLANES` | `2` | Interleaved planes |
| `FB_TYPE_TEXT` | `3` | Text/attributes |
| `FB_TYPE_VGA_PLANES` | `4` | EGA/VGA planes |
| `FB_TYPE_FOURCC` | `5` | Type identified by a V4L2 FOURCC |

### FB_VBLANK (9)

| Name | Value | Comment |
|------|-------|---------|
| `FB_VBLANK_VBLANKING` | `0x001` | currently in a vertical blank |
| `FB_VBLANK_HBLANKING` | `0x002` | currently in a horizontal blank |
| `FB_VBLANK_HAVE_VBLANK` | `0x004` | vertical blanks can be detected |
| `FB_VBLANK_HAVE_HBLANK` | `0x008` | horizontal blanks can be detected |
| `FB_VBLANK_HAVE_COUNT` | `0x010` | global retrace counter is available |
| `FB_VBLANK_HAVE_VCOUNT` | `0x020` | the vcount field is valid |
| `FB_VBLANK_HAVE_HCOUNT` | `0x040` | the hcount field is valid |
| `FB_VBLANK_VSYNCING` | `0x080` | currently in a vsync |
| `FB_VBLANK_HAVE_VSYNC` | `0x100` | vertical syncs can be detected |

### FB_VISUAL (7)

| Name | Value | Comment |
|------|-------|---------|
| `FB_VISUAL_MONO01` | `0` | Monochr. 1=Black 0=White |
| `FB_VISUAL_MONO10` | `1` | Monochr. 1=White 0=Black |
| `FB_VISUAL_TRUECOLOR` | `2` | True color |
| `FB_VISUAL_PSEUDOCOLOR` | `3` | Pseudo color (like atari) |
| `FB_VISUAL_DIRECTCOLOR` | `4` | Direct color |
| `FB_VISUAL_STATIC_PSEUDOCOLOR` | `5` | Pseudo color readonly |
| `FB_VISUAL_FOURCC` | `6` | Visual identified by a V4L2 FOURCC |

### FB_VMODE (8)

| Name | Value | Comment |
|------|-------|---------|
| `FB_VMODE_NONINTERLACED` | `0` | non interlaced |
| `FB_VMODE_INTERLACED` | `1` | interlaced |
| `FB_VMODE_DOUBLE` | `2` | double scan |
| `FB_VMODE_ODD_FLD_FIRST` | `4` | interlaced: top line first |
| `FB_VMODE_MASK` | `255` |  |
| `FB_VMODE_YWRAP` | `256` | ywrap instead of panning |
| `FB_VMODE_SMOOTH_XPAN` | `512` | smooth xpan possible (internally used) |
| `FB_VMODE_CONUPDATE` | `512` | don't update x/yoffset |

### UNCATEGORIZED (21)

| Name | Value | Comment |
|------|-------|---------|
| `FB_MAX` | `32` | sufficient for now |
| `FBIOGET_VSCREENINFO` | `0x4600` |  |
| `FBIOPUT_VSCREENINFO` | `0x4601` |  |
| `FBIOGET_FSCREENINFO` | `0x4602` |  |
| `FBIOGETCMAP` | `0x4604` |  |
| `FBIOPUTCMAP` | `0x4605` |  |
| `FBIOPAN_DISPLAY` | `0x4606` |  |
| `FBIO_CURSOR` | `_IOWR('F', 0x08, struct fb_cursor)` |  |
| `FBIOGET_CON2FBMAP` | `0x460F` |  |
| `FBIOPUT_CON2FBMAP` | `0x4610` |  |
| `FBIOBLANK` | `0x4611` | arg: 0 or vesa level + 1 |
| `FBIOGET_VBLANK` | `_IOR('F', 0x12, struct fb_vblank)` |  |
| `FBIO_ALLOC` | `0x4613` |  |
| `FBIO_FREE` | `0x4614` |  |
| `FBIOGET_GLYPH` | `0x4615` |  |
| `FBIOGET_HWCINFO` | `0x4616` |  |
| `FBIOPUT_MODEINFO` | `0x4617` |  |
| `FBIOGET_DISPINFO` | `0x4618` |  |
| `FBIO_WAITFORVSYNC` | `_IOW('F', 0x20, __u32)` |  |
| `ROP_COPY` | `0` |  |
| `ROP_XOR` | `1` |  |

## Structs (11)


### `struct fb_fix_screeninfo`

| Type | Field | Array |
|------|-------|-------|
| `char` | `id` | `16` |
| `__u32` | `smem_len` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `type_aux` | `-` |
| `__u32` | `visual` | `-` |
| `__u16` | `xpanstep` | `-` |
| `__u16` | `ypanstep` | `-` |
| `__u16` | `ywrapstep` | `-` |
| `__u32` | `line_length` | `-` |
| `__u32` | `mmio_len` | `-` |
| `__u32` | `accel` | `-` |
| `__u16` | `capabilities` | `-` |
| `__u16` | `reserved` | `2` |

### `struct fb_bitfield`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `offset` | `-` |
| `__u32` | `length` | `-` |
| `__u32` | `msb_right` | `-` |

### `struct fb_var_screeninfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `xres` | `-` |
| `__u32` | `yres` | `-` |
| `__u32` | `xres_virtual` | `-` |
| `__u32` | `yres_virtual` | `-` |
| `__u32` | `xoffset` | `-` |
| `__u32` | `yoffset` | `-` |
| `__u32` | `bits_per_pixel` | `-` |
| `__u32` | `grayscale` | `-` |
| `__u32` | `nonstd` | `-` |
| `__u32` | `activate` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `accel_flags` | `-` |
| `__u32` | `pixclock` | `-` |
| `__u32` | `left_margin` | `-` |
| `__u32` | `right_margin` | `-` |
| `__u32` | `upper_margin` | `-` |
| `__u32` | `lower_margin` | `-` |
| `__u32` | `hsync_len` | `-` |
| `__u32` | `vsync_len` | `-` |
| `__u32` | `sync` | `-` |
| `__u32` | `vmode` | `-` |
| `__u32` | `rotate` | `-` |
| `__u32` | `colorspace` | `-` |
| `__u32` | `reserved` | `4` |

### `struct fb_cmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `start` | `-` |
| `__u32` | `len` | `-` |

### `struct fb_con2fbmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `console` | `-` |
| `__u32` | `framebuffer` | `-` |

### `struct fb_vblank`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `vcount` | `-` |
| `__u32` | `hcount` | `-` |
| `__u32` | `reserved` | `4` |

### `struct fb_copyarea`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dx` | `-` |
| `__u32` | `dy` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `sx` | `-` |
| `__u32` | `sy` | `-` |

### `struct fb_fillrect`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dx` | `-` |
| `__u32` | `dy` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `color` | `-` |
| `__u32` | `rop` | `-` |

### `struct fb_image`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `dx` | `-` |
| `__u32` | `dy` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `fg_color` | `-` |
| `__u32` | `bg_color` | `-` |
| `__u8` | `depth` | `-` |

### `struct fbcurpos`

| Type | Field | Array |
|------|-------|-------|

### `struct fb_cursor`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `set` | `-` |
| `__u16` | `enable` | `-` |
| `__u16` | `rop` | `-` |