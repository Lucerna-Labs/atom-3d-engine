# bpf.h

**Source:** `bpf.h`


## Includes

- `linux/types.h`
- `linux/bpf_common.h`

## Defines (65 total)


### BPF_BUILD (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_BUILD_ID_SIZE` | `20` |  |

### BPF_F (22)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_F_ALLOW_OVERRIDE` | `(1U << 0)` |  |
| `BPF_F_ALLOW_MULTI` | `(1U << 1)` |  |
| `BPF_F_REPLACE` | `(1U << 2)` |  |
| `BPF_F_BEFORE` | `(1U << 3)` |  |
| `BPF_F_AFTER` | `(1U << 4)` |  |
| `BPF_F_ID` | `(1U << 5)` |  |
| `BPF_F_PREORDER` | `(1U << 6)` |  |
| `BPF_F_LINK` | `BPF_F_LINK` | 1 << 13 |
| `BPF_F_STRICT_ALIGNMENT` | `(1U << 0)` |  |
| `BPF_F_ANY_ALIGNMENT` | `(1U << 1)` |  |
| `BPF_F_TEST_RND_HI32` | `(1U << 2)` |  |
| `BPF_F_TEST_STATE_FREQ` | `(1U << 3)` |  |
| `BPF_F_SLEEPABLE` | `(1U << 4)` |  |
| `BPF_F_XDP_HAS_FRAGS` | `(1U << 5)` |  |
| `BPF_F_XDP_DEV_BOUND_ONLY` | `(1U << 6)` |  |
| `BPF_F_TEST_REG_INVARIANTS` | `(1U << 7)` |  |
| `BPF_F_NETFILTER_IP_DEFRAG` | `(1U << 0)` |  |
| `BPF_F_QUERY_EFFECTIVE` | `(1U << 0)` |  |
| `BPF_F_TEST_RUN_ON_CPU` | `(1U << 0)` |  |
| `BPF_F_TEST_XDP_LIVE_FRAMES` | `(1U << 1)` |  |
| `BPF_F_TEST_SKB_CHECKSUM_COMPLETE` | `(1U << 2)` |  |
| `BPF_F_REDIRECT_FLAGS` | `(BPF_F_INGRESS \| BPF_F_BROADCAST \| BPF_F_EXCLUDE_INGRESS)` |  |

### BPF_FROM (2)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_FROM_LE` | `BPF_TO_LE` |  |
| `BPF_FROM_BE` | `BPF_TO_BE` |  |

### BPF_LOAD (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_LOAD_ACQ` | `0x100` | load-acquire |

### BPF_OBJ (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_OBJ_NAME_LEN` | `16U` |  |

### BPF_PSEUDO (8)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_PSEUDO_MAP_FD` | `1` |  |
| `BPF_PSEUDO_MAP_IDX` | `5` |  |
| `BPF_PSEUDO_MAP_VALUE` | `2` |  |
| `BPF_PSEUDO_MAP_IDX_VALUE` | `6` |  |
| `BPF_PSEUDO_BTF_ID` | `3` |  |
| `BPF_PSEUDO_FUNC` | `4` |  |
| `BPF_PSEUDO_CALL` | `1` |  |
| `BPF_PSEUDO_KFUNC_CALL` | `2` |  |

### BPF_STORE (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_STORE_REL` | `0x110` | store-release |

### BPF_TAG (1)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_TAG_SIZE` | `8` |  |

### BPF_TO (2)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_TO_LE` | `0x00` | convert to little-endian |
| `BPF_TO_BE` | `0x08` | convert to big-endian |

### MAX_BPF (3)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_BPF_REG` | `__MAX_BPF_REG` |  |
| `MAX_BPF_ATTACH_TYPE` | `__MAX_BPF_ATTACH_TYPE` |  |
| `MAX_BPF_LINK_TYPE` | `__MAX_BPF_LINK_TYPE` |  |

### UNCATEGORIZED (22)

| Name | Value | Comment |
|------|-------|---------|
| `BPF_JMP32` | `0x06` | jmp mode in word width |
| `BPF_ALU64` | `0x07` | alu mode in double word width |
| `BPF_DW` | `0x18` | double word (64-bit) |
| `BPF_MEMSX` | `0x80` | load with sign extension |
| `BPF_ATOMIC` | `0xc0` | atomic memory ops - op type in immediate |
| `BPF_XADD` | `0xc0` | exclusive add - legacy name |
| `BPF_MOV` | `0xb0` | mov reg to reg |
| `BPF_ARSH` | `0xc0` | sign extending arithmetic shift right |
| `BPF_END` | `0xd0` | flags for endianness conversion: |
| `BPF_JNE` | `0x50` | jump != |
| `BPF_JLT` | `0xa0` | LT is unsigned, '<' |
| `BPF_JLE` | `0xb0` | LE is unsigned, '<=' |
| `BPF_JSGT` | `0x60` | SGT is signed '>', GT in x86 |
| `BPF_JSGE` | `0x70` | SGE is signed '>=', GE in x86 |
| `BPF_JSLT` | `0xc0` | SLT is signed, '<' |
| `BPF_JSLE` | `0xd0` | SLE is signed, '<=' |
| `BPF_JCOND` | `0xe0` | conditional pseudo jumps: may_goto, goto_or_nop |
| `BPF_CALL` | `0x80` | function call |
| `BPF_EXIT` | `0x90` | function return |
| `BPF_FETCH` | `0x01` | not an opcode on its own, used to build others |
| `BPF_XCHG` | `(0xe0 \| BPF_FETCH)` | atomic exchange |
| `BPF_CMPXCHG` | `(0xf0 \| BPF_FETCH)` | atomic compare-and-write |

