# vfio.h

**Source:** `vfio.h`


## Includes

- `linux/types.h`
- `linux/ioctl.h`
- `linux/stddef.h`

## Defines (182 total)


### UNCATEGORIZED (6)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_TYPE1_IOMMU` | `1` |  |
| `VFIO_TYPE1v2_IOMMU` | `3` |  |
| `VFIO_EEH` | `5` |  |
| `__VFIO_RESERVED_TYPE1_NESTING_IOMMU` | `6` | Implies v2 |
| `VFIO_TYPE` | `(';')` |  |
| `VFIO_BASE` | `100` |  |

### VFIO_API (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_API_VERSION` | `0` |  |

### VFIO_CHECK (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_CHECK_EXTENSION` | `_IO(VFIO_TYPE, VFIO_BASE + 1)` |  |

### VFIO_DEVICE (67)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_DEVICE_FLAGS_RESET` | `(1 << 0)` | Device supports reset |
| `VFIO_DEVICE_FLAGS_PCI` | `(1 << 1)` | vfio-pci device |
| `VFIO_DEVICE_FLAGS_PLATFORM` | `(1 << 2)` | vfio-platform device |
| `VFIO_DEVICE_FLAGS_AMBA` | `(1 << 3)` | vfio-amba device |
| `VFIO_DEVICE_FLAGS_CCW` | `(1 << 4)` | vfio-ccw device |
| `VFIO_DEVICE_FLAGS_AP` | `(1 << 5)` | vfio-ap device |
| `VFIO_DEVICE_FLAGS_FSL_MC` | `(1 << 6)` | vfio-fsl-mc device |
| `VFIO_DEVICE_FLAGS_CAPS` | `(1 << 7)` | Info supports caps |
| `VFIO_DEVICE_FLAGS_CDX` | `(1 << 8)` | vfio-cdx device |
| `VFIO_DEVICE_GET_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 7)` |  |
| `VFIO_DEVICE_API_PCI_STRING` | `"vfio-pci"` |  |
| `VFIO_DEVICE_API_PLATFORM_STRING` | `"vfio-platform"` |  |
| `VFIO_DEVICE_API_AMBA_STRING` | `"vfio-amba"` |  |
| `VFIO_DEVICE_API_CCW_STRING` | `"vfio-ccw"` |  |
| `VFIO_DEVICE_API_AP_STRING` | `"vfio-ap"` |  |
| `VFIO_DEVICE_INFO_CAP_ZPCI_BASE` | `1` |  |
| `VFIO_DEVICE_INFO_CAP_ZPCI_GROUP` | `2` |  |
| `VFIO_DEVICE_INFO_CAP_ZPCI_UTIL` | `3` |  |
| `VFIO_DEVICE_INFO_CAP_ZPCI_PFIP` | `4` |  |
| `VFIO_DEVICE_INFO_CAP_PCI_ATOMIC_COMP` | `5` |  |
| `VFIO_DEVICE_GET_REGION_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 8)` |  |
| `VFIO_DEVICE_GFX_LINK_STATE_UP` | `1` |  |
| `VFIO_DEVICE_GFX_LINK_STATE_DOWN` | `2` |  |
| `VFIO_DEVICE_STATE_V1_STOP` | `(0)` |  |
| `VFIO_DEVICE_STATE_V1_RUNNING` | `(1 << 0)` |  |
| `VFIO_DEVICE_STATE_V1_SAVING` | `(1 << 1)` |  |
| `VFIO_DEVICE_STATE_V1_RESUMING` | `(1 << 2)` |  |
| `VFIO_DEVICE_STATE_MASK` | `(VFIO_DEVICE_STATE_V1_RUNNING \| ` |  |
| `VFIO_DEVICE_GET_IRQ_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 9)` |  |
| `VFIO_DEVICE_SET_IRQS` | `_IO(VFIO_TYPE, VFIO_BASE + 10)` |  |
| `VFIO_DEVICE_RESET` | `_IO(VFIO_TYPE, VFIO_BASE + 11)` |  |
| `VFIO_DEVICE_GET_PCI_HOT_RESET_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 12)` |  |
| `VFIO_DEVICE_PCI_HOT_RESET` | `_IO(VFIO_TYPE, VFIO_BASE + 13)` |  |
| `VFIO_DEVICE_QUERY_GFX_PLANE` | `_IO(VFIO_TYPE, VFIO_BASE + 14)` |  |
| `VFIO_DEVICE_GET_GFX_DMABUF` | `_IO(VFIO_TYPE, VFIO_BASE + 15)` |  |
| `VFIO_DEVICE_IOEVENTFD_8` | `(1 << 0)` | 1-byte write |
| `VFIO_DEVICE_IOEVENTFD_16` | `(1 << 1)` | 2-byte write |
| `VFIO_DEVICE_IOEVENTFD_32` | `(1 << 2)` | 4-byte write |
| `VFIO_DEVICE_IOEVENTFD_64` | `(1 << 3)` | 8-byte write |
| `VFIO_DEVICE_IOEVENTFD_SIZE_MASK` | `(0xf)` |  |
| `VFIO_DEVICE_IOEVENTFD` | `_IO(VFIO_TYPE, VFIO_BASE + 16)` |  |
| `VFIO_DEVICE_FEATURE_MASK` | `(0xffff)` | 16-bit feature index |
| `VFIO_DEVICE_FEATURE_GET` | `(1 << 16)` | Get feature into data[] |
| `VFIO_DEVICE_FEATURE_SET` | `(1 << 17)` | Set feature from data[] |
| `VFIO_DEVICE_FEATURE_PROBE` | `(1 << 18)` | Probe feature support |
| `VFIO_DEVICE_FEATURE` | `_IO(VFIO_TYPE, VFIO_BASE + 17)` |  |
| `VFIO_DEVICE_BIND_FLAG_TOKEN` | `(1 << 0)` |  |
| `VFIO_DEVICE_BIND_IOMMUFD` | `_IO(VFIO_TYPE, VFIO_BASE + 18)` |  |
| `VFIO_DEVICE_ATTACH_PASID` | `(1 << 0)` |  |
| `VFIO_DEVICE_ATTACH_IOMMUFD_PT` | `_IO(VFIO_TYPE, VFIO_BASE + 19)` |  |

