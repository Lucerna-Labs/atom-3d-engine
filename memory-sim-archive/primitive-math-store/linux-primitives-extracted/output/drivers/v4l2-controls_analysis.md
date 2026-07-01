# v4l2-controls.h

**Source:** `v4l2-controls.h`


## Includes

- `linux/const.h`
- `linux/types.h`

## Defines (808 total)


### UNCATEGORIZED (808)

| Name | Value | Comment |
|------|-------|---------|
| `V4L2_CTRL_CLASS_USER` | `0x00980000` | Old-style 'user' controls |
| `V4L2_CTRL_CLASS_CODEC` | `0x00990000` | Stateful codec controls |
| `V4L2_CTRL_CLASS_CAMERA` | `0x009a0000` | Camera class controls |
| `V4L2_CTRL_CLASS_FM_TX` | `0x009b0000` | FM Modulator controls |
| `V4L2_CTRL_CLASS_FLASH` | `0x009c0000` | Camera flash controls |
| `V4L2_CTRL_CLASS_JPEG` | `0x009d0000` | JPEG-compression controls |
| `V4L2_CTRL_CLASS_IMAGE_SOURCE` | `0x009e0000` | Image source controls |
| `V4L2_CTRL_CLASS_IMAGE_PROC` | `0x009f0000` | Image processing controls |
| `V4L2_CTRL_CLASS_DV` | `0x00a00000` | Digital Video controls |
| `V4L2_CTRL_CLASS_FM_RX` | `0x00a10000` | FM Receiver controls |
| `V4L2_CTRL_CLASS_RF_TUNER` | `0x00a20000` | RF tuner controls |
| `V4L2_CTRL_CLASS_DETECT` | `0x00a30000` | Detection controls |
| `V4L2_CTRL_CLASS_CODEC_STATELESS` | `0x00a40000` | Stateless codecs controls |
| `V4L2_CTRL_CLASS_COLORIMETRY` | `0x00a50000` | Colorimetry controls |
| `V4L2_CID_BASE` | `(V4L2_CTRL_CLASS_USER \| 0x900)` |  |
| `V4L2_CID_USER_BASE` | `V4L2_CID_BASE` |  |
| `V4L2_CID_USER_CLASS` | `(V4L2_CTRL_CLASS_USER \| 1)` |  |
| `V4L2_CID_BRIGHTNESS` | `(V4L2_CID_BASE+0)` |  |
| `V4L2_CID_CONTRAST` | `(V4L2_CID_BASE+1)` |  |
| `V4L2_CID_SATURATION` | `(V4L2_CID_BASE+2)` |  |
| `V4L2_CID_HUE` | `(V4L2_CID_BASE+3)` |  |
| `V4L2_CID_AUDIO_VOLUME` | `(V4L2_CID_BASE+5)` |  |
| `V4L2_CID_AUDIO_BALANCE` | `(V4L2_CID_BASE+6)` |  |
| `V4L2_CID_AUDIO_BASS` | `(V4L2_CID_BASE+7)` |  |
| `V4L2_CID_AUDIO_TREBLE` | `(V4L2_CID_BASE+8)` |  |
| `V4L2_CID_AUDIO_MUTE` | `(V4L2_CID_BASE+9)` |  |
| `V4L2_CID_AUDIO_LOUDNESS` | `(V4L2_CID_BASE+10)` |  |
| `V4L2_CID_BLACK_LEVEL` | `(V4L2_CID_BASE+11)` | Deprecated |
| `V4L2_CID_AUTO_WHITE_BALANCE` | `(V4L2_CID_BASE+12)` |  |
| `V4L2_CID_DO_WHITE_BALANCE` | `(V4L2_CID_BASE+13)` |  |
| `V4L2_CID_RED_BALANCE` | `(V4L2_CID_BASE+14)` |  |
| `V4L2_CID_BLUE_BALANCE` | `(V4L2_CID_BASE+15)` |  |
| `V4L2_CID_GAMMA` | `(V4L2_CID_BASE+16)` |  |
| `V4L2_CID_WHITENESS` | `(V4L2_CID_GAMMA)` | Deprecated |
| `V4L2_CID_EXPOSURE` | `(V4L2_CID_BASE+17)` |  |
| `V4L2_CID_AUTOGAIN` | `(V4L2_CID_BASE+18)` |  |
| `V4L2_CID_GAIN` | `(V4L2_CID_BASE+19)` |  |
| `V4L2_CID_HFLIP` | `(V4L2_CID_BASE+20)` |  |
| `V4L2_CID_VFLIP` | `(V4L2_CID_BASE+21)` |  |
| `V4L2_CID_POWER_LINE_FREQUENCY` | `(V4L2_CID_BASE+24)` |  |
| `V4L2_CID_HUE_AUTO` | `(V4L2_CID_BASE+25)` |  |
| `V4L2_CID_WHITE_BALANCE_TEMPERATURE` | `(V4L2_CID_BASE+26)` |  |
| `V4L2_CID_SHARPNESS` | `(V4L2_CID_BASE+27)` |  |
| `V4L2_CID_BACKLIGHT_COMPENSATION` | `(V4L2_CID_BASE+28)` |  |
| `V4L2_CID_CHROMA_AGC` | `(V4L2_CID_BASE+29)` |  |
| `V4L2_CID_COLOR_KILLER` | `(V4L2_CID_BASE+30)` |  |
| `V4L2_CID_COLORFX` | `(V4L2_CID_BASE+31)` |  |
| `V4L2_CID_AUTOBRIGHTNESS` | `(V4L2_CID_BASE+32)` |  |
| `V4L2_CID_BAND_STOP_FILTER` | `(V4L2_CID_BASE+33)` |  |
| `V4L2_CID_ROTATE` | `(V4L2_CID_BASE+34)` |  |

