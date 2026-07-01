# Remaining Linux UAPI Headers to Extract

**Status:** 260 of 957 headers parsed (27% complete)  
**Remaining:** ~697 headers

---

## Summary by Category

### Already Extracted (260 headers, ~1.46 MB)

| Category | Headers | Size | Key Patterns |
|----------|---------|------|--------------|
| Core uapi | 19 | ~137 KB | ioctl, capabilities, event FDs, futex, netlink, perf_event, BPF, KVM |
| GPU drivers | 15 | ~200 KB | DRM buffer objects, command submission, contexts, fences |
| Generic drivers | 13 | ~126 KB | GPIO, I2C, USB, PCI, VirtIO, NVMe, V4L2 |
| Network/Audio | 25 | ~153 KB | ethtool, nl80211 WiFi, CAN bus, ALSA PCM/MIDI |
| Input/Security/FS | 38 | ~289 KB | input events, KVM, VFIO, seccomp, landlock, btrfs/ext4 |
| Batch 5 (misc) | 47 | ~206 KB | DVB, ELF, FUSE, HID, Hyper-V, acct, aio |
| Batch 6 (network/IPC) | 64 | ~257 KB | if_*, IPv4/v6, IPC, IPMI, netfilter, PPP |
| Power Management | 9 | ~38 KB | APM, IPMI, PMU, suspend, AMD APML |
| Crypto/Security | 9 | ~40 KB | AF_ALG, keyctl, TLS, IPsec, RNG |
| Networking | 21 | ~103 KB | Sockets, TCP/UDP, MPTCP, rtnetlink, ethtool |

---

## Major Categories Remaining (~697 headers)

### 1. Architecture-Specific Headers (`asm/`, `asm-generic/`)

**Location:** `include/uapi/asm*/`  
**Count:** ~50 headers  
**Purpose:** Architecture-specific syscall numbers, types, constants

**Key files:**
- `asm-generic/unistd.h` - Syscall number assignments
- `asm-generic/ioctl.h` - ioctl encoding (already have linux version)
- `asm-generic/mman-common.h` - Memory mapping flags
- `asm-generic/signal.h` - Signal definitions
- `asm-generic/termbits.h` - Terminal control bits
- `asm-generic/ipcbuf.h`, `msgbuf.h`, `sembuf.h`, `shmbuf.h` - IPC buffer structures
- `asm-generic/ucontext.h` - User context for signal handlers
- `asm-generic/hugetlb_encode.h` - HugeTLB encoding
- `asm-generic/bitsperlong.h` - Architecture word size
- `asm-generic/int-l64.h`, `int-ll64.h` - Integer type definitions
- `asm-generic/posix_types.h` - POSIX type definitions
- `asm-generic/swab.h` - Byte swapping macros
- `asm-generic/param.h` - System limits
- `asm-generic/types.h` - Basic types
- `asm-generic/setup.h` - Boot parameters
- `asm-generic/poll.h` - Poll event flags
- `asm-generic/errno-base.h` - Base errno values

**Ordo Relevance:** Low - mostly architecture-specific constants. Syscall numbers useful for tracing.

---

### 2. Accelerator / AI Hardware (`drm/*_accel.h`, `accel/`)

**Location:** `include/uapi/drm/`, `include/uapi/linux/`  
**Count:** ~10 headers  
**Purpose:** AI/ML accelerator interfaces

**Key files:**
- `drm/amdxdna_accel.h` - AMD XDNA AI accelerator
- `drm/ethosu_accel.h` - ARM Ethos-U NPU
- `drm/ivpu_accel.h` - Intel VPU (Vision Processing Unit)
- `drm/qaic_accel.h` - Qualcomm AI accelerator
- `drm/rocket_accel.h` - Rocket accelerator
- `cxl/features.h` - Compute Express Link features
- `fwctl/*.h` - Firmware control (bnxt, cxl, mlx5, pds)

**Ordo Relevance:** High - hardware acceleration for ML inference in Ordo subagents.

---

### 3. Legacy / Specialized Networking

**Location:** `include/uapi/linux/`  
**Count:** ~80 headers  
**Purpose:** Specialized network protocols

