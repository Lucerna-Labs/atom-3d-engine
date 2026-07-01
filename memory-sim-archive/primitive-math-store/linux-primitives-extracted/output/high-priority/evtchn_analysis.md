# evtchn.h

**Source:** `evtchn.h`


## Defines (8 total)


### IOCTL_EVTCHN (8)

| Name | Value | Comment |
|------|-------|---------|
| `IOCTL_EVTCHN_BIND_VIRQ` | `` |  |
| `IOCTL_EVTCHN_BIND_INTERDOMAIN` | `` |  |
| `IOCTL_EVTCHN_BIND_UNBOUND_PORT` | `` |  |
| `IOCTL_EVTCHN_UNBIND` | `` |  |
| `IOCTL_EVTCHN_NOTIFY` | `` |  |
| `IOCTL_EVTCHN_RESET` | `` |  |
| `IOCTL_EVTCHN_RESTRICT_DOMID` | `` |  |
| `IOCTL_EVTCHN_BIND_STATIC` | `` |  |

## Structs (7)


### `struct ioctl_evtchn_bind_virq`

| Type | Field | Array |
|------|-------|-------|

### `struct ioctl_evtchn_bind_interdomain`

| Type | Field | Array |
|------|-------|-------|

### `struct ioctl_evtchn_bind_unbound_port`

| Type | Field | Array |
|------|-------|-------|

### `struct ioctl_evtchn_unbind`

| Type | Field | Array |
|------|-------|-------|

### `struct ioctl_evtchn_notify`

| Type | Field | Array |
|------|-------|-------|

### `struct ioctl_evtchn_restrict_domid`

| Type | Field | Array |
|------|-------|-------|
| `domid_t` | `domid` | `-` |

### `struct ioctl_evtchn_bind`

| Type | Field | Array |
|------|-------|-------|