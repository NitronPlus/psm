use std::collections::BTreeMap;
use dirs;
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use std::fs;
use serde_json;
use cli_table::{Cell, CellStruct, format::Justify, print_stdout, Style, Table};

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
    username: String,
    address: String,
    port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerCollection {
    hosts: BTreeMap<String, Server>,
}

trait PrettyJson {
    fn pretty_json(&self) -> String where Self: Serialize {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl PrettyJson for ServerCollection {}

impl PrettyJson for AppConfig {}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // alias: Option<String>,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        alias: String,
        username: String,
        address: String,
        #[clap(default_value_t = 22)]
        port: u16,
    },
    Remove {
        alias: String,
    },
    Modify {},
    Go {
        alias: String
    },
    Link {},
}

fn main() {
    let config = init();
    let cli = Cli::parse();
    let mut collection = read_servers(&config.server_path);
    match &cli.command {
        Some(Commands::Create { alias, username, address, port }) => {
            match collection.hosts.get(alias) {
                None => {
                    let server = Server {
                        username: username.to_string(),
                        address: address.to_string(),
                        port: Some(port.to_owned()),
                    };
                    collection.hosts.insert(alias.to_string(), server);
                    std::fs::write(&config.server_path, collection.pretty_json()).unwrap();
                    show_table(collection);
                }
                _ => {
                    println!("{} was already exists", alias)
                }
            }
        }
        Some(Commands::Remove { alias }) => {
            collection.hosts.remove(alias);
            show_table(collection);
        }
        Some(Commands::Modify {}) => {
            println!("Not printing testing lists...");
        }
        Some(Commands::Go { alias }) => {
            match collection.hosts.get(alias) {
                None => show_table(collection),
                Some(server) => {
                    let host = format!("{}@{}", server.username, server.address);
                    let port = format!("-p{}", server.port.unwrap());
                    std::process::Command::new(config.ssh_client_path)
                        .arg(host).arg(port)
                        .spawn().unwrap().wait().unwrap();
                }
            };
        }
        Some(Commands::Link {}) => {
            println!("Not printing testing lists...");
        }
        None => {
            if !collection.hosts.is_empty() {
                show_table(collection);
            }
        }
    }
}

fn get_home_dir() -> PathBuf {
    let dir = dirs::home_dir();
    match dir {
        Some(t) => t,
        None => panic!("cannot find user home dir")
    }
}

fn search_server<'a>(alias: &'a String, collection: &'a ServerCollection) -> Option<&'a Server> {
    collection.hosts.get(alias)
}

fn init() -> AppConfig {
    let home_dir = get_home_dir();
    let app_config_path = &home_dir.join(".".to_owned() + option_env!("CARGO_PKG_NAME").unwrap());
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
        std::fs::write(config_path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
    }
    let v = std::fs::read_to_string(config_path).unwrap();
    serde_json::from_str(&v).unwrap()
}

fn read_servers(path: &PathBuf) -> ServerCollection {
    let v = std::fs::read_to_string(&path).unwrap();
    serde_json::from_str(&v).unwrap()
}

fn show_table(collection: ServerCollection) {
    if collection.hosts.is_empty() == false {
        let title = vec![
            "Alias".cell().bold(true),
            "Username".cell().bold(true),
            "Address".cell().bold(true),
            "Port".cell().bold(true),
        ];
        let mut table: Vec<Vec<CellStruct>> = Vec::new();
        for (alias, server) in collection.hosts {
            let port = match server.port {
                None => 22,
                Some(p) => p
            };
            let col = vec![
                alias.cell(),
                server.username.cell().justify(Justify::Right),
                server.address.cell().justify(Justify::Right),
                port.cell().justify(Justify::Right),
            ];
            table.push(col);
        }
        print_stdout(table.table().title(title)).unwrap();
    }
}