*...and 17 more*

### VFIO_DMA (7)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_DMA_CC_IOMMU` | `4` |  |
| `VFIO_DMA_MAP_FLAG_READ` | `(1 << 0)` | readable from device |
| `VFIO_DMA_MAP_FLAG_WRITE` | `(1 << 1)` | writable from device |
| `VFIO_DMA_MAP_FLAG_VADDR` | `(1 << 2)` |  |
| `VFIO_DMA_UNMAP_FLAG_GET_DIRTY_BITMAP` | `(1 << 0)` |  |
| `VFIO_DMA_UNMAP_FLAG_ALL` | `(1 << 1)` |  |
| `VFIO_DMA_UNMAP_FLAG_VADDR` | `(1 << 2)` |  |

### VFIO_EEH (16)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_EEH_PE_DISABLE` | `0` | Disable EEH functionality |
| `VFIO_EEH_PE_ENABLE` | `1` | Enable EEH functionality |
| `VFIO_EEH_PE_UNFREEZE_IO` | `2` | Enable IO for frozen PE |
| `VFIO_EEH_PE_UNFREEZE_DMA` | `3` | Enable DMA for frozen PE |
| `VFIO_EEH_PE_GET_STATE` | `4` | PE state retrieval |
| `VFIO_EEH_PE_STATE_NORMAL` | `0` | PE in functional state |
| `VFIO_EEH_PE_STATE_RESET` | `1` | PE reset in progress |
| `VFIO_EEH_PE_STATE_STOPPED` | `2` | Stopped DMA and IO |
| `VFIO_EEH_PE_STATE_STOPPED_DMA` | `4` | Stopped DMA only |
| `VFIO_EEH_PE_STATE_UNAVAIL` | `5` | State unavailable |
| `VFIO_EEH_PE_RESET_DEACTIVATE` | `5` | Deassert PE reset |
| `VFIO_EEH_PE_RESET_HOT` | `6` | Assert hot reset |
| `VFIO_EEH_PE_RESET_FUNDAMENTAL` | `7` | Assert fundamental reset |
| `VFIO_EEH_PE_CONFIGURE` | `8` | PE configuration |
| `VFIO_EEH_PE_INJECT_ERR` | `9` | Inject EEH error |
| `VFIO_EEH_PE_OP` | `_IO(VFIO_TYPE, VFIO_BASE + 21)` |  |

### VFIO_GET (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_GET_API_VERSION` | `_IO(VFIO_TYPE, VFIO_BASE + 0)` |  |

