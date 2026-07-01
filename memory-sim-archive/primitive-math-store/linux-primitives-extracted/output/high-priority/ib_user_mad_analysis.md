# ib_user_mad.h

**Source:** `ib_user_mad.h`


## Includes

- `linux/types.h`
- `rdma/rdma_user_ioctl.h`

## Defines (3 total)


### IB_USER (3)

| Name | Value | Comment |
|------|-------|---------|
| `IB_USER_MAD_ABI_VERSION` | `5` |  |
| `IB_USER_MAD_LONGS_PER_METHOD_MASK` | `(128 / (8 * sizeof (long)))` |  |
| `IB_USER_MAD_REG_FLAGS_CAP` | `(IB_USER_MAD_USER_RMPP)` |  |

## Structs (5)


### `struct ib_user_mad_hdr_old`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `status` | `-` |
| `__u32` | `timeout_ms` | `-` |
| `__u32` | `retries` | `-` |
| `__u32` | `length` | `-` |
| `__be32` | `qpn` | `-` |
| `__be32` | `qkey` | `-` |
| `__be16` | `lid` | `-` |
| `__u8` | `sl` | `-` |
| `__u8` | `path_bits` | `-` |
| `__u8` | `grh_present` | `-` |
| `__u8` | `gid_index` | `-` |
| `__u8` | `hop_limit` | `-` |
| `__u8` | `traffic_class` | `-` |
| `__u8` | `gid` | `16` |
| `__be32` | `flow_label` | `-` |

### `struct ib_user_mad_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `status` | `-` |
| `__u32` | `timeout_ms` | `-` |
| `__u32` | `retries` | `-` |
| `__u32` | `length` | `-` |
| `__be32` | `qpn` | `-` |
| `__be32` | `qkey` | `-` |
| `__be16` | `lid` | `-` |
| `__u8` | `sl` | `-` |
| `__u8` | `path_bits` | `-` |
| `__u8` | `grh_present` | `-` |
| `__u8` | `gid_index` | `-` |
| `__u8` | `hop_limit` | `-` |
| `__u8` | `traffic_class` | `-` |
| `__u8` | `gid` | `16` |
| `__be32` | `flow_label` | `-` |
| `__u16` | `pkey_index` | `-` |
| `__u8` | `reserved` | `6` |

### `struct ib_user_mad`

| Type | Field | Array |
|------|-------|-------|

### `struct ib_user_mad_reg_req`

*Packed structure*

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `packed_ulong` | `method_mask` | `IB_USER_MAD_LONGS_PER_METHOD_MASK` |
| `__u8` | `qpn` | `-` |
| `__u8` | `mgmt_class` | `-` |
| `__u8` | `mgmt_class_version` | `-` |
| `__u8` | `oui` | `3` |
| `__u8` | `rmpp_version` | `-` |

### `struct ib_user_mad_reg_req2`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `id` | `-` |
| `__u32` | `qpn` | `-` |
| `__u8` | `mgmt_class` | `-` |
| `__u8` | `mgmt_class_version` | `-` |
| `__u16` | `res` | `-` |
| `__u32` | `flags` | `-` |
| `__aligned_u64` | `method_mask` | `2` |
| `__u32` | `oui` | `-` |
| `__u8` | `rmpp_version` | `-` |
| `__u8` | `reserved` | `3` |