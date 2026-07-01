# cifs_mount.h

**Source:** `cifs_mount.h`


## Defines (5 total)


### CIFS_MAX (4)

| Name | Value | Comment |
|------|-------|---------|
| `CIFS_MAX_DOMAINNAME_LEN` | `256` | max fully qualified domain name |
| `CIFS_MAX_USERNAME_LEN` | `256` | reasonable max for current servers |
| `CIFS_MAX_PASSWORD_LEN` | `512` | Windows max seems to be 256 wide chars |
| `CIFS_MAX_SHARE_LEN` | `256` | reasonable max share name length |

### CIFS_NI (1)

| Name | Value | Comment |
|------|-------|---------|
| `CIFS_NI_MAXHOST` | `1024` | max host name length (256 * 4 bytes) |