### XDP_PACKET (1)

| Name | Value | Comment |
|------|-------|---------|
| `XDP_PACKET_HEADROOM` | `256` |  |

## Structs (115)


### `struct bpf_insn`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `code` | `-` |
| `__s16` | `off` | `-` |
| `__s32` | `imm` | `-` |

### `struct bpf_lpm_trie_key`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prefixlen` | `-` |
| `__u8` | `data` | `0` |

### `struct bpf_lpm_trie_key_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prefixlen` | `-` |

### `struct bpf_lpm_trie_key_u8`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prefixlen` | `-` |

### `struct bpf_cgroup_storage_key`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cgroup_inode_id` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct anonymous_5`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_fd` | `-` |

### `struct anonymous_6`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `cgroup_fd` | `-` |
| `__u64` | `cgroup_id` | `-` |

### `struct anonymous_7`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tid` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `pid_fd` | `-` |

### `struct bpf_stack_build_id`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `status` | `-` |
| `__u64` | `offset` | `-` |
| `__u64` | `ip` | `-` |

### `struct anonymous_9`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_type` | `-` |
| `__u32` | `key_size` | `-` |
| `__u32` | `value_size` | `-` |
| `__u32` | `max_entries` | `-` |
| `__u32` | `map_flags` | `-` |
| `__u32` | `inner_map_fd` | `-` |
| `__u32` | `numa_node` | `-` |
| `char` | `map_name` | `BPF_OBJ_NAME_LEN` |
| `__u32` | `map_ifindex` | `-` |
| `__u32` | `btf_fd` | `-` |
| `__u32` | `btf_key_type_id` | `-` |
| `__u32` | `btf_value_type_id` | `-` |
| `__u32` | `btf_vmlinux_value_type_id` | `-` |
| `__u64` | `map_extra` | `-` |
| `__s32` | `value_type_btf_obj_fd` | `-` |
| `__s32` | `map_token_fd` | `-` |
| `__aligned_u64` | `excl_prog_hash` | `-` |
| `__u32` | `excl_prog_hash_size` | `-` |

### `struct anonymous_10`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_fd` | `-` |
| `__aligned_u64` | `key` | `-` |
| `__aligned_u64` | `value` | `-` |
| `__aligned_u64` | `next_key` | `-` |
| `__u64` | `flags` | `-` |

### `struct anonymous_11`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `in_batch` | `-` |
| `__aligned_u64` | `out_batch` | `-` |
| `__aligned_u64` | `keys` | `-` |
| `__aligned_u64` | `values` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `map_fd` | `-` |
| `__u64` | `elem_flags` | `-` |
| `__u64` | `flags` | `-` |

### `struct anonymous_12`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prog_type` | `-` |
| `__u32` | `insn_cnt` | `-` |
| `__aligned_u64` | `insns` | `-` |
| `__aligned_u64` | `license` | `-` |
| `__u32` | `log_level` | `-` |
| `__u32` | `log_size` | `-` |
| `__aligned_u64` | `log_buf` | `-` |
| `__u32` | `kern_version` | `-` |
| `__u32` | `prog_flags` | `-` |
| `char` | `prog_name` | `BPF_OBJ_NAME_LEN` |
| `__u32` | `prog_ifindex` | `-` |
| `__u32` | `expected_attach_type` | `-` |
| `__u32` | `prog_btf_fd` | `-` |
| `__u32` | `func_info_rec_size` | `-` |
| `__aligned_u64` | `func_info` | `-` |
| `__u32` | `func_info_cnt` | `-` |
| `__u32` | `line_info_rec_size` | `-` |
| `__aligned_u64` | `line_info` | `-` |
| `__u32` | `line_info_cnt` | `-` |
| `__u32` | `attach_btf_id` | `-` |
| `__u32` | `attach_prog_fd` | `-` |
| `__u32` | `attach_btf_obj_fd` | `-` |
| `__u32` | `core_relo_cnt` | `-` |
| `__aligned_u64` | `fd_array` | `-` |
| `__aligned_u64` | `core_relos` | `-` |
| `__u32` | `core_relo_rec_size` | `-` |
| `__u32` | `log_true_size` | `-` |
| `__s32` | `prog_token_fd` | `-` |
| `__u32` | `fd_array_cnt` | `-` |
| `__aligned_u64` | `signature` | `-` |
| `__u32` | `signature_size` | `-` |
| `__s32` | `keyring_id` | `-` |

