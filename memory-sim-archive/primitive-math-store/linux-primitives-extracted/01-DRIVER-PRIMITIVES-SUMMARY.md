# Linux Driver Primitives Extraction

**Date:** 2026-06-20  
**Source:** `F:\OPENCLAW-PROJECTS\linux-master/include/uapi/`  
**Category:** Device driver interfaces (graphics, media, audio, bus, storage, network)

---

## Parsed Driver Headers (13)

| Subsystem | Headers | Defines | Structs | Key Patterns |
|-----------|---------|---------|---------|--------------|
| **Graphics (DRM)** | `drm.h`, `drm_mode.h` | 320 | 104 | Buffer objects, modesetting, planes, CRTCs, encoders, connectors |
| **Video (V4L2)** | `v4l2-common.h`, `v4l2-controls.h` | 828 | 48 | Control classes, pixel formats, buffer queues, streaming |
| **Audio (ALSA)** | `asound.h` | 356 | 51 | PCM streams, MIDI, mixer, sequencer, hwdep |
| **PCI** | `pci.h` | 5 | 0 | Config space access, MSI/MSI-X |
| **I2C** | `i2c-dev.h` | 11 | 2 | SMBus protocol, device emulation |
| **GPIO** | `gpio.h` | 40 | 16 | Line request, event monitoring, chip info |
| **USB** | `usbdevice_fs.h` | 60 | 13 | Endpoint claims, URB submission, reclaim |
| **VirtIO** | `virtio_pci.h`, `virtio_net.h`, `virtio_blk.h` | 269 | 73 | Virtqueue rings, feature negotiation, config space |
| **NVMe** | `nvme_ioctl.h` | 15 | 4 | Namespace management, passthrough commands |

---

## Key Driver Primitive Patterns

### 1. DRM (Direct Rendering Manager) - Graphics Stack

**Core abstraction:** Everything is an object with ID and properties.

**Object types:**
- **Framebuffer** - Pixel buffer with format/size/stride
- **CRTC** - Scanout engine (reads framebuffer, generates timing)
- **Plane** - Image composition layer (primary, cursor, overlay)
- **Encoder** - Converts CRTC output to signal format
- **Connector** - Physical port (HDMI, DP, VGA) with EDID
- **Property** - Typed key-value pairs on objects (enums, ranges, blobs)

**Ioctl categories:**
```
DRM_IOCTL_VERSION          - Query driver/version info
DRM_IOCTL_GET_CAP          - Query capabilities
DRM_IOCTL_GEM_*            - Graphics Execution Manager (buffer objects)
DRM_IOCTL_PRIME_*          - DMA-buf sharing (cross-device)
DRM_IOCTL_MODE_*           - Modesetting operations
DRM_IOCTL_SYNCOBJ_*        - Sync objects (fence-based synchronization)
```

**Buffer management:**
- GEM (Graphics Execution Manager): `drm_gem_close`, `drm_gem_flink`, `drm_gem_open`
- PRIME: Import/export dma-buf fds across devices
- Framebuffer creation: `drm_mode_fb_cmd2` with format modifiers

**Modesetting flow:**
1. Get resources: `DRM_IOCTL_MODE_GETRESOURCES` → CRTCs, connectors, encoders
2. Set CRTC config: `DRM_IOCTL_MODE_SETCRTC` → framebuffer + mode + connectors
3. Page flip: `DRM_IOCTL_MODE_PAGE_FLIP` → atomic or legacy flip
4. Wait for vblank: `DRM_IOCTL_MODE_WAIT_VBLANK` → event-driven sync

**Atomic API:**
- `drm_mode_atomic_req_property` - Batch multiple property changes
- `DRM_MODE_ATOMIC_NONBLOCK` - Don't block, return `-EBUSY` if not ready
- `DRM_MODE_ATOMIC_TEST_ONLY` - Validate without applying
- All-or-nothing commit semantics

**Ordo Application:** Resource lifecycle management (create → configure → use → destroy), atomic state commits, property-based configuration.