### VFIO_GFX (3)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_GFX_PLANE_TYPE_PROBE` | `(1 << 0)` |  |
| `VFIO_GFX_PLANE_TYPE_DMABUF` | `(1 << 1)` |  |
| `VFIO_GFX_PLANE_TYPE_REGION` | `(1 << 2)` |  |

### VFIO_GROUP (6)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_GROUP_FLAGS_VIABLE` | `(1 << 0)` |  |
| `VFIO_GROUP_FLAGS_CONTAINER_SET` | `(1 << 1)` |  |
| `VFIO_GROUP_GET_STATUS` | `_IO(VFIO_TYPE, VFIO_BASE + 3)` |  |
| `VFIO_GROUP_SET_CONTAINER` | `_IO(VFIO_TYPE, VFIO_BASE + 4)` |  |
| `VFIO_GROUP_UNSET_CONTAINER` | `_IO(VFIO_TYPE, VFIO_BASE + 5)` |  |
| `VFIO_GROUP_GET_DEVICE_FD` | `_IO(VFIO_TYPE, VFIO_BASE + 6)` |  |

### VFIO_IOMMU (20)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_IOMMU_INFO_PGSIZES` | `(1 << 0)` | supported page sizes info |
| `VFIO_IOMMU_INFO_CAPS` | `(1 << 1)` | Info supports caps |
| `VFIO_IOMMU_TYPE1_INFO_CAP_IOVA_RANGE` | `1` |  |
| `VFIO_IOMMU_TYPE1_INFO_CAP_MIGRATION` | `2` |  |
| `VFIO_IOMMU_TYPE1_INFO_DMA_AVAIL` | `3` |  |
| `VFIO_IOMMU_GET_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 12)` |  |
| `VFIO_IOMMU_MAP_DMA` | `_IO(VFIO_TYPE, VFIO_BASE + 13)` |  |
| `VFIO_IOMMU_UNMAP_DMA` | `_IO(VFIO_TYPE, VFIO_BASE + 14)` |  |
| `VFIO_IOMMU_ENABLE` | `_IO(VFIO_TYPE, VFIO_BASE + 15)` |  |
| `VFIO_IOMMU_DISABLE` | `_IO(VFIO_TYPE, VFIO_BASE + 16)` |  |
| `VFIO_IOMMU_DIRTY_PAGES_FLAG_START` | `(1 << 0)` |  |
| `VFIO_IOMMU_DIRTY_PAGES_FLAG_STOP` | `(1 << 1)` |  |
| `VFIO_IOMMU_DIRTY_PAGES_FLAG_GET_BITMAP` | `(1 << 2)` |  |
| `VFIO_IOMMU_DIRTY_PAGES` | `_IO(VFIO_TYPE, VFIO_BASE + 17)` |  |
| `VFIO_IOMMU_SPAPR_INFO_DDW` | `(1 << 0)` | DDW supported |
| `VFIO_IOMMU_SPAPR_TCE_GET_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 12)` |  |
| `VFIO_IOMMU_SPAPR_REGISTER_MEMORY` | `_IO(VFIO_TYPE, VFIO_BASE + 17)` |  |
| `VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY` | `_IO(VFIO_TYPE, VFIO_BASE + 18)` |  |
| `VFIO_IOMMU_SPAPR_TCE_CREATE` | `_IO(VFIO_TYPE, VFIO_BASE + 19)` |  |
| `VFIO_IOMMU_SPAPR_TCE_REMOVE` | `_IO(VFIO_TYPE, VFIO_BASE + 20)` |  |

### VFIO_IRQ (12)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_IRQ_INFO_EVENTFD` | `(1 << 0)` |  |
| `VFIO_IRQ_INFO_MASKABLE` | `(1 << 1)` |  |
| `VFIO_IRQ_INFO_AUTOMASKED` | `(1 << 2)` |  |
| `VFIO_IRQ_INFO_NORESIZE` | `(1 << 3)` |  |
| `VFIO_IRQ_SET_DATA_NONE` | `(1 << 0)` | Data not present |
| `VFIO_IRQ_SET_DATA_BOOL` | `(1 << 1)` | Data is bool (u8) |
| `VFIO_IRQ_SET_DATA_EVENTFD` | `(1 << 2)` | Data is eventfd (s32) |
| `VFIO_IRQ_SET_ACTION_MASK` | `(1 << 3)` | Mask interrupt |
| `VFIO_IRQ_SET_ACTION_UNMASK` | `(1 << 4)` | Unmask interrupt |
| `VFIO_IRQ_SET_ACTION_TRIGGER` | `(1 << 5)` | Trigger interrupt |
| `VFIO_IRQ_SET_DATA_TYPE_MASK` | `(VFIO_IRQ_SET_DATA_NONE \| ` |  |
| `VFIO_IRQ_SET_ACTION_TYPE_MASK` | `(VFIO_IRQ_SET_ACTION_MASK \| ` |  |

