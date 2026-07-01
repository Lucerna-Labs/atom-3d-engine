# ib_user_verbs.h

**Source:** `ib_user_verbs.h`


## Includes

- `linux/types.h`

## Defines (6 total)


### IB_DEVICE (1)

| Name | Value | Comment |
|------|-------|---------|
| `IB_DEVICE_NAME_MAX` | `64` |  |

### IB_USER (5)

| Name | Value | Comment |
|------|-------|---------|
| `IB_USER_VERBS_ABI_VERSION` | `6` |  |
| `IB_USER_VERBS_CMD_THRESHOLD` | `50` |  |
| `IB_USER_VERBS_CMD_COMMAND_MASK` | `0xff` |  |
| `IB_USER_VERBS_CMD_FLAG_EXTENDED` | `0x80000000u` |  |
| `IB_USER_VERBS_MAX_LOG_IND_TBL_SIZE` | `0x0d` |  |

## Structs (133)


### `struct ib_uverbs_async_event_desc`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `element` | `-` |
| `__u32` | `event_type` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_comp_event_desc`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `cq_handle` | `-` |

### `struct ib_uverbs_cq_moderation_caps`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `max_cq_moderation_count` | `-` |
| `__u16` | `max_cq_moderation_period` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_cmd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `command` | `-` |
| `__u16` | `in_words` | `-` |
| `__u16` | `out_words` | `-` |

### `struct ib_uverbs_ex_cmd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u16` | `provider_in_words` | `-` |
| `__u16` | `provider_out_words` | `-` |
| `__u32` | `cmd_hdr_reserved` | `-` |

### `struct ib_uverbs_get_context`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |

### `struct ib_uverbs_get_context_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `async_fd` | `-` |
| `__u32` | `num_comp_vectors` | `-` |

### `struct ib_uverbs_query_device`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |

### `struct ib_uverbs_query_device_resp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `fw_ver` | `-` |
| `__be64` | `node_guid` | `-` |
| `__be64` | `sys_image_guid` | `-` |
| `__aligned_u64` | `max_mr_size` | `-` |
| `__aligned_u64` | `page_size_cap` | `-` |
| `__u32` | `vendor_id` | `-` |
| `__u32` | `vendor_part_id` | `-` |
| `__u32` | `hw_ver` | `-` |
| `__u32` | `max_qp` | `-` |
| `__u32` | `max_qp_wr` | `-` |
| `__u32` | `device_cap_flags` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `max_sge_rd` | `-` |
| `__u32` | `max_cq` | `-` |
| `__u32` | `max_cqe` | `-` |
| `__u32` | `max_mr` | `-` |
| `__u32` | `max_pd` | `-` |
| `__u32` | `max_qp_rd_atom` | `-` |
| `__u32` | `max_ee_rd_atom` | `-` |
| `__u32` | `max_res_rd_atom` | `-` |
| `__u32` | `max_qp_init_rd_atom` | `-` |
| `__u32` | `max_ee_init_rd_atom` | `-` |
| `__u32` | `atomic_cap` | `-` |
| `__u32` | `max_ee` | `-` |
| `__u32` | `max_rdd` | `-` |
| `__u32` | `max_mw` | `-` |
| `__u32` | `max_raw_ipv6_qp` | `-` |
| `__u32` | `max_raw_ethy_qp` | `-` |
| `__u32` | `max_mcast_grp` | `-` |
| `__u32` | `max_mcast_qp_attach` | `-` |
| `__u32` | `max_total_mcast_qp_attach` | `-` |
| `__u32` | `max_ah` | `-` |
| `__u32` | `max_fmr` | `-` |
| `__u32` | `max_map_per_fmr` | `-` |
| `__u32` | `max_srq` | `-` |
| `__u32` | `max_srq_wr` | `-` |
| `__u32` | `max_srq_sge` | `-` |
| `__u16` | `max_pkeys` | `-` |
| `__u8` | `local_ca_ack_delay` | `-` |
| `__u8` | `phys_port_cnt` | `-` |
| `__u8` | `reserved` | `4` |

