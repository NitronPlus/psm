use dirs;
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use std::fs;
use serde_json;

const SERVER: &str = r#"{
    "server": []
}"#;

const APP_NAME: &str = "psm";

static mut L: &str = "5";

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
    server: Vec<Server>,
}

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
        port: Option<u16>,
    },
    Remove {},
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
            let server = {
                Server {
                    alias: alias.to_string(),
                    username: username.to_string(),
                    address: address.to_string(),
                    port: port.to_owned(),
                }
            };
            println!("{:?}", server);
        }
        Some(Commands::Remove {}) => {
            println!("Not printing testing lists...");
        }
        Some(Commands::Modify {}) => {
            println!("Not printing testing lists...");
        },
        Some(Commands::Go {alias}) => {
            search_server(alias, config);
        },
        Some(Commands::Link {}) => {
            println!("Not printing testing lists...");
        }
        None => {
            let servers = read_servers(config.server_path);
            if  servers.server.is_empty() == false {
                for server in servers.server {
                    let port = match server.port {
                        Some(p) => p,
                        _ => 22
                    };
                    println!("{}, {}, {}, {}", server.alias, server.username,server.address, port);
                }
            }
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
fn search_server(server_name:&String, config:AppConfig)  {
    let servers = read_servers(config.server_path);
    for server in servers.server {
        if server_name.eq(&server.alias) {
            let port = match server.port {
                Some(p) => p,
                _ => 22
            };
            let cmd = format!("{}@{}",  server.username, server.address);
            let p = format!("-p{}", port);
            std::process::Command::new(config.ssh_client_path.to_str().unwrap())
                .arg(cmd).arg(p).spawn().unwrap().wait();
        }
    }
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
            ssh_client_path: PathBuf::from("ssh")
        };
        std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
    }
    let v = std::fs::read_to_string(config_path).unwrap();
    let config: AppConfig = serde_json::from_str(&v).unwrap();
    config
}

fn read_servers(path: PathBuf) -> ServerCollection {
    let v = std::fs::read_to_string(path).unwrap();
    let servers: ServerCollection = serde_json::from_str(&v).unwrap();
    servers
}