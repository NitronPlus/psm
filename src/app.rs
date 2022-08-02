use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::server::{Server, ServerCollection};

pub struct App {
    config: Config,
}

impl App {
    pub fn init(config: Config) -> Self {
        Self { config }
    }
    pub fn run(&self) {
        let cli = Cli::parse();
        let mut collection: ServerCollection =
            ServerCollection::read_from(&self.config.server_file_path);
        match collection.get(&cli.alias) {
            Some(server) => {
                self.connect(server);
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
                        .save_to(&self.config.server_file_path);
                    collection.show_table();
                }
                _ => {
                    println!("Server alias {} was already exists", alias)
                }
            },
            Some(Commands::Remove { alias }) => {
                collection
                    .remove(alias)
                    .save_to(&self.config.server_file_path);
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
                        .save_to(&self.config.server_file_path);
                }
                None => {
                    println!("Cannot find specify alias")
                }
            },
            Some(Commands::Rename { alias, new_alias }) => {
                if collection.rename(alias, new_alias) {
                    collection.save_to(&self.config.server_file_path);
                    println!("Server alias {} has been rename to {}", alias, new_alias);
                } else {
                    println!("Cannot find specify alias");
                }
            }
            Some(Commands::Go { alias }) => {
                match collection.get(alias) {
                    None => collection.show_table(),
                    Some(server) => self.connect(server),
                };
            }
            Some(Commands::List {}) => {
                collection.show_table();
            }
            Some(Commands::Link { alias }) => {
                match collection.get(alias) {
                    None => collection.show_table(),
                    Some(server) => {
                        self.copy_id(server);
                    }
                };
            }
            Some(Commands::Copy {
                recursive,
                download,
                local,
                remote,
            }) => {
                let (alias, path) = Self::parse_remote(remote);
                if *download && (local.len() != 1) {
                    println!("local path must be one");
                    std::process::exit(1);
                }
                match collection.get(&alias.to_string()) {
                    None => collection.show_table(),
                    Some(server) => {
                        if *download {
                            self.download(server, &local[0], path, *recursive);
                        } else {
                            self.upload(server, local, path, *recursive);
                        }
                    }
                };
            }
            Some(Commands::Download {
                recursive,
                remote,
                local,
            }) => {
                let (alias, path) = Self::parse_remote(remote);
                match collection.get(&alias.to_string()) {
                    None => collection.show_table(),
                    Some(server) => self.download(server, local, path, *recursive),
                };
            }
            Some(Commands::Set {
                pub_key_path,
                server_path,
                client_path,
                scp_path,
            }) => {
                let config = Config {
                    pub_key_path: match pub_key_path {
                        Some(val) => Self::path_exists(val),
                        _ => self.config.pub_key_path.to_path_buf(),
                    },
                    server_file_path: match server_path {
                        Some(val) => val.to_path_buf(),
                        _ => self.config.server_file_path.to_path_buf(),
                    },
                    ssh_client_app_path: match client_path {
                        Some(val) => val.to_path_buf(),
                        _ => self.config.ssh_client_app_path.to_path_buf(),
                    },
                    scp_app_path: match scp_path {
                        Some(val) => val.to_path_buf(),
                        _ => self.config.scp_app_path.to_path_buf(),
                    },
                };
                config.save();
            }
            None => {}
        }
    }

    fn parse_remote(remote: &String) -> (&str, &str) {
        let x: Vec<&str> = remote.split(':').collect();
        let (alias, path) = if let [alias, path] = x[..] {
            (alias, path)
        } else {
            println!("{} is not a valid remote path", remote);
            std::process::exit(1);
        };
        (alias, path)
    }

    fn path_exists(path: &PathBuf) -> PathBuf {
        if !path.exists() {
            println!("{:?} not found!", path);
            std::process::exit(1);
        }
        path.to_path_buf()
    }

    fn connect(&self, server: &Server) {
        let host = format!("{}@{}", server.username, server.address);
        let port = format!("-p{}", server.port);
        let args = vec![host, port];
        Command::new(&self.config.ssh_client_app_path)
            .args(args)
            .status()
            .unwrap();
    }

    fn copy_id(&self, server: &Server) {
        let key_string = std::fs::read_to_string(&self.config.pub_key_path).unwrap();
        let host = format!("{}@{}", server.username, server.address);
        let port = format!("-p{}", server.port);
        let key_string = key_string.replace('\n', "").replace('\r', "");
        let insert_key_cmd = format!(
            "grep -cq '{key_string}' ~/.ssh/authorized_keys || echo {key_string} >> ~/.ssh/authorized_keys ; exit 0;");
        let args = vec![host, port, insert_key_cmd];
        let status = Command::new(&self.config.ssh_client_app_path)
            .args(args)
            .status();
        match status {
            Ok(val) => {
                if let Some(0) = val.code() {
                    println!("Key has been install to {}", server.address)
                } else {
                    println!("Cannot install key to {}", server.address)
                }
            }
            Err(_err) => println!("Fatal error while install key"),
        }
    }

    fn upload(&self, server: &Server, local: &[String], remote: &str, recursive: bool) {
        let host = format!("{}@{}:{}", server.username, server.address, remote);
        let port = if recursive {
            format!("-rP{}", server.port)
        } else {
            format!("-P{}", server.port)
        };
        let mut args = vec![port];
        for path in local.iter() {
            args.push(path.to_string())
        }
        args.push(host);
        Command::new(&self.config.scp_app_path)
            .args(args)
            .status()
            .unwrap();
    }

    // download file from server
    fn download(&self, server: &Server, local: &String, remote: &str, recursive: bool) {
        let host = format!("{}@{}:{}", server.username, server.address, remote);
        let port = if recursive {
            format!("-rP{}", server.port)
        } else {
            format!("-P{}", server.port)
        };
        let mut args = vec![port];
        args.push(host);
        args.push(local.to_string());
        Command::new(&self.config.scp_app_path)
            .args(args)
            .status()
            .unwrap();
    }
}

pub(crate) trait StorageObject {
    fn pretty_json(&self) -> String;
    fn save_to<P: AsRef<Path>>(&self, path: P)
    where
        Self: Serialize;
    fn read_from<T: Default + DeserializeOwned + Serialize, P: AsRef<Path>>(path: P) -> T;
}

impl<T: Serialize> StorageObject for T {
    fn pretty_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    fn save_to<P: AsRef<Path>>(&self, path: P) {
        std::fs::write(path, self.pretty_json()).unwrap();
    }
    fn read_from<R: Default + DeserializeOwned + Serialize, P: AsRef<Path>>(path: P) -> R {
        let v = std::fs::read_to_string(path).unwrap_or_else(|_| R::default().pretty_json());
        serde_json::from_str::<R>(&v).unwrap()
    }
}
