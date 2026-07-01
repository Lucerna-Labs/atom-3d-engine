# rdma_user_cm.h

**Source:** `rdma_user_cm.h`


## Includes

- `linux/types.h`
- `linux/socket.h`
- `linux/in6.h`
- `rdma/ib_user_verbs.h`
- `rdma/ib_user_sa.h`

## Defines (3 total)


### RDMA_MAX (1)

| Name | Value | Comment |
|------|-------|---------|
| `RDMA_MAX_PRIVATE_DATA` | `256` |  |

### RDMA_USER (2)

| Name | Value | Comment |
|------|-------|---------|
| `RDMA_USER_CM_ABI_VERSION` | `4` |  |
| `RDMA_USER_CM_IB_SERVICE_NAME_SIZE` | `64` |  |

## Structs (35)


### `struct rdma_ucm_cmd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cmd` | `-` |
| `__u16` | `in` | `-` |
| `__u16` | `out` | `-` |

### `struct rdma_ucm_create_id`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `uid` | `-` |
| `__aligned_u64` | `response` | `-` |
| `__u16` | `ps` | `-` |
| `__u8` | `qp_type` | `-` |
| `__u8` | `reserved` | `5` |

### `struct rdma_ucm_create_id_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |

### `struct rdma_ucm_destroy_id`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_destroy_id_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `events_reported` | `-` |

### `struct rdma_ucm_bind_ip`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `id` | `-` |

### `struct rdma_ucm_bind`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u16` | `addr_size` | `-` |
| `__u16` | `reserved` | `-` |

### `struct rdma_ucm_resolve_ip`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `timeout_ms` | `-` |

### `struct rdma_ucm_resolve_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `timeout_ms` | `-` |
| `__u16` | `src_size` | `-` |
| `__u16` | `dst_size` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_resolve_route`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `timeout_ms` | `-` |

### `struct rdma_ucm_query`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `option` | `-` |

### `struct rdma_ucm_query_route_resp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `node_guid` | `-` |
| `__u32` | `num_paths` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `reserved` | `3` |
| `__u32` | `ibdev_index` | `-` |
| `__u32` | `reserved1` | `-` |

### `struct rdma_ucm_query_addr_resp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `node_guid` | `-` |
| `__u8` | `port_num` | `-` |
| `__u8` | `reserved` | `-` |
| `__u16` | `pkey` | `-` |
| `__u16` | `src_size` | `-` |
| `__u16` | `dst_size` | `-` |
| `__u32` | `ibdev_index` | `-` |
| `__u32` | `reserved1` | `-` |

### `struct rdma_ucm_query_path_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_paths` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_query_ib_service_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `num_service_recs` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_conn_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `qp_num` | `-` |
| `__u32` | `qkey` | `-` |
| `__u8` | `private_data` | `RDMA_MAX_PRIVATE_DATA` |
| `__u8` | `private_data_len` | `-` |
| `__u8` | `srq` | `-` |
| `__u8` | `responder_resources` | `-` |
| `__u8` | `initiator_depth` | `-` |
| `__u8` | `flow_control` | `-` |
| `__u8` | `retry_count` | `-` |
| `__u8` | `rnr_retry_count` | `-` |
| `__u8` | `valid` | `-` |

### `struct rdma_ucm_ud_param`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `qp_num` | `-` |
| `__u32` | `qkey` | `-` |
| `__u8` | `private_data` | `RDMA_MAX_PRIVATE_DATA` |
| `__u8` | `private_data_len` | `-` |
| `__u8` | `reserved` | `7` |

### `struct rdma_ucm_ece`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `vendor_id` | `-` |
| `__u32` | `attr_mod` | `-` |

### `struct rdma_ucm_connect`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_listen`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `backlog` | `-` |

### `struct rdma_ucm_accept`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `uid` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_reject`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u8` | `private_data_len` | `-` |
| `__u8` | `reason` | `-` |
| `__u8` | `reserved` | `2` |
| `__u8` | `private_data` | `RDMA_MAX_PRIVATE_DATA` |

### `struct rdma_ucm_disconnect`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |

### `struct rdma_ucm_init_qp_attr`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `qp_state` | `-` |

### `struct rdma_ucm_notify`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `event` | `-` |

### `struct rdma_ucm_join_ip_mcast`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `uid` | `-` |
| `__u32` | `id` | `-` |

### `struct rdma_ucm_join_mcast`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__aligned_u64` | `uid` | `-` |
| `__u32` | `id` | `-` |
| `__u16` | `addr_size` | `-` |
| `__u16` | `join_flags` | `-` |

### `struct rdma_ucm_get_event`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |

### `struct rdma_ucm_event_resp`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `uid` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `event` | `-` |
| `__u32` | `status` | `-` |
| `__u32` | `arg32` | `2` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_set_option`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `optval` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `level` | `-` |
| `__u32` | `optname` | `-` |
| `__u32` | `optlen` | `-` |

### `struct rdma_ucm_migrate_id`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `response` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `fd` | `-` |

### `struct rdma_ucm_migrate_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `events_reported` | `-` |

### `struct rdma_ucm_ib_service`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `service_id` | `-` |
| `__u8` | `service_name` | `RDMA_USER_CM_IB_SERVICE_NAME_SIZE` |
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_resolve_ib_service`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `reserved` | `-` |

### `struct rdma_ucm_write_cm_event`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `reserved` | `-` |
| `__u32` | `event` | `-` |
| `__u32` | `status` | `-` |
| `__u64` | `arg` | `-` |