**Key files:**
- **ATM:** `atm.h`, `atmapi.h`, `atmarp.h`, `atmbr2684.h`, `atmclip.h`, `atmdev.h`, `atmioc.h`, `atmlec.h`, `atmmpc.h`, `atmppp.h`, `atmsap.h`, `atmsvc.h`, `atm_eni.h`, `atm_he.h`, `atm_idt77105.h`, `atm_nicstar.h`, `atm_tcp.h`, `atm_zatm.h`
- **ISDN:** `isdn/capicmd.h`, `isdn/capilib.h`, `isdn/divert.h`, `isdn/isdn_common.h`, `isdn/isdn_ioctl.h`, `isdn/isdnload.h`, `isdn/pmp.h`, `isdndev.h`
- **AX.25 (Amateur Radio):** `ax25.h`
- **NetROM:** `netrom.h`
- **ROSE:** `rose.h`
- **X.25:** `x25.h`, `x25_hdlc.h`
- **Econet:** `econet.h`
- **IrDA:** `irda.h`
- **WAN protocols:** `hdlc.h`, `frad.h`, `dlci.h`, `cisco.h`, `sdla.h`, `lapb.h`, `wanrouter.h`
- **LLC:** `llc.h`
- **IPX:** `ipx.h`
- **AppleTalk:** `atalk.h`
- **DECnet:** `dn_*` headers
- **RDS:** `rds.h`
- **TIPC:** `tipc.h`, `tipc_sockets_diag.h`
- **RxRPC:** `rxrpc.h`
- **CAIF:** `caif_socket.h`, `caif/if_caif.h`
- **Phonet:** `phonet.h`
- **IUCV:** `iucv.h`
- **SNA:** `sna.h`
- **AF_KCM:** `kcm.h` (already parsed)
- **SMC:** `smc.h`
- **XDP Sockets:** `xdp_diag.h`, `linux/if_xdp.h`
- **VSOCK:** `vm_sockets.h`, `vm_sockets_diag.h`, `vsockmon.h`, `virtio_vsock.h`
- **MPTCP:** Already parsed
- **CAN:** Already parsed (gw, isotp, j1939, raw)
- **Bluetooth:** Not in uapi (kernel-internal only)
- **NFC:** `nfc.h`, `nfcmrvl.h`, `nfcsim.h`
- **WiMAX:** `wimax.h`, `wimax/i2400m.h`

**Ordo Relevance:** Low-Medium - mostly legacy/specialized protocols. XDP sockets useful for high-performance packet I/O.

---

### 4. Storage / Filesystem IOCTLS

**Location:** `include/uapi/linux/`, `include/uapi/scsi/`, `include/uapi/mtd/`  
**Count:** ~60 headers  
**Purpose:** Storage device and filesystem-specific ioctls

**Key files:**
- **SCSI:** `scsi/scsi_bsg_mpi.h`, `scsi/scsi.h`, `scsi/scsi_ioctl.h`, `scsi/fc/fs_ls.h`, `scsi/fc/fs_ns.h`
- **NVMe:** Already parsed (`nvme_ioctl.h`)
- **MTD (Memory Technology Device):** `mtd/abi.h`, `mtd/inftl-user.h`, `mtd/nftl-user.h`, `mtd/ubi-user.h`
- **UBI/UBIFS:** `ubi-user.h`, `ubifs_user.h`
- **Filesystem-specific:**
  - `btrfs.h`, `btrfs_tree.h` - Already parsed
  - `ext4.h` - Already parsed
  - `f2fs.h` - Already parsed
  - `xfs/xfs_fs.h` - XFS filesystem
  - `nilfs2_api.h` - NILFS2
  - `ocfs2/ocfs2_fs.h` - OCFS2 cluster filesystem
  - `gfs2_ondisk.h` - GFS2 cluster filesystem
  - `ceph_fs.h` - Ceph distributed filesystem
  - `nfs/nfs.h`, `nfs/nfs3.h`, `nfs/nfs4.h`, `nfs/nfsacl.h` - NFS
  - `cifs/cifs_mount.h` - CIFS/SMB
  - `9p/9p.h` - Plan 9 filesystem
  - `afs/afs.h` - AFS filesystem
  - `ceph_fs.h` - Ceph
  - `jffs2.h` - Already parsed
  - `squashfs_fs.h` - SquashFS
  - `cramfs_fs.h` - Compressed ROM filesystem
  - `romfs_fs.h` - ROM filesystem
  - `sysv_fs.h` - System V filesystem
  - `minix_fs.h` - Already parsed
  - `affs_hardblocks.h` - Amiga filesystem
  - `adfs_fs.h` - Acorn filesystem
  - `ufs/ufs.h` - Unix File System
  - `hfs/hfs.h` - HFS
  - `hfsplus/hfsplus_fs.h` - HFS+
  - `omfs/omfs_fs.h` - OS/2 filesystem
  - `bfs_fs.h` - BeOS filesystem
  - `qnx6/fs_qnx6.h` - QNX6
  - `ntfs_fs.h` - NTFS
  - `exfat/exfat_fs.h` - exFAT
  - `fat.h` - FAT/VFAT
  - `ext2/ext2_fs.h` - ext2
  - `reiserfs/reiserfs_fs.h` - ReiserFS
  - `iso_fs.h` - Already parsed
  - `udf/udf_fs.h` - UDF
  - `ecryptfs.h` - eCryptfs encrypted filesystem
  - `overlayfs/ovl_fs.h` - Overlay filesystem
  - `fuse/fuse.h` - Already parsed
  - `pstore_inode.h` - Pstore
  - `configfs.h` - ConfigFS
  - `debugfs.h` - DebugFS
  - `tracefs.h` - TraceFS
  - `hugetlb.h` - HugeTLB
  - `shm.h` - Shared memory
  - `tmpfs.h` - Tmpfs
  - `ramfs.h` - Ramfs
  - `proc_fs.h` - Proc filesystem
  - `sysfs.h` - Sysfs
  - `devpts_fs.h` - Devpts
  - `autofs/autofs_dev_ioctl.h` - Autofs
  - `binfmts.h` - Already parsed
  - `quota.h` - Already parsed
  - `dm-ioctl.h` - Already parsed
  - `md.h` - Multiple device (RAID)
  - `raid/md.h` - RAID
  - `raid/md_p.h` - RAID private
  - `raid/md_u.h` - RAID user
  - `zram/zram.h` - ZRAM compressed RAM disk
  - `bcache/bcache.h` - Bcache
  - `drbd/drbd_headers.h` - DRBD
  - `loop.h` - Already parsed
  - `cdrom.h` - Already parsed
  - `hdreg.h` - Already parsed
  - `fd.h` - Floppy disk
  - `mtio.h` - Magnetic tape
  - `bsg.h` - Already parsed
  - `blk-crypto.h` - Already parsed
  - `blktrace_api.h` - Already parsed
  - `blkpg.h` - Already parsed
  - `blkdev.h` - Already parsed