### `struct ib_uverbs_ex_query_device`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_odp_caps`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `general_caps` | `-` |
| `__u32` | `rc_odp_caps` | `-` |
| `__u32` | `uc_odp_caps` | `-` |
| `__u32` | `ud_odp_caps` | `-` |
| `__u32` | `reserved` | `-` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rc_odp_caps` | `-` |
| `__u32` | `uc_odp_caps` | `-` |
| `__u32` | `ud_odp_caps` | `-` |

### `struct ib_uverbs_rss_caps`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `supported_qpts` | `-` |
| `__u32` | `max_rwq_indirection_tables` | `-` |
| `__u32` | `max_rwq_indirection_table_size` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_tm_caps`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_rndv_hdr_size` | `-` |
| `__u32` | `max_num_tags` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `max_ops` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_ex_query_device_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |
| `__aligned_u64` | `timestamp_mask` | `-` |
| `__aligned_u64` | `hca_core_clock` | `-` |
| `__aligned_u64` | `device_cap_flags_ex` | `-` |
| `__u32` | `max_wq_type_rq` | `-` |
| `__u32` | `raw_packet_caps` | `-` |
| `__aligned_u64` | `max_dm_size` | `-` |
| `__u32` | `xrc_odp_caps` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_query_port`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `reserved` | `7` |

### `struct ib_uverbs_query_port_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `port_cap_flags` | `-` |
| `__u32` | `max_msg_sz` | `-` |
| `__u32` | `bad_pkey_cntr` | `-` |
| `__u32` | `qkey_viol_cntr` | `-` |
| `__u32` | `gid_tbl_len` | `-` |
| `__u16` | `pkey_tbl_len` | `-` |
| `__u16` | `lid` | `-` |
| `__u16` | `sm_lid` | `-` |
| `__u8` | `state` | `-` |
| `__u8` | `max_mtu` | `-` |
| `__u8` | `active_mtu` | `-` |
| `__u8` | `lmc` | `-` |
| `__u8` | `max_vl_num` | `-` |
| `__u8` | `sm_sl` | `-` |
| `__u8` | `subnet_timeout` | `-` |
| `__u8` | `init_type_reply` | `-` |
| `__u8` | `active_width` | `-` |
| `__u8` | `active_speed` | `-` |
| `__u8` | `phys_state` | `-` |
| `__u8` | `link_layer` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ib_uverbs_alloc_pd`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |

### `struct ib_uverbs_alloc_pd_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pd_handle` | `-` |

### `struct ib_uverbs_dealloc_pd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pd_handle` | `-` |

### `struct ib_uverbs_open_xrcd`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `fd` | `-` |
| `__u32` | `oflags` | `-` |

### `struct ib_uverbs_open_xrcd_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `xrcd_handle` | `-` |

### `struct ib_uverbs_close_xrcd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `xrcd_handle` | `-` |

### `struct ib_uverbs_reg_mr`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `start` | `-` |
| `__aligned_u64` | `length` | `-` |
| `__aligned_u64` | `hca_va` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `access_flags` | `-` |

### `struct ib_uverbs_reg_mr_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mr_handle` | `-` |
| `__u32` | `lkey` | `-` |
| `__u32` | `rkey` | `-` |

### `struct ib_uverbs_rereg_mr`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `mr_handle` | `-` |
| `__u32` | `flags` | `-` |
| `__aligned_u64` | `start` | `-` |
| `__aligned_u64` | `length` | `-` |
| `__aligned_u64` | `hca_va` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `access_flags` | `-` |

### `struct ib_uverbs_rereg_mr_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `lkey` | `-` |
| `__u32` | `rkey` | `-` |

### `struct ib_uverbs_dereg_mr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mr_handle` | `-` |

### `struct ib_uverbs_alloc_mw`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u8` | `mw_type` | `-` |
| `__u8` | `reserved` | `3` |

### `struct ib_uverbs_alloc_mw_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mw_handle` | `-` |
| `__u32` | `rkey` | `-` |

### `struct ib_uverbs_dealloc_mw`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mw_handle` | `-` |

### `struct ib_uverbs_create_comp_channel`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |

### `struct ib_uverbs_create_comp_channel_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `fd` | `-` |

### `struct ib_uverbs_create_cq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `cqe` | `-` |
| `__u32` | `comp_vector` | `-` |
| `__s32` | `comp_channel` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_ex_create_cq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `cqe` | `-` |
| `__u32` | `comp_vector` | `-` |
| `__s32` | `comp_channel` | `-` |
| `__u32` | `comp_mask` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_create_cq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cq_handle` | `-` |
| `__u32` | `cqe` | `-` |
| `__aligned_u64` | `driver_data` | `0` |

### `struct ib_uverbs_ex_create_cq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |

### `struct ib_uverbs_resize_cq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `cq_handle` | `-` |
| `__u32` | `cqe` | `-` |

### `struct ib_uverbs_resize_cq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cqe` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_poll_cq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `cq_handle` | `-` |
| `__u32` | `ne` | `-` |

### `struct ib_uverbs_wc`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `wr_id` | `-` |
| `__u32` | `status` | `-` |
| `__u32` | `opcode` | `-` |
| `__u32` | `vendor_err` | `-` |
| `__u32` | `byte_len` | `-` |
| `__be32` | `imm_data` | `-` |
| `__u32` | `invalidate_rkey` | `-` |
| `__u32` | `qp_num` | `-` |
| `__u32` | `src_qp` | `-` |
| `__u32` | `wc_flags` | `-` |
| `__u16` | `pkey_index` | `-` |
| `__u16` | `slid` | `-` |
| `__u8` | `sl` | `-` |
| `__u8` | `dlid_path_bits` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ib_uverbs_poll_cq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `count` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_req_notify_cq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cq_handle` | `-` |
| `__u32` | `solicited_only` | `-` |

### `struct ib_uverbs_destroy_cq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `cq_handle` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_destroy_cq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_events_reported` | `-` |
| `__u32` | `async_events_reported` | `-` |

### `struct ib_uverbs_global_route`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `dgid` | `16` |
| `__u32` | `flow_label` | `-` |
| `__u8` | `sgid_index` | `-` |
| `__u8` | `hop_limit` | `-` |
| `__u8` | `traffic_class` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ib_uverbs_ah_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `dlid` | `-` |
| `__u8` | `sl` | `-` |
| `__u8` | `src_path_bits` | `-` |
| `__u8` | `static_rate` | `-` |
| `__u8` | `is_global` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ib_uverbs_qp_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `qp_attr_mask` | `-` |
| `__u32` | `qp_state` | `-` |
| `__u32` | `cur_qp_state` | `-` |
| `__u32` | `path_mtu` | `-` |
| `__u32` | `path_mig_state` | `-` |
| `__u32` | `qkey` | `-` |
| `__u32` | `rq_psn` | `-` |
| `__u32` | `sq_psn` | `-` |
| `__u32` | `dest_qp_num` | `-` |
| `__u32` | `qp_access_flags` | `-` |
| `__u32` | `max_send_wr` | `-` |
| `__u32` | `max_recv_wr` | `-` |
| `__u32` | `max_send_sge` | `-` |
| `__u32` | `max_recv_sge` | `-` |
| `__u32` | `max_inline_data` | `-` |
| `__u16` | `pkey_index` | `-` |
| `__u16` | `alt_pkey_index` | `-` |
| `__u8` | `en_sqd_async_notify` | `-` |
| `__u8` | `sq_draining` | `-` |
| `__u8` | `max_rd_atomic` | `-` |
| `__u8` | `max_dest_rd_atomic` | `-` |
| `__u8` | `min_rnr_timer` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `timeout` | `-` |
| `__u8` | `retry_cnt` | `-` |
| `__u8` | `rnr_retry` | `-` |
| `__u8` | `alt_port_num` | `-` |
| `__u8` | `alt_timeout` | `-` |
| `__u8` | `reserved` | `5` |

### `struct ib_uverbs_create_qp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `send_cq_handle` | `-` |
| `__u32` | `recv_cq_handle` | `-` |
| `__u32` | `srq_handle` | `-` |
| `__u32` | `max_send_wr` | `-` |
| `__u32` | `max_recv_wr` | `-` |
| `__u32` | `max_send_sge` | `-` |
| `__u32` | `max_recv_sge` | `-` |
| `__u32` | `max_inline_data` | `-` |
| `__u8` | `sq_sig_all` | `-` |
| `__u8` | `qp_type` | `-` |
| `__u8` | `is_srq` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ib_uverbs_ex_create_qp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `send_cq_handle` | `-` |
| `__u32` | `recv_cq_handle` | `-` |
| `__u32` | `srq_handle` | `-` |
| `__u32` | `max_send_wr` | `-` |
| `__u32` | `max_recv_wr` | `-` |
| `__u32` | `max_send_sge` | `-` |
| `__u32` | `max_recv_sge` | `-` |
| `__u32` | `max_inline_data` | `-` |
| `__u8` | `sq_sig_all` | `-` |
| `__u8` | `qp_type` | `-` |
| `__u8` | `is_srq` | `-` |
| `__u8` | `reserved` | `-` |
| `__u32` | `comp_mask` | `-` |
| `__u32` | `create_flags` | `-` |
| `__u32` | `rwq_ind_tbl_handle` | `-` |
| `__u32` | `source_qpn` | `-` |