*...and 758 more*

## Structs (47)


### `struct v4l2_ctrl_h264_sps`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `profile_idc` | `-` |
| `__u8` | `constraint_set_flags` | `-` |
| `__u8` | `level_idc` | `-` |
| `__u8` | `seq_parameter_set_id` | `-` |
| `__u8` | `chroma_format_idc` | `-` |
| `__u8` | `bit_depth_luma_minus8` | `-` |
| `__u8` | `bit_depth_chroma_minus8` | `-` |
| `__u8` | `log2_max_frame_num_minus4` | `-` |
| `__u8` | `pic_order_cnt_type` | `-` |
| `__u8` | `log2_max_pic_order_cnt_lsb_minus4` | `-` |
| `__u8` | `max_num_ref_frames` | `-` |
| `__u8` | `num_ref_frames_in_pic_order_cnt_cycle` | `-` |
| `__s32` | `offset_for_ref_frame` | `255` |
| `__s32` | `offset_for_non_ref_pic` | `-` |
| `__s32` | `offset_for_top_to_bottom_field` | `-` |
| `__u16` | `pic_width_in_mbs_minus1` | `-` |
| `__u16` | `pic_height_in_map_units_minus1` | `-` |
| `__u32` | `flags` | `-` |

### `struct v4l2_ctrl_h264_pps`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `pic_parameter_set_id` | `-` |
| `__u8` | `seq_parameter_set_id` | `-` |
| `__u8` | `num_slice_groups_minus1` | `-` |
| `__u8` | `num_ref_idx_l0_default_active_minus1` | `-` |
| `__u8` | `num_ref_idx_l1_default_active_minus1` | `-` |
| `__u8` | `weighted_bipred_idc` | `-` |
| `__s8` | `pic_init_qp_minus26` | `-` |
| `__s8` | `pic_init_qs_minus26` | `-` |
| `__s8` | `chroma_qp_index_offset` | `-` |
| `__s8` | `second_chroma_qp_index_offset` | `-` |
| `__u16` | `flags` | `-` |

### `struct v4l2_ctrl_h264_scaling_matrix`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `scaling_list_4x4` | `6][16` |
| `__u8` | `scaling_list_8x8` | `6][64` |

### `struct v4l2_h264_weight_factors`

| Type | Field | Array |
|------|-------|-------|
| `__s16` | `luma_weight` | `32` |
| `__s16` | `luma_offset` | `32` |
| `__s16` | `chroma_weight` | `32][2` |
| `__s16` | `chroma_offset` | `32][2` |

