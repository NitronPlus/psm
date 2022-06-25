## v0.4.1
* Add ```-r``` flag for subcommand ```psm cp``` for recursively copy entire directories.
* Subcommand ```psm cp``` support wildcard for local files. e.g.
```bash
psm cp path/to/*.files aliat:/path/to/dest 
```

## v0.4.0
* Rename subcommand ```psm cp``` to ```psm ln```
* Rename psm config field, please change the config file manually. In the meantime, subcommand ```psm set``` is also changed.  
```json
{
  "pub_key_path": "path/to/pub_key",
  "server_file_path": "/path/to/server.json",
  "ssh_client_app_path": "path/to/ssh_client_app",
  "scp_app_path": "path/to/scp_app"
}
```
* Rename ```psm set -p "path/to/pub_key"``` to ```psm set -k "path/to/pub_key"```
* Add ```psm set -a "path/to/scp_app"``` to specify scp app path
* New subcommand ```psm cp``` for copy file or dir to remote server.
* Refactor code