**Ordo Relevance:** Medium - filesystem ioctls for checkpoint/restore, storage encryption, RAID management.

---

### 5. Multimedia / DVB (Digital Video Broadcasting)

**Location:** `include/uapi/linux/dvb/`, `include/uapi/linux/media.h`  
**Count:** ~20 headers (6 already parsed)  
**Purpose:** Digital TV/video capture, decoding, encoding

**Key files:**
- `dvb/audio.h` - Already parsed
- `dvb/dmx.h` - Already parsed
- `dvb/frontend.h` - Already parsed
- `dvb/net.h` - Already parsed
- `dvb/osd.h` - Already parsed
- `dvb/video.h` - Already parsed
- `dvb/ca.h` - Conditional access
- `dvb/version.h` - DVB version
- `media.h` - Already parsed
- `videodev2.h` - V4L2 video capture (in drivers/)
- `v4l2-common.h` - Already parsed
- `v4l2-controls.h` - Already parsed
- `v4l2-mediabus.h` - V4L2 media bus formats
- `v4l2-dv-timings.h` - V4L2 DV timings
- `cec.h` - Consumer Electronics Control
- `lirc.h` - IR remote control

**Ordo Relevance:** Low - specific to TV/video capture hardware.

---

### 6. Embedded / Mobile / Specialized Hardware

**Location:** `include/uapi/linux/`, `include/uapi/misc/`  
**Count:** ~100 headers  
**Purpose:** Embedded system interfaces