---

### 2. V4L2 (Video4Linux2) - Video Capture/Output

**Control hierarchy:**
```
V4L2_CTRL_CLASS_USER       (0x00980000) - Legacy user controls
V4L2_CTRL_CLASS_CODEC      (0x00990000) - Stateful codec controls
V4L2_CTRL_CLASS_CAMERA     (0x009a0000) - Camera class controls
V4L2_CTRL_CLASS_FM_TX      (0x009b0000) - FM Modulator
V4L2_CTRL_CLASS_FLASH      (0x009c0000) - Flash controls
V4L2_CTRL_CLASS_JPEG       (0x009d0000) - JPEG compression
V4L2_CTRL_CLASS_IMAGE_SRC  (0x009e0000) - Image source
V4L2_CTRL_CLASS_IMAGE_PROC (0x009f0000) - Image processing
V4L2_CTRL_CLASS_DV         (0x00a00000) - Digital Video
V4L2_CTRL_CLASS_FM_RX      (0x00a10000) - FM Receiver
V4L2_CTRL_CLASS_RF_TUNER   (0x00a20000) - RF tuner
V4L2_CTRL_CLASS_DETECT     (0x00a30000) - Detection
V4L2_CTRL_CLASS_CODEC_STATELESS (0x00a40000) - Stateless codecs
V4L2_CTRL_CLASS_COLORIMETRY     (0x00a50000) - Colorimetry
```

**User controls (80+):**
- Basic: `BRIGHTNESS`, `CONTRAST`, `SATURATION`, `HUE`
- Audio: `AUDIO_VOLUME`, `AUDIO_BALANCE`, `AUDIO_BASS`, `AUDIO_TREBLE`, `AUDIO_MUTE`
- White balance: `AUTO_WHITE_BALANCE`, `WHITE_BALANCE_TEMPERATURE`, `RED_BALANCE`, `BLUE_BALANCE`
- Gain: `EXPOSURE`, `AUTOGAIN`, `GAIN`
- Geometry: `HFLIP`, `VFLIP`, `ROTATE`
- Effects: `COLORFX`, `SHARPNESS`, `BACKLIGHT_COMPENSATION`, `CHROMA_AGC`, `COLOR_KILLER`

**Control types:**
- Integer: min/max/step/default
- Boolean: 0 or 1
- Menu: enumerated string options
- Button: write-only trigger
- Integer64: 64-bit values
- String: variable-length strings
- Bitmask: bit vector
- Integer menu: enumerated integer options
- Compound: struct/array payloads (H.264 SPS, VP8 frame params, etc.)

**Buffer queueing:**
- MMAP: Kernel allocates, userspace maps
- USERPTR: Userspace allocates, passes pointers
- DMABUF: File descriptor import/export
- Queue ops: `VIDIOC_QBUF` (enqueue), `VIDIOC_DQBUF` (dequeue), `VIDIOC_STREAMON`, `VIDIOC_STREAMOFF`

**Pixel formats:**
- FourCC codes: `'RGB3'`, `'YUYV'`, `'MJPG'`, `'H264'`, etc.
- Format info: `VIDIOC_ENUM_FMT`, `VIDIOC_S_FMT`, `VIDIOC_G_FMT`
- Plane separation: Multi-planar API for YUV (separate Y/U/V planes)

**Ordo Application:** Parameterized control hierarchies (class → control → value), streaming buffer queues, format negotiation.

---

### 3. ALSA (Advanced Linux Sound Architecture)

**Card hierarchy:**
```
Card N
├── PCM device M         (playback/capture streams)
│   ├── Substream 0
│   ├── Substream 1
│   └── ...
├── Mixer                (controls: volume, mute, switches)
├── Sequencer            (MIDI routing, synthesis)
├── RawHW                (hwdep: device-specific interface)
└── Timer                (high-resolution timers)
```