### `struct ib_uverbs_open_qp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `qpn` | `-` |
| `__u8` | `qp_type` | `-` |
| `__u8` | `reserved` | `7` |

### `struct ib_uverbs_create_qp_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `qp_handle` | `-` |
| `__u32` | `qpn` | `-` |
| `__u32` | `max_send_wr` | `-` |
| `__u32` | `max_recv_wr` | `-` |
| `__u32` | `max_send_sge` | `-` |
| `__u32` | `max_recv_sge` | `-` |
| `__u32` | `max_inline_data` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `driver_data` | `0` |

### `struct ib_uverbs_ex_create_qp_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |

### `struct ib_uverbs_qp_dest`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `dgid` | `16` |
| `__u32` | `flow_label` | `-` |
| `__u16` | `dlid` | `-` |
| `__u16` | `reserved` | `-` |
| `__u8` | `sgid_index` | `-` |
| `__u8` | `hop_limit` | `-` |
| `__u8` | `traffic_class` | `-` |
| `__u8` | `sl` | `-` |
| `__u8` | `src_path_bits` | `-` |
| `__u8` | `static_rate` | `-` |
| `__u8` | `is_global` | `-` |
| `__u8` | `port_num` | `-` |

### `struct ib_uverbs_query_qp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `qp_handle` | `-` |
| `__u32` | `attr_mask` | `-` |

### `struct ib_uverbs_query_qp_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_send_wr` | `-` |
| `__u32` | `max_recv_wr` | `-` |
| `__u32` | `max_send_sge` | `-` |
| `__u32` | `max_recv_sge` | `-` |
| `__u32` | `max_inline_data` | `-` |
| `__u32` | `qkey` | `-` |
| `__u32` | `rq_psn` | `-` |
| `__u32` | `sq_psn` | `-` |
| `__u32` | `dest_qp_num` | `-` |
| `__u32` | `qp_access_flags` | `-` |
| `__u16` | `pkey_index` | `-` |
| `__u16` | `alt_pkey_index` | `-` |
| `__u8` | `qp_state` | `-` |
| `__u8` | `cur_qp_state` | `-` |
| `__u8` | `path_mtu` | `-` |
| `__u8` | `path_mig_state` | `-` |
| `__u8` | `sq_draining` | `-` |
| `__u8` | `max_rd_atomic` | `-` |
| `__u8` | `max_dest_rd_atomic` | `-` |
| `__u8` | `min_rnr_timer` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `timeout` | `-` |
| `__u8` | `retry_cnt` | `-` |
| `__u8` | `rnr_retry` | `-` |
| `__u8` | `alt_port_num` | `-` |
| `__u8` | `alt_timeout` | `-` |
| `__u8` | `sq_sig_all` | `-` |
| `__u8` | `reserved` | `5` |

### `struct ib_uverbs_modify_qp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `qp_handle` | `-` |
| `__u32` | `attr_mask` | `-` |
| `__u32` | `qkey` | `-` |
| `__u32` | `rq_psn` | `-` |
| `__u32` | `sq_psn` | `-` |
| `__u32` | `dest_qp_num` | `-` |
| `__u32` | `qp_access_flags` | `-` |
| `__u16` | `pkey_index` | `-` |
| `__u16` | `alt_pkey_index` | `-` |
| `__u8` | `qp_state` | `-` |
| `__u8` | `cur_qp_state` | `-` |
| `__u8` | `path_mtu` | `-` |
| `__u8` | `path_mig_state` | `-` |
| `__u8` | `en_sqd_async_notify` | `-` |
| `__u8` | `max_rd_atomic` | `-` |
| `__u8` | `max_dest_rd_atomic` | `-` |
| `__u8` | `min_rnr_timer` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `timeout` | `-` |
| `__u8` | `retry_cnt` | `-` |
| `__u8` | `rnr_retry` | `-` |
| `__u8` | `alt_port_num` | `-` |
| `__u8` | `alt_timeout` | `-` |
| `__u8` | `reserved` | `2` |
| `__aligned_u64` | `driver_data` | `0` |

