# features.h

**Source:** `features.h`


## Includes

- `linux/types.h`
- `linux/uuid.h`

## Defines (19 total)


### CXL_CMD (12)

| Name | Value | Comment |
|------|-------|---------|
| `CXL_CMD_CONFIG_CHANGE_COLD_RESET` | `BIT(0)` |  |
| `CXL_CMD_CONFIG_CHANGE_IMMEDIATE` | `BIT(1)` |  |
| `CXL_CMD_DATA_CHANGE_IMMEDIATE` | `BIT(2)` |  |
| `CXL_CMD_POLICY_CHANGE_IMMEDIATE` | `BIT(3)` |  |
| `CXL_CMD_LOG_CHANGE_IMMEDIATE` | `BIT(4)` |  |
| `CXL_CMD_SECURITY_STATE_CHANGE` | `BIT(5)` |  |
| `CXL_CMD_BACKGROUND` | `BIT(6)` |  |
| `CXL_CMD_BGCMD_ABORT_SUPPORTED` | `BIT(7)` |  |
| `CXL_CMD_EFFECTS_VALID` | `BIT(9)` |  |
| `CXL_CMD_CONFIG_CHANGE_CONV_RESET` | `BIT(10)` |  |
| `CXL_CMD_CONFIG_CHANGE_CXL_RESET` | `BIT(11)` |  |
| `CXL_CMD_EFFECTS_RESERVED` | `GENMASK(15, 12)` |  |

### CXL_FEATURE (4)

| Name | Value | Comment |
|------|-------|---------|
| `CXL_FEATURE_F_CHANGEABLE` | `BIT(0)` |  |
| `CXL_FEATURE_F_PERSIST_FW_UPDATE` | `BIT(4)` |  |
| `CXL_FEATURE_F_DEFAULT_SEL` | `BIT(5)` |  |
| `CXL_FEATURE_F_SAVED_SEL` | `BIT(6)` |  |

### CXL_SET (2)

| Name | Value | Comment |
|------|-------|---------|
| `CXL_SET_FEAT_FLAG_DATA_TRANSFER_MASK` | `GENMASK(2, 0)` |  |
| `CXL_SET_FEAT_FLAG_DATA_SAVED_ACROSS_RESET` | `BIT(3)` |  |

### UNCATEGORIZED (1)

| Name | Value | Comment |
|------|-------|---------|
| `__uapi_uuid_t` | `uuid_t` |  |

## Structs (5)


### `struct cxl_mbox_get_sup_feats_in`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `count` | `-` |
| `__le16` | `start_idx` | `-` |
| `__u8` | `reserved` | `2` |

### `struct cxl_feat_entry`

| Type | Field | Array |
|------|-------|-------|
| `__uapi_uuid_t` | `uuid` | `-` |
| `__le16` | `id` | `-` |
| `__le16` | `get_feat_size` | `-` |
| `__le16` | `set_feat_size` | `-` |
| `__le32` | `flags` | `-` |
| `__u8` | `get_feat_ver` | `-` |
| `__u8` | `set_feat_ver` | `-` |
| `__le16` | `effects` | `-` |
| `__u8` | `reserved` | `18` |

### `struct cxl_mbox_get_sup_feats_out`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `num_entries` | `-` |
| `__le16` | `supported_feats` | `-` |
| `__u8` | `reserved` | `4` |

### `struct cxl_mbox_get_feat_in`

| Type | Field | Array |
|------|-------|-------|
| `__uapi_uuid_t` | `uuid` | `-` |
| `__le16` | `offset` | `-` |
| `__le16` | `count` | `-` |
| `__u8` | `selection` | `-` |

### `struct cxl_mbox_set_feat_in`

| Type | Field | Array |
|------|-------|-------|
| `__uapi_uuid_t` | `uuid` | `-` |
| `__le32` | `flags` | `-` |
| `__le16` | `offset` | `-` |
| `__u8` | `version` | `-` |
| `__u8` | `rsvd` | `9` |