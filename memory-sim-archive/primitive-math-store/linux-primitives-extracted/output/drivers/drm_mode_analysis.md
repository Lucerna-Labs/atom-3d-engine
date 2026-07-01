# drm_mode.h

**Source:** `drm_mode.h`


## Includes

- `linux/bits.h`
- `linux/const.h`
- `drm.h`

## Defines (154 total)


### DRM_CONNECTOR (1)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_CONNECTOR_NAME_LEN` | `32` |  |

### DRM_DISPLAY (1)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_DISPLAY_MODE_LEN` | `32` |  |

### DRM_MODE (150)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_MODE_TYPE_BUILTIN` | `(1<<0)` | deprecated |
| `DRM_MODE_TYPE_CLOCK_C` | `((1<<1) \| DRM_MODE_TYPE_BUILTIN)` | deprecated |
| `DRM_MODE_TYPE_CRTC_C` | `((1<<2) \| DRM_MODE_TYPE_BUILTIN)` | deprecated |
| `DRM_MODE_TYPE_PREFERRED` | `(1<<3)` |  |
| `DRM_MODE_TYPE_DEFAULT` | `(1<<4)` | deprecated |
| `DRM_MODE_TYPE_USERDEF` | `(1<<5)` |  |
| `DRM_MODE_TYPE_DRIVER` | `(1<<6)` |  |
| `DRM_MODE_TYPE_ALL` | `(DRM_MODE_TYPE_PREFERRED \|	` |  |
| `DRM_MODE_FLAG_PHSYNC` | `(1<<0)` |  |
| `DRM_MODE_FLAG_NHSYNC` | `(1<<1)` |  |
| `DRM_MODE_FLAG_PVSYNC` | `(1<<2)` |  |
| `DRM_MODE_FLAG_NVSYNC` | `(1<<3)` |  |
| `DRM_MODE_FLAG_INTERLACE` | `(1<<4)` |  |
| `DRM_MODE_FLAG_DBLSCAN` | `(1<<5)` |  |
| `DRM_MODE_FLAG_CSYNC` | `(1<<6)` |  |
| `DRM_MODE_FLAG_PCSYNC` | `(1<<7)` |  |
| `DRM_MODE_FLAG_NCSYNC` | `(1<<8)` |  |
| `DRM_MODE_FLAG_HSKEW` | `(1<<9)` | hskew provided |
| `DRM_MODE_FLAG_BCAST` | `(1<<10)` | deprecated |
| `DRM_MODE_FLAG_PIXMUX` | `(1<<11)` | deprecated |
| `DRM_MODE_FLAG_DBLCLK` | `(1<<12)` |  |
| `DRM_MODE_FLAG_CLKDIV2` | `(1<<13)` |  |
| `DRM_MODE_FLAG_3D_MASK` | `(0x1f<<14)` |  |
| `DRM_MODE_FLAG_3D_NONE` | `(0<<14)` |  |
| `DRM_MODE_FLAG_3D_FRAME_PACKING` | `(1<<14)` |  |
| `DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE` | `(2<<14)` |  |
| `DRM_MODE_FLAG_3D_LINE_ALTERNATIVE` | `(3<<14)` |  |
| `DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL` | `(4<<14)` |  |
| `DRM_MODE_FLAG_3D_L_DEPTH` | `(5<<14)` |  |
| `DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH` | `(6<<14)` |  |
| `DRM_MODE_FLAG_3D_TOP_AND_BOTTOM` | `(7<<14)` |  |
| `DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF` | `(8<<14)` |  |
| `DRM_MODE_PICTURE_ASPECT_NONE` | `0` |  |
| `DRM_MODE_PICTURE_ASPECT_4_3` | `1` |  |
| `DRM_MODE_PICTURE_ASPECT_16_9` | `2` |  |
| `DRM_MODE_PICTURE_ASPECT_64_27` | `3` |  |
| `DRM_MODE_PICTURE_ASPECT_256_135` | `4` |  |
| `DRM_MODE_CONTENT_TYPE_NO_DATA` | `0` |  |
| `DRM_MODE_CONTENT_TYPE_GRAPHICS` | `1` |  |
| `DRM_MODE_CONTENT_TYPE_PHOTO` | `2` |  |
| `DRM_MODE_CONTENT_TYPE_CINEMA` | `3` |  |
| `DRM_MODE_CONTENT_TYPE_GAME` | `4` |  |
| `DRM_MODE_FLAG_PIC_AR_MASK` | `(0x0F<<19)` |  |
| `DRM_MODE_FLAG_PIC_AR_NONE` | `` |  |
| `DRM_MODE_FLAG_PIC_AR_4_3` | `` |  |
| `DRM_MODE_FLAG_PIC_AR_16_9` | `` |  |
| `DRM_MODE_FLAG_PIC_AR_64_27` | `` |  |
| `DRM_MODE_FLAG_PIC_AR_256_135` | `` |  |
| `DRM_MODE_FLAG_ALL` | `(DRM_MODE_FLAG_PHSYNC \|		` |  |
| `DRM_MODE_DPMS_ON` | `0` |  |