### `struct anonymous_13`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `pathname` | `-` |
| `__u32` | `bpf_fd` | `-` |
| `__u32` | `file_flags` | `-` |
| `__s32` | `path_fd` | `-` |

### `struct anonymous_14`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `target_fd` | `-` |
| `__u32` | `target_ifindex` | `-` |
| `__u32` | `attach_bpf_fd` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `attach_flags` | `-` |
| `__u32` | `replace_bpf_fd` | `-` |
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |

### `struct anonymous_15`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prog_fd` | `-` |
| `__u32` | `retval` | `-` |
| `__u32` | `data_size_in` | `-` |
| `__u32` | `data_size_out` | `-` |
| `__aligned_u64` | `data_in` | `-` |
| `__aligned_u64` | `data_out` | `-` |
| `__u32` | `repeat` | `-` |
| `__u32` | `duration` | `-` |
| `__u32` | `ctx_size_in` | `-` |
| `__u32` | `ctx_size_out` | `-` |
| `__aligned_u64` | `ctx_in` | `-` |
| `__aligned_u64` | `ctx_out` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `cpu` | `-` |
| `__u32` | `batch_size` | `-` |

### `struct anonymous_16`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `start_id` | `-` |
| `__u32` | `prog_id` | `-` |
| `__u32` | `map_id` | `-` |
| `__u32` | `btf_id` | `-` |
| `__u32` | `link_id` | `-` |
| `__u32` | `next_id` | `-` |
| `__u32` | `open_flags` | `-` |
| `__s32` | `fd_by_id_token_fd` | `-` |

### `struct anonymous_17`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bpf_fd` | `-` |
| `__u32` | `info_len` | `-` |
| `__aligned_u64` | `info` | `-` |

### `struct anonymous_18`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `target_fd` | `-` |
| `__u32` | `target_ifindex` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `query_flags` | `-` |
| `__u32` | `attach_flags` | `-` |
| `__aligned_u64` | `prog_ids` | `-` |
| `__u32` | `prog_cnt` | `-` |
| `__u32` | `count` | `-` |
| `__aligned_u64` | `prog_attach_flags` | `-` |
| `__aligned_u64` | `link_ids` | `-` |
| `__aligned_u64` | `link_attach_flags` | `-` |
| `__u64` | `revision` | `-` |

### `struct anonymous_19`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `name` | `-` |
| `__u32` | `prog_fd` | `-` |
| `__aligned_u64` | `cookie` | `-` |

### `struct anonymous_20`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `btf` | `-` |
| `__aligned_u64` | `btf_log_buf` | `-` |
| `__u32` | `btf_size` | `-` |
| `__u32` | `btf_log_size` | `-` |
| `__u32` | `btf_log_level` | `-` |
| `__u32` | `btf_log_true_size` | `-` |
| `__u32` | `btf_flags` | `-` |
| `__s32` | `btf_token_fd` | `-` |

### `struct anonymous_21`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pid` | `-` |
| `__u32` | `fd` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `buf_len` | `-` |
| `__aligned_u64` | `buf` | `-` |
| `__u32` | `prog_id` | `-` |
| `__u32` | `fd_type` | `-` |
| `__u64` | `probe_offset` | `-` |
| `__u64` | `probe_addr` | `-` |

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prog_fd` | `-` |
| `__u32` | `map_fd` | `-` |
| `__u32` | `target_fd` | `-` |
| `__u32` | `target_ifindex` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `target_btf_id` | `-` |
| `__aligned_u64` | `iter_info` | `-` |
| `__u32` | `iter_info_len` | `-` |
| `__u64` | `bpf_cookie` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `cnt` | `-` |
| `__aligned_u64` | `syms` | `-` |
| `__aligned_u64` | `addrs` | `-` |
| `__aligned_u64` | `cookies` | `-` |
| `__u32` | `target_btf_id` | `-` |
| `__u64` | `cookie` | `-` |
| `__u32` | `pf` | `-` |
| `__u32` | `hooknum` | `-` |
| `__s32` | `priority` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |
| `__aligned_u64` | `path` | `-` |
| `__aligned_u64` | `offsets` | `-` |
| `__aligned_u64` | `ref_ctr_offsets` | `-` |
| `__aligned_u64` | `cookies` | `-` |
| `__u32` | `cnt` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |

### `struct anonymous_23`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `iter_info` | `-` |
| `__u32` | `iter_info_len` | `-` |

### `struct anonymous_24`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `bpf_cookie` | `-` |

### `struct anonymous_25`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `cnt` | `-` |
| `__aligned_u64` | `syms` | `-` |
| `__aligned_u64` | `addrs` | `-` |
| `__aligned_u64` | `cookies` | `-` |

### `struct anonymous_26`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `target_btf_id` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_27`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pf` | `-` |
| `__u32` | `hooknum` | `-` |
| `__s32` | `priority` | `-` |
| `__u32` | `flags` | `-` |

### `struct anonymous_28`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |

### `struct anonymous_29`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `path` | `-` |
| `__aligned_u64` | `offsets` | `-` |
| `__aligned_u64` | `ref_ctr_offsets` | `-` |
| `__aligned_u64` | `cookies` | `-` |
| `__u32` | `cnt` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pid` | `-` |

