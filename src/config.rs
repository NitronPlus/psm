use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::StorageObject;
use crate::server::ServerCollection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub pub_key_path: PathBuf,
    pub server_path: PathBuf,
    pub ssh_client_path: PathBuf,
}

impl Config {
    pub fn init() -> Self {
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
                Config::read_from(config_path)
            }
            None => {
                println!("Cannot find user's home dir");
                std::process::exit(1);
            }
        }
    }

    pub fn save(&self) {
        let home_dir = dirs::home_dir().unwrap();
        let config_path = home_dir
            .join(".".to_owned() + env!("CARGO_PKG_NAME"))
            .join("config.json");
        self.save_to(config_path)
    }
}