*...and 100 more*

### DRM_PROP (1)

| Name | Value | Comment |
|------|-------|---------|
| `DRM_PROP_NAME_LEN` | `32` |  |

### FORMAT_BLOB (1)

| Name | Value | Comment |
|------|-------|---------|
| `FORMAT_BLOB_CURRENT` | `1` |  |

## Structs (46)


### `struct drm_mode_modeinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `clock` | `-` |
| `__u16` | `hdisplay` | `-` |
| `__u16` | `hsync_start` | `-` |
| `__u16` | `hsync_end` | `-` |
| `__u16` | `htotal` | `-` |
| `__u16` | `hskew` | `-` |
| `__u16` | `vdisplay` | `-` |
| `__u16` | `vsync_start` | `-` |
| `__u16` | `vsync_end` | `-` |
| `__u16` | `vtotal` | `-` |
| `__u16` | `vscan` | `-` |
| `__u32` | `vrefresh` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `type` | `-` |
| `char` | `name` | `DRM_DISPLAY_MODE_LEN` |

### `struct drm_mode_card_res`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `fb_id_ptr` | `-` |
| `__u64` | `crtc_id_ptr` | `-` |
| `__u64` | `connector_id_ptr` | `-` |
| `__u64` | `encoder_id_ptr` | `-` |
| `__u32` | `count_fbs` | `-` |
| `__u32` | `count_crtcs` | `-` |
| `__u32` | `count_connectors` | `-` |
| `__u32` | `count_encoders` | `-` |
| `__u32` | `min_width` | `-` |
| `__u32` | `max_width` | `-` |
| `__u32` | `min_height` | `-` |
| `__u32` | `max_height` | `-` |

### `struct drm_mode_crtc`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `set_connectors_ptr` | `-` |
| `__u32` | `count_connectors` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__u32` | `fb_id` | `-` |
| `__u32` | `x` | `-` |
| `__u32` | `y` | `-` |
| `__u32` | `gamma_size` | `-` |
| `__u32` | `mode_valid` | `-` |

### `struct drm_mode_set_plane`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `plane_id` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__u32` | `fb_id` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `crtc_x` | `-` |
| `__s32` | `crtc_y` | `-` |
| `__u32` | `crtc_w` | `-` |
| `__u32` | `crtc_h` | `-` |
| `__u32` | `src_x` | `-` |
| `__u32` | `src_y` | `-` |
| `__u32` | `src_h` | `-` |
| `__u32` | `src_w` | `-` |

### `struct drm_mode_get_plane`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `plane_id` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__u32` | `fb_id` | `-` |
| `__u32` | `possible_crtcs` | `-` |
| `__u32` | `gamma_size` | `-` |
| `__u32` | `count_format_types` | `-` |
| `__u64` | `format_type_ptr` | `-` |

### `struct drm_mode_get_plane_res`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `plane_id_ptr` | `-` |
| `__u32` | `count_planes` | `-` |

### `struct drm_mode_get_encoder`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `encoder_id` | `-` |
| `__u32` | `encoder_type` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__u32` | `possible_crtcs` | `-` |
| `__u32` | `possible_clones` | `-` |

### `struct drm_mode_get_connector`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `encoders_ptr` | `-` |
| `__u64` | `modes_ptr` | `-` |
| `__u64` | `props_ptr` | `-` |
| `__u64` | `prop_values_ptr` | `-` |
| `__u32` | `count_modes` | `-` |
| `__u32` | `count_props` | `-` |
| `__u32` | `count_encoders` | `-` |
| `__u32` | `encoder_id` | `-` |
| `__u32` | `connector_id` | `-` |
| `__u32` | `connector_type` | `-` |
| `__u32` | `connector_type_id` | `-` |
| `__u32` | `connection` | `-` |
| `__u32` | `mm_width` | `-` |
| `__u32` | `mm_height` | `-` |
| `__u32` | `subpixel` | `-` |
| `__u32` | `pad` | `-` |

### `struct drm_mode_property_enum`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `value` | `-` |
| `char` | `name` | `DRM_PROP_NAME_LEN` |

### `struct drm_mode_get_property`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `values_ptr` | `-` |
| `__u64` | `enum_blob_ptr` | `-` |
| `__u32` | `prop_id` | `-` |
| `__u32` | `flags` | `-` |
| `char` | `name` | `DRM_PROP_NAME_LEN` |
| `__u32` | `count_values` | `-` |
| `__u32` | `count_enum_blobs` | `-` |

### `struct drm_mode_connector_set_property`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `value` | `-` |
| `__u32` | `prop_id` | `-` |
| `__u32` | `connector_id` | `-` |

### `struct drm_mode_obj_get_properties`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `props_ptr` | `-` |
| `__u64` | `prop_values_ptr` | `-` |
| `__u32` | `count_props` | `-` |
| `__u32` | `obj_id` | `-` |
| `__u32` | `obj_type` | `-` |

### `struct drm_mode_obj_set_property`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `value` | `-` |
| `__u32` | `prop_id` | `-` |
| `__u32` | `obj_id` | `-` |
| `__u32` | `obj_type` | `-` |

### `struct drm_mode_get_blob`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `blob_id` | `-` |
| `__u32` | `length` | `-` |
| `__u64` | `data` | `-` |

### `struct drm_mode_fb_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_id` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `pitch` | `-` |
| `__u32` | `bpp` | `-` |
| `__u32` | `depth` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_mode_fb_cmd2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_id` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `pixel_format` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handles` | `4` |
| `__u32` | `pitches` | `4` |
| `__u32` | `offsets` | `4` |
| `__u64` | `modifier` | `4` |

### `struct drm_mode_fb_dirty_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `color` | `-` |
| `__u32` | `num_clips` | `-` |
| `__u64` | `clips_ptr` | `-` |

