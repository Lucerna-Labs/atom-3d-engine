# asequencer.h

**Source:** `asequencer.h`


## Includes

- `sound/asound.h`

## Defines (186 total)


### SNDRV_SEQ (183)

| Name | Value | Comment |
|------|-------|---------|
| `SNDRV_SEQ_VERSION` | `SNDRV_PROTOCOL_VERSION(1, 0, 5)` |  |
| `SNDRV_SEQ_EVENT_SYSTEM` | `0` |  |
| `SNDRV_SEQ_EVENT_RESULT` | `1` |  |
| `SNDRV_SEQ_EVENT_NOTE` | `5` |  |
| `SNDRV_SEQ_EVENT_NOTEON` | `6` |  |
| `SNDRV_SEQ_EVENT_NOTEOFF` | `7` |  |
| `SNDRV_SEQ_EVENT_KEYPRESS` | `8` |  |
| `SNDRV_SEQ_EVENT_CONTROLLER` | `10` |  |
| `SNDRV_SEQ_EVENT_PGMCHANGE` | `11` |  |
| `SNDRV_SEQ_EVENT_CHANPRESS` | `12` |  |
| `SNDRV_SEQ_EVENT_PITCHBEND` | `13` | *< from -8192 to 8191 |
| `SNDRV_SEQ_EVENT_CONTROL14` | `14` | *< 14 bit controller value |
| `SNDRV_SEQ_EVENT_NONREGPARAM` | `15` | *< 14 bit NRPN address + 14 bit unsigned value |
| `SNDRV_SEQ_EVENT_REGPARAM` | `16` | *< 14 bit RPN address + 14 bit unsigned value |
| `SNDRV_SEQ_EVENT_SONGPOS` | `20` | Song Position Pointer with LSB and MSB values |
| `SNDRV_SEQ_EVENT_SONGSEL` | `21` | Song Select with song ID number |
| `SNDRV_SEQ_EVENT_QFRAME` | `22` | midi time code quarter frame |
| `SNDRV_SEQ_EVENT_TIMESIGN` | `23` | SMF Time Signature event |
| `SNDRV_SEQ_EVENT_KEYSIGN` | `24` | SMF Key Signature event |
| `SNDRV_SEQ_EVENT_START` | `30` | midi Real Time Start message |
| `SNDRV_SEQ_EVENT_CONTINUE` | `31` | midi Real Time Continue message |
| `SNDRV_SEQ_EVENT_STOP` | `32` | midi Real Time Stop message |
| `SNDRV_SEQ_EVENT_SETPOS_TICK` | `33` | set tick queue position |
| `SNDRV_SEQ_EVENT_SETPOS_TIME` | `34` | set realtime queue position |
| `SNDRV_SEQ_EVENT_TEMPO` | `35` | (SMF) Tempo event |
| `SNDRV_SEQ_EVENT_CLOCK` | `36` | midi Real Time Clock message |
| `SNDRV_SEQ_EVENT_TICK` | `37` | midi Real Time Tick message |
| `SNDRV_SEQ_EVENT_QUEUE_SKEW` | `38` | skew queue tempo |
| `SNDRV_SEQ_EVENT_TUNE_REQUEST` | `40` | tune request |
| `SNDRV_SEQ_EVENT_RESET` | `41` | reset to power-on state |
| `SNDRV_SEQ_EVENT_SENSING` | `42` | "active sensing" event |
| `SNDRV_SEQ_EVENT_ECHO` | `50` | echo event |
| `SNDRV_SEQ_EVENT_OSS` | `51` | OSS raw event |
| `SNDRV_SEQ_EVENT_CLIENT_START` | `60` | new client has connected |
| `SNDRV_SEQ_EVENT_CLIENT_EXIT` | `61` | client has left the system |
| `SNDRV_SEQ_EVENT_CLIENT_CHANGE` | `62` | client status/info has changed |
| `SNDRV_SEQ_EVENT_PORT_START` | `63` | new port was created |
| `SNDRV_SEQ_EVENT_PORT_EXIT` | `64` | port was deleted from system |
| `SNDRV_SEQ_EVENT_PORT_CHANGE` | `65` | port status/info has changed |
| `SNDRV_SEQ_EVENT_PORT_SUBSCRIBED` | `66` | ports connected |
| `SNDRV_SEQ_EVENT_PORT_UNSUBSCRIBED` | `67` | ports disconnected |
| `SNDRV_SEQ_EVENT_UMP_EP_CHANGE` | `68` | UMP EP info has changed |
| `SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE` | `69` | UMP block info has changed |
| `SNDRV_SEQ_EVENT_USR0` | `90` |  |
| `SNDRV_SEQ_EVENT_USR1` | `91` |  |
| `SNDRV_SEQ_EVENT_USR2` | `92` |  |
| `SNDRV_SEQ_EVENT_USR3` | `93` |  |
| `SNDRV_SEQ_EVENT_USR4` | `94` |  |
| `SNDRV_SEQ_EVENT_USR5` | `95` |  |
| `SNDRV_SEQ_EVENT_USR6` | `96` |  |