**PCM interface:**
- Open: `snd_pcm_open(device, stream_type)` → `snd_pcm_t*`
- Setup: `snd_pcm_hw_params_set_*()` (format, channels, rate, period size, buffer size)
- Start: `snd_pcm_start()`
- Transfer: `snd_pcm_writei()` / `snd_pcm_readi()` (interleaved) or `snd_pcm_writen()` (non-interleaved)
- Status: `snd_pcm_avail_update()` - available frames
- Sync: `snd_pcm_wait()` - block until ready
- Drain: `snd_pcm_drain()` - finish playback

**Data formats:**
- Signed/unsigned: `S8`, `U8`, `S16_LE`, `S16_BE`, `S24_LE`, `S32_LE`, `FLOAT_LE`, `FLOAT64_LE`
- Channels: 1 (mono) to 8+ (surround)
- Rates: 8000, 11025, 16000, 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000 Hz

**Mixer controls:**
- Types: `bool`, `int`, `enum`
- Namespacing: `"Master Playback Volume"`, `"Headphone Playback Switch"`, `"Mic Boost Capture"`
- Value ranges: 0-63, -6528 to 0 dB, etc.
- Events: Poll for control changes

**Sequencer (MIDI):**
- Clients and ports: Addressed as `client:port` (e.g., `128:0`)
- Event types: Note On/Off, Control Change, Program Change, Pitch Bend, SysEx
- Subscription: Connect ports for automatic routing
- Time stamping: Real-time or monotonic clock

**Ordo Application:** Hierarchical resource model (card → device → subdevice), streaming data transfer with explicit buffer management, event-driven control changes.

---

### 4. GPIO (General Purpose I/O)

**Chip model:**
```c
struct gpiochip_info {
    char name[32];           // Chip name
    char label[32];          // Driver label
    __u32 lines;             // Number of GPIO lines
    __u32 flags;             // GPIOCHIP_* flags
};
```

**Line request:**
```c
struct gpiohandle_request {
    struct gpiohandle_data lineoffsets[64];  // Line offsets
    __u32 flags;              // GPIOHANDLE_REQUEST_*
    char consumer_label[32];  // Requestor label
    int fd;                   // Returned file descriptor
};
```

**Request flags:**
- `GPIOHANDLE_REQUEST_INPUT` - Input mode
- `GPIOHANDLE_REQUEST_OUTPUT` - Output mode
- `GPIOHANDLE_REQUEST_ACTIVE_LOW` - Inverted polarity
- `GPIOHANDLE_REQUEST_OPEN_DRAIN` - Open-drain mode
- `GPIOHANDLE_REQUEST_OPEN_SOURCE` - Open-source mode
- `GPIOHANDLE_REQUEST_BIAS_PULL_UP` - Pull-up resistor
- `GPIOHANDLE_REQUEST_BIAS_PULL_DOWN` - Pull-down resistor
- `GPIOHANDLE_REQUEST_BIAS_DISABLE` - No bias
- `GPIOHANDLE_REQUEST_EVENT_BOTH_EDGES` - Edge event monitoring
- `GPIOHANDLE_REQUEST_EVENT_RISING_EDGE` - Rising edge events
- `GPIOHANDLE_REQUEST_EVENT_FALLING_EDGE` - Falling edge events

**Operations:**
- Get value: `GPIOHANDLE_GET_LINE_VALUES_IOCTL`
- Set value: `GPIOHANDLE_SET_LINE_VALUES_IOCTL`
- Get config: `GPIOHANDLE_GET_CONFIG_IOCTL`
- Set config: `GPIOHANDLE_SET_CONFIG_IOCTL`
- Line info: `GPIO_GET_LINEINFO_IOCTL`

**Event reading:**
```c
struct gpioline_event {
    __s64 timestamp;      // Nanoseconds
    __u32 id;             // GPIOEVENT_EVENT_RISING/FALLING
    __u32 offset;         // Line offset within chip
};
```

**Ordo Application:** Resource request with explicit flags, event-driven state changes, bulk operations on multiple resources.

---

### 5. USB Device Filesystem