### `struct ib_uverbs_ex_modify_qp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `rate_limit` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_ex_modify_qp_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |

### `struct ib_uverbs_destroy_qp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `qp_handle` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_destroy_qp_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `events_reported` | `-` |

### `struct ib_uverbs_sge`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `addr` | `-` |
| `__u32` | `length` | `-` |
| `__u32` | `lkey` | `-` |

### `struct ib_uverbs_send_wr`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `wr_id` | `-` |
| `__u32` | `num_sge` | `-` |
| `__u32` | `opcode` | `-` |
| `__u32` | `send_flags` | `-` |
| `__be32` | `imm_data` | `-` |
| `__u32` | `invalidate_rkey` | `-` |
| `__aligned_u64` | `remote_addr` | `-` |
| `__u32` | `rkey` | `-` |
| `__u32` | `reserved` | `-` |
| `__aligned_u64` | `remote_addr` | `-` |
| `__aligned_u64` | `compare_add` | `-` |
| `__aligned_u64` | `swap` | `-` |
| `__u32` | `rkey` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `ah` | `-` |
| `__u32` | `remote_qpn` | `-` |
| `__u32` | `remote_qkey` | `-` |
| `__u32` | `reserved` | `-` |

### `struct anonymous_63`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `remote_addr` | `-` |
| `__u32` | `rkey` | `-` |
| `__u32` | `reserved` | `-` |

### `struct anonymous_64`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `remote_addr` | `-` |
| `__aligned_u64` | `compare_add` | `-` |
| `__aligned_u64` | `swap` | `-` |
| `__u32` | `rkey` | `-` |
| `__u32` | `reserved` | `-` |

### `struct anonymous_65`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ah` | `-` |
| `__u32` | `remote_qpn` | `-` |
| `__u32` | `remote_qkey` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_post_send`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `qp_handle` | `-` |
| `__u32` | `wr_count` | `-` |
| `__u32` | `sge_count` | `-` |
| `__u32` | `wqe_size` | `-` |

### `struct ib_uverbs_post_send_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bad_wr` | `-` |

### `struct ib_uverbs_recv_wr`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `wr_id` | `-` |
| `__u32` | `num_sge` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_post_recv`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `qp_handle` | `-` |
| `__u32` | `wr_count` | `-` |
| `__u32` | `sge_count` | `-` |
| `__u32` | `wqe_size` | `-` |

### `struct ib_uverbs_post_recv_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bad_wr` | `-` |

### `struct ib_uverbs_post_srq_recv`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `srq_handle` | `-` |
| `__u32` | `wr_count` | `-` |
| `__u32` | `sge_count` | `-` |
| `__u32` | `wqe_size` | `-` |

### `struct ib_uverbs_post_srq_recv_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bad_wr` | `-` |

### `struct ib_uverbs_create_ah`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_create_ah_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ah_handle` | `-` |

### `struct ib_uverbs_destroy_ah`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ah_handle` | `-` |

### `struct ib_uverbs_attach_mcast`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `gid` | `16` |
| `__u32` | `qp_handle` | `-` |
| `__u16` | `mlid` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_detach_mcast`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `gid` | `16` |
| `__u32` | `qp_handle` | `-` |
| `__u16` | `mlid` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |
| `__aligned_u64` | `flow_spec_data` | `0` |

### `struct ib_uverbs_flow_eth_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `dst_mac` | `6` |
| `__u8` | `src_mac` | `6` |
| `__be16` | `ether_type` | `-` |
| `__be16` | `vlan_tag` | `-` |

### `struct ib_uverbs_flow_spec_eth`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_81`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_ipv4_filter`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `src_ip` | `-` |
| `__be32` | `dst_ip` | `-` |
| `__u8` | `proto` | `-` |
| `__u8` | `tos` | `-` |
| `__u8` | `ttl` | `-` |
| `__u8` | `flags` | `-` |

### `struct ib_uverbs_flow_spec_ipv4`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_84`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_tcp_udp_filter`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `dst_port` | `-` |
| `__be16` | `src_port` | `-` |

### `struct ib_uverbs_flow_spec_tcp_udp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_87`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_ipv6_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `src_ip` | `16` |
| `__u8` | `dst_ip` | `16` |
| `__be32` | `flow_label` | `-` |
| `__u8` | `next_hdr` | `-` |
| `__u8` | `traffic_class` | `-` |
| `__u8` | `hop_limit` | `-` |
| `__u8` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_ipv6`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_90`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_action_tag`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |
| `__u32` | `tag_id` | `-` |
| `__u32` | `reserved1` | `-` |

### `struct anonymous_92`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_action_drop`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_94`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_action_handle`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `reserved1` | `-` |