### `struct anonymous_30`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |

### `struct anonymous_31`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `relative_fd` | `-` |
| `__u32` | `relative_id` | `-` |
| `__u64` | `expected_revision` | `-` |

### `struct anonymous_32`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `link_fd` | `-` |
| `__u32` | `new_prog_fd` | `-` |
| `__u32` | `new_map_fd` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `old_prog_fd` | `-` |
| `__u32` | `old_map_fd` | `-` |

### `struct anonymous_33`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `link_fd` | `-` |

### `struct anonymous_34`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |

### `struct anonymous_35`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `link_fd` | `-` |
| `__u32` | `flags` | `-` |

### `struct anonymous_36`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `prog_fd` | `-` |
| `__u32` | `map_fd` | `-` |
| `__u32` | `flags` | `-` |

### `struct anonymous_37`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `bpffs_fd` | `-` |

### `struct anonymous_38`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `stream_buf` | `-` |
| `__u32` | `stream_buf_len` | `-` |
| `__u32` | `stream_id` | `-` |
| `__u32` | `prog_fd` | `-` |

### `struct anonymous_39`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_fd` | `-` |
| `__u32` | `prog_fd` | `-` |
| `__u32` | `flags` | `-` |

### `struct __sk_buff`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `len` | `-` |
| `__u32` | `pkt_type` | `-` |
| `__u32` | `mark` | `-` |
| `__u32` | `queue_mapping` | `-` |
| `__u32` | `protocol` | `-` |
| `__u32` | `vlan_present` | `-` |
| `__u32` | `vlan_tci` | `-` |
| `__u32` | `vlan_proto` | `-` |
| `__u32` | `priority` | `-` |
| `__u32` | `ingress_ifindex` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `tc_index` | `-` |
| `__u32` | `cb` | `5` |
| `__u32` | `hash` | `-` |
| `__u32` | `tc_classid` | `-` |
| `__u32` | `data` | `-` |
| `__u32` | `data_end` | `-` |
| `__u32` | `napi_id` | `-` |
| `__u32` | `family` | `-` |
| `__u32` | `remote_ip4` | `-` |
| `__u32` | `local_ip4` | `-` |
| `__u32` | `remote_ip6` | `4` |
| `__u32` | `local_ip6` | `4` |
| `__u32` | `remote_port` | `-` |
| `__u32` | `local_port` | `-` |
| `__u32` | `data_meta` | `-` |
| `__u64` | `tstamp` | `-` |
| `__u32` | `wire_len` | `-` |
| `__u32` | `gso_segs` | `-` |
| `__u32` | `gso_size` | `-` |
| `__u8` | `tstamp_type` | `-` |
| `__u64` | `hwtstamp` | `-` |

### `struct bpf_tunnel_key`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tunnel_id` | `-` |
| `__u32` | `remote_ipv4` | `-` |
| `__u32` | `remote_ipv6` | `4` |
| `__u8` | `tunnel_tos` | `-` |
| `__u8` | `tunnel_ttl` | `-` |
| `__u16` | `tunnel_ext` | `-` |
| `__be16` | `tunnel_flags` | `-` |
| `__u32` | `tunnel_label` | `-` |
| `__u32` | `local_ipv4` | `-` |
| `__u32` | `local_ipv6` | `4` |

### `struct bpf_xfrm_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `reqid` | `-` |
| `__u32` | `spi` | `-` |
| `__u16` | `family` | `-` |
| `__u16` | `ext` | `-` |
| `__u32` | `remote_ipv4` | `-` |
| `__u32` | `remote_ipv6` | `4` |

### `struct bpf_sock`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `bound_dev_if` | `-` |
| `__u32` | `family` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `protocol` | `-` |
| `__u32` | `mark` | `-` |
| `__u32` | `priority` | `-` |
| `__u32` | `src_ip4` | `-` |
| `__u32` | `src_ip6` | `4` |
| `__u32` | `src_port` | `-` |
| `__be16` | `dst_port` | `-` |
| `__u32` | `dst_ip4` | `-` |
| `__u32` | `dst_ip6` | `4` |
| `__u32` | `state` | `-` |
| `__s32` | `rx_queue_mapping` | `-` |

