use std::path::{Path, PathBuf};

use clap::Parser;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::server::{Server, ServerCollection};

pub struct App {}

impl App {
    pub fn run() {
        let config = Config::init();
        let cli = Cli::parse();
        let mut collection: ServerCollection = ServerCollection::read_from(&config.server_path);
        match collection.get(&cli.alias) {
            Some(server) => {
                server.connect(&config);
            }
            None => {}
        };
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
                println!("Server alias {} has been removed", alias)
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
                    println!("Server alias {} has been rename to {}", alias, new_alias);
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
            Some(Commands::Link { alias }) => {
                match collection.get(alias) {
                    None => collection.show_table(),
                    Some(server) => {
                        server.copy_id(&config);
                    }
                };
            }
            Some(Commands::List {}) => {
                collection.show_table();
            }
            Some(Commands::Set {
                pub_key_path,
                server_path,
                client_path,
            }) => {
                let config = Config {
                    pub_key_path: match pub_key_path {
                        Some(val) => App::path_exists(val),
                        _ => config.pub_key_path,
                    },
                    server_path: match server_path {
                        Some(val) => App::path_exists(val),
                        _ => config.server_path,
                    },
                    ssh_client_path: match client_path {
                        Some(val) => App::path_exists(val),
                        _ => config.ssh_client_path,
                    },
                };
                config.save();
            }
            None => {}
        }
    }

    pub fn path_exists(path: &String) -> PathBuf {
        let path = PathBuf::from(path);
        if !path.exists() {
            println!("{:?} not found!", path);
            std::process::exit(1);
        }
        path
    }
}

pub(crate) trait StorageObject {
    fn pretty_json(&self) -> String;
    fn save_to<P: AsRef<Path>>(&self, path: P)
    where
        Self: Serialize;
    fn read_from<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> T;
}

impl<T: Serialize> StorageObject for T {
    fn pretty_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn save_to<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, self.pretty_json()).unwrap();
    }
    fn read_from<R: DeserializeOwned, P: AsRef<Path>>(path: P) -> R {
        let v = std::fs::read_to_string(path).unwrap();
        serde_json::from_str::<R>(&v).unwrap()
    }
}