**Device claim:**
```c
struct usbdevfs_claiminterface {
    unsigned int interface;
};
// IOCTL: USBDEVFS_CLAIMINTERFACE, USBDEVFS_RELEASEINTERFACE
```

**URB (USB Request Block) submission:**
```c
struct usbdevfs_urb {
    unsigned char type;          // PIPE_CONTROL, PIPE_BULK, PIPE_INTERRUPT, PIPE_ISOCHRONOUS
    unsigned char endpoint;      // Endpoint number + direction
    unsigned int flags;          // Short-not-ok, zero-packet, etc.
    void *buffer;                // Data buffer
    int buffer_length;           // Buffer size
    int actual_length;           // Actual transferred (filled by kernel)
    int start_frame;             // Start frame (isochronous)
    int number_of_packets;       // ISO packet count
    unsigned int error_count;    // Error count (filled by kernel)
    unsigned int signr;          // Signal on completion (0 = none)
    void *usercontext;           // User context pointer
    struct usbdevfs_iso_packet_desc iso_frame_desc[0];  // ISO packet array
};
```

**URB types:**
- `PIPE_CONTROL` - Control transfers (setup packets)
- `PIPE_BULK` - Bulk transfers (large data, no timing guarantee)
- `PIPE_INTERRUPT` - Interrupt transfers (periodic, small data)
- `PIPE_ISOCHRONOUS` - Isochronous transfers (timing guaranteed, no retry)

**Operations:**
- Submit URB: `USBDEVFS_SUBMITURB` - Non-blocking submission
- Reap URB: `USBDEVFS_REAPURB` / `USBDEVFS_REAPURBNDELAY` - Get completion
- Discard URB: `USBDEVFS_DISCARDURB` - Cancel pending URB
- Claim interface: Required before using endpoints
- Release interface: Free interface for other drivers
- Reset: `USBDEVFS_RESET` - Reset device state
- Clear halt: `USBDEVFS_CLEAR_HALT` - Clear endpoint stall

**Zero-copy:**
- `USBDEVFS_ZERO_COPY` - Map userspace buffers directly

**Ordo Application:** Asynchronous operation submission/reaping, completion callbacks via signals/polling, explicit resource claiming.

---

### 6. VirtIO - Virtual Device Transport

**Virtqueue ring structure:**
```
Descriptor Table (avail to driver)
├── addr (guest-physical)
├── len
├── flags (NEXT, WRITE)
└── next (index)

Available Ring (driver → device)
├── idx (updated atomically)
├── ring[idx] = descriptor index
└── used_event (interrupt threshold)

Used Ring (device → driver)
├── idx (updated atomically)
├── ring[idx] = descriptor index
└── avail_event (kick threshold)
```

**Feature negotiation:**
1. Driver reads device features: `virtio_pci_get_device_features()`
2. Driver writes guest features: `virtio_pci_set_driver_features()`
3. Both sides agree on intersection

**Common features:**
- `VIRTIO_F_NOTIFY_ON_EMPTY` - Interrupt when ring empties
- `VIRTIO_F_ANY_LAYOUT` - Flexible descriptor layout
- `VIRTIO_F_VERSION_1` - VirtIO 1.0 compliance
- `VIRTIO_F_ACCESS_PLATFORM` - Platform-specific access (IOMMU, encryption)
- `VIRTIO_F_RING_PACKED` - Packed ring layout (more efficient)
- `VIRTIO_F_IN_ORDER` - Used buffers returned in order
- `VIRTIO_F_ORDER_PLATFORM` - Memory barriers handled by platform

**VirtIO Net features:**
- `VIRTIO_NET_F_CSUM` - Checksum offload
- `VIRTIO_NET_F_GUEST_CSUM` - Guest checksum offload
- `VIRTIO_NET_F_CTRL_VQ` - Control virtqueue
- `VIRTIO_NET_F_MQ` - Multiqueue support
- `VIRTIO_NET_F_RSC_EXT` - RSC extension
- `VIRTIO_NET_F_STANDBY` - Standby link

