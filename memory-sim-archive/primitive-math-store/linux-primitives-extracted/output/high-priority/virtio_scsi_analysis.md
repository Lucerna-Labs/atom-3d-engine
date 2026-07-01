# virtio_scsi.h

**Source:** `virtio_scsi.h`


## Includes

- `linux/virtio_types.h`

## Defines (44 total)


### VIRTIO_SCSI (44)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_SCSI_CDB_DEFAULT_SIZE` | `32` |  |
| `VIRTIO_SCSI_SENSE_DEFAULT_SIZE` | `96` |  |
| `VIRTIO_SCSI_CDB_SIZE` | `VIRTIO_SCSI_CDB_DEFAULT_SIZE` |  |
| `VIRTIO_SCSI_SENSE_SIZE` | `VIRTIO_SCSI_SENSE_DEFAULT_SIZE` |  |
| `VIRTIO_SCSI_F_INOUT` | `0` |  |
| `VIRTIO_SCSI_F_HOTPLUG` | `1` |  |
| `VIRTIO_SCSI_F_CHANGE` | `2` |  |
| `VIRTIO_SCSI_F_T10_PI` | `3` |  |
| `VIRTIO_SCSI_S_OK` | `0` |  |
| `VIRTIO_SCSI_S_OVERRUN` | `1` |  |
| `VIRTIO_SCSI_S_ABORTED` | `2` |  |
| `VIRTIO_SCSI_S_BAD_TARGET` | `3` |  |
| `VIRTIO_SCSI_S_RESET` | `4` |  |
| `VIRTIO_SCSI_S_BUSY` | `5` |  |
| `VIRTIO_SCSI_S_TRANSPORT_FAILURE` | `6` |  |
| `VIRTIO_SCSI_S_TARGET_FAILURE` | `7` |  |
| `VIRTIO_SCSI_S_NEXUS_FAILURE` | `8` |  |
| `VIRTIO_SCSI_S_FAILURE` | `9` |  |
| `VIRTIO_SCSI_S_FUNCTION_SUCCEEDED` | `10` |  |
| `VIRTIO_SCSI_S_FUNCTION_REJECTED` | `11` |  |
| `VIRTIO_SCSI_S_INCORRECT_LUN` | `12` |  |
| `VIRTIO_SCSI_T_TMF` | `0` |  |
| `VIRTIO_SCSI_T_AN_QUERY` | `1` |  |
| `VIRTIO_SCSI_T_AN_SUBSCRIBE` | `2` |  |
| `VIRTIO_SCSI_T_TMF_ABORT_TASK` | `0` |  |
| `VIRTIO_SCSI_T_TMF_ABORT_TASK_SET` | `1` |  |
| `VIRTIO_SCSI_T_TMF_CLEAR_ACA` | `2` |  |
| `VIRTIO_SCSI_T_TMF_CLEAR_TASK_SET` | `3` |  |
| `VIRTIO_SCSI_T_TMF_I_T_NEXUS_RESET` | `4` |  |
| `VIRTIO_SCSI_T_TMF_LOGICAL_UNIT_RESET` | `5` |  |
| `VIRTIO_SCSI_T_TMF_QUERY_TASK` | `6` |  |
| `VIRTIO_SCSI_T_TMF_QUERY_TASK_SET` | `7` |  |
| `VIRTIO_SCSI_T_EVENTS_MISSED` | `0x80000000` |  |
| `VIRTIO_SCSI_T_NO_EVENT` | `0` |  |
| `VIRTIO_SCSI_T_TRANSPORT_RESET` | `1` |  |
| `VIRTIO_SCSI_T_ASYNC_NOTIFY` | `2` |  |
| `VIRTIO_SCSI_T_PARAM_CHANGE` | `3` |  |
| `VIRTIO_SCSI_EVT_RESET_HARD` | `0` |  |
| `VIRTIO_SCSI_EVT_RESET_RESCAN` | `1` |  |
| `VIRTIO_SCSI_EVT_RESET_REMOVED` | `2` |  |
| `VIRTIO_SCSI_S_SIMPLE` | `0` |  |
| `VIRTIO_SCSI_S_ORDERED` | `1` |  |
| `VIRTIO_SCSI_S_HEAD` | `2` |  |
| `VIRTIO_SCSI_S_ACA` | `3` |  |

## Structs (9)


### `struct virtio_scsi_cmd_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `lun` | `8` |
| `__virtio64` | `tag` | `-` |
| `__u8` | `task_attr` | `-` |
| `__u8` | `prio` | `-` |
| `__u8` | `crn` | `-` |
| `__u8` | `cdb` | `VIRTIO_SCSI_CDB_SIZE` |

### `struct virtio_scsi_cmd_req_pi`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `lun` | `8` |
| `__virtio64` | `tag` | `-` |
| `__u8` | `task_attr` | `-` |
| `__u8` | `prio` | `-` |
| `__u8` | `crn` | `-` |
| `__virtio32` | `pi_bytesout` | `-` |
| `__virtio32` | `pi_bytesin` | `-` |
| `__u8` | `cdb` | `VIRTIO_SCSI_CDB_SIZE` |

### `struct virtio_scsi_cmd_resp`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `sense_len` | `-` |
| `__virtio32` | `resid` | `-` |
| `__virtio16` | `status_qualifier` | `-` |
| `__u8` | `status` | `-` |
| `__u8` | `response` | `-` |
| `__u8` | `sense` | `VIRTIO_SCSI_SENSE_SIZE` |

### `struct virtio_scsi_ctrl_tmf_req`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `type` | `-` |
| `__virtio32` | `subtype` | `-` |
| `__u8` | `lun` | `8` |
| `__virtio64` | `tag` | `-` |

### `struct virtio_scsi_ctrl_tmf_resp`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `response` | `-` |

### `struct virtio_scsi_ctrl_an_req`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `type` | `-` |
| `__u8` | `lun` | `8` |
| `__virtio32` | `event_requested` | `-` |

### `struct virtio_scsi_ctrl_an_resp`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `event_actual` | `-` |
| `__u8` | `response` | `-` |

### `struct virtio_scsi_event`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `event` | `-` |
| `__u8` | `lun` | `8` |
| `__virtio32` | `reason` | `-` |

### `struct virtio_scsi_config`

| Type | Field | Array |
|------|-------|-------|
| `__virtio32` | `num_queues` | `-` |
| `__virtio32` | `seg_max` | `-` |
| `__virtio32` | `max_sectors` | `-` |
| `__virtio32` | `cmd_per_lun` | `-` |
| `__virtio32` | `event_info_size` | `-` |
| `__virtio32` | `sense_size` | `-` |
| `__virtio32` | `cdb_size` | `-` |
| `__virtio16` | `max_channel` | `-` |
| `__virtio16` | `max_target` | `-` |
| `__virtio32` | `max_lun` | `-` |