**Key files:**
- **GNSS:** `gnss/gnss_serial.h`, `gnss/gnss-ubx.h`, `gnss/gnss-nmea.h`
- **IIO (Industrial I/O):** `iio/events.h`, `iio/types.h`, `iio/buffer.h`
- **PWM:** `pwm.h`
- **Regulator:** `regulator/consumer.h`, `regulator/driver.h`, `regulator/machine.h`
- **Clock:** `clk.h`
- **Pinctrl:** `pinctrl/pinctrl-state.h`, `pinctrl/devinfo.h`
- **GPIO:** Already parsed
- **I2C:** Already parsed (`i2c-dev.h`, `i2c.h`)
- **SPI:** `spi/spi.h`
- **W1 (1-Wire):** `w1.h`
- **PPS (Pulse Per Second):** `pps.h`
- **PTP (Precision Time Protocol):** `ptp_clock.h`
- **MFD (Multi-Function Device):** `mfd/core.h`, `mfd/wm831x/core.h`
- **SoundWire:** `soundwire/soundwire.h`
- **Thunderbolt:** `thunderbolt.h`
- **PECI (Platform Environment Control Interface):** `peci.h`
- **ACPI:** `acpi/ioctl.h`, `acpi/actypes.h`
- **Firmware:** `firmware/efi/efi.h`, `firmware/cs_dsp.h`
- **FPGA:** `fpga/fpga-image.h`, `fpga/fpga-region.h`, `fpga/dfl.h`
- **TEE (Trusted Execution Environment):** `tee.h`, `tee_gen.h`, `tee_optee.h`
- **NVMEM:** `nvmem/consumer.h`
- **LEDs:** `leds.h`
- **Backlight:** `backlight.h`
- **LCD:** `lcd.h`
- **Thermal:** `thermal.h`
- **Hwmon:** `hwmon.h`
- **Power Supply:** `power_supply.h`
- **Battery:** `battery.h`
- **AC Adapter:** `ac.h`
- **Watchdog:** `watchdog.h`
- **RTC:** `rtc/rtc.h`
- **Parport:** `parport.h`, `ppdev.h`, `lp.h`
- **Gameport:** `gameport.h`
- **Input:** Already parsed extensively
- **HID:** Already parsed (`hidraw.h`, `uhid.h`, `hid.h`, `hiddev.h`)
- **Misc:** `misc.h`, `misc_devices/*.h`

**Ordo Relevance:** Medium-Low - mostly embedded/IoT specific. TEE useful for secure enclaves.

---

### 7. Virtualization / Hypervisors

**Location:** `include/uapi/linux/`, `include/uapi/xen/`, `include/uapi/kvm/`  
**Count:** ~30 headers (KVM already parsed)  
**Purpose:** Virtual machine interfaces

**Key files:**
- `kvm.h` - Already parsed (606 defines!)
- `kvm_para.h` - Already parsed
- `kvm_host.h` - KVM host interface
- `vfio.h` - Already parsed
- `vfio_ccw.h` - Already parsed
- `vfio_pci.h` - VFIO PCI
- `vfio_ap.h` - VFIO crypto
- `xen/evtchn.h` - Xen event channels
- `xen/gntdev.h` - Xen grant device
- `xen/privcmd.h` - Xen privileged command
- `xen/sched.h` - Xen scheduler
- `xen/xenbus_dev.h` - Xenbus device
- `acrn/acrn_dev.h` - ACRN hypervisor
- `acrn/battlefield.h` - ACRN battlefield
- `lguest_launcher.h` - Lguest (deprecated)
- `visorclient/visorclient.h` - Hyper-V visor
- `hyperv.h` - Already parsed
- `virtio_*.h` - Multiple VirtIO devices (many already parsed)
- `rpmsg.h` - Remote processor messaging
- `remoteproc.h` - Remote processor control

**Ordo Relevance:** High - virtualization primitives for Ordo subagent isolation.

---

### 8. Kernel Debugging / Tracing / Profiling

**Location:** `include/uapi/linux/`  
**Count:** ~20 headers  
**Purpose:** Debugging, tracing, profiling interfaces

**Key files:**
- `perf_event.h` - Already parsed (147 defines, 39 structs)
- `blktrace_api.h` - Already parsed
- `ftrace.h` - Function tracer
- `kprobes.h` - Kernel probes
- `uprobes.h` - User-space probes
- `kgdb.h` - Kernel debugger
- `trace.h` - Generic trace
- `event.h` - Event tracing
- `bpf_perf_event.h` - Already parsed
- `bpf.h` - Already parsed (65 defines, 115 structs)
- `bpf_raw_tracepoint.h` - BPF raw tracepoints
- `bpf_test_run.h` - BPF test runner
- `seccomp.h` - Already parsed
- `ptrace.h` - Already parsed
- `userfaultfd.h` - Already parsed
- `coredump.h` - Core dump format
- `elf.h` - Already parsed (319 defines, 20 structs)
- `elfcore.h` - ELF core dump
- `a.out.h` - a.out binary format
- `coff.h` - Already parsed
- `binfmts.h` - Already parsed

**Ordo Relevance:** High - tracing/profiling for Ordo runtime observability.

---

### 9. Security Modules / Key Management

**Location:** `include/uapi/linux/`  
**Count:** ~20 headers (many already parsed)  
**Purpose:** Security modules, key management

