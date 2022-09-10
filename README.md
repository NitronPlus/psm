![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange.svg)

# Personal SSH server Management Tool

## Introduction

> A cross-platform cli tool for multi remote SSH server management.

[Change Log](CHANGELOG.md)

## Requirements

ssh and scp installed on system.

## Usage

### Show list

```bash
> psm ls
```

### Create a server alias

```bash
> psm new alias_name username@host[:port] (port is optional, default is 22)
```

### Connect to an aliased server

```bash
> psm alias_name
> psm go alias_name
```

### Rename an alias

```bash
> psm mv old_alias_name new_alias_name
```

### Remove an alias

```bash
> psm rm alias
```

### Modify alias

```bash
> psm upd alias_name username@host[:port] 
```

### Copy RSA public key to server


```bash
> psm ln alias_name
```

### Copy local file/dir to remote server


```bash
> psm cp path/to/source alias_name:path/to/destination
# Recursively copy entire directories
> psm cp -r path/to/dir alias_name:path/to/destination
```

### Configure PSM

```bash
# set ssh_client path
> psm set -c "C:\path\to\ssh_client" 
# set server file path 
> psm set -s "C:\path\to\server.json" 
# set public key path
> psm set -k "C:\path\to\id_rsa.pub" 
# set server file path and public key path in one time
> psm set -s "C:\path\to\server.json" -k "C:\path\to\id_rsa.pub"   
```

### Show command/subcommand help

```bash
# show command help info
> psm help  
# show specify subcommand help info
> psm help subcommand  
```

## Todo

- [x] Basic feature (List/Create/Remove/Rename/Modify/Connect)
- [x] Copy RSA public key to server
- [ ] Different RSA key for each alias
- [x] Config use cli
- [ ] Test

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