### `struct bpf_tcp_sock`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `snd_cwnd` | `-` |
| `__u32` | `srtt_us` | `-` |
| `__u32` | `rtt_min` | `-` |
| `__u32` | `snd_ssthresh` | `-` |
| `__u32` | `rcv_nxt` | `-` |
| `__u32` | `snd_nxt` | `-` |
| `__u32` | `snd_una` | `-` |
| `__u32` | `mss_cache` | `-` |
| `__u32` | `ecn_flags` | `-` |
| `__u32` | `rate_delivered` | `-` |
| `__u32` | `rate_interval_us` | `-` |
| `__u32` | `packets_out` | `-` |
| `__u32` | `retrans_out` | `-` |
| `__u32` | `total_retrans` | `-` |
| `__u32` | `segs_in` | `-` |
| `__u32` | `data_segs_in` | `-` |
| `__u32` | `segs_out` | `-` |
| `__u32` | `data_segs_out` | `-` |
| `__u32` | `lost_out` | `-` |
| `__u32` | `sacked_out` | `-` |
| `__u64` | `bytes_received` | `-` |
| `__u64` | `bytes_acked` | `-` |
| `__u32` | `dsack_dups` | `-` |
| `__u32` | `delivered` | `-` |
| `__u32` | `delivered_ce` | `-` |
| `__u32` | `icsk_retransmits` | `-` |

### `struct bpf_sock_tuple`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `saddr` | `-` |
| `__be32` | `daddr` | `-` |
| `__be16` | `sport` | `-` |
| `__be16` | `dport` | `-` |
| `__be32` | `saddr` | `4` |
| `__be32` | `daddr` | `4` |
| `__be16` | `sport` | `-` |
| `__be16` | `dport` | `-` |

### `struct anonymous_46`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `saddr` | `-` |
| `__be32` | `daddr` | `-` |
| `__be16` | `sport` | `-` |
| `__be16` | `dport` | `-` |

### `struct anonymous_47`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `saddr` | `4` |
| `__be32` | `daddr` | `4` |
| `__be16` | `sport` | `-` |
| `__be16` | `dport` | `-` |

### `struct bpf_xdp_sock`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `queue_id` | `-` |

### `struct xdp_md`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `data` | `-` |
| `__u32` | `data_end` | `-` |
| `__u32` | `data_meta` | `-` |
| `__u32` | `ingress_ifindex` | `-` |
| `__u32` | `rx_queue_index` | `-` |
| `__u32` | `egress_ifindex` | `-` |

### `struct bpf_devmap_val`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifindex` | `-` |
| `int` | `fd` | `-` |
| `__u32` | `id` | `-` |

### `struct bpf_cpumap_val`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `qsize` | `-` |
| `int` | `fd` | `-` |
| `__u32` | `id` | `-` |

### `struct sk_msg_md`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `family` | `-` |
| `__u32` | `remote_ip4` | `-` |
| `__u32` | `local_ip4` | `-` |
| `__u32` | `remote_ip6` | `4` |
| `__u32` | `local_ip6` | `4` |
| `__u32` | `remote_port` | `-` |
| `__u32` | `local_port` | `-` |
| `__u32` | `size` | `-` |

### `struct sk_reuseport_md`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `len` | `-` |
| `__u32` | `eth_protocol` | `-` |
| `__u32` | `ip_protocol` | `-` |
| `__u32` | `bind_inany` | `-` |
| `__u32` | `hash` | `-` |

### `struct bpf_prog_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `id` | `-` |
| `__u8` | `tag` | `BPF_TAG_SIZE` |
| `__u32` | `jited_prog_len` | `-` |
| `__u32` | `xlated_prog_len` | `-` |
| `__aligned_u64` | `jited_prog_insns` | `-` |
| `__aligned_u64` | `xlated_prog_insns` | `-` |
| `__u64` | `load_time` | `-` |
| `__u32` | `created_by_uid` | `-` |
| `__u32` | `nr_map_ids` | `-` |
| `__aligned_u64` | `map_ids` | `-` |
| `char` | `name` | `BPF_OBJ_NAME_LEN` |
| `__u32` | `ifindex` | `-` |
| `__u64` | `netns_dev` | `-` |
| `__u64` | `netns_ino` | `-` |
| `__u32` | `nr_jited_ksyms` | `-` |
| `__u32` | `nr_jited_func_lens` | `-` |
| `__aligned_u64` | `jited_ksyms` | `-` |
| `__aligned_u64` | `jited_func_lens` | `-` |
| `__u32` | `btf_id` | `-` |
| `__u32` | `func_info_rec_size` | `-` |
| `__aligned_u64` | `func_info` | `-` |
| `__u32` | `nr_func_info` | `-` |
| `__u32` | `nr_line_info` | `-` |
| `__aligned_u64` | `line_info` | `-` |
| `__aligned_u64` | `jited_line_info` | `-` |
| `__u32` | `nr_jited_line_info` | `-` |
| `__u32` | `line_info_rec_size` | `-` |
| `__u32` | `jited_line_info_rec_size` | `-` |
| `__u32` | `nr_prog_tags` | `-` |
| `__aligned_u64` | `prog_tags` | `-` |
| `__u64` | `run_time_ns` | `-` |
| `__u64` | `run_cnt` | `-` |
| `__u64` | `recursion_misses` | `-` |
| `__u32` | `verified_insns` | `-` |
| `__u32` | `attach_btf_obj_id` | `-` |
| `__u32` | `attach_btf_id` | `-` |

