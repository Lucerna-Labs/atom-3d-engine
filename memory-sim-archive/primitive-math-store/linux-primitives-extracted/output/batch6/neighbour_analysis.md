# neighbour.h

**Source:** `neighbour.h`


## Includes

- `linux/types.h`
- `linux/netlink.h`

## Defines (24 total)


### NTF_EXT (4)

| Name | Value | Comment |
|------|-------|---------|
| `NTF_EXT_LEARNED` | `(1 << 4)` |  |
| `NTF_EXT_MANAGED` | `(1 << 0)` |  |
| `NTF_EXT_LOCKED` | `(1 << 1)` |  |
| `NTF_EXT_EXT_VALIDATED` | `(1 << 2)` |  |

### UNCATEGORIZED (20)

| Name | Value | Comment |
|------|-------|---------|
| `NDA_MAX` | `(__NDA_MAX - 1)` |  |
| `NTF_USE` | `(1 << 0)` |  |
| `NTF_SELF` | `(1 << 1)` |  |
| `NTF_MASTER` | `(1 << 2)` |  |
| `NTF_PROXY` | `(1 << 3)` | == ATF_PUBL |
| `NTF_OFFLOADED` | `(1 << 5)` |  |
| `NTF_STICKY` | `(1 << 6)` |  |
| `NTF_ROUTER` | `(1 << 7)` |  |
| `NUD_INCOMPLETE` | `0x01` |  |
| `NUD_REACHABLE` | `0x02` |  |
| `NUD_STALE` | `0x04` |  |
| `NUD_DELAY` | `0x08` |  |
| `NUD_PROBE` | `0x10` |  |
| `NUD_FAILED` | `0x20` |  |
| `NUD_NOARP` | `0x40` |  |
| `NUD_PERMANENT` | `0x80` |  |
| `NUD_NONE` | `0x00` |  |
| `NDTPA_MAX` | `(__NDTPA_MAX - 1)` |  |
| `NDTA_MAX` | `(__NDTA_MAX - 1)` |  |
| `NFEA_MAX` | `(__NFEA_MAX - 1)` |  |

## Structs (5)


### `struct ndmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `ndm_family` | `-` |
| `__u8` | `ndm_pad1` | `-` |
| `__u16` | `ndm_pad2` | `-` |
| `__s32` | `ndm_ifindex` | `-` |
| `__u16` | `ndm_state` | `-` |
| `__u8` | `ndm_flags` | `-` |
| `__u8` | `ndm_type` | `-` |

### `struct nda_cacheinfo`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `ndm_confirmed` | `-` |
| `__u32` | `ndm_used` | `-` |
| `__u32` | `ndm_updated` | `-` |
| `__u32` | `ndm_refcnt` | `-` |

### `struct ndt_stats`

| Type | Field | Array |
|------|-------|-------|
| `__u64` | `ndts_allocs` | `-` |
| `__u64` | `ndts_destroys` | `-` |
| `__u64` | `ndts_hash_grows` | `-` |
| `__u64` | `ndts_res_failed` | `-` |
| `__u64` | `ndts_lookups` | `-` |
| `__u64` | `ndts_hits` | `-` |
| `__u64` | `ndts_rcv_probes_mcast` | `-` |
| `__u64` | `ndts_rcv_probes_ucast` | `-` |
| `__u64` | `ndts_periodic_gc_runs` | `-` |
| `__u64` | `ndts_forced_gc_runs` | `-` |
| `__u64` | `ndts_table_fulls` | `-` |

### `struct ndtmsg`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `ndtm_family` | `-` |
| `__u8` | `ndtm_pad1` | `-` |
| `__u16` | `ndtm_pad2` | `-` |

### `struct ndt_config`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `ndtc_key_len` | `-` |
| `__u16` | `ndtc_entry_size` | `-` |
| `__u32` | `ndtc_entries` | `-` |
| `__u32` | `ndtc_last_flush` | `-` |
| `__u32` | `ndtc_last_rand` | `-` |
| `__u32` | `ndtc_hash_rnd` | `-` |
| `__u32` | `ndtc_hash_mask` | `-` |
| `__u32` | `ndtc_hash_chain_gc` | `-` |
| `__u32` | `ndtc_proxy_qlen` | `-` |