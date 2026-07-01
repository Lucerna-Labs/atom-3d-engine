# virtio_pci.h

**Source:** `virtio_pci.h`


## Includes

- `linux/types.h`
- `linux/const.h`

## Defines (88 total)


### MAX_CAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_CAP_ID` | `__KERNEL_DIV_ROUND_UP(VIRTIO_DEV_PARTS_CAP + 1, 64)` |  |

### VIRTIO_ADMIN (29)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_ADMIN_STATUS_OK` | `0` |  |
| `VIRTIO_ADMIN_CMD_LIST_QUERY` | `0x0` |  |
| `VIRTIO_ADMIN_CMD_LIST_USE` | `0x1` |  |
| `VIRTIO_ADMIN_GROUP_TYPE_SELF` | `0x0` |  |
| `VIRTIO_ADMIN_GROUP_TYPE_SRIOV` | `0x1` |  |
| `VIRTIO_ADMIN_CMD_LEGACY_COMMON_CFG_WRITE` | `0x2` |  |
| `VIRTIO_ADMIN_CMD_LEGACY_COMMON_CFG_READ` | `0x3` |  |
| `VIRTIO_ADMIN_CMD_LEGACY_DEV_CFG_WRITE` | `0x4` |  |
| `VIRTIO_ADMIN_CMD_LEGACY_DEV_CFG_READ` | `0x5` |  |
| `VIRTIO_ADMIN_CMD_LEGACY_NOTIFY_INFO` | `0x6` |  |
| `VIRTIO_ADMIN_CMD_CAP_ID_LIST_QUERY` | `0x7` |  |
| `VIRTIO_ADMIN_CMD_DEVICE_CAP_GET` | `0x8` |  |
| `VIRTIO_ADMIN_CMD_DRIVER_CAP_SET` | `0x9` |  |
| `VIRTIO_ADMIN_CMD_RESOURCE_OBJ_CREATE` | `0xa` |  |
| `VIRTIO_ADMIN_CMD_RESOURCE_OBJ_DESTROY` | `0xd` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_GET` | `0xe` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_GET` | `0xf` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_SET` | `0x10` |  |
| `VIRTIO_ADMIN_CMD_DEV_MODE_SET` | `0x11` |  |
| `VIRTIO_ADMIN_CMD_NOTIFY_INFO_FLAGS_END` | `0` |  |
| `VIRTIO_ADMIN_CMD_NOTIFY_INFO_FLAGS_OWNER_DEV` | `0x1` |  |
| `VIRTIO_ADMIN_CMD_NOTIFY_INFO_FLAGS_OWNER_MEM` | `0x2` |  |
| `VIRTIO_ADMIN_CMD_MAX_NOTIFY_INFO` | `4` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_TYPE_SIZE` | `0` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_TYPE_COUNT` | `1` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_TYPE_LIST` | `2` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_GET_TYPE_SELECTED` | `0` |  |
| `VIRTIO_ADMIN_CMD_DEV_PARTS_GET_TYPE_ALL` | `1` |  |
| `VIRTIO_ADMIN_CMD_DEV_MODE_F_STOPPED` | `0` |  |

### VIRTIO_DEV (2)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_DEV_PARTS_CAP` | `0x0000` |  |
| `VIRTIO_DEV_PART_F_OPTIONAL` | `0` |  |

### VIRTIO_MSI (3)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_MSI_CONFIG_VECTOR` | `20` |  |
| `VIRTIO_MSI_QUEUE_VECTOR` | `22` |  |
| `VIRTIO_MSI_NO_VECTOR` | `0xffff` |  |

