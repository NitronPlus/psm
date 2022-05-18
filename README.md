![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange.svg)

# Personal SSH server Management Tool

## Introduction

> A cross-platform cli tool for multi remote SSH server management.

## How To Use

### Show list

```bash
> psm ls
```

### Create a server alias

```bash
> psm new alias_name username server_address ssh_port(optional, defautl is 22)
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

Remove an alias
```bash
> psm rm alias
```

### Modify alias fields

It just modifies the present fields.

```bash
> psm upd alias_name -u username -a server_address -p port // will modifiy all fields
> psm upd alias_name -u username // just modify username
> psm upd alias_name -p port // just modify port
> psm upd alias_name -u username -p port // just modify username and port
```

### Copy RSA public key to server

NOTICE: This command won't check if the public key has existed in target server.

```bash
> psm cp alias_name
```

### Configure PSM

```bash
> psm set -c "C:\path\to\ssh_client" // set ssh_client path
> psm set -s "C:\path\to\server.json" // set server file path
> psm set -p "C:\path\to\id_rsa.pub" // set public key path
> psm set -s "C:\path\to\server.json" -p "C:\path\to\id_rsa.pub" // set server file path and public key path in one time 
```

### Show command/subcommand help

```bash
> psm help // show command help
> psm help subcommand  // show specify subcommand help
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
