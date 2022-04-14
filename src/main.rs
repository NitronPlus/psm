use dirs;
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use std::fs;
use serde_json;
use cli_table::{Cell, CellStruct, format::Justify, print_stdout, Style, Table};

const SERVER: &str = r#"{
    "hosts": []
}"#;

const APP_NAME: &str = "psm";

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    pub_key_path: PathBuf,
    server_path: PathBuf,
    ssh_client_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    alias: String,
    username: String,
    address: String,
    port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerCollection {
    hosts: Vec<Server>,
}

trait PrettyJson {
    fn pretty_json(&self) -> String where Self: Serialize {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl ServerCollection {
    fn add(mut self, server: Server) -> Self {
        self.hosts.push(server);
        self
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
    match &cli.command {
        Some(Commands::Create { alias, username, address, port }) => {
            let collection = read_servers(&config.server_path);
            if search_server(alias, &collection) == false {
                let server = {
                    Server {
                        alias: alias.to_string(),
                        username: username.to_string(),
                        address: address.to_string(),
                        port: Some(port.to_owned()),
                    }
                };
                let c = collection.add(server);
                std::fs::write(&config.server_path, c.pretty_json()).unwrap();
                println!("server alias {:?} created", c);
            } else {
                println!("server alias {} is exists", alias);
            }
        }
        Some(Commands::Remove {alias}) => {
            let collection = read_servers(&config.server_path);
            println!("Not printing testing lists...");
        }
        Some(Commands::Modify {}) => {
            println!("Not printing testing lists...");
        }
        Some(Commands::Go { alias }) => {
            let collection = read_servers(&config.server_path);
            search_server(alias, &collection);
        }
        Some(Commands::Link {}) => {
            println!("Not printing testing lists...");
        }
        None => {
            let hosts = read_servers(&config.server_path);
            show_table(hosts);
        }
    }
    // let target = &cli.alias;

    // let servers = read_servers(config.server_path);
    // for server in servers.server {
    //     let _port = match server.port {
    //         Some(p) => p,
    //         _ => 22
    //     };
    //     // if target.eq(&server.alias) {
    //     //     let cmd = format!("{} {}@{} -p{}", config.ssh_client_path.to_str().unwrap(), server.username, server.address, port);
    //     //     println!("{}", cmd)
    //     // }
    // }

    // std::process::Command::new(config.ssh_client_path).spawn().unwrap();
}

fn get_home_dir() -> PathBuf {
    let dir = dirs::home_dir();
    match dir {
        Some(t) => t,
        None => panic!("cannot find user home dir")
    }
}

// fn get_config_dir() -> PathBuf {
//     let dir = dirs::config_dir();
//     match dir  {
//         Some(t) => t,
//         None => panic!("cannot find user home dir")
//     }
// }
fn search_server(server_name: &String, collection: &ServerCollection) -> bool {
    let mut is_match: bool = false;
    for server in &collection.hosts {
        if server_name.eq(&server.alias) {
            is_match = true;
            break;
        }
        // if server_name.eq(&server.alias) {
        //     let port = match server.port {
        //         Some(p) => p,
        //         _ => 22
        //     };
        //     let cmd = format!("{}@{}", server.username, server.address);
        //     let p = format!("-p{}", port);
        //     std::process::Command::new(config.ssh_client_path.to_str().unwrap())
        //         .arg(cmd).arg(p).spawn().unwrap().wait().unwrap();
        // }
    }
    is_match
}

fn init() -> AppConfig {
    let home_dir = get_home_dir();
    let app_config_path = &home_dir.join(".".to_owned() + self::APP_NAME);
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
        for server in collection.hosts {
            let port = match server.port {
                None => 22,
                Some(p) => p
            };
            let col = vec![
                server.alias.cell(),
                server.username.cell().justify(Justify::Right),
                server.address.cell().justify(Justify::Right),
                port.cell().justify(Justify::Right),
            ];
            table.push(col);
        }
        print_stdout(table.table().title(title)).unwrap();
    }
}