### VFIO_MIG (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_MIG_GET_PRECOPY_INFO` | `_IO(VFIO_TYPE, VFIO_BASE + 21)` |  |

### VFIO_MIGRATION (3)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_MIGRATION_STOP_COPY` | `(1 << 0)` |  |
| `VFIO_MIGRATION_P2P` | `(1 << 1)` |  |
| `VFIO_MIGRATION_PRE_COPY` | `(1 << 2)` |  |

### VFIO_NOIOMMU (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_NOIOMMU_IOMMU` | `8` |  |

### VFIO_PCI (7)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_PCI_ATOMIC_COMP32` | `(1 << 0)` |  |
| `VFIO_PCI_ATOMIC_COMP64` | `(1 << 1)` |  |
| `VFIO_PCI_ATOMIC_COMP128` | `(1 << 2)` |  |
| `VFIO_PCI_DEVID_OWNED` | `0` |  |
| `VFIO_PCI_DEVID_NOT_OWNED` | `-1` |  |
| `VFIO_PCI_HOT_RESET_FLAG_DEV_ID` | `(1 << 0)` |  |
| `VFIO_PCI_HOT_RESET_FLAG_DEV_ID_OWNED` | `(1 << 1)` |  |

### VFIO_PRECOPY (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_PRECOPY_INFO_REINIT` | `(1 << 0)` | output - new initial data is present |

### VFIO_REGION (24)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_REGION_INFO_FLAG_READ` | `(1 << 0)` | Region supports read |
| `VFIO_REGION_INFO_FLAG_WRITE` | `(1 << 1)` | Region supports write |
| `VFIO_REGION_INFO_FLAG_MMAP` | `(1 << 2)` | Region supports mmap |
| `VFIO_REGION_INFO_FLAG_CAPS` | `(1 << 3)` | Info supports caps |
| `VFIO_REGION_INFO_CAP_SPARSE_MMAP` | `1` |  |
| `VFIO_REGION_INFO_CAP_TYPE` | `2` |  |
| `VFIO_REGION_TYPE_PCI_VENDOR_TYPE` | `(1 << 31)` |  |
| `VFIO_REGION_TYPE_PCI_VENDOR_MASK` | `(0xffff)` |  |
| `VFIO_REGION_TYPE_GFX` | `(1)` |  |
| `VFIO_REGION_TYPE_CCW` | `(2)` |  |
| `VFIO_REGION_TYPE_MIGRATION_DEPRECATED` | `(3)` |  |
| `VFIO_REGION_SUBTYPE_INTEL_IGD_OPREGION` | `(1)` |  |
| `VFIO_REGION_SUBTYPE_INTEL_IGD_HOST_CFG` | `(2)` |  |
| `VFIO_REGION_SUBTYPE_INTEL_IGD_LPC_CFG` | `(3)` |  |
| `VFIO_REGION_SUBTYPE_NVIDIA_NVLINK2_RAM` | `(1)` |  |
| `VFIO_REGION_SUBTYPE_IBM_NVLINK2_ATSD` | `(1)` |  |
| `VFIO_REGION_SUBTYPE_GFX_EDID` | `(1)` |  |
| `VFIO_REGION_SUBTYPE_CCW_ASYNC_CMD` | `(1)` |  |
| `VFIO_REGION_SUBTYPE_CCW_SCHIB` | `(2)` |  |
| `VFIO_REGION_SUBTYPE_CCW_CRW` | `(3)` |  |
| `VFIO_REGION_SUBTYPE_MIGRATION_DEPRECATED` | `(1)` |  |
| `VFIO_REGION_INFO_CAP_MSIX_MAPPABLE` | `3` |  |
| `VFIO_REGION_INFO_CAP_NVLINK2_SSATGT` | `4` |  |
| `VFIO_REGION_INFO_CAP_NVLINK2_LNKSPD` | `5` |  |

