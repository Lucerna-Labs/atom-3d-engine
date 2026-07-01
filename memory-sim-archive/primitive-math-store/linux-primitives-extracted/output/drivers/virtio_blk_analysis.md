# virtio_blk.h

**Source:** `virtio_blk.h`


## Includes

- `linux/types.h`
- `linux/virtio_ids.h`
- `linux/virtio_config.h`
- `linux/virtio_types.h`

## Defines (55 total)


### VIRTIO_BLK (55)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_BLK_F_SIZE_MAX` | `1` | Indicates maximum segment size |
| `VIRTIO_BLK_F_SEG_MAX` | `2` | Indicates maximum # of segments |
| `VIRTIO_BLK_F_GEOMETRY` | `4` | Legacy geometry available |
| `VIRTIO_BLK_F_RO` | `5` | Disk is read-only |
| `VIRTIO_BLK_F_BLK_SIZE` | `6` | Block size of disk is available |
| `VIRTIO_BLK_F_TOPOLOGY` | `10` | Topology information is available |
| `VIRTIO_BLK_F_MQ` | `12` | support more than one vq |
| `VIRTIO_BLK_F_DISCARD` | `13` | DISCARD is supported |
| `VIRTIO_BLK_F_WRITE_ZEROES` | `14` | WRITE ZEROES is supported |
| `VIRTIO_BLK_F_SECURE_ERASE` | `16` | Secure Erase is supported |
| `VIRTIO_BLK_F_ZONED` | `17` | Zoned block device |
| `VIRTIO_BLK_F_BARRIER` | `0` | Does host support barriers? |
| `VIRTIO_BLK_F_SCSI` | `7` | Supports scsi command passthru |
| `VIRTIO_BLK_F_FLUSH` | `9` | Flush command supported |
| `VIRTIO_BLK_F_CONFIG_WCE` | `11` | Writeback mode available in config |
| `VIRTIO_BLK_F_WCE` | `VIRTIO_BLK_F_FLUSH` |  |
| `VIRTIO_BLK_ID_BYTES` | `20` | ID string length |
| `VIRTIO_BLK_T_IN` | `0` |  |
| `VIRTIO_BLK_T_OUT` | `1` |  |
| `VIRTIO_BLK_T_SCSI_CMD` | `2` |  |
| `VIRTIO_BLK_T_FLUSH` | `4` |  |
| `VIRTIO_BLK_T_GET_ID` | `8` |  |
| `VIRTIO_BLK_T_DISCARD` | `11` |  |
| `VIRTIO_BLK_T_WRITE_ZEROES` | `13` |  |
| `VIRTIO_BLK_T_SECURE_ERASE` | `14` |  |
| `VIRTIO_BLK_T_ZONE_APPEND` | `15` |  |
| `VIRTIO_BLK_T_ZONE_REPORT` | `16` |  |
| `VIRTIO_BLK_T_ZONE_OPEN` | `18` |  |
| `VIRTIO_BLK_T_ZONE_CLOSE` | `20` |  |
| `VIRTIO_BLK_T_ZONE_FINISH` | `22` |  |
| `VIRTIO_BLK_T_ZONE_RESET` | `24` |  |
| `VIRTIO_BLK_T_ZONE_RESET_ALL` | `26` |  |
| `VIRTIO_BLK_T_BARRIER` | `0x80000000` |  |
| `VIRTIO_BLK_Z_NONE` | `0` |  |
| `VIRTIO_BLK_Z_HM` | `1` |  |
| `VIRTIO_BLK_Z_HA` | `2` |  |
| `VIRTIO_BLK_ZT_CONV` | `1` |  |
| `VIRTIO_BLK_ZT_SWR` | `2` |  |
| `VIRTIO_BLK_ZT_SWP` | `3` |  |
| `VIRTIO_BLK_ZS_NOT_WP` | `0` |  |
| `VIRTIO_BLK_ZS_EMPTY` | `1` |  |
| `VIRTIO_BLK_ZS_IOPEN` | `2` |  |
| `VIRTIO_BLK_ZS_EOPEN` | `3` |  |
| `VIRTIO_BLK_ZS_CLOSED` | `4` |  |
| `VIRTIO_BLK_ZS_RDONLY` | `13` |  |
| `VIRTIO_BLK_ZS_FULL` | `14` |  |
| `VIRTIO_BLK_ZS_OFFLINE` | `15` |  |
| `VIRTIO_BLK_WRITE_ZEROES_FLAG_UNMAP` | `0x00000001` |  |
| `VIRTIO_BLK_S_OK` | `0` |  |
| `VIRTIO_BLK_S_IOERR` | `1` |  |