**VirtIO Block features:**
- `VIRTIO_BLK_F_RO` - Read-only device
- `VIRTIO_BLK_F_SCSI` - SCSI command passthrough
- `VIRTIO_BLK_F_FLUSH` - Cache flush support
- `VIRTIO_BLK_F_TOPOLOGY` - Topology info (alignment, optimal I/O size)
- `VIRTIO_BLK_F_SECURE_ERASE` - Secure erase command
- `VIRTIO_BLK_F_WRITE_ZEROES` - Write zeroes command
- `VIRTIO_BLK_F_MQ` - Multi-queue support

**Config space:**
- Device-specific configuration read/write via MMIO
- Example: VirtIO Net config has `mac[ETH_ALEN]`, `status`, `max_virtqueue_pairs`, `mtu`

**Ordo Application:** Ring-buffer IPC with atomic index updates, feature negotiation for capability discovery, multi-queue scaling.

---

### 7. NVMe ioctl

**Namespace management:**
```c
struct nvme_id_ns {
    __le64 nsze;          // Namespace size (LBAs)
    __le64 ncap;          // Namespace capacity
    __le64 nuse;          // Namespace utilization
    __u8 nsfeat;          // Namespace features
    __u8 nlbaaf;          // Number of LBA formats
    __u8 flbas;           // Formatted LBA size
    // ... more fields
};
```

**Passthrough commands:**
```c
struct nvme_passthru_cmd {
    __u32 opcode;
    __u32 flags;
    __u32 rsvd;
    __u32 nsid;
    __u32 cdw2[10];       // Command dword 2-11
    __u64 metadata;
    __u64 addr;           // Data buffer pointer
    __u32 metadata_len;
    __u32 data_len;
    __u32 timeout_ms;
    __u32 result;         // Command result
};
```

**Admin commands:**
- `nvme_admin_identify` - Identify controller/namespace
- `nvme_admin_create_sq` - Create submission queue
- `nvme_admin_delete_sq` - Delete submission queue
- `nvme_admin_create_cq` - Create completion queue
- `nvme_admin_delete_cq` - Delete completion queue
- `nvme_admin_ns_attach` - Attach namespace to controller
- `nvme_admin_detach_ns` - Detach namespace
- `nvme_admin_create_ns` - Create namespace
- `nvme_admin_delete_ns` - Delete namespace
- `nvme_admin_fw_commit` - Firmware commit
- `nvme_admin_fw_download` - Firmware download

**Namespace commands:**
- `nvme_cmd_flush` - Flush cache
- `nvme_cmd_write` / `nvme_cmd_read` - Read/write
- `nvme_cmd_write_zeroes` - Write zeroes (deallocate)
- `nvme_cmd_dsm` - Dataset management (trim/unmap)
- `nvme_cmd_verify` - Verify data integrity

**Ordo Application:** Command submission with explicit timeouts, result codes, metadata separation from data payload.

---

## Cross-Driver Patterns

### A. File Descriptor Lifecycle

All drivers follow similar fd lifecycle:
```
open("/dev/...") → configure via ioctl → mmap() optional → poll()/select() → close()
```

**Ordo Application:** Handle-based resources with explicit lifecycle phases.

### B. Ioctl Categories by Function

| Category | Purpose | Examples |
|----------|---------|----------|
| Query | Get static info | `GET_INFO`, `GET_CAP`, `ENUM_*` |
| Configure | Set parameters | `SET_CONFIG`, `SET_PARAMS` |
| Allocate | Create resources | `CREATE`, `ALLOC`, `OPEN` |
| Free | Destroy resources | `DESTROY`, `FREE`, `CLOSE` |
| Queue | Buffer management | `QBUF`, `DQBUF`, `SUBMIT` |
| Control | Start/stop/sync | `START`, `STOP`, `SYNC`, `WAIT` |
| Event | Async notifications | `READ_EVENT`, `POLL` |

**Ordo Application:** Message type taxonomy could mirror these categories.