### VFIO_SET (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_SET_IOMMU` | `_IO(VFIO_TYPE, VFIO_BASE + 2)` |  |

### VFIO_SPAPR (2)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_SPAPR_TCE_IOMMU` | `2` |  |
| `VFIO_SPAPR_TCE_v2_IOMMU` | `7` |  |

### VFIO_UNMAP (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_UNMAP_ALL` | `9` |  |

### VFIO_UPDATE (1)

| Name | Value | Comment |
|------|-------|---------|
| `VFIO_UPDATE_VADDR` | `10` |  |

## Structs (51)


### `struct vfio_info_cap_header`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `id` | `-` |
| `__u16` | `version` | `-` |
| `__u32` | `next` | `-` |

### `struct vfio_group_status`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |

### `struct vfio_device_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `num_regions` | `-` |
| `__u32` | `num_irqs` | `-` |
| `__u32` | `cap_offset` | `-` |
| `__u32` | `pad` | `-` |

### `struct vfio_device_info_cap_pci_atomic_comp`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u32` | `reserved` | `-` |

### `struct vfio_region_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `index` | `-` |
| `__u32` | `cap_offset` | `-` |
| `__aligned_u64` | `size` | `-` |
| `__aligned_u64` | `offset` | `-` |

### `struct vfio_region_sparse_mmap_area`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `offset` | `-` |
| `__aligned_u64` | `size` | `-` |

### `struct vfio_region_info_cap_sparse_mmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nr_areas` | `-` |
| `__u32` | `reserved` | `-` |

### `struct vfio_region_info_cap_type`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `subtype` | `-` |

### `struct vfio_region_gfx_edid`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `edid_offset` | `-` |
| `__u32` | `edid_max_size` | `-` |
| `__u32` | `edid_size` | `-` |
| `__u32` | `max_xres` | `-` |
| `__u32` | `max_yres` | `-` |
| `__u32` | `link_state` | `-` |

### `struct vfio_device_migration_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `device_state` | `-` |
| `__u32` | `reserved` | `-` |
| `__aligned_u64` | `pending_bytes` | `-` |
| `__aligned_u64` | `data_offset` | `-` |
| `__aligned_u64` | `data_size` | `-` |

### `struct vfio_region_info_cap_nvlink2_ssatgt`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `tgt` | `-` |

### `struct vfio_region_info_cap_nvlink2_lnkspd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `link_speed` | `-` |
| `__u32` | `__pad` | `-` |

### `struct vfio_irq_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `index` | `-` |
| `__u32` | `count` | `-` |

### `struct vfio_irq_set`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `index` | `-` |
| `__u32` | `start` | `-` |
| `__u32` | `count` | `-` |

### `struct vfio_pci_dependent_device`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `group_id` | `-` |
| `__u32` | `devid` | `-` |
| `__u16` | `segment` | `-` |
| `__u8` | `bus` | `-` |
| `__u8` | `devfn` | `-` |

### `struct vfio_pci_hot_reset_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `count` | `-` |

### `struct vfio_pci_hot_reset`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `count` | `-` |

### `struct vfio_device_gfx_plane_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `drm_plane_type` | `-` |
| `__u32` | `drm_format` | `-` |
| `__aligned_u64` | `drm_format_mod` | `-` |
| `__u32` | `width` | `-` |
| `__u32` | `height` | `-` |
| `__u32` | `stride` | `-` |
| `__u32` | `size` | `-` |
| `__u32` | `x_pos` | `-` |
| `__u32` | `y_pos` | `-` |
| `__u32` | `x_hot` | `-` |
| `__u32` | `y_hot` | `-` |
| `__u32` | `region_index` | `-` |
| `__u32` | `dmabuf_id` | `-` |
| `__u32` | `reserved` | `-` |

### `struct vfio_device_ioeventfd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__aligned_u64` | `offset` | `-` |
| `__aligned_u64` | `data` | `-` |
| `__s32` | `fd` | `-` |
| `__u32` | `reserved` | `-` |

### `struct vfio_device_feature`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |

### `struct vfio_device_bind_iommufd`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__s32` | `iommufd` | `-` |
| `__u32` | `out_devid` | `-` |
| `__aligned_u64` | `token_uuid_ptr` | `-` |