### `struct bpf_map_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `key_size` | `-` |
| `__u32` | `value_size` | `-` |
| `__u32` | `max_entries` | `-` |
| `__u32` | `map_flags` | `-` |
| `char` | `name` | `BPF_OBJ_NAME_LEN` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `btf_vmlinux_value_type_id` | `-` |
| `__u64` | `netns_dev` | `-` |
| `__u64` | `netns_ino` | `-` |
| `__u32` | `btf_id` | `-` |
| `__u32` | `btf_key_type_id` | `-` |
| `__u32` | `btf_value_type_id` | `-` |
| `__u32` | `btf_vmlinux_id` | `-` |
| `__u64` | `map_extra` | `-` |
| `__aligned_u64` | `hash` | `-` |
| `__u32` | `hash_size` | `-` |

### `struct bpf_btf_info`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `btf` | `-` |
| `__u32` | `btf_size` | `-` |
| `__u32` | `id` | `-` |
| `__aligned_u64` | `name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `kernel_btf` | `-` |

### `struct bpf_link_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `id` | `-` |
| `__u32` | `prog_id` | `-` |
| `__aligned_u64` | `tp_name` | `-` |
| `__u32` | `tp_name_len` | `-` |
| `__u64` | `cookie` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `target_obj_id` | `-` |
| `__u32` | `target_btf_id` | `-` |
| `__u64` | `cookie` | `-` |
| `__u64` | `cgroup_id` | `-` |
| `__u32` | `attach_type` | `-` |
| `__aligned_u64` | `target_name` | `-` |
| `__u32` | `target_name_len` | `-` |
| `__u32` | `map_id` | `-` |
| `__u64` | `cgroup_id` | `-` |
| `__u32` | `order` | `-` |
| `__u32` | `tid` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `netns_ino` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `map_id` | `-` |
| `__u32` | `pf` | `-` |
| `__u32` | `hooknum` | `-` |
| `__s32` | `priority` | `-` |
| `__u32` | `flags` | `-` |
| `__aligned_u64` | `addrs` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `missed` | `-` |
| `__aligned_u64` | `cookies` | `-` |
| `__aligned_u64` | `path` | `-` |
| `__aligned_u64` | `offsets` | `-` |
| `__aligned_u64` | `ref_ctr_offsets` | `-` |
| `__aligned_u64` | `cookies` | `-` |
| `__u32` | `path_size` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pid` | `-` |
| `__u32` | `type` | `-` |
| `__aligned_u64` | `file_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `offset` | `-` |
| `__u64` | `cookie` | `-` |
| `__u64` | `ref_ctr_offset` | `-` |
| `__aligned_u64` | `func_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `offset` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `missed` | `-` |
| `__u64` | `cookie` | `-` |
| `__aligned_u64` | `tp_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u64` | `cookie` | `-` |
| `__u64` | `config` | `-` |
| `__u32` | `type` | `-` |
| `__u64` | `cookie` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u32` | `attach_type` | `-` |
| `__u32` | `map_id` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct anonymous_58`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `tp_name` | `-` |
| `__u32` | `tp_name_len` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_59`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `attach_type` | `-` |
| `__u32` | `target_obj_id` | `-` |
| `__u32` | `target_btf_id` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_60`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cgroup_id` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct anonymous_61`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `target_name` | `-` |
| `__u32` | `target_name_len` | `-` |
| `__u32` | `map_id` | `-` |
| `__u64` | `cgroup_id` | `-` |
| `__u32` | `order` | `-` |
| `__u32` | `tid` | `-` |
| `__u32` | `pid` | `-` |

### `struct anonymous_62`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_id` | `-` |

### `struct anonymous_63`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cgroup_id` | `-` |
| `__u32` | `order` | `-` |

### `struct anonymous_64`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `tid` | `-` |
| `__u32` | `pid` | `-` |

### `struct anonymous_65`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `netns_ino` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct anonymous_66`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifindex` | `-` |

### `struct anonymous_67`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_id` | `-` |