### VIRTIO_PCI (50)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_PCI_HOST_FEATURES` | `0` |  |
| `VIRTIO_PCI_GUEST_FEATURES` | `4` |  |
| `VIRTIO_PCI_QUEUE_PFN` | `8` |  |
| `VIRTIO_PCI_QUEUE_NUM` | `12` |  |
| `VIRTIO_PCI_QUEUE_SEL` | `14` |  |
| `VIRTIO_PCI_QUEUE_NOTIFY` | `16` |  |
| `VIRTIO_PCI_STATUS` | `18` |  |
| `VIRTIO_PCI_ISR` | `19` |  |
| `VIRTIO_PCI_ABI_VERSION` | `0` |  |
| `VIRTIO_PCI_QUEUE_ADDR_SHIFT` | `12` |  |
| `VIRTIO_PCI_VRING_ALIGN` | `4096` |  |
| `VIRTIO_PCI_ISR_CONFIG` | `0x2` |  |
| `VIRTIO_PCI_CAP_COMMON_CFG` | `1` |  |
| `VIRTIO_PCI_CAP_NOTIFY_CFG` | `2` |  |
| `VIRTIO_PCI_CAP_ISR_CFG` | `3` |  |
| `VIRTIO_PCI_CAP_DEVICE_CFG` | `4` |  |
| `VIRTIO_PCI_CAP_PCI_CFG` | `5` |  |
| `VIRTIO_PCI_CAP_SHARED_MEMORY_CFG` | `8` |  |
| `VIRTIO_PCI_CAP_VENDOR_CFG` | `9` |  |
| `VIRTIO_PCI_CAP_VNDR` | `0` |  |
| `VIRTIO_PCI_CAP_NEXT` | `1` |  |
| `VIRTIO_PCI_CAP_LEN` | `2` |  |
| `VIRTIO_PCI_CAP_CFG_TYPE` | `3` |  |
| `VIRTIO_PCI_CAP_BAR` | `4` |  |
| `VIRTIO_PCI_CAP_OFFSET` | `8` |  |
| `VIRTIO_PCI_CAP_LENGTH` | `12` |  |
| `VIRTIO_PCI_NOTIFY_CAP_MULT` | `16` |  |
| `VIRTIO_PCI_COMMON_DFSELECT` | `0` |  |
| `VIRTIO_PCI_COMMON_DF` | `4` |  |
| `VIRTIO_PCI_COMMON_GFSELECT` | `8` |  |
| `VIRTIO_PCI_COMMON_GF` | `12` |  |
| `VIRTIO_PCI_COMMON_MSIX` | `16` |  |
| `VIRTIO_PCI_COMMON_NUMQ` | `18` |  |
| `VIRTIO_PCI_COMMON_STATUS` | `20` |  |
| `VIRTIO_PCI_COMMON_CFGGENERATION` | `21` |  |
| `VIRTIO_PCI_COMMON_Q_SELECT` | `22` |  |
| `VIRTIO_PCI_COMMON_Q_SIZE` | `24` |  |
| `VIRTIO_PCI_COMMON_Q_MSIX` | `26` |  |
| `VIRTIO_PCI_COMMON_Q_ENABLE` | `28` |  |
| `VIRTIO_PCI_COMMON_Q_NOFF` | `30` |  |
| `VIRTIO_PCI_COMMON_Q_DESCLO` | `32` |  |
| `VIRTIO_PCI_COMMON_Q_DESCHI` | `36` |  |
| `VIRTIO_PCI_COMMON_Q_AVAILLO` | `40` |  |
| `VIRTIO_PCI_COMMON_Q_AVAILHI` | `44` |  |
| `VIRTIO_PCI_COMMON_Q_USEDLO` | `48` |  |
| `VIRTIO_PCI_COMMON_Q_USEDHI` | `52` |  |
| `VIRTIO_PCI_COMMON_Q_NDATA` | `56` |  |
| `VIRTIO_PCI_COMMON_Q_RESET` | `58` |  |
| `VIRTIO_PCI_COMMON_ADM_Q_IDX` | `60` |  |
| `VIRTIO_PCI_COMMON_ADM_Q_NUM` | `62` |  |

### VIRTIO_RESOURCE (3)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_RESOURCE_OBJ_DEV_PARTS` | `0` |  |
| `VIRTIO_RESOURCE_OBJ_DEV_PARTS_TYPE_GET` | `0` |  |
| `VIRTIO_RESOURCE_OBJ_DEV_PARTS_TYPE_SET` | `1` |  |

## Structs (32)


### `struct virtio_pci_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cap_vndr` | `-` |
| `__u8` | `cap_next` | `-` |
| `__u8` | `cap_len` | `-` |
| `__u8` | `cfg_type` | `-` |
| `__u8` | `bar` | `-` |
| `__u8` | `id` | `-` |
| `__u8` | `padding` | `2` |
| `__le32` | `offset` | `-` |
| `__le32` | `length` | `-` |

### `struct virtio_pci_vndr_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `cap_vndr` | `-` |
| `__u8` | `cap_next` | `-` |
| `__u8` | `cap_len` | `-` |
| `__u8` | `cfg_type` | `-` |
| `__u16` | `vendor_id` | `-` |

### `struct virtio_pci_cap64`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `offset_hi` | `-` |
| `__le32` | `length_hi` | `-` |

### `struct virtio_pci_notify_cap`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `notify_off_multiplier` | `-` |