**Key files:**
- `keyctl.h` - Already parsed
- `key.h` - Key subsystem
- `encrypted-keys/encrypted-keys.h` - Encrypted keys
- `trusted_keys/trusted_keys.h` - Trusted platform module keys
- `pkcs7.h` - PKCS#7 certificates
- `crypto.h` - Crypto user API
- `ima.h` - Integrity Measurement Architecture
- `evm.h` - Extended Verification Module
- `selinux.h` - SELinux
- `apparmor.h` - AppArmor
- `smack.h` - SMACK
- `tomoyo.h` - Tomoyo
- `landlock.h` - Already parsed
- `integrity.h` - Integrity subsystem
- `audit.h` - Already parsed (312 defines)
- `fscrypt.h` - Filesystem encryption
- `fsverity.h` - Already parsed
- `dm-verity.h` - Device mapper verity

**Ordo Relevance:** High - security modules for Ordo sandboxing.

---

### 10. Wireless / RF / Specialized Radio

**Location:** `include/uapi/linux/`, `include/uapi/net/`  
**Count:** ~40 headers  
**Purpose:** Wireless protocols beyond WiFi

**Key files:**
- `nl80211.h` - Already parsed (WiFi cfg80211)
- `wireless.h` - Already parsed (legacy wireless)
- `rfkill.h` - RF kill switch
- `bluetooth/` - Not in uapi (kernel-internal)
- `nfc.h` - Near Field Communication
- `wimax.h` - WiMAX
- `mesh/` - 802.11s mesh
- `batman_adv.h` - BATMAN-adv mesh routing
- `batadv_packet.h` - BATMAN packets
- `ieee80211_radiotap.h` - Radiotap header
- `iw_cm.h` - InfiniBand connection manager
- `rdma/` - RDMA verbs
- `ib_mad.h` - InfiniBand management datagrams
- `ib_sa.h` - InfiniBand subnet admin
- `ib_user_mad.h` - Userspace MAD
- `ib_user_verbs.h` - Userspace verbs
- `ipoib.h` - IP over InfiniBand

**Ordo Relevance:** Low-Medium - mostly specialized radio protocols.

---

### 11. Misc / Obsolete / Rarely Used

**Location:** Various  
**Count:** ~200 headers  
**Purpose:** Miscellaneous, obsolete, or rarely used interfaces