*...and 5 more*

## Structs (8)


### `struct virtio_blk_config`

| Type | Field | Array |
|------|-------|-------|
| `__virtio64` | `capacity` | `-` |
| `__virtio32` | `size_max` | `-` |
| `__virtio32` | `seg_max` | `-` |
| `__virtio16` | `cylinders` | `-` |
| `__u8` | `heads` | `-` |
| `__u8` | `sectors` | `-` |
| `__virtio32` | `blk_size` | `-` |
| `__u8` | `physical_block_exp` | `-` |
| `__u8` | `alignment_offset` | `-` |
| `__virtio16` | `min_io_size` | `-` |
| `__virtio32` | `opt_io_size` | `-` |
| `__u8` | `wce` | `-` |
| `__u8` | `unused` | `-` |
| `__virtio16` | `num_queues` | `-` |
| `__virtio32` | `max_discard_sectors` | `-` |
| `__virtio32` | `max_discard_seg` | `-` |
| `__virtio32` | `discard_sector_alignment` | `-` |
| `__virtio32` | `max_write_zeroes_sectors` | `-` |
| `__virtio32` | `max_write_zeroes_seg` | `-` |
| `__u8` | `write_zeroes_may_unmap` | `-` |
| `__u8` | `unused1` | `3` |
| `__virtio32` | `max_secure_erase_sectors` | `-` |
| `__virtio32` | `max_secure_erase_seg` | `-` |
| `__virtio32` | `secure_erase_sector_alignment` | `-` |
| `__virtio32` | `zone_sectors` | `-` |
| `__virtio32` | `max_open_zones` | `-` |
| `__virtio32` | `max_active_zones` | `-` |
| `__virtio32` | `max_append_sectors` | `-` |
| `__virtio32` | `write_granularity` | `-` |
| `__u8` | `model` | `-` |
| `__u8` | `unused2` | `3` |

### `struct virtio_blk_geometry`

| Type | Field | Array |
|------|-------|-------|
| `__virtio16` | `cylinders` | `-` |
| `__u8` | `heads` | `-` |
| `__u8` | `sectors` | `-` |

### `struct virtio_blk_zoned_characteristics`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `zone_sectors` | `-` |
| `__virtio32` | `max_open_zones` | `-` |
| `__virtio32` | `max_active_zones` | `-` |
| `__virtio32` | `max_append_sectors` | `-` |
| `__virtio32` | `write_granularity` | `-` |
| `__u8` | `model` | `-` |
| `__u8` | `unused2` | `3` |

### `struct virtio_blk_outhdr`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `type` | `-` |
| `__virtio32` | `ioprio` | `-` |
| `__virtio64` | `sector` | `-` |

### `struct virtio_blk_zone_descriptor`

| Type | Field | Array |
|------|-------|-------|
| `__virtio64` | `z_cap` | `-` |
| `__virtio64` | `z_start` | `-` |
| `__virtio64` | `z_wp` | `-` |
| `__u8` | `z_type` | `-` |
| `__u8` | `z_state` | `-` |
| `__u8` | `reserved` | `38` |

### `struct virtio_blk_zone_report`

| Type | Field | Array |
|------|-------|-------|
| `__virtio64` | `nr_zones` | `-` |
| `__u8` | `reserved` | `56` |

### `struct virtio_blk_discard_write_zeroes`

| Type | Field | Array |
|------|-------|-------|
| `__le64` | `sector` | `-` |
| `__le32` | `num_sectors` | `-` |
| `__le32` | `flags` | `-` |

### `struct virtio_scsi_inhdr`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `errors` | `-` |
| `__virtio32` | `data_len` | `-` |
| `__virtio32` | `sense_len` | `-` |
| `__virtio32` | `residual` | `-` |