### C. Capability Discovery Pattern

Universal pattern across all drivers:
```c
// 1. Query version/features
ioctl(fd, DRM_IOCTL_VERSION, &version);
ioctl(fd, DRM_IOCTL_GET_CAP, &cap);

// 2. Negotiate/enable features
ioctl(fd, DRM_IOCTL_SET_CLIENT_CAP, &client_cap);

// 3. Use features conditionally
if (cap.value & FEATURE_X) {
    // use feature
}
```

**Ordo Application:** Explicit capability negotiation before feature use.

### D. Atomic Operations

DRM atomic API pattern:
```c
// 1. Create atomic request
req = atomic_new();

// 2. Add property changes
atomic_add_property(req, crtc_id, "ACTIVE", 1);
atomic_add_property(req, plane_id, "FB_ID", fb_handle);
atomic_add_property(req, connector_id, "CRTC_ID", crtc_id);

// 3. Validate (optional)
atomic_commit(req, TEST_ONLY);  // returns -EINVAL if invalid

// 4. Commit
ret = atomic_commit(req, NONBLOCK);
if (ret == -EBUSY) {
    // Retry later
}
```

**Ordo Application:** Batch configuration changes with validate-then-commit semantics.

### E. Event-Driven Completion

Pattern across DRM, V4L2, GPIO, USB:
```c
// 1. Enable events
set_flag(ASYNC_EVENTS);

// 2. Submit operation
submit_work();

// 3. Poll/wait for completion
poll(fd, POLLIN);

// 4. Read event struct
read(fd, &event, sizeof(event));

// 5. Process result
handle_event(event);
```

**Ordo Application:** Bus message subscriptions with pollable completion handles.

---

## Files Generated

```
F:\OPENCLAW-PROJECTS\linux-primitives-extracted\output\drivers\
├── drm_analysis.md            (16.8 KB, 58 structs)
├── drm_mode_analysis.md       (14.1 KB, 46 structs)
├── v4l2-common_analysis.md    (1.5 KB)
├── v4l2-controls_analysis.md  (25 KB, 808 defines, 47 structs)
├── asound_analysis.md         (22.6 KB, 51 structs)
├── pci_analysis.md            (0.7 KB)
├── i2c-dev_analysis.md        (1.2 KB)
├── gpio_analysis.md           (6.9 KB, 16 structs)
├── usbdevice_fs_analysis.md   (6.7 KB, 13 structs)
├── virtio_pci_analysis.md     (10.8 KB, 32 structs)
├── virtio_net_analysis.md     (10.5 KB, 33 structs)
├── virtio_blk_analysis.md     (5.5 KB, 8 structs)
└── nvme_ioctl_analysis.md     (3.7 KB, 4 structs)
```

**Total:** ~126 KB of driver primitive analysis

---

## Next Steps

1. ✅ Parse core uapi headers (19 files)
2. ✅ Parse driver headers (13 files)
3. ⏳ Expand to remaining driver categories:
   - Network: `ethtool.h`, `wireless.h`, `can.h`, `bluetooth/*.h`
   - Storage: `cdrom.h`, `hdreg.h`, `loop.h`, `dm-ioctl.h`, `btrfs.h`, `ext4.h`
   - Input: `input.h`, `joystick.h`, `hidraw.h`, `ff.h`
   - Misc: `watchdog.h`, `rtc.h`, `random.h`, `kvm.h` (already done)
   - VFIO: `vfio.h` (device passthrough)
   - Userfault: `userfaultfd.h` (page fault handling)
   - Binder: `binder.h` (Android IPC)
4. ⏳ Write category summaries (`02-graphics-primitives.md`, `03-media-primitives.md`, etc.)
5. ⏳ Draft `99-ordo-mappings.md` with concrete proposals

---

**Total extraction so far:**
- Core uapi: 19 headers → ~137 KB
- Drivers: 13 headers → ~126 KB
- **Combined:** 32 headers → ~263 KB, 1,700+ defines, 400+ structs
