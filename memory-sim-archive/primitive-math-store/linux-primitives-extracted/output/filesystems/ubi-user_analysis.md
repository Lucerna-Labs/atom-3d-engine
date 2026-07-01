# ubi-user.h

**Source:** `ubi-user.h`


## Includes

- `linux/types.h`

## Defines (27 total)


### MAX_UBI (1)

| Name | Value | Comment |
|------|-------|---------|
| `MAX_UBI_MTD_NAME_LEN` | `127` |  |

### UBI_CTRL (1)

| Name | Value | Comment |
|------|-------|---------|
| `UBI_CTRL_IOC_MAGIC` | `'o'` |  |

### UBI_DEV (1)

| Name | Value | Comment |
|------|-------|---------|
| `UBI_DEV_NUM_AUTO` | `(-1)` |  |

### UBI_IOC (1)

| Name | Value | Comment |
|------|-------|---------|
| `UBI_IOC_MAGIC` | `'o'` |  |

### UBI_MAX (2)

| Name | Value | Comment |
|------|-------|---------|
| `UBI_MAX_VOLUME_NAME` | `127` |  |
| `UBI_MAX_RNVOL` | `32` |  |

### UBI_VOL (3)

| Name | Value | Comment |
|------|-------|---------|
| `UBI_VOL_NUM_AUTO` | `(-1)` |  |
| `UBI_VOL_IOC_MAGIC` | `'O'` |  |
| `UBI_VOL_VALID_FLGS` | `(UBI_VOL_SKIP_CRC_CHECK_FLG)` |  |

### UNCATEGORIZED (18)

| Name | Value | Comment |
|------|-------|---------|
| `UBI_IOCMKVOL` | `_IOW(UBI_IOC_MAGIC, 0, struct ubi_mkvol_req)` |  |
| `UBI_IOCRMVOL` | `_IOW(UBI_IOC_MAGIC, 1, __s32)` |  |
| `UBI_IOCRSVOL` | `_IOW(UBI_IOC_MAGIC, 2, struct ubi_rsvol_req)` |  |
| `UBI_IOCRNVOL` | `_IOW(UBI_IOC_MAGIC, 3, struct ubi_rnvol_req)` |  |
| `UBI_IOCRPEB` | `_IOW(UBI_IOC_MAGIC, 4, __s32)` |  |
| `UBI_IOCSPEB` | `_IOW(UBI_IOC_MAGIC, 5, __s32)` |  |
| `UBI_IOCECNFO` | `_IOWR(UBI_IOC_MAGIC, 6, struct ubi_ecinfo_req)` |  |
| `UBI_IOCATT` | `_IOW(UBI_CTRL_IOC_MAGIC, 64, struct ubi_attach_req)` |  |
| `UBI_IOCDET` | `_IOW(UBI_CTRL_IOC_MAGIC, 65, __s32)` |  |
| `UBI_IOCVOLUP` | `_IOW(UBI_VOL_IOC_MAGIC, 0, __s64)` |  |
| `UBI_IOCEBER` | `_IOW(UBI_VOL_IOC_MAGIC, 1, __s32)` |  |
| `UBI_IOCEBCH` | `_IOW(UBI_VOL_IOC_MAGIC, 2, __s32)` |  |
| `UBI_IOCEBMAP` | `_IOW(UBI_VOL_IOC_MAGIC, 3, struct ubi_map_req)` |  |
| `UBI_IOCEBUNMAP` | `_IOW(UBI_VOL_IOC_MAGIC, 4, __s32)` |  |
| `UBI_IOCEBISMAP` | `_IOR(UBI_VOL_IOC_MAGIC, 5, __s32)` |  |
| `UBI_IOCSETVOLPROP` | `_IOW(UBI_VOL_IOC_MAGIC, 6, ` |  |
| `UBI_IOCVOLCRBLK` | `_IOW(UBI_VOL_IOC_MAGIC, 7, struct ubi_blkcreate_req)` |  |
| `UBI_IOCVOLRMBLK` | `_IO(UBI_VOL_IOC_MAGIC, 8)` |  |

## Structs (10)


### `struct ubi_attach_req`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `ubi_num` | `-` |
| `__s32` | `mtd_num` | `-` |
| `__s32` | `vid_hdr_offset` | `-` |
| `__s16` | `max_beb_per1024` | `-` |
| `__s8` | `disable_fm` | `-` |
| `__s8` | `need_resv_pool` | `-` |
| `__s8` | `padding` | `8` |

### `struct ubi_mkvol_req`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `vol_id` | `-` |
| `__s32` | `alignment` | `-` |
| `__s64` | `bytes` | `-` |
| `__s8` | `vol_type` | `-` |
| `__u8` | `flags` | `-` |
| `__s16` | `name_len` | `-` |
| `__s8` | `padding2` | `4` |
| `char` | `name` | `UBI_MAX_VOLUME_NAME + 1` |

### `struct ubi_rsvol_req`

| Type | Field | Array |
|------|-------|-------|
| `__s64` | `bytes` | `-` |
| `__s32` | `vol_id` | `-` |

### `struct ubi_rnvol_req`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `count` | `-` |
| `__s8` | `padding1` | `12` |
| `__s32` | `vol_id` | `-` |
| `__s16` | `name_len` | `-` |
| `__s8` | `padding2` | `2` |
| `char` | `name` | `UBI_MAX_VOLUME_NAME + 1` |

### `struct anonymous_4`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `vol_id` | `-` |
| `__s16` | `name_len` | `-` |
| `__s8` | `padding2` | `2` |
| `char` | `name` | `UBI_MAX_VOLUME_NAME + 1` |

### `struct ubi_ecinfo_req`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `start` | `-` |
| `__s32` | `length` | `-` |
| `__s32` | `read_length` | `-` |
| `__s8` | `padding` | `16` |

### `struct ubi_leb_change_req`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `lnum` | `-` |
| `__s32` | `bytes` | `-` |
| `__s8` | `dtype` | `-` |
| `__s8` | `padding` | `7` |

### `struct ubi_map_req`

| Type | Field | Array |
|------|-------|-------|
| `__s32` | `lnum` | `-` |
| `__s8` | `dtype` | `-` |
| `__s8` | `padding` | `3` |

### `struct ubi_set_vol_prop_req`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `property` | `-` |
| `__u8` | `padding` | `7` |
| `__u64` | `value` | `-` |

### `struct ubi_blkcreate_req`

| Type | Field | Array |
|------|-------|-------|
| `__s8` | `padding` | `128` |