### `struct vfio_device_attach_iommufd_pt`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pt_id` | `-` |
| `__u32` | `pasid` | `-` |

### `struct vfio_device_detach_iommufd_pt`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `pasid` | `-` |

### `struct vfio_device_feature_migration`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `flags` | `-` |

### `struct vfio_device_feature_mig_state`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `device_state` | `-` |
| `__s32` | `data_fd` | `-` |

### `struct vfio_precopy_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__aligned_u64` | `initial_bytes` | `-` |
| `__aligned_u64` | `dirty_bytes` | `-` |

### `struct vfio_device_low_power_entry_with_wakeup`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `wakeup_eventfd` | `-` |
| `__u32` | `reserved` | `-` |

### `struct vfio_device_feature_dma_logging_control`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `page_size` | `-` |
| `__u32` | `num_ranges` | `-` |
| `__u32` | `__reserved` | `-` |
| `__aligned_u64` | `ranges` | `-` |

### `struct vfio_device_feature_dma_logging_range`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `iova` | `-` |
| `__aligned_u64` | `length` | `-` |

### `struct vfio_device_feature_dma_logging_report`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `iova` | `-` |
| `__aligned_u64` | `length` | `-` |
| `__aligned_u64` | `page_size` | `-` |
| `__aligned_u64` | `bitmap` | `-` |

### `struct vfio_device_feature_mig_data_size`

| Type | Field | Array |
|------|-------|-------|
| `__aligned_u64` | `stop_copy_length` | `-` |

### `struct vfio_device_feature_bus_master`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `op` | `-` |

### `struct vfio_region_dma_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `offset` | `-` |
| `__u64` | `length` | `-` |

### `struct vfio_device_feature_dma_buf`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `region_index` | `-` |
| `__u32` | `open_flags` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `nr_ranges` | `-` |

### `struct vfio_iommu_type1_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__aligned_u64` | `iova_pgsizes` | `-` |
| `__u32` | `cap_offset` | `-` |
| `__u32` | `pad` | `-` |

### `struct vfio_iova_range`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `start` | `-` |
| `__u64` | `end` | `-` |

### `struct vfio_iommu_type1_info_cap_iova_range`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nr_iovas` | `-` |
| `__u32` | `reserved` | `-` |

### `struct vfio_iommu_type1_info_cap_migration`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `flags` | `-` |
| `__u64` | `pgsize_bitmap` | `-` |
| `__u64` | `max_dirty_bitmap_size` | `-` |

### `struct vfio_iommu_type1_info_dma_avail`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `avail` | `-` |

### `struct vfio_iommu_type1_dma_map`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `vaddr` | `-` |
| `__u64` | `iova` | `-` |
| `__u64` | `size` | `-` |

### `struct vfio_bitmap`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `pgsize` | `-` |
| `__u64` | `size` | `-` |

### `struct vfio_iommu_type1_dma_unmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `iova` | `-` |
| `__u64` | `size` | `-` |

### `struct vfio_iommu_type1_dirty_bitmap`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |

### `struct vfio_iommu_type1_dirty_bitmap_get`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `iova` | `-` |
| `__u64` | `size` | `-` |

### `struct vfio_iommu_spapr_tce_ddw_info`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `pgsizes` | `-` |
| `__u32` | `max_dynamic_windows_supported` | `-` |
| `__u32` | `levels` | `-` |

### `struct vfio_iommu_spapr_tce_info`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `dma32_window_start` | `-` |
| `__u32` | `dma32_window_size` | `-` |

### `struct vfio_eeh_pe_err`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `type` | `-` |
| `__u32` | `func` | `-` |
| `__u64` | `addr` | `-` |
| `__u64` | `mask` | `-` |

### `struct vfio_eeh_pe_op`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `op` | `-` |

### `struct vfio_iommu_spapr_register_memory`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `vaddr` | `-` |
| `__u64` | `size` | `-` |

### `struct vfio_iommu_spapr_tce_create`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u32` | `page_shift` | `-` |
| `__u32` | `__resv1` | `-` |
| `__u64` | `window_size` | `-` |
| `__u32` | `levels` | `-` |
| `__u32` | `__resv2` | `-` |
| `__u64` | `start_addr` | `-` |

### `struct vfio_iommu_spapr_tce_remove`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `argsz` | `-` |
| `__u32` | `flags` | `-` |
| `__u64` | `start_addr` | `-` |