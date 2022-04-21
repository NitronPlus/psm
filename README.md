![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange.svg)

# Personal SSH server Management Tool

## Introduction

> A cli tool for multi remote SSH server management.

## How To Use

Show list

```bash
> psm ls
```

Create a server alias

```bash
> psm new alias_name username server_address ssh_port(optional, defautl is 22)
```

Connect to an aliased server

```bash
> psm go alias_name
```

Rename an alias

```bash
> psm mv old_alias_name new_alias_name
```

Remove an alias
```bash
> psm rm alias
```

Modify alias fields, it just modifies the present fields. 

```bash
> psm upd alias_name -u username -a server_address -p port // will modifiy all fields
> psm upd alias_name -u username // just modify username
> psm upd alias_name -p port // just modify port
> psm upd alias_name -u username -p port // just modify username and port
```

Show command/subcommand help

```bash
> psm help // show command help
> psm help subcommand  // show specify subcommand help
```

## Todo

- [x] Basic feature (List/Create/Remove/Rename/Modify/Connect)
- [ ] Copy public RSA key to server
- [ ] Different RSA key for each alias
- [ ] Config use cli
- [ ] Test

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