### `struct v4l2_ctrl_h264_pred_weights`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `luma_log2_weight_denom` | `-` |
| `__u16` | `chroma_log2_weight_denom` | `-` |

### `struct v4l2_h264_reference`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `fields` | `-` |
| `__u8` | `index` | `-` |

### `struct v4l2_ctrl_h264_slice_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `header_bit_size` | `-` |
| `__u32` | `first_mb_in_slice` | `-` |
| `__u8` | `slice_type` | `-` |
| `__u8` | `colour_plane_id` | `-` |
| `__u8` | `redundant_pic_cnt` | `-` |
| `__u8` | `cabac_init_idc` | `-` |
| `__s8` | `slice_qp_delta` | `-` |
| `__s8` | `slice_qs_delta` | `-` |
| `__u8` | `disable_deblocking_filter_idc` | `-` |
| `__s8` | `slice_alpha_c0_offset_div2` | `-` |
| `__s8` | `slice_beta_offset_div2` | `-` |
| `__u8` | `num_ref_idx_l0_active_minus1` | `-` |
| `__u8` | `num_ref_idx_l1_active_minus1` | `-` |
| `__u8` | `reserved` | `-` |
| `__u32` | `flags` | `-` |

### `struct v4l2_h264_dpb_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `reference_ts` | `-` |
| `__u32` | `pic_num` | `-` |
| `__u16` | `frame_num` | `-` |
| `__u8` | `fields` | `-` |
| `__u8` | `reserved` | `5` |
| `__s32` | `top_field_order_cnt` | `-` |
| `__s32` | `bottom_field_order_cnt` | `-` |
| `__u32` | `flags` | `-` |

### `struct v4l2_ctrl_h264_decode_params`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `nal_ref_idc` | `-` |
| `__u16` | `frame_num` | `-` |
| `__s32` | `top_field_order_cnt` | `-` |
| `__s32` | `bottom_field_order_cnt` | `-` |
| `__u16` | `idr_pic_id` | `-` |
| `__u16` | `pic_order_cnt_lsb` | `-` |
| `__s32` | `delta_pic_order_cnt_bottom` | `-` |
| `__s32` | `delta_pic_order_cnt0` | `-` |
| `__s32` | `delta_pic_order_cnt1` | `-` |
| `__u32` | `dec_ref_pic_marking_bit_size` | `-` |
| `__u32` | `pic_order_cnt_bit_size` | `-` |
| `__u32` | `slice_group_change_cycle` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `flags` | `-` |

### `struct v4l2_ctrl_fwht_params`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `backward_ref_ts` | `-` |
| `__u32` | `version` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `colorspace` | `-` |
| `__u32` | `xfer_func` | `-` |
| `__u32` | `ycbcr_enc` | `-` |
| `__u32` | `quantization` | `-` |

### `struct v4l2_vp8_segment`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `quant_update` | `4` |
| `__s8` | `lf_update` | `4` |
| `__u8` | `segment_probs` | `3` |
| `__u8` | `padding` | `-` |
| `__u32` | `flags` | `-` |

### `struct v4l2_vp8_loop_filter`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `ref_frm_delta` | `4` |
| `__s8` | `mb_mode_delta` | `4` |
| `__u8` | `sharpness_level` | `-` |
| `__u8` | `level` | `-` |
| `__u16` | `padding` | `-` |
| `__u32` | `flags` | `-` |

### `struct v4l2_vp8_quantization`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `y_ac_qi` | `-` |
| `__s8` | `y_dc_delta` | `-` |
| `__s8` | `y2_dc_delta` | `-` |
| `__s8` | `y2_ac_delta` | `-` |
| `__s8` | `uv_dc_delta` | `-` |
| `__s8` | `uv_ac_delta` | `-` |
| `__u16` | `padding` | `-` |

### `struct v4l2_vp8_entropy`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `coeff_probs` | `4][8][3][V4L2_VP8_COEFF_PROB_CNT` |
| `__u8` | `y_mode_probs` | `4` |
| `__u8` | `uv_mode_probs` | `3` |
| `__u8` | `mv_probs` | `2][V4L2_VP8_MV_PROB_CNT` |
| `__u8` | `padding` | `3` |

