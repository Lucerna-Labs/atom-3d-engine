# virtio_fs.h

**Source:** `virtio_fs.h`


## Includes

- `linux/types.h`
- `linux/virtio_ids.h`
- `linux/virtio_config.h`
- `linux/virtio_types.h`

## Defines (1 total)


### VIRTIO_FS (1)

| Name | Value | Comment |
|------|-------|---------|
| `VIRTIO_FS_SHMCAP_ID_CACHE` | `0` |  |

## Structs (1)


### `struct virtio_fs_config`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `tag` | `36` |
| `__le32` | `num_request_queues` | `-` |