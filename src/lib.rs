use clap::{Parser, Subcommand};
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub struct App {}

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
    #[clap(
        about = "Create alias for a remote SSH server",
        name = "new",
        display_order = 3
    )]
    Create {
        alias: String,
        username: String,
        address: String,
        #[clap(default_value_t = 22)]
        port: u16,
    },
    #[clap(about = "Remove the specify alias", name = "rm", display_order = 4)]
    Remove { alias: String },
    #[clap(about = "Modify the specify alias", name = "upd", display_order = 6)]
    Modify {
        alias: String,
        #[clap(short, display_order = 1)]
        username: Option<String>,
        #[clap(short, display_order = 2)]
        address: Option<String>,
        #[clap(short, display_order = 3)]
        port: Option<u16>,
    },
    #[clap(about = "Rename the specify alias", name = "mv", display_order = 5)]
    Rename { alias: String, new_alias: String },
    #[clap(about = "Connect to the specify server alias", display_order = 1)]
    Go { alias: String },
    #[clap(about = "List all server alias", name = "ls", display_order = 2)]
    List {},
    #[clap(
        about = "Copy rsa pub key to remote server(not implement!)",
        name = "cp"
    )]
    Link {},
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pub_key_path: PathBuf,
    server_path: PathBuf,
    ssh_client_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    username: String,
    address: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerCollection {
    hosts: BTreeMap<String, Server>,
}

trait PrettyJson {
    fn pretty_json(&self) -> String;
}

trait SaveToFile {
    fn save_to<P: AsRef<Path>>(&self, path: P)
    where
        Self: Serialize;
}

trait ReadFromFile {
    fn read_from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> T {
        let v = std::fs::read_to_string(path).unwrap();
        serde_json::from_str::<T>(&v).unwrap()
    }
}

impl App {
    pub fn run() {
        let config = Config::init();
        let cli = Cli::parse();
        let mut collection: ServerCollection =
            ServerCollection::read_from_file(&config.server_path);
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
                        port: port.to_owned(),
                    };
                    collection
                        .insert(alias.to_string(), server)
                        .save_to(&config.server_path);
                    collection.show_table();
                }
                _ => {
                    println!("Server alias {} was already exists", alias)
                }
            },
            Some(Commands::Remove { alias }) => {
                collection.remove(alias).save_to(&config.server_path);
                println!("Server alias {} have been removed", alias)
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
                            Some(val) => val.to_owned(),
                            _ => server.port,
                        },
                    };
                    collection
                        .remove(alias)
                        .insert(alias.to_string(), server)
                        .save_to(&config.server_path);
                }
                None => {
                    println!("Cannot find specify alias")
                }
            },
            Some(Commands::Rename { alias, new_alias }) => {
                if collection.rename(alias, new_alias) {
                    collection.save_to(&config.server_path);
                    println!("Server alias {} have been rename to {}", alias, new_alias);
                } else {
                    println!("Cannot find specify alias");
                }
            }
            Some(Commands::Go { alias }) => {
                match collection.get(alias) {
                    None => collection.show_table(),
                    Some(server) => {
                        server.connect(&config);
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
}

impl Config {
    fn init() -> Self {
        match dirs::home_dir() {
            Some(home_dir) => {
                let app_config_path = home_dir.join(".".to_owned() + env!("CARGO_PKG_NAME"));
                let pub_key_path = home_dir.join(".ssh").join("id_rsa.pub");
                let server_path = app_config_path.join("server.json");
                let config_path = app_config_path.join("config.json");
                if !app_config_path.exists() {
                    std::fs::create_dir(&app_config_path).unwrap();
                    ServerCollection::init(&server_path);
                    let config = Config {
                        pub_key_path,
                        server_path,
                        ssh_client_path: PathBuf::from("ssh"),
                    };
                    config.save_to(&config_path);
                }
                Config::read_from_file(config_path)
            }
            None => panic!("cannot find user home dir"),
        }
    }
}

impl Server {
    fn connect(&self, config: &Config) {
        let host = format!("{}@{}", self.username, self.address);
        let port = format!("-p{}", self.port);
        std::process::Command::new(&config.ssh_client_path)
            .arg(host)
            .arg(port)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

impl ServerCollection {
    fn init(path: &Path) {
        Self {
            hosts: BTreeMap::new(),
        }
        .save_to(path);
    }

    fn get(&mut self, key: &String) -> Option<&Server> {
        self.hosts.get(key)
    }

    fn insert(&mut self, key: String, server: Server) -> &mut Self {
        self.hosts.insert(key, server);
        self
    }

    fn remove(&mut self, key: &String) -> &mut Self {
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
                let port = server.port;
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

impl<T: Serialize> PrettyJson for T {
    fn pretty_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

impl<T: PrettyJson> SaveToFile for T {
    fn save_to<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, self.pretty_json()).unwrap();
    }
}

impl<T: DeserializeOwned> ReadFromFile for T {}