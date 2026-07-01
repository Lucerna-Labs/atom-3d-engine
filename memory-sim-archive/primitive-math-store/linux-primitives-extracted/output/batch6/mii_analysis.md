# mii.h

**Source:** `mii.h`


## Includes

- `linux/types.h`
- `linux/ethtool.h`

## Defines (134 total)


### ADVERTISE_PAUSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `ADVERTISE_PAUSE_CAP` | `0x0400` | Try for pause |
| `ADVERTISE_PAUSE_ASYM` | `0x0800` | Try for asymetric pause |

### FLOW_CTRL (2)

| Name | Value | Comment |
|------|-------|---------|
| `FLOW_CTRL_TX` | `0x01` |  |
| `FLOW_CTRL_RX` | `0x02` |  |

### LPA_PAUSE (2)

| Name | Value | Comment |
|------|-------|---------|
| `LPA_PAUSE_CAP` | `0x0400` | Can pause |
| `LPA_PAUSE_ASYM` | `0x0800` | Can pause asymetrically |

### LPA_SGMII (13)

| Name | Value | Comment |
|------|-------|---------|
| `LPA_SGMII_SPD_MASK` | `0x0c00` | SGMII speed mask |
| `LPA_SGMII_FULL_DUPLEX` | `0x1000` | SGMII full duplex |
| `LPA_SGMII_DPX_SPD_MASK` | `0x1C00` | SGMII duplex and speed bits |
| `LPA_SGMII_10` | `0x0000` | 10Mbps |
| `LPA_SGMII_10HALF` | `0x0000` | Can do 10mbps half-duplex |
| `LPA_SGMII_10FULL` | `0x1000` | Can do 10mbps full-duplex |
| `LPA_SGMII_100` | `0x0400` | 100Mbps |
| `LPA_SGMII_100HALF` | `0x0400` | Can do 100mbps half-duplex |
| `LPA_SGMII_100FULL` | `0x1400` | Can do 100mbps full-duplex |
| `LPA_SGMII_1000` | `0x0800` | 1000Mbps |
| `LPA_SGMII_1000HALF` | `0x0800` | Can do 1000mbps half-duplex |
| `LPA_SGMII_1000FULL` | `0x1800` | Can do 1000mbps full-duplex |
| `LPA_SGMII_LINK` | `0x8000` | PHY link with copper-side partner |

### MII_MMD (7)

| Name | Value | Comment |
|------|-------|---------|
| `MII_MMD_CTRL` | `0x0d` | MMD Access Control Register |
| `MII_MMD_DATA` | `0x0e` | MMD Access Data Register |
| `MII_MMD_CTRL_DEVAD_MASK` | `0x1f` | Mask MMD DEVAD |
| `MII_MMD_CTRL_ADDR` | `0x0000` | Address |
| `MII_MMD_CTRL_NOINCR` | `0x4000` | no post increment |
| `MII_MMD_CTRL_INCR_RDWT` | `0x8000` | post increment on reads & writes |
| `MII_MMD_CTRL_INCR_ON_WT` | `0xC000` | post increment on writes only |

### UNCATEGORIZED (108)

| Name | Value | Comment |
|------|-------|---------|
| `MII_BMCR` | `0x00` | Basic mode control register |
| `MII_BMSR` | `0x01` | Basic mode status register |
| `MII_PHYSID1` | `0x02` | PHYS ID 1 |
| `MII_PHYSID2` | `0x03` | PHYS ID 2 |
| `MII_ADVERTISE` | `0x04` | Advertisement control reg |
| `MII_LPA` | `0x05` | Link partner ability reg |
| `MII_EXPANSION` | `0x06` | Expansion register |
| `MII_CTRL1000` | `0x09` | 1000BASE-T control |
| `MII_STAT1000` | `0x0a` | 1000BASE-T status |
| `MII_ESTATUS` | `0x0f` | Extended Status |
| `MII_DCOUNTER` | `0x12` | Disconnect counter |
| `MII_FCSCOUNTER` | `0x13` | False carrier counter |
| `MII_NWAYTEST` | `0x14` | N-way auto-neg test reg |
| `MII_RERRCOUNTER` | `0x15` | Receive error counter |
| `MII_SREVISION` | `0x16` | Silicon revision |
| `MII_RESV1` | `0x17` | Reserved... |
| `MII_LBRERROR` | `0x18` | Lpback, rx, bypass error |
| `MII_PHYADDR` | `0x19` | PHY address |
| `MII_RESV2` | `0x1a` | Reserved... |
| `MII_TPISTATUS` | `0x1b` | TPI status for 10mbps |
| `MII_NCONFIG` | `0x1c` | Network interface config |
| `BMCR_RESV` | `0x003f` | Unused... |
| `BMCR_SPEED1000` | `0x0040` | MSB of Speed (1000) |
| `BMCR_CTST` | `0x0080` | Collision test |
| `BMCR_FULLDPLX` | `0x0100` | Full duplex |
| `BMCR_ANRESTART` | `0x0200` | Auto negotiation restart |
| `BMCR_ISOLATE` | `0x0400` | Isolate data paths from MII |
| `BMCR_PDOWN` | `0x0800` | Enable low power state |
| `BMCR_ANENABLE` | `0x1000` | Enable auto negotiation |
| `BMCR_SPEED100` | `0x2000` | Select 100Mbps |
| `BMCR_LOOPBACK` | `0x4000` | TXD loopback bits |
| `BMCR_RESET` | `0x8000` | Reset to default state |
| `BMCR_SPEED10` | `0x0000` | Select 10Mbps |
| `BMSR_ERCAP` | `0x0001` | Ext-reg capability |
| `BMSR_JCD` | `0x0002` | Jabber detected |
| `BMSR_LSTATUS` | `0x0004` | Link status |
| `BMSR_ANEGCAPABLE` | `0x0008` | Able to do auto-negotiation |
| `BMSR_RFAULT` | `0x0010` | Remote fault detected |
| `BMSR_ANEGCOMPLETE` | `0x0020` | Auto-negotiation complete |
| `BMSR_RESV` | `0x00c0` | Unused... |
| `BMSR_ESTATEN` | `0x0100` | Extended Status in R15 |
| `BMSR_100HALF2` | `0x0200` | Can do 100BASE-T2 HDX |
| `BMSR_100FULL2` | `0x0400` | Can do 100BASE-T2 FDX |
| `BMSR_10HALF` | `0x0800` | Can do 10mbps, half-duplex |
| `BMSR_10FULL` | `0x1000` | Can do 10mbps, full-duplex |
| `BMSR_100HALF` | `0x2000` | Can do 100mbps, half-duplex |
| `BMSR_100FULL` | `0x4000` | Can do 100mbps, full-duplex |
| `BMSR_100BASE4` | `0x8000` | Can do 100mbps, 4k packets |
| `ADVERTISE_SLCT` | `0x001f` | Selector bits |
| `ADVERTISE_CSMA` | `0x0001` | Only selector supported |

*...and 58 more*

## Structs (1)


### `struct mii_ioctl_data`

| Type | Field | Array |
|------|-------|-------|
| `__u16` | `phy_id` | `-` |
| `__u16` | `reg_num` | `-` |
| `__u16` | `val_in` | `-` |
| `__u16` | `val_out` | `-` |