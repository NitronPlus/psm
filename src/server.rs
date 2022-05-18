use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use serde::{Deserialize, Serialize};

use crate::app::StorageObject;
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCollection {
    hosts: BTreeMap<String, Server>,
}

impl ServerCollection {
    pub fn init(path: &Path) {
        Self {
            hosts: BTreeMap::new(),
        }
        .save_to(path);
    }

    pub fn get(&mut self, key: &String) -> Option<&Server> {
        self.hosts.get(key)
    }

    pub fn insert(&mut self, key: String, server: Server) -> &mut Self {
        self.hosts.insert(key, server);
        self
    }

    pub fn remove(&mut self, key: &String) -> &mut Self {
        self.hosts.remove(key);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.hosts.is_empty()
    }

    pub fn rename(&mut self, from: &String, to: &String) -> bool {
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

    pub fn show_table(&self) {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub username: String,
    pub address: String,
    pub port: u16,
}

impl Server {
    pub fn connect(&self, config: &Config) {
        let host = format!("{}@{}", self.username, self.address);
        let port = format!("-p{}", self.port);
        let args = vec![host, port];
        Command::new(&config.ssh_client_path)
            .args(args)
            .status()
            .unwrap();
    }

    pub fn copy_id(&self, config: &Config) {
        let key_string = std::fs::read_to_string(&config.pub_key_path).unwrap();
        let host = format!("{}@{}", self.username, self.address);
        let port = format!("-p{}", self.port);
        let insert_key_cmd = format!(
            "echo {} >> ~/.ssh/authorized_keys ; exit 0;",
            key_string.replace('\n', "").replace('\r', "")
        );
        let args = vec![host, port, insert_key_cmd];
        let status = Command::new(&config.ssh_client_path).args(args).status();
        match status {
            Ok(val) => {
                if let Some(0) = val.code() {
                    println!("Key has been install to {}", self.address)
                } else {
                    println!("Cannot install key to {}", self.address)
                }
            }
            Err(_err) => println!("Fatal error while install key"),
        }
    }
}
