use clap::{Parser, Subcommand};
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

const SERVER: &str = r#"{
    "hosts": {}
}"#;

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    pub_key_path: PathBuf,
    server_path: PathBuf,
    ssh_client_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    pub username: String,
    pub address: String,
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerCollection {
    hosts: BTreeMap<String, Server>,
}

trait PrettyJson {
    fn pretty_json(&self) -> String
    where
        Self: Serialize,
    {
        serde_json::to_string_pretty(self).unwrap()
    }
}

trait SaveToFile {
    fn save_to(&self, path: &PathBuf)
    where
        Self: PrettyJson,
        Self: Serialize,
    {
        std::fs::write(&path, self.pretty_json()).unwrap();
    }
}

impl ServerCollection {
    fn get(&mut self, key: &String) -> Option<&Server> {
        self.hosts.get(key)
    }
    fn insert(&mut self, key: String, server: Server) -> Option<Server> {
        self.hosts.insert(key, server)
    }
    fn remove(&mut self, key: &String) -> &mut ServerCollection {
        self.hosts.remove(key);
        self
    }
    fn is_empty(&self) -> bool {
        self.hosts.is_empty()
    }

    fn rename(&mut self, from: &String, to: &String) -> bool {
        match self.get(from) {
            None => false,
            Some(server) => {
                let new_value = Server {
                    username: server.username.to_string(),
                    address: server.address.to_string(),
                    port: server.port,
                };
                self.remove(from).insert(to.to_string(), new_value);
                true
            }
        }
    }

    fn show_table(&self) {
        if !self.is_empty() {
            let title = vec![
                "Alias".cell().bold(true),
                "Username".cell().bold(true),
                "Address".cell().bold(true),
                "Port".cell().bold(true),
            ];
            let mut table: Vec<Vec<CellStruct>> = Vec::new();
            for (alias, server) in &self.hosts {
                let port = server.port.unwrap_or(22);
                let col = vec![
                    alias.cell(),
                    server.username.to_string().cell().justify(Justify::Right),
                    server.address.to_string().cell().justify(Justify::Right),
                    port.cell().justify(Justify::Right),
                ];
                table.push(col);
            }
            print_stdout(table.table().title(title)).unwrap();
        }
    }
}

impl PrettyJson for ServerCollection {}

impl PrettyJson for AppConfig {}

impl SaveToFile for ServerCollection {}

impl SaveToFile for AppConfig {}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
#[clap(subcommand_negates_reqs(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Create alias for a remote SSH server", name = "new")]
    Create {
        alias: String,
        username: String,
        address: String,
        #[clap(default_value_t = 22)]
        port: u16,
    },
    #[clap(about = "Remove the specify alias", name = "rm")]
    Remove {
        alias: String,
    },
    #[clap(about = "Modify the specify alias", name = "edit")]
    Modify {
        alias: String,
        username: Option<String>,
        address: Option<String>,
        port: Option<u16>,
    },
    #[clap(about = "Rename the specify alias", name = "mv")]
    Rename {
        alias: String,
        new_alias: String,
    },
    #[clap(about = "Connect to the specify alias server")]
    Go {
        alias: String,
    },
    #[clap(about = "List all alias server", name = "ls")]
    List {},
    Link {},
}

fn main() {
    let config = init();
    let cli = Cli::parse();
    let mut collection = read_servers(&config.server_path);
    match &cli.command {
        Some(Commands::Create {
            alias,
            username,
            address,
            port,
        }) => match collection.get(alias) {
            None => {
                let server = Server {
                    username: username.to_string(),
                    address: address.to_string(),
                    port: Some(port.to_owned()),
                };
                collection.insert(alias.to_string(), server);
                collection.show_table();
            }
            _ => {
                println!("Server alias {} was already exists", alias)
            }
        },
        Some(Commands::Remove { alias }) => {
            collection.remove(alias).show_table();
        }
        Some(Commands::Modify {
            alias,
            username,
            address,
            port,
        }) => match collection.get(alias) {
            Some(server) => {
                let server = Server {
                    username: match username {
                        Some(val) => val.to_string(),
                        _ => server.username.to_string(),
                    },
                    address: match address {
                        Some(val) => val.to_string(),
                        _ => server.address.to_string(),
                    },
                    port: match port {
                        Some(val) => Some(val.to_owned()),
                        _ => server.port,
                    },
                };
                collection.remove(alias).insert(alias.to_string(), server);
                collection.save_to(&config.server_path);
            }
            None => {
                println!("Cannot find specify alias")
            }
        },
        Some(Commands::Rename { alias, new_alias }) => {
            if collection.rename(alias, new_alias) {
                collection.save_to(&config.server_path);
                println!("Server alias {} was rename to {}", alias, new_alias);
            } else {
                println!("Cannot find specify alias");
            }
        }
        Some(Commands::Go { alias }) => {
            match collection.get(alias) {
                None => collection.show_table(),
                Some(server) => {
                    let host = format!("{}@{}", server.username, server.address);
                    let port = format!("-p{}", server.port.unwrap());
                    std::process::Command::new(&config.ssh_client_path)
                        .arg(host)
                        .arg(port)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                }
            };
        }
        Some(Commands::Link {}) => {
            println!("Will implement in future!");
        }
        Some(Commands::List {}) => {
            collection.show_table();
        }
        None => {}
    }
}

fn get_home_dir() -> PathBuf {
    let dir = dirs::home_dir();
    match dir {
        Some(t) => t,
        None => panic!("cannot find user home dir"),
    }
}

fn init() -> AppConfig {
    let home_dir = get_home_dir();
    let app_config_path = &home_dir.join(".".to_owned() + env!("CARGO_PKG_NAME"));
    let key_path = &home_dir.join(".ssh").join("id_rsa.pub");
    let server_path = &app_config_path.join("server.json");
    let config_path = &app_config_path.join("config.json");
    if !app_config_path.exists() {
        fs::create_dir(&app_config_path).unwrap();
        std::fs::write(server_path, self::SERVER).unwrap();
        let config = AppConfig {
            pub_key_path: key_path.to_path_buf(),
            server_path: server_path.to_path_buf(),
            ssh_client_path: PathBuf::from("ssh"),
        };
        config.save_to(config_path);
    }
    let v = std::fs::read_to_string(config_path).unwrap();
    serde_json::from_str(&v).unwrap()
}

fn read_servers(path: &PathBuf) -> ServerCollection {
    let v = std::fs::read_to_string(&path).unwrap();
    serde_json::from_str(&v).unwrap()
}
