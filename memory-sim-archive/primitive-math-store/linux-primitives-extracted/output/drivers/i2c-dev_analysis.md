# i2c-dev.h

**Source:** `i2c-dev.h`


## Includes

- `linux/types.h`
- `linux/compiler.h`

## Defines (11 total)


### UNCATEGORIZED (11)

| Name | Value | Comment |
|------|-------|---------|
| `I2C_RETRIES` | `0x0701	/* number of times a device address should` |  |
| `I2C_TIMEOUT` | `0x0702` | set timeout in units of 10 ms |
| `I2C_SLAVE` | `0x0703` | Use this slave address |
| `I2C_SLAVE_FORCE` | `0x0706	/* Use this slave address, even if it` |  |
| `I2C_TENBIT` | `0x0704` | 0 for 7 bit addrs, != 0 for 10 bit |
| `I2C_FUNCS` | `0x0705` | Get the adapter functionality mask |
| `I2C_RDWR` | `0x0707` | Combined R/W transfer (one STOP only) |
| `I2C_PEC` | `0x0708` | != 0 to use PEC with SMBus |
| `I2C_SMBUS` | `0x0720` | SMBus transfer |
| `I2C_RDWR_IOCTL_MAX_MSGS` | `42` |  |
| `I2C_RDRW_IOCTL_MAX_MSGS` | `I2C_RDWR_IOCTL_MAX_MSGS` |  |

## Structs (2)


### `struct i2c_smbus_ioctl_data`

| Type | Field | Array |
|------|-------|-------|
| `__u8` | `read_write` | `-` |
| `__u8` | `command` | `-` |
| `__u32` | `size` | `-` |

### `struct i2c_rdwr_ioctl_data`

| Type | Field | Array |
|------|-------|-------|
| `__u32` | `nmsgs` | `-` |