### `struct v4l2_vp8_entropy_coder_state`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `range` | `-` |
| `__u8` | `value` | `-` |
| `__u8` | `bit_count` | `-` |
| `__u8` | `padding` | `-` |

### `struct v4l2_ctrl_vp8_frame`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `width` | `-` |
| `__u16` | `height` | `-` |
| `__u8` | `horizontal_scale` | `-` |
| `__u8` | `vertical_scale` | `-` |
| `__u8` | `version` | `-` |
| `__u8` | `prob_skip_false` | `-` |
| `__u8` | `prob_intra` | `-` |
| `__u8` | `prob_last` | `-` |
| `__u8` | `prob_gf` | `-` |
| `__u8` | `num_dct_parts` | `-` |
| `__u32` | `first_part_size` | `-` |
| `__u32` | `first_part_header_bits` | `-` |
| `__u32` | `dct_part_sizes` | `8` |
| `__u64` | `last_frame_ts` | `-` |
| `__u64` | `golden_frame_ts` | `-` |
| `__u64` | `alt_frame_ts` | `-` |
| `__u64` | `flags` | `-` |

### `struct v4l2_ctrl_mpeg2_sequence`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `horizontal_size` | `-` |
| `__u16` | `vertical_size` | `-` |
| `__u32` | `vbv_buffer_size` | `-` |
| `__u16` | `profile_and_level_indication` | `-` |
| `__u8` | `chroma_format` | `-` |
| `__u8` | `flags` | `-` |

### `struct v4l2_ctrl_mpeg2_picture`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `backward_ref_ts` | `-` |
| `__u64` | `forward_ref_ts` | `-` |
| `__u32` | `flags` | `-` |
| `__u8` | `f_code` | `2][2` |
| `__u8` | `picture_coding_type` | `-` |
| `__u8` | `picture_structure` | `-` |
| `__u8` | `intra_dc_precision` | `-` |
| `__u8` | `reserved` | `5` |

### `struct v4l2_ctrl_mpeg2_quantisation`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `intra_quantiser_matrix` | `64` |
| `__u8` | `non_intra_quantiser_matrix` | `64` |
| `__u8` | `chroma_intra_quantiser_matrix` | `64` |
| `__u8` | `chroma_non_intra_quantiser_matrix` | `64` |

### `struct v4l2_ctrl_hevc_sps`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `video_parameter_set_id` | `-` |
| `__u8` | `seq_parameter_set_id` | `-` |
| `__u16` | `pic_width_in_luma_samples` | `-` |
| `__u16` | `pic_height_in_luma_samples` | `-` |
| `__u8` | `bit_depth_luma_minus8` | `-` |
| `__u8` | `bit_depth_chroma_minus8` | `-` |
| `__u8` | `log2_max_pic_order_cnt_lsb_minus4` | `-` |
| `__u8` | `sps_max_dec_pic_buffering_minus1` | `-` |
| `__u8` | `sps_max_num_reorder_pics` | `-` |
| `__u8` | `sps_max_latency_increase_plus1` | `-` |
| `__u8` | `log2_min_luma_coding_block_size_minus3` | `-` |
| `__u8` | `log2_diff_max_min_luma_coding_block_size` | `-` |
| `__u8` | `log2_min_luma_transform_block_size_minus2` | `-` |
| `__u8` | `log2_diff_max_min_luma_transform_block_size` | `-` |
| `__u8` | `max_transform_hierarchy_depth_inter` | `-` |
| `__u8` | `max_transform_hierarchy_depth_intra` | `-` |
| `__u8` | `pcm_sample_bit_depth_luma_minus1` | `-` |
| `__u8` | `pcm_sample_bit_depth_chroma_minus1` | `-` |
| `__u8` | `log2_min_pcm_luma_coding_block_size_minus3` | `-` |
| `__u8` | `log2_diff_max_min_pcm_luma_coding_block_size` | `-` |
| `__u8` | `num_short_term_ref_pic_sets` | `-` |
| `__u8` | `num_long_term_ref_pics_sps` | `-` |
| `__u8` | `chroma_format_idc` | `-` |
| `__u8` | `sps_max_sub_layers_minus1` | `-` |
| `__u8` | `reserved` | `6` |
| `__u64` | `flags` | `-` |