### `struct anonymous_68`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pf` | `-` |
| `__u32` | `hooknum` | `-` |
| `__s32` | `priority` | `-` |
| `__u32` | `flags` | `-` |

### `struct anonymous_69`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `addrs` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `missed` | `-` |
| `__aligned_u64` | `cookies` | `-` |

### `struct anonymous_70`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `path` | `-` |
| `__aligned_u64` | `offsets` | `-` |
| `__aligned_u64` | `ref_ctr_offsets` | `-` |
| `__aligned_u64` | `cookies` | `-` |
| `__u32` | `path_size` | `-` |
| `__u32` | `count` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pid` | `-` |

### `struct anonymous_71`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__aligned_u64` | `file_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `offset` | `-` |
| `__u64` | `cookie` | `-` |
| `__u64` | `ref_ctr_offset` | `-` |
| `__aligned_u64` | `func_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `offset` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `missed` | `-` |
| `__u64` | `cookie` | `-` |
| `__aligned_u64` | `tp_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u64` | `cookie` | `-` |
| `__u64` | `config` | `-` |
| `__u32` | `type` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_72`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `file_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `offset` | `-` |
| `__u64` | `cookie` | `-` |
| `__u64` | `ref_ctr_offset` | `-` |

### `struct anonymous_73`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `func_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u32` | `offset` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `missed` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_74`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `tp_name` | `-` |
| `__u32` | `name_len` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_75`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `config` | `-` |
| `__u32` | `type` | `-` |
| `__u64` | `cookie` | `-` |

### `struct anonymous_76`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifindex` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct anonymous_77`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ifindex` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct anonymous_78`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `map_id` | `-` |
| `__u32` | `attach_type` | `-` |

### `struct bpf_token_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `allowed_cmds` | `-` |
| `__u64` | `allowed_maps` | `-` |
| `__u64` | `allowed_progs` | `-` |
| `__u64` | `allowed_attachs` | `-` |

### `struct bpf_sock_addr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `user_family` | `-` |
| `__u32` | `user_ip4` | `-` |
| `__u32` | `user_ip6` | `4` |
| `__u32` | `user_port` | `-` |
| `__u32` | `family` | `-` |
| `__u32` | `type` | `-` |
| `__u32` | `protocol` | `-` |
| `__u32` | `msg_src_ip4` | `-` |
| `__u32` | `msg_src_ip6` | `4` |

### `struct bpf_sock_ops`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |
| `__u32` | `args` | `4` |
| `__u32` | `reply` | `-` |
| `__u32` | `replylong` | `4` |
| `__u32` | `family` | `-` |
| `__u32` | `remote_ip4` | `-` |
| `__u32` | `local_ip4` | `-` |
| `__u32` | `remote_ip6` | `4` |
| `__u32` | `local_ip6` | `4` |
| `__u32` | `remote_port` | `-` |
| `__u32` | `local_port` | `-` |
| `__u32` | `is_fullsock` | `-` |
| `__u32` | `snd_cwnd` | `-` |
| `__u32` | `srtt_us` | `-` |
| `__u32` | `bpf_sock_ops_cb_flags` | `-` |
| `__u32` | `state` | `-` |
| `__u32` | `rtt_min` | `-` |
| `__u32` | `snd_ssthresh` | `-` |
| `__u32` | `rcv_nxt` | `-` |
| `__u32` | `snd_nxt` | `-` |
| `__u32` | `snd_una` | `-` |
| `__u32` | `mss_cache` | `-` |
| `__u32` | `ecn_flags` | `-` |
| `__u32` | `rate_delivered` | `-` |
| `__u32` | `rate_interval_us` | `-` |
| `__u32` | `packets_out` | `-` |
| `__u32` | `retrans_out` | `-` |
| `__u32` | `total_retrans` | `-` |
| `__u32` | `segs_in` | `-` |
| `__u32` | `data_segs_in` | `-` |
| `__u32` | `segs_out` | `-` |
| `__u32` | `data_segs_out` | `-` |
| `__u32` | `lost_out` | `-` |
| `__u32` | `sacked_out` | `-` |
| `__u32` | `sk_txhash` | `-` |
| `__u64` | `bytes_received` | `-` |
| `__u64` | `bytes_acked` | `-` |
| `__u32` | `skb_len` | `-` |
| `__u32` | `skb_tcp_flags` | `-` |
| `__u64` | `skb_hwtstamp` | `-` |

### `struct bpf_perf_event_value`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `counter` | `-` |
| `__u64` | `enabled` | `-` |
| `__u64` | `running` | `-` |

### `struct bpf_cgroup_dev_ctx`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `access_type` | `-` |
| `__u32` | `major` | `-` |
| `__u32` | `minor` | `-` |

### `struct bpf_raw_tracepoint_args`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `args` | `0` |

