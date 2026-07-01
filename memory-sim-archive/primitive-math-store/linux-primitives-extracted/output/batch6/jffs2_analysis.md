# jffs2.h

**Source:** `jffs2.h`


## Includes

- `linux/types.h`
- `linux/magic.h`

## Defines (37 total)


### KSAMTIB_CIGAM (1)

| Name | Value | Comment |
|------|-------|---------|
| `KSAMTIB_CIGAM_2SFFJ` | `0x8519` | For detecting wrong-endian fs |

### UNCATEGORIZED (36)

| Name | Value | Comment |
|------|-------|---------|
| `JFFS2_OLD_MAGIC_BITMASK` | `0x1984` |  |
| `JFFS2_MAGIC_BITMASK` | `0x1985` |  |
| `JFFS2_EMPTY_BITMASK` | `0xffff` |  |
| `JFFS2_DIRTY_BITMASK` | `0x0000` |  |
| `JFFS2_SUM_MAGIC` | `0x02851885` |  |
| `JFFS2_MAX_NAME_LEN` | `254` |  |
| `JFFS2_MIN_DATA_LEN` | `128` |  |
| `JFFS2_COMPR_NONE` | `0x00` |  |
| `JFFS2_COMPR_ZERO` | `0x01` |  |
| `JFFS2_COMPR_RTIME` | `0x02` |  |
| `JFFS2_COMPR_RUBINMIPS` | `0x03` |  |
| `JFFS2_COMPR_COPY` | `0x04` |  |
| `JFFS2_COMPR_DYNRUBIN` | `0x05` |  |
| `JFFS2_COMPR_ZLIB` | `0x06` |  |
| `JFFS2_COMPR_LZO` | `0x07` |  |
| `JFFS2_COMPAT_MASK` | `0xc000` | What do to if an unknown nodetype is found |
| `JFFS2_NODE_ACCURATE` | `0x2000` |  |
| `JFFS2_FEATURE_INCOMPAT` | `0xc000` |  |
| `JFFS2_FEATURE_ROCOMPAT` | `0x8000` |  |
| `JFFS2_FEATURE_RWCOMPAT_COPY` | `0x4000` |  |
| `JFFS2_FEATURE_RWCOMPAT_DELETE` | `0x0000` |  |
| `JFFS2_NODETYPE_DIRENT` | `(JFFS2_FEATURE_INCOMPAT \| JFFS2_NODE_ACCURATE \| 1)` |  |
| `JFFS2_NODETYPE_INODE` | `(JFFS2_FEATURE_INCOMPAT \| JFFS2_NODE_ACCURATE \| 2)` |  |
| `JFFS2_NODETYPE_CLEANMARKER` | `(JFFS2_FEATURE_RWCOMPAT_DELETE \| JFFS2_NODE_ACCURATE \| 3)` |  |
| `JFFS2_NODETYPE_PADDING` | `(JFFS2_FEATURE_RWCOMPAT_DELETE \| JFFS2_NODE_ACCURATE \| 4)` |  |
| `JFFS2_NODETYPE_SUMMARY` | `(JFFS2_FEATURE_RWCOMPAT_DELETE \| JFFS2_NODE_ACCURATE \| 6)` |  |
| `JFFS2_NODETYPE_XATTR` | `(JFFS2_FEATURE_INCOMPAT \| JFFS2_NODE_ACCURATE \| 8)` |  |
| `JFFS2_NODETYPE_XREF` | `(JFFS2_FEATURE_INCOMPAT \| JFFS2_NODE_ACCURATE \| 9)` |  |
| `JFFS2_XPREFIX_USER` | `1` | for "user." |
| `JFFS2_XPREFIX_SECURITY` | `2` | for "security." |
| `JFFS2_XPREFIX_ACL_ACCESS` | `3` | for "system.posix_acl_access" |
| `JFFS2_XPREFIX_ACL_DEFAULT` | `4` | for "system.posix_acl_default" |
| `JFFS2_XPREFIX_TRUSTED` | `5` | for "trusted.*" |
| `JFFS2_ACL_VERSION` | `0x0001` |  |
| `JFFS2_INO_FLAG_PREREAD` | `1	/* Do read_inode() for this one at` |  |
| `JFFS2_INO_FLAG_USERCOMPR` | `2	/* User has requested a specific` |  |