### `struct virtio_pci_common_cfg`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `device_feature_select` | `-` |
| `__le32` | `device_feature` | `-` |
| `__le32` | `guest_feature_select` | `-` |
| `__le32` | `guest_feature` | `-` |
| `__le16` | `msix_config` | `-` |
| `__le16` | `num_queues` | `-` |
| `__u8` | `device_status` | `-` |
| `__u8` | `config_generation` | `-` |
| `__le16` | `queue_select` | `-` |
| `__le16` | `queue_size` | `-` |
| `__le16` | `queue_msix_vector` | `-` |
| `__le16` | `queue_enable` | `-` |
| `__le16` | `queue_notify_off` | `-` |
| `__le32` | `queue_desc_lo` | `-` |
| `__le32` | `queue_desc_hi` | `-` |
| `__le32` | `queue_avail_lo` | `-` |
| `__le32` | `queue_avail_hi` | `-` |
| `__le32` | `queue_used_lo` | `-` |
| `__le32` | `queue_used_hi` | `-` |

### `struct virtio_pci_modern_common_cfg`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `queue_notify_data` | `-` |
| `__le16` | `queue_reset` | `-` |
| `__le16` | `admin_queue_index` | `-` |
| `__le16` | `admin_queue_num` | `-` |

### `struct virtio_pci_cfg_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `pci_cfg_data` | `4` |

### `struct virtio_admin_cmd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `opcode` | `-` |
| `__le16` | `group_type` | `-` |
| `__u8` | `reserved1` | `12` |
| `__le64` | `group_member_id` | `-` |

### `struct virtio_admin_cmd_status`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `status` | `-` |
| `__le16` | `status_qualifier` | `-` |
| `__u8` | `reserved2` | `4` |

### `struct virtio_admin_cmd_legacy_wr_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `offset` | `-` |
| `__u8` | `reserved` | `7` |

### `struct virtio_admin_cmd_legacy_rd_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `offset` | `-` |

### `struct virtio_admin_cmd_notify_info_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |
| `__u8` | `bar` | `-` |
| `__u8` | `padding` | `6` |
| `__le64` | `offset` | `-` |

### `struct virtio_admin_cmd_notify_info_result`

| Type | Field | Array |
|------|-------|-------|

### `struct virtio_dev_parts_cap`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `get_parts_resource_objects_limit` | `-` |
| `__u8` | `set_parts_resource_objects_limit` | `-` |

### `struct virtio_admin_cmd_query_cap_id_result`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `supported_caps` | `MAX_CAP_ID` |

### `struct virtio_admin_cmd_cap_get_data`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `id` | `-` |
| `__u8` | `reserved` | `6` |

### `struct virtio_admin_cmd_cap_set_data`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `id` | `-` |
| `__u8` | `reserved` | `6` |

### `struct virtio_admin_cmd_resource_obj_cmd_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `type` | `-` |
| `__u8` | `reserved` | `2` |
| `__le32` | `id` | `-` |

### `struct virtio_admin_cmd_resource_obj_create_data`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `flags` | `-` |

### `struct virtio_resource_obj_dev_parts`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `reserved` | `7` |

### `struct virtio_admin_cmd_dev_parts_metadata_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `reserved` | `7` |

### `struct virtio_dev_part_hdr`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `part_type` | `-` |
| `__u8` | `flags` | `-` |
| `__u8` | `reserved` | `-` |
| `__le32` | `offset` | `-` |
| `__le32` | `reserved` | `-` |
| `__le16` | `index` | `-` |
| `__u8` | `reserved` | `6` |
| `__le32` | `length` | `-` |

### `struct anonymous_22`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `offset` | `-` |
| `__le32` | `reserved` | `-` |

### `struct anonymous_23`

| Type | Field | Array |
|------|-------|-------|
| `__le16` | `index` | `-` |
| `__u8` | `reserved` | `6` |

### `struct virtio_dev_part`

| Type | Field | Array |
|------|-------|-------|

### `struct virtio_admin_cmd_dev_parts_metadata_result`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `reserved` | `-` |
| `__le32` | `count` | `-` |
| `__le32` | `reserved` | `-` |
| `__le32` | `count` | `-` |
| `__le32` | `reserved` | `-` |

### `struct anonymous_26`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `size` | `-` |
| `__le32` | `reserved` | `-` |

### `struct anonymous_27`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `count` | `-` |
| `__le32` | `reserved` | `-` |

### `struct anonymous_28`

| Type | Field | Array |
|------|-------|-------|
| `__le32` | `count` | `-` |
| `__le32` | `reserved` | `-` |

### `struct virtio_admin_cmd_dev_parts_get_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `type` | `-` |
| `__u8` | `reserved` | `7` |

### `struct virtio_admin_cmd_dev_parts_set_data`

| Type | Field | Array |
|------|-------|-------|

### `struct virtio_admin_cmd_dev_mode_set_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `flags` | `-` |