**Examples:**
- `adb.h` - Apple Desktop Bus
- `amt.h` - Active Management Technology
- `arm_sdei.h` - ARM SDEI
- `aspeed-lpc-ctrl.h` - ASPEED LPC control
- `aspeed-p2a-ctrl.h` - ASPEED P2A control
- `aspeed-video.h` - ASPEED video
- `agpgart.h` - AGP GART
- `am437x-vpfe.h` - TI VPFE
- `apm_bios.h` - Already parsed
- `arcfb.h` - ARC framebuffer
- `auto_dev-ioctl.h` - Autofs
- `auto_fs.h`, `auto_fs4.h` - Automount
- `bcm933xx_hcs.h` - Broadcom HCS
- `coda_psdev.h` - Coda filesystem
- `connector.h` - Already parsed
- `cuda.h` - Already parsed
- `dccp.h` - Already parsed
- `devlink.h` - Already parsed
- `dlm.h`, `dlm_device.h` - Already parsed
- `edd.h` - Already parsed
- `errqueue.h` - Already parsed
- `errno.h` - Already parsed
- `eventpoll.h` - Already parsed
- `fb.h` - Already parsed
- `fd.h` - Already parsed
- `fiemap.h` - Already parsed
- `filter.h` - Already parsed
- `fs.h` - Already parsed
- `fuse.h` - Already parsed
- `genwqe/genwqe_card.h` - Already parsed
- `gitodev.h` - GITO device
- `hdlcdrv.h` - HDLC driver
- `hibernate.h` - Hibernation
- `hpet.h` - Already parsed
- `idn.h` - Internationalized Domain Names
- `if_bridge.h` - Already parsed
- `if_cablemodem.h` - Cable modem
- `if_frad.h` - Frame relay
- `if_hsr.h` - High-availability Seamless Redundancy
- `if_infiniband.h` - Already parsed
- `if_macsec.h` - Already parsed
- `if_macvlan.h` - MAC VLAN
- `if_packet.h` - Already parsed
- `if_pppox.h` - Already parsed
- `if_slip.h` - Already parsed
- `if_team.h` - Already parsed
- `if_tun.h` - Already parsed
- `if_tunnel.h` - Already parsed
- `if_udp_tunnel.h` - UDP tunnel
- `if_vlan.h` - Already parsed
- `if_xfrm.h` - XFRM interface
- `ife.h` - Already parsed
- `igmp.h` - Already parsed
- `ila.h` - Already parsed
- `inet_diag.h` - Already parsed
- `inet_lro.h` - Large Receive Offload
- `inet_ndisc.h` - IPv6 neighbor discovery
- `inet_sctp.h` - SCTP
- `in_route.h` - Already parsed
- `ip_vs.h` - Already parsed
- `ipmi_bmc.h` - Already parsed
- `ipmi_msgdefs.h` - Already parsed
- `ipmi_ssif_bmc.h` - Already parsed
- `ipmi.h` - Already parsed
- `ipc.h` - Already parsed
- `ipmi.h` - Already parsed
- `ipmi_msgdefs.h` - Already parsed
- `ipv6_route.h` - Already parsed
- `iso_fs.h` - Already parsed
- `isdn/` - ISDN (obsolete)
- `isis.h` - IS-IS routing
- `iw_cm.h` - Already queried
- `jffs2.h` - Already parsed
- `kcm.h` - Already parsed
- `kd.h` - Already parsed
- `kernelcapi.h` - Kernel CAPI
- `kexec.h` - Already parsed
- `kfd_sysfs.h` - Already parsed
- `kvm_para.h` - Already parsed
- `lbiquorum.h` - Load balancer quorum
- `lcd.h` - LCD control
- `leds.h` - LED control
- `lirc.h` - IR remote
- `lvm.h` - LVM
- `magic.h` - Already parsed
- `matroxfb.h` - Already parsed
- `media.h` - Already parsed
- `membarrier.h` - Already parsed
- `memfd.h` - Already parsed
- `mempolicy.h` - Already parsed
- `mii.h` - Already parsed
- `minix_fs.h` - Already parsed
- `misc.h` - Misc devices
- `mman.h` - Already parsed
- `mnt_namespace.h` - Mount namespace
- `mqueue.h` - Already parsed
- `mrst_mcu.h` - Moorestown MCU
- `msg.h` - Already parsed
- `mtio.h` - Already parsed
- `net.h` - Already parsed
- `net_dropmonitor.h` - Network drop monitor
- `netconf.h` - Already parsed
- `netdevice.h` - Already parsed
- `netfilter/` - Netfilter (partially parsed)
- `netlink_diag.h` - Already parsed
- `netlink.h` - Already parsed
- `nbd.h` - Already parsed
- `ncp.h` - NCP (NetWare)
- `neighbour.h` - Already parsed
- `nf_conntrack_*.h` - Conntrack helpers
- `nf_log.h` - Netfilter logging
- `nf_queue.h` - Netfilter queue
- `nfs/` - NFS
- `nfsacl.h` - NFS ACL
- `nfsd/` - NFS daemon
- `nilfs2_api.h` - NILFS2
- `nitro_enclaves.h` - AWS Nitro Enclaves
- `nl80211.h` - Already parsed
- `nlcpy.h` - Netlink copy
- `n_rdisc.h` - RDS
- `ntsync.h` - NT sync
- `nvfuser.h` - NVIDIA fuser
- `ocfs2/` - OCFS2
- `omapfb.h` - OMAP framebuffer
- `openvswitch.h` - Open vSwitch
- `ospf.h` - OSPF routing
- `packet_diag.h` - Packet socket diagnostics
- `patchkey.h` - Patch key
- `pci_regs.h` - PCI registers
- `pcitest.h` - PCIe test
- `pdcp.h` - PDCP
- `pegasus.h` - Pegasus USB Ethernet
- `perf_event.h` - Already parsed
- `personality.h` - Personality flags
- `pfkeyv2.h` - Already parsed
- `phonet/` - Phonet
- `pkt_cls.h` - Packet classifier
- `pkt_sched.h` - Packet scheduler
- `plastic.h` - PLASTIC
- `pmu.h` - Already parsed
- `portmux.h` - Port multiplexer
- `ppp-comp.h` - Already parsed
- `ppp_defs.h` - Already parsed
- `ppp-ioctl.h` - Already parsed
- `pppoe.h` - PPPoE
- `pptp.h` - PPTP
- `prctl.h` - Already parsed
- `psample.h` - Packet sampling
- `ptp_clock.h` - PTP clock
- `pulse8-cec.h` - Pulse8 CEC
- `qemu_fw_cfg.h` - QEMU firmware config
- `qrtr.h` - Qualcomm RPC Router
- `quota.h` - Already parsed
- `random.h` - Already parsed
- `ras/` - RAS (Reliability, Availability, Serviceability)
- `route.h` - Routing
- `rpc_pipe_fs.h` - RPC pipe filesystem
- `rpmsg/` - RPMsg
- `rtnetlink.h` - Already parsed
- `scc.h` - SCC
- `sched.h` - Scheduler
- `sci.h` - SCI
- `sctp.h` - SCTP
- `sdla.h` - SDLA
- `secctl.h` - Security control
- `securebits.h` - Secure bits
- `selinux.h` - SELinux
- `sem.h` - Semaphores
- `sendpage.h` - Sendpage
- `serdes.h` - SerDes
- `serial.h` - Serial
- `serio.h` - Serio
- `sg.h` - SCSI generic
- `sh.h` - SuperH
- `shared_IRQ.h` - Shared IRQ
- `shm.h` - Shared memory
- `signal.h` - Signals
- `sisfb.h` - SiS framebuffer
- `smb.h` - SMB
- `smack.h` - SMACK
- `smbios.h` - SMBIOS
- `smsc.h` - SMSC
- `snd_compr_offload.h` - Sound compress offload
- `sock_diag.h` - Already parsed
- `sockios.h` - Already parsed
- `socket.h` - Already parsed
- `sonypi.h` - Sony PI
- `spider_net.h` - Spider network
- `spi/` - SPI
- `splice.h` - Splice
- `sqfs.h` - SquashFS
- `ssb.h` - SSB
- `stat.h` - Already parsed
- `statfs.h` - Statfs
- `stddef.h` - Standard definitions
- `sti.h` - STI
- `stk1160.h` - STK1160
- `strparser.h` - String parser
- `suspend_ioctls.h` - Already parsed
- `svga.h` - SVGA
- `synclink.h` - SyncLink
- `sysctl.h` - Already parsed
- `sysinfo.h` - System info
- `syslog.h` - Syslog
- `t1pia.h` - T1PIA
- `taskstats.h` - Task statistics
- `tcp_metrics.h` - Already parsed
- `tcp.h` - Already parsed
- `tec.h` - TEC
- `tee.h` - TEE
- `termios.h` - Termios
- `tftp.h` - TFTP
- `thermal.h` - Thermal
- `time.h` - Time
- `timespec.h` - Timespec
- `timerfd.h` - Already parsed
- `tipc.h` - TIPC
- `tls.h` - Already parsed
- `tmiofb.h` - TMIO framebuffer
- `tomoyo.h` - Tomoyo
- `tpm.h` - TPM
- `tracer.h` - Tracer
- `traffic_control.h` - Traffic control
- `transcode.h` - Transcode
- `tun.h` - TUN/TAP
- `tvnorms.h` - TV norms
- `uaccess.h` - User access
- `ubifs_user.h` - UBIFS user
- `ubi-user.h` - UBI user
- `ucd.h` - UCD
- `udf/` - UDF
- `udp.h` - Already parsed
- `uhid.h` - Already parsed
- `uinput.h` - Already parsed
- `ultrasound.h` - Ultrasound
- `ums.h` - USB mass storage
- `un.h` - Unix domain
- `unistd.h` - Unistd
- `unix_diag.h` - Unix socket diagnostics
- `unwind.h` - Unwind
- `usb/` - USB
- `usbdevice_fs.h` - Already parsed
- `usbip.h` - Already parsed
- `user.h` - User
- `userfaultfd.h` - Already parsed
- `utime.h` - Utime
- `utsname.h` - Utsname
- `uuid.h` - UUID
- `v4l2-common.h` - Already parsed
- `v4l2-controls.h` - Already parsed
- `v4l2-subdev.h` - V4L2 subdev
- `vboxguest.h` - VirtualBox guest
- `vdso.h` - VDSO
- `versatile.h` - Versatile
- `veth.h` - Virtual Ethernet
- `video/` - Video
- `videodev2.h` - Video device
- `virtio_balloon.h` - VirtIO balloon
- `virtio_blk.h` - Already parsed
- `virtio_console.h` - VirtIO console
- `virtio_crypto.h` - Already parsed
- `virtio_fs.h` - VirtIO filesystem
- `virtio_gpu.h` - VirtIO GPU
- `virtio_input.h` - VirtIO input
- `virtio_mem.h` - VirtIO memory
- `virtio_mmio.h` - VirtIO MMIO
- `virtio_net.h` - Already parsed
- `virtio_pci.h` - Already parsed
- `virtio_pm.h` - VirtIO power management
- `virtio_rng.h` - Already parsed
- `virtio_scsi.h` - VirtIO SCSI
- `virtio_snd.h` - VirtIO sound
- `virtio_spi.h` - VirtIO SPI
- `virtio_vdpa.h` - VirtIO VDPA
- `virtio_vsock.h` - VirtIO vsock
- `vm_sockets_diag.h` - VM sockets diagnostics
- `vm_sockets.h` - VM sockets
- `vmci.h` - VMware CI
- `vme.h` - VME bus
- `vmpressure.h` - VM pressure
- `vsockmon.h` - Vsock monitor
- `vt.h` - Virtual terminal
- `vtpm_proxy.h` - vTPM proxy
- `vxlan.h` - VXLAN
- `wanrouter.h` - WAN router
- `watchdog.h` - Watchdog
- `wb.h` - Writeback
- `wifi.h` - WiFi
- `wimax.h` - WiMAX
- `wireless.h` - Already parsed
- `wl12xx.h` - WL12XX
- `wlcore.h` - WL core
- `wman.h` - WMAN
- `wnck.h` - WNCK
- `wol.h` - Wake-on-LAN
- `wpan.h` - WPAN
- `wqe.h` - WQE
- `x25.h` - X.25
- `xattr.h` - Extended attributes
- `xcb.h` - XCB
- `xengnt.h` - Xen grant
- `xenvif.h` - Xen VIF
- `xfs/` - XFS
- `xhci.h` - XHCI
- `xiaomi.h` - Xiaomi
- `xilinx-sdfec.h` - Xilinx SDFEC
- `xl.h` - XL
- `xml.h` - XML
- `xn.h` - XN
- `xns.h` - XNS
- `xor.h` - XOR
- `xprt.h` - XPRT
- `xtensa.h` - Xtensa
- `yama.h` - Yama LSM
- `zcrypt.h` - ZCrypt
- `zephyr.h` - Zephyr
- `zlib.h` - Zlib
- `zone.h` - Zone
- `zorro.h` - Zorro
- `zram.h` - ZRAM
- `zswap.h` - Zswap
- `zx.h` - ZX