### `struct v4l2_ctrl_hevc_pps`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `pic_parameter_set_id` | `-` |
| `__u8` | `num_extra_slice_header_bits` | `-` |
| `__u8` | `num_ref_idx_l0_default_active_minus1` | `-` |
| `__u8` | `num_ref_idx_l1_default_active_minus1` | `-` |
| `__s8` | `init_qp_minus26` | `-` |
| `__u8` | `diff_cu_qp_delta_depth` | `-` |
| `__s8` | `pps_cb_qp_offset` | `-` |
| `__s8` | `pps_cr_qp_offset` | `-` |
| `__u8` | `num_tile_columns_minus1` | `-` |
| `__u8` | `num_tile_rows_minus1` | `-` |
| `__u8` | `column_width_minus1` | `20` |
| `__u8` | `row_height_minus1` | `22` |
| `__s8` | `pps_beta_offset_div2` | `-` |
| `__s8` | `pps_tc_offset_div2` | `-` |
| `__u8` | `log2_parallel_merge_level_minus2` | `-` |
| `__u8` | `reserved` | `-` |
| `__u64` | `flags` | `-` |

### `struct v4l2_hevc_dpb_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `timestamp` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `field_pic` | `-` |
| `__u16` | `reserved` | `-` |
| `__s32` | `pic_order_cnt_val` | `-` |