### `struct anonymous_96`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_action_count`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |
| `__u32` | `handle` | `-` |
| `__u32` | `reserved1` | `-` |

### `struct anonymous_98`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_tunnel_filter`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `tunnel_id` | `-` |

### `struct ib_uverbs_flow_spec_tunnel`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_101`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_spec_esp_filter`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `spi` | `-` |
| `__u32` | `seq` | `-` |

### `struct ib_uverbs_flow_spec_esp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_104`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_gre_filter`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `c_ks_res0_ver` | `-` |
| `__be16` | `protocol` | `-` |
| `__be32` | `key` | `-` |

### `struct ib_uverbs_flow_spec_gre`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_107`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_mpls_filter`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `label` | `-` |

### `struct ib_uverbs_flow_spec_mpls`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct anonymous_110`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct ib_uverbs_flow_attr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u16` | `size` | `-` |
| `__u16` | `priority` | `-` |
| `__u8` | `num_of_specs` | `-` |
| `__u8` | `reserved` | `2` |
| `__u8` | `port` | `-` |
| `__u32` | `flags` | `-` |

### `struct ib_uverbs_create_flow`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `qp_handle` | `-` |

### `struct ib_uverbs_create_flow_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `flow_handle` | `-` |

### `struct ib_uverbs_destroy_flow`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `flow_handle` | `-` |

### `struct ib_uverbs_create_srq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `max_wr` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `srq_limit` | `-` |

### `struct ib_uverbs_create_xsrq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `srq_type` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `max_wr` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `srq_limit` | `-` |
| `__u32` | `max_num_tags` | `-` |
| `__u32` | `xrcd_handle` | `-` |
| `__u32` | `cq_handle` | `-` |

### `struct ib_uverbs_create_srq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `srq_handle` | `-` |
| `__u32` | `max_wr` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `srqn` | `-` |

### `struct ib_uverbs_modify_srq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `srq_handle` | `-` |
| `__u32` | `attr_mask` | `-` |
| `__u32` | `max_wr` | `-` |
| `__u32` | `srq_limit` | `-` |

### `struct ib_uverbs_query_srq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `srq_handle` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_query_srq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `max_wr` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `srq_limit` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_destroy_srq`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `srq_handle` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_destroy_srq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `events_reported` | `-` |

### `struct ib_uverbs_ex_create_wq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `wq_type` | `-` |
| `__aligned_u64` | `user_handle` | `-` |
| `__u32` | `pd_handle` | `-` |
| `__u32` | `cq_handle` | `-` |
| `__u32` | `max_wr` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `create_flags` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_ex_create_wq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |
| `__u32` | `wq_handle` | `-` |
| `__u32` | `max_wr` | `-` |
| `__u32` | `max_sge` | `-` |
| `__u32` | `wqn` | `-` |

### `struct ib_uverbs_ex_destroy_wq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `wq_handle` | `-` |

### `struct ib_uverbs_ex_destroy_wq_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |
| `__u32` | `events_reported` | `-` |
| `__u32` | `reserved` | `-` |

### `struct ib_uverbs_ex_modify_wq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `attr_mask` | `-` |
| `__u32` | `wq_handle` | `-` |
| `__u32` | `wq_state` | `-` |
| `__u32` | `curr_wq_state` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `flags_mask` | `-` |

### `struct ib_uverbs_ex_create_rwq_ind_table`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `log_ind_tbl_size` | `-` |

### `struct ib_uverbs_ex_create_rwq_ind_table_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `response_length` | `-` |
| `__u32` | `ind_tbl_handle` | `-` |
| `__u32` | `ind_tbl_num` | `-` |

### `struct ib_uverbs_ex_destroy_rwq_ind_table`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `comp_mask` | `-` |
| `__u32` | `ind_tbl_handle` | `-` |

### `struct ib_uverbs_cq_moderation`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `cq_count` | `-` |
| `__u16` | `cq_period` | `-` |

### `struct ib_uverbs_ex_modify_cq`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cq_handle` | `-` |
| `__u32` | `attr_mask` | `-` |
| `__u32` | `reserved` | `-` |