**Ordo Relevance:** Low - mostly obsolete, platform-specific, or rarely used.

---

## Priority Recommendations

### High Priority (Parse Next)

1. **Accelerator/AI headers** (`drm/*_accel.h`, `cxl/`, `fwctl/`) - ML inference for Ordo
2. **Virtualization** (`vfio_pci.h`, `xen/`, `acrn/`) - Subagent isolation
3. **XDP sockets** (`if_xdp.h`, `xdp_diag.h`) - High-performance packet I/O
4. **TEE** (`tee.h`, `tee_gen.h`, `tee_optee.h`) - Secure enclaves
5. **Tracing** (`ftrace.h`, `kprobes.h`, `uprobes.h`) - Runtime observability

### Medium Priority

6. **Filesystem IOCTLS** (`xfs/`, `zfs/`, `btrfs` more, `fscrypt.h`) - Storage encryption
7. **Security modules** (`selinux.h`, `apparmor.h`, `ima.h`, `evm.h`) - Sandboxing
8. **RDMA/InfiniBand** (`ib_*.h`, `rdma/`) - Low-latency interconnect
9. **VirtIO remaining** (`virtio_fs.h`, `virtio_snd.h`, `virtio_console.h`) - VM integration
10. **TPM** (`tpm.h`) - Hardware security