*...and 133 more*

### UNCATEGORIZED (3)

| Name | Value | Comment |
|------|-------|---------|
| `NO_CLIENT` | `((__force snd_seq_client_type_t) 0)` |  |
| `USER_CLIENT` | `((__force snd_seq_client_type_t) 1)` |  |
| `KERNEL_CLIENT` | `((__force snd_seq_client_type_t) 2)` |  |

## Structs (31)


### `struct snd_seq_addr`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_connect`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_note`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_ctrl`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_raw8`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_raw32`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_ext`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_result`

| Type | Field | Array |
|------|-------|-------|
| `int` | `event` | `-` |
| `int` | `result` | `-` |

### `struct snd_seq_real_time`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_queue_skew`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_queue_control`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_quote`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_ev_ump_notify`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_event`

| Type | Field | Array |
|------|-------|-------|
| `snd_seq_event_type_t` | `type` | `-` |
| `char` | `tag` | `-` |

### `struct snd_seq_ump_event`

| Type | Field | Array |
|------|-------|-------|
| `snd_seq_event_type_t` | `type` | `-` |
| `char` | `tag` | `-` |

### `struct snd_seq_event_bounce`

| Type | Field | Array |
|------|-------|-------|
| `int` | `err` | `-` |

### `struct snd_seq_system_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `queues` | `-` |
| `int` | `clients` | `-` |
| `int` | `ports` | `-` |
| `int` | `channels` | `-` |
| `int` | `cur_clients` | `-` |
| `int` | `cur_queues` | `-` |
| `char` | `reserved` | `24` |

### `struct snd_seq_running_info`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_client_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `client` | `-` |
| `snd_seq_client_type_t` | `type` | `-` |
| `char` | `name` | `64` |
| `int` | `num_ports` | `-` |
| `int` | `event_lost` | `-` |
| `int` | `card` | `-` |
| `int` | `pid` | `-` |
| `char` | `reserved` | `48` |

### `struct snd_seq_client_pool`

| Type | Field | Array |
|------|-------|-------|
| `int` | `client` | `-` |
| `int` | `output_pool` | `-` |
| `int` | `input_pool` | `-` |
| `int` | `output_room` | `-` |
| `int` | `output_free` | `-` |
| `int` | `input_free` | `-` |
| `char` | `reserved` | `64` |

### `struct snd_seq_remove_events`

| Type | Field | Array |
|------|-------|-------|
| `int` | `type` | `-` |
| `char` | `tag` | `-` |
| `int` | `reserved` | `10` |

### `struct snd_seq_port_info`

| Type | Field | Array |
|------|-------|-------|
| `char` | `name` | `64` |
| `int` | `midi_channels` | `-` |
| `int` | `midi_voices` | `-` |
| `int` | `synth_voices` | `-` |
| `int` | `read_use` | `-` |
| `int` | `write_use` | `-` |
| `char` | `reserved` | `57` |

### `struct snd_seq_queue_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `queue` | `-` |
| `int` | `owner` | `-` |
| `char` | `name` | `64` |
| `char` | `reserved` | `60` |

### `struct snd_seq_queue_status`

| Type | Field | Array |
|------|-------|-------|
| `int` | `queue` | `-` |
| `int` | `events` | `-` |
| `snd_seq_tick_time_t` | `tick` | `-` |
| `int` | `running` | `-` |
| `int` | `flags` | `-` |
| `char` | `reserved` | `64` |

### `struct snd_seq_queue_tempo`

| Type | Field | Array |
|------|-------|-------|
| `int` | `queue` | `-` |
| `int` | `ppq` | `-` |
| `char` | `reserved` | `22` |

### `struct snd_seq_queue_timer`

| Type | Field | Array |
|------|-------|-------|
| `int` | `queue` | `-` |
| `int` | `type` | `-` |
| `char` | `reserved` | `64` |

### `struct anonymous_26`

| Type | Field | Array |
|------|-------|-------|

### `struct snd_seq_queue_client`

| Type | Field | Array |
|------|-------|-------|
| `int` | `queue` | `-` |
| `int` | `client` | `-` |
| `int` | `used` | `-` |
| `char` | `reserved` | `64` |

### `struct snd_seq_port_subscribe`

| Type | Field | Array |
|------|-------|-------|
| `char` | `reserved` | `64` |

### `struct snd_seq_query_subs`

| Type | Field | Array |
|------|-------|-------|
| `int` | `type` | `-` |
| `int` | `index` | `-` |
| `int` | `num_subs` | `-` |
| `char` | `reserved` | `64` |

### `struct snd_seq_client_ump_info`

| Type | Field | Array |
|------|-------|-------|
| `int` | `client` | `-` |
| `int` | `type` | `-` |