### `struct bpf_fib_lookup`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `family` | `-` |
| `__u8` | `l4_protocol` | `-` |
| `__be16` | `sport` | `-` |
| `__be16` | `dport` | `-` |
| `__u16` | `tot_len` | `-` |
| `__u16` | `mtu_result` | `-` |
| `__u32` | `ifindex` | `-` |
| `__u8` | `tos` | `-` |
| `__be32` | `flowinfo` | `-` |
| `__u32` | `rt_metric` | `-` |
| `__be32` | `ipv4_src` | `-` |
| `__u32` | `ipv6_src` | `4` |
| `__be32` | `ipv4_dst` | `-` |
| `__u32` | `ipv6_dst` | `4` |
| `__be16` | `h_vlan_proto` | `-` |
| `__be16` | `h_vlan_TCI` | `-` |
| `__u32` | `tbid` | `-` |
| `__u32` | `mark` | `-` |
| `__u8` | `smac` | `6` |
| `__u8` | `dmac` | `6` |

### `struct anonymous_86`

| Type | Field | Array |
|------|-------|-------|
| `__be16` | `h_vlan_proto` | `-` |
| `__be16` | `h_vlan_TCI` | `-` |

### `struct anonymous_87`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `mark` | `-` |

### `struct anonymous_88`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `smac` | `6` |
| `__u8` | `dmac` | `6` |

### `struct bpf_redir_neigh`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nh_family` | `-` |
| `__be32` | `ipv4_nh` | `-` |
| `__u32` | `ipv6_nh` | `4` |

### `struct bpf_flow_keys`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `nhoff` | `-` |
| `__u16` | `thoff` | `-` |
| `__u16` | `addr_proto` | `-` |
| `__u8` | `is_frag` | `-` |
| `__u8` | `is_first_frag` | `-` |
| `__u8` | `is_encap` | `-` |
| `__u8` | `ip_proto` | `-` |
| `__be16` | `n_proto` | `-` |
| `__be16` | `sport` | `-` |
| `__be16` | `dport` | `-` |
| `__be32` | `ipv4_src` | `-` |
| `__be32` | `ipv4_dst` | `-` |
| `__u32` | `ipv6_src` | `4` |
| `__u32` | `ipv6_dst` | `4` |
| `__u32` | `flags` | `-` |
| `__be32` | `flow_label` | `-` |

### `struct anonymous_91`

| Type | Field | Array |
|------|-------|-------|
| `__be32` | `ipv4_src` | `-` |
| `__be32` | `ipv4_dst` | `-` |

### `struct anonymous_92`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ipv6_src` | `4` |
| `__u32` | `ipv6_dst` | `4` |

### `struct bpf_func_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `insn_off` | `-` |
| `__u32` | `type_id` | `-` |

### `struct bpf_line_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `insn_off` | `-` |
| `__u32` | `file_name_off` | `-` |
| `__u32` | `line_off` | `-` |
| `__u32` | `line_col` | `-` |

### `struct bpf_spin_lock`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `val` | `-` |

### `struct bpf_timer`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `2` |

### `struct bpf_task_work`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `-` |

### `struct bpf_wq`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `2` |

### `struct bpf_dynptr`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `2` |

### `struct bpf_list_head`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `2` |

### `struct bpf_list_node`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `3` |

### `struct bpf_rb_root`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `2` |

### `struct bpf_rb_node`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `4` |

### `struct bpf_refcount`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `__opaque` | `1` |

### `struct bpf_sysctl`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `write` | `-` |
| `__u32` | `file_pos` | `-` |

### `struct bpf_sockopt`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `level` | `-` |
| `__s32` | `optname` | `-` |
| `__s32` | `optlen` | `-` |
| `__s32` | `retval` | `-` |

### `struct bpf_pidns_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `pid` | `-` |
| `__u32` | `tgid` | `-` |

### `struct bpf_sk_lookup`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `cookie` | `-` |
| `__u32` | `family` | `-` |
| `__u32` | `protocol` | `-` |
| `__u32` | `remote_ip4` | `-` |
| `__u32` | `remote_ip6` | `4` |
| `__be16` | `remote_port` | `-` |
| `__u32` | `local_ip4` | `-` |
| `__u32` | `local_ip6` | `4` |
| `__u32` | `local_port` | `-` |
| `__u32` | `ingress_ifindex` | `-` |

### `struct btf_ptr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type_id` | `-` |
| `__u32` | `flags` | `-` |

### `struct sample`

| Type | Field | Array |
|------|-------|-------|

### `struct anonymous_111`

| Type | Field | Array |
|------|-------|-------|

### `struct bpf_core_relo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `insn_off` | `-` |
| `__u32` | `type_id` | `-` |
| `__u32` | `access_str_off` | `-` |

### `struct bpf_iter_num`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `__opaque` | `1` |

### `struct bpf_insn_array_value`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `orig_off` | `-` |
| `__u32` | `xlated_off` | `-` |
| `__u32` | `jitted_off` | `-` |