### Low Priority (Parse If Needed)

11. **Legacy networking** (ATM, ISDN, AX.25, etc.) - Obsolete protocols
12. **Multimedia/DVB** - TV capture (not relevant to Ordo)
13. **Embedded hardware** (GNSS, IIO, PWM, etc.) - IoT-specific
14. **Architecture-specific** (`asm/`) - Mostly constants
15. **Obsolete/misc** - Rarely used interfaces

---

## Estimated Effort

- **High priority (5 categories):** ~50 headers → ~200 KB analysis
- **Medium priority (5 categories):** ~100 headers → ~400 KB analysis
- **Low priority (remaining):** ~550 headers → ~2 MB analysis

**Total remaining:** ~697 headers → ~2.6 MB additional analysis

**Current total:** 260 headers → ~1.46 MB  
**Final total:** ~957 headers → ~4 MB complete extraction

---

## Conclusion

The **most valuable remaining headers** for Ordo are:

1. **AI accelerators** - Hardware ML inference
2. **Virtualization** - Subagent isolation via VFIO/Xen
3. **XDP sockets** - Fast packet I/O
4. **TEE** - Secure computation enclaves
5. **Tracing** - Runtime observability

The remaining ~600 headers are mostly legacy protocols, embedded hardware interfaces, architecture-specific constants, or obsolete features with limited applicability to Ordo's architecture.

**Recommendation:** Parse the high-priority categories (50 headers) to capture the most relevant patterns, then consider the extraction complete for practical Ordo design purposes.