### `struct v4l2_hevc_pred_weight_table`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `delta_luma_weight_l0` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__s8` | `luma_offset_l0` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__s8` | `delta_chroma_weight_l0` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2` |
| `__s8` | `chroma_offset_l0` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2` |
| `__s8` | `delta_luma_weight_l1` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__s8` | `luma_offset_l1` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__s8` | `delta_chroma_weight_l1` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2` |
| `__s8` | `chroma_offset_l1` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX][2` |
| `__u8` | `luma_log2_weight_denom` | `-` |
| `__s8` | `delta_chroma_log2_weight_denom` | `-` |

### `struct v4l2_ctrl_hevc_slice_params`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bit_size` | `-` |
| `__u32` | `data_byte_offset` | `-` |
| `__u32` | `num_entry_point_offsets` | `-` |
| `__u8` | `nal_unit_type` | `-` |
| `__u8` | `nuh_temporal_id_plus1` | `-` |
| `__u8` | `slice_type` | `-` |
| `__u8` | `colour_plane_id` | `-` |
| `__s32` | `slice_pic_order_cnt` | `-` |
| `__u8` | `num_ref_idx_l0_active_minus1` | `-` |
| `__u8` | `num_ref_idx_l1_active_minus1` | `-` |
| `__u8` | `collocated_ref_idx` | `-` |
| `__u8` | `five_minus_max_num_merge_cand` | `-` |
| `__s8` | `slice_qp_delta` | `-` |
| `__s8` | `slice_cb_qp_offset` | `-` |
| `__s8` | `slice_cr_qp_offset` | `-` |
| `__s8` | `slice_act_y_qp_offset` | `-` |
| `__s8` | `slice_act_cb_qp_offset` | `-` |
| `__s8` | `slice_act_cr_qp_offset` | `-` |
| `__s8` | `slice_beta_offset_div2` | `-` |
| `__s8` | `slice_tc_offset_div2` | `-` |
| `__u8` | `pic_struct` | `-` |
| `__u8` | `reserved0` | `3` |
| `__u32` | `slice_segment_addr` | `-` |
| `__u8` | `ref_idx_l0` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__u8` | `ref_idx_l1` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__u16` | `short_term_ref_pic_set_size` | `-` |
| `__u16` | `long_term_ref_pic_set_size` | `-` |
| `__u8` | `reserved1` | `2` |
| `__u64` | `flags` | `-` |

### `struct v4l2_ctrl_hevc_decode_params`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `pic_order_cnt_val` | `-` |
| `__u16` | `short_term_ref_pic_set_size` | `-` |
| `__u16` | `long_term_ref_pic_set_size` | `-` |
| `__u8` | `num_active_dpb_entries` | `-` |
| `__u8` | `num_poc_st_curr_before` | `-` |
| `__u8` | `num_poc_st_curr_after` | `-` |
| `__u8` | `num_poc_lt_curr` | `-` |
| `__u8` | `poc_st_curr_before` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__u8` | `poc_st_curr_after` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__u8` | `poc_lt_curr` | `V4L2_HEVC_DPB_ENTRIES_NUM_MAX` |
| `__u8` | `num_delta_pocs_of_ref_rps_idx` | `-` |
| `__u8` | `reserved` | `3` |
| `__u64` | `flags` | `-` |

### `struct v4l2_ctrl_hevc_scaling_matrix`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `scaling_list_4x4` | `6][16` |
| `__u8` | `scaling_list_8x8` | `6][64` |
| `__u8` | `scaling_list_16x16` | `6][64` |
| `__u8` | `scaling_list_32x32` | `2][64` |
| `__u8` | `scaling_list_dc_coef_16x16` | `6` |
| `__u8` | `scaling_list_dc_coef_32x32` | `2` |

### `struct v4l2_ctrl_hevc_ext_sps_st_rps`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `delta_idx_minus1` | `-` |
| `__u8` | `delta_rps_sign` | `-` |
| `__u8` | `num_negative_pics` | `-` |
| `__u8` | `num_positive_pics` | `-` |
| `__u32` | `used_by_curr_pic` | `-` |
| `__u32` | `use_delta_flag` | `-` |
| `__u16` | `abs_delta_rps_minus1` | `-` |
| `__u16` | `delta_poc_s0_minus1` | `16` |
| `__u16` | `delta_poc_s1_minus1` | `16` |
| `__u16` | `flags` | `-` |

### `struct v4l2_ctrl_hevc_ext_sps_lt_rps`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `lt_ref_pic_poc_lsb_sps` | `-` |
| `__u16` | `flags` | `-` |

### `struct v4l2_vp9_loop_filter`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `ref_deltas` | `4` |
| `__s8` | `mode_deltas` | `2` |
| `__u8` | `level` | `-` |
| `__u8` | `sharpness` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `reserved` | `7` |

### `struct v4l2_vp9_quantization`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `base_q_idx` | `-` |
| `__s8` | `delta_q_y_dc` | `-` |
| `__s8` | `delta_q_uv_dc` | `-` |
| `__s8` | `delta_q_uv_ac` | `-` |
| `__u8` | `reserved` | `4` |

### `struct v4l2_vp9_segmentation`

| Type | Field | Array |
|------|-------|-------|
| `__s16` | `feature_data` | `8][4` |
| `__u8` | `feature_enabled` | `8` |
| `__u8` | `tree_probs` | `7` |
| `__u8` | `pred_probs` | `3` |
| `__u8` | `flags` | `-` |
| `__u8` | `reserved` | `5` |

### `struct v4l2_ctrl_vp9_frame`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u16` | `compressed_header_size` | `-` |
| `__u16` | `uncompressed_header_size` | `-` |
| `__u16` | `frame_width_minus_1` | `-` |
| `__u16` | `frame_height_minus_1` | `-` |
| `__u16` | `render_width_minus_1` | `-` |
| `__u16` | `render_height_minus_1` | `-` |
| `__u64` | `last_frame_ts` | `-` |
| `__u64` | `golden_frame_ts` | `-` |
| `__u64` | `alt_frame_ts` | `-` |
| `__u8` | `ref_frame_sign_bias` | `-` |
| `__u8` | `reset_frame_context` | `-` |
| `__u8` | `frame_context_idx` | `-` |
| `__u8` | `profile` | `-` |
| `__u8` | `bit_depth` | `-` |
| `__u8` | `interpolation_filter` | `-` |
| `__u8` | `tile_cols_log2` | `-` |
| `__u8` | `tile_rows_log2` | `-` |
| `__u8` | `reference_mode` | `-` |
| `__u8` | `reserved` | `7` |

### `struct v4l2_vp9_mv_probs`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `joint` | `3` |
| `__u8` | `sign` | `2` |
| `__u8` | `classes` | `2][10` |
| `__u8` | `class0_bit` | `2` |
| `__u8` | `bits` | `2][10` |
| `__u8` | `class0_fr` | `2][2][3` |
| `__u8` | `fr` | `2][3` |
| `__u8` | `class0_hp` | `2` |
| `__u8` | `hp` | `2` |

### `struct v4l2_ctrl_vp9_compressed_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `tx_mode` | `-` |
| `__u8` | `tx8` | `2][1` |
| `__u8` | `tx16` | `2][2` |
| `__u8` | `tx32` | `2][3` |
| `__u8` | `coef` | `4][2][2][6][6][3` |
| `__u8` | `skip` | `3` |
| `__u8` | `inter_mode` | `7][3` |
| `__u8` | `interp_filter` | `4][2` |
| `__u8` | `is_inter` | `4` |
| `__u8` | `comp_mode` | `5` |
| `__u8` | `single_ref` | `5][2` |
| `__u8` | `comp_ref` | `5` |
| `__u8` | `y_mode` | `4][9` |
| `__u8` | `uv_mode` | `10][9` |
| `__u8` | `partition` | `16][3` |

### `struct v4l2_ctrl_av1_sequence`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u8` | `seq_profile` | `-` |
| `__u8` | `order_hint_bits` | `-` |
| `__u8` | `bit_depth` | `-` |
| `__u8` | `reserved` | `-` |
| `__u16` | `max_frame_width_minus_1` | `-` |
| `__u16` | `max_frame_height_minus_1` | `-` |

### `struct v4l2_ctrl_av1_tile_group_entry`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tile_offset` | `-` |
| `__u32` | `tile_size` | `-` |
| `__u32` | `tile_row` | `-` |
| `__u32` | `tile_col` | `-` |

### `struct v4l2_av1_global_motion`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `V4L2_AV1_TOTAL_REFS_PER_FRAME` |
| `__s32` | `params` | `V4L2_AV1_TOTAL_REFS_PER_FRAME][6` |
| `__u8` | `invalid` | `-` |
| `__u8` | `reserved` | `3` |

### `struct v4l2_av1_loop_restoration`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `lr_unit_shift` | `-` |
| `__u8` | `lr_uv_shift` | `-` |
| `__u8` | `reserved` | `-` |
| `__u32` | `loop_restoration_size` | `V4L2_AV1_MAX_NUM_PLANES` |

### `struct v4l2_av1_cdef`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `damping_minus_3` | `-` |
| `__u8` | `bits` | `-` |
| `__u8` | `y_pri_strength` | `V4L2_AV1_CDEF_MAX` |
| `__u8` | `y_sec_strength` | `V4L2_AV1_CDEF_MAX` |
| `__u8` | `uv_pri_strength` | `V4L2_AV1_CDEF_MAX` |
| `__u8` | `uv_sec_strength` | `V4L2_AV1_CDEF_MAX` |

### `struct v4l2_av1_segmentation`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `last_active_seg_id` | `-` |
| `__u8` | `feature_enabled` | `V4L2_AV1_MAX_SEGMENTS` |
| `__s16` | `feature_data` | `V4L2_AV1_MAX_SEGMENTS][V4L2_AV1_SEG_LVL_MAX` |

### `struct v4l2_av1_loop_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `level` | `4` |
| `__u8` | `sharpness` | `-` |
| `__s8` | `ref_deltas` | `V4L2_AV1_TOTAL_REFS_PER_FRAME` |
| `__s8` | `mode_deltas` | `2` |
| `__u8` | `delta_lf_res` | `-` |

### `struct v4l2_av1_quantization`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `base_q_idx` | `-` |
| `__s8` | `delta_q_y_dc` | `-` |
| `__s8` | `delta_q_u_dc` | `-` |
| `__s8` | `delta_q_u_ac` | `-` |
| `__s8` | `delta_q_v_dc` | `-` |
| `__s8` | `delta_q_v_ac` | `-` |
| `__u8` | `qm_y` | `-` |
| `__u8` | `qm_u` | `-` |
| `__u8` | `qm_v` | `-` |
| `__u8` | `delta_q_res` | `-` |

### `struct v4l2_av1_tile_info`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `context_update_tile_id` | `-` |
| `__u8` | `tile_cols` | `-` |
| `__u8` | `tile_rows` | `-` |
| `__u32` | `mi_col_starts` | `V4L2_AV1_MAX_TILE_COLS + 1` |
| `__u32` | `mi_row_starts` | `V4L2_AV1_MAX_TILE_ROWS + 1` |
| `__u32` | `width_in_sbs_minus_1` | `V4L2_AV1_MAX_TILE_COLS` |
| `__u32` | `height_in_sbs_minus_1` | `V4L2_AV1_MAX_TILE_ROWS` |
| `__u8` | `tile_size_bytes` | `-` |
| `__u8` | `reserved` | `3` |

### `struct v4l2_ctrl_av1_frame`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `superres_denom` | `-` |
| `__u8` | `skip_mode_frame` | `2` |
| `__u8` | `primary_ref_frame` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `order_hint` | `-` |
| `__u32` | `upscaled_width` | `-` |
| `__u32` | `frame_width_minus_1` | `-` |
| `__u32` | `frame_height_minus_1` | `-` |
| `__u16` | `render_width_minus_1` | `-` |
| `__u16` | `render_height_minus_1` | `-` |
| `__u32` | `current_frame_id` | `-` |
| `__u32` | `buffer_removal_time` | `V4L2_AV1_MAX_OPERATING_POINTS` |
| `__u8` | `reserved` | `4` |
| `__u32` | `order_hints` | `V4L2_AV1_TOTAL_REFS_PER_FRAME` |
| `__u64` | `reference_frame_ts` | `V4L2_AV1_TOTAL_REFS_PER_FRAME` |
| `__s8` | `ref_frame_idx` | `V4L2_AV1_REFS_PER_FRAME` |
| `__u8` | `refresh_frame_flags` | `-` |

### `struct v4l2_ctrl_av1_film_grain`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `cr_mult` | `-` |
| `__u16` | `grain_seed` | `-` |
| `__u8` | `film_grain_params_ref_idx` | `-` |
| `__u8` | `num_y_points` | `-` |
| `__u8` | `point_y_value` | `V4L2_AV1_MAX_NUM_Y_POINTS` |
| `__u8` | `point_y_scaling` | `V4L2_AV1_MAX_NUM_Y_POINTS` |
| `__u8` | `num_cb_points` | `-` |
| `__u8` | `point_cb_value` | `V4L2_AV1_MAX_NUM_CB_POINTS` |
| `__u8` | `point_cb_scaling` | `V4L2_AV1_MAX_NUM_CB_POINTS` |
| `__u8` | `num_cr_points` | `-` |
| `__u8` | `point_cr_value` | `V4L2_AV1_MAX_NUM_CR_POINTS` |
| `__u8` | `point_cr_scaling` | `V4L2_AV1_MAX_NUM_CR_POINTS` |
| `__u8` | `grain_scaling_minus_8` | `-` |
| `__u8` | `ar_coeff_lag` | `-` |
| `__u8` | `ar_coeffs_y_plus_128` | `V4L2_AV1_AR_COEFFS_SIZE` |
| `__u8` | `ar_coeffs_cb_plus_128` | `V4L2_AV1_AR_COEFFS_SIZE` |
| `__u8` | `ar_coeffs_cr_plus_128` | `V4L2_AV1_AR_COEFFS_SIZE` |
| `__u8` | `ar_coeff_shift_minus_6` | `-` |
| `__u8` | `grain_scale_shift` | `-` |
| `__u8` | `cb_mult` | `-` |
| `__u8` | `cb_luma_mult` | `-` |
| `__u8` | `cr_luma_mult` | `-` |
| `__u16` | `cb_offset` | `-` |
| `__u16` | `cr_offset` | `-` |
| `__u8` | `reserved` | `4` |

### `struct v4l2_ctrl_hdr10_cll_info`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `max_content_light_level` | `-` |
| `__u16` | `max_pic_average_light_level` | `-` |

### `struct v4l2_ctrl_hdr10_mastering_display`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `display_primaries_x` | `3` |
| `__u16` | `display_primaries_y` | `3` |
| `__u16` | `white_point_x` | `-` |
| `__u16` | `white_point_y` | `-` |
| `__u32` | `max_display_mastering_luminance` | `-` |
| `__u32` | `min_display_mastering_luminance` | `-` |