### `struct drm_mode_mode_cmd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `connector_id` | `-` |

### `struct drm_mode_cursor`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__s32` | `x` | `-` |
| `__s32` | `y` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `handle` | `-` |

### `struct drm_mode_cursor2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `crtc_id` | `-` |
| `__s32` | `x` | `-` |
| `__s32` | `y` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `handle` | `-` |
| `__s32` | `hot_x` | `-` |
| `__s32` | `hot_y` | `-` |

### `struct drm_mode_crtc_lut`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc_id` | `-` |
| `__u32` | `gamma_size` | `-` |
| `__u64` | `red` | `-` |
| `__u64` | `green` | `-` |
| `__u64` | `blue` | `-` |

### `struct drm_color_ctm`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `matrix` | `9` |

### `struct drm_color_ctm_3x4`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `matrix` | `12` |

### `struct drm_color_lut`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `red` | `-` |
| `__u16` | `green` | `-` |
| `__u16` | `blue` | `-` |
| `__u16` | `reserved` | `-` |

### `struct drm_color_lut32`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `red` | `-` |
| `__u32` | `green` | `-` |
| `__u32` | `blue` | `-` |
| `__u32` | `reserved` | `-` |

### `struct drm_plane_size_hint`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `width` | `-` |
| `__u16` | `height` | `-` |

### `struct hdr_metadata_infoframe`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `eotf` | `-` |
| `__u8` | `metadata_type` | `-` |
| `__u16` | `max_display_mastering_luminance` | `-` |
| `__u16` | `min_display_mastering_luminance` | `-` |
| `__u16` | `max_cll` | `-` |
| `__u16` | `max_fall` | `-` |

### `struct anonymous_27`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_28`

| Type | Field | Array |
|------|-------|-------|

### `struct hdr_output_metadata`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `metadata_type` | `-` |

### `struct drm_mode_crtc_page_flip`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc_id` | `-` |
| `__u32` | `fb_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `-` |
| `__u64` | `user_data` | `-` |

### `struct drm_mode_crtc_page_flip_target`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `crtc_id` | `-` |
| `__u32` | `fb_id` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `sequence` | `-` |
| `__u64` | `user_data` | `-` |

### `struct drm_mode_create_dumb`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `height` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `bpp` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `pitch` | `-` |
| `__u64` | `size` | `-` |

### `struct drm_mode_map_dumb`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `offset` | `-` |

### `struct drm_mode_destroy_dumb`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `handle` | `-` |

### `struct drm_mode_atomic`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `count_objs` | `-` |
| `__u64` | `objs_ptr` | `-` |
| `__u64` | `count_props_ptr` | `-` |
| `__u64` | `props_ptr` | `-` |
| `__u64` | `prop_values_ptr` | `-` |
| `__u64` | `reserved` | `-` |
| `__u64` | `user_data` | `-` |

### `struct drm_format_modifier_blob`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `version` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `count_formats` | `-` |
| `__u32` | `formats_offset` | `-` |
| `__u32` | `count_modifiers` | `-` |
| `__u32` | `modifiers_offset` | `-` |

### `struct drm_format_modifier`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `formats` | `-` |
| `__u32` | `offset` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `modifier` | `-` |

### `struct drm_mode_create_blob`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `data` | `-` |
| `__u32` | `length` | `-` |
| `__u32` | `blob_id` | `-` |

### `struct drm_mode_destroy_blob`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `blob_id` | `-` |

### `struct drm_mode_create_lease`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `object_ids` | `-` |
| `__u32` | `object_count` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `lessee_id` | `-` |
| `__u32` | `fd` | `-` |

### `struct drm_mode_list_lessees`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count_lessees` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `lessees_ptr` | `-` |

### `struct drm_mode_get_lease`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count_objects` | `-` |
| `__u32` | `pad` | `-` |
| `__u64` | `objects_ptr` | `-` |

### `struct drm_mode_revoke_lease`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `lessee_id` | `-` |

### `struct drm_mode_rect`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `x1` | `-` |
| `__s32` | `y1` | `-` |
| `__s32` | `x2` | `-` |
| `__s32` | `y2` | `-` |

### `struct drm_mode_closefb`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fb_id` | `-` |
| `__u32` | `pad` | `-` |