## Structs (9)


### `struct anonymous_0`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `v32` | `-` |

### `struct anonymous_1`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `m` | `-` |

### `struct anonymous_2`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `v16` | `-` |

### `struct jffs2_unknown_node`

| Type | Field | Array |
|------|-------|-------|
| `jint16_t` | `magic` | `-` |
| `jint16_t` | `nodetype` | `-` |
| `jint32_t` | `totlen` | `-` |
| `jint32_t` | `hdr_crc` | `-` |

### `struct jffs2_raw_dirent`

| Type | Field | Array |
|------|-------|-------|
| `jint16_t` | `magic` | `-` |
| `jint16_t` | `nodetype` | `-` |
| `jint32_t` | `totlen` | `-` |
| `jint32_t` | `hdr_crc` | `-` |
| `jint32_t` | `pino` | `-` |
| `jint32_t` | `version` | `-` |
| `jint32_t` | `ino` | `-` |
| `jint32_t` | `mctime` | `-` |
| `__u8` | `nsize` | `-` |
| `__u8` | `type` | `-` |
| `__u8` | `unused` | `2` |
| `jint32_t` | `node_crc` | `-` |
| `jint32_t` | `name_crc` | `-` |

### `struct jffs2_raw_inode`

| Type | Field | Array |
|------|-------|-------|
| `jint16_t` | `magic` | `-` |
| `jint16_t` | `nodetype` | `-` |
| `jint32_t` | `totlen` | `-` |
| `jint32_t` | `hdr_crc` | `-` |
| `jint32_t` | `ino` | `-` |
| `jint32_t` | `version` | `-` |
| `jmode_t` | `mode` | `-` |
| `jint16_t` | `uid` | `-` |
| `jint16_t` | `gid` | `-` |
| `jint32_t` | `isize` | `-` |
| `jint32_t` | `atime` | `-` |
| `jint32_t` | `mtime` | `-` |
| `jint32_t` | `ctime` | `-` |
| `jint32_t` | `offset` | `-` |
| `jint32_t` | `csize` | `-` |
| `jint32_t` | `dsize` | `-` |
| `__u8` | `compr` | `-` |
| `__u8` | `usercompr` | `-` |
| `jint16_t` | `flags` | `-` |
| `jint32_t` | `data_crc` | `-` |
| `jint32_t` | `node_crc` | `-` |

### `struct jffs2_raw_xattr`

| Type | Field | Array |
|------|-------|-------|
| `jint16_t` | `magic` | `-` |
| `jint16_t` | `nodetype` | `-` |
| `jint32_t` | `totlen` | `-` |
| `jint32_t` | `hdr_crc` | `-` |
| `jint32_t` | `xid` | `-` |
| `jint32_t` | `version` | `-` |
| `__u8` | `xprefix` | `-` |
| `__u8` | `name_len` | `-` |
| `jint16_t` | `value_len` | `-` |
| `jint32_t` | `data_crc` | `-` |
| `jint32_t` | `node_crc` | `-` |

### `struct jffs2_raw_xref`

| Type | Field | Array |
|------|-------|-------|
| `jint16_t` | `magic` | `-` |
| `jint16_t` | `nodetype` | `-` |
| `jint32_t` | `totlen` | `-` |
| `jint32_t` | `hdr_crc` | `-` |
| `jint32_t` | `ino` | `-` |
| `jint32_t` | `xid` | `-` |
| `jint32_t` | `xseqno` | `-` |
| `jint32_t` | `node_crc` | `-` |

### `struct jffs2_raw_summary`

| Type | Field | Array |
|------|-------|-------|
| `jint16_t` | `magic` | `-` |
| `jint16_t` | `nodetype` | `-` |
| `jint32_t` | `totlen` | `-` |
| `jint32_t` | `hdr_crc` | `-` |
| `jint32_t` | `sum_num` | `-` |
| `jint32_t` | `cln_mkr` | `-` |
| `jint32_t` | `padded` | `-` |
| `jint32_t` | `sum_crc` | `-` |
| `jint32_t` | `node_crc` | `-` |