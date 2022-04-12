use std::fs;
use dirs;
use std::path::{PathBuf};
// use std::fs;

const CONFIG:&str = r#"[tsm]
key_path = "{key}"
server_path = "{server}""#;


fn main() -> std::io::Result<()> {
    // let home_dir  = get_home_dir();
    // println!("dir {:?}", &home_dir.as_os_str());
    // let config_dir  = get_config_dir();
    // println!("dir {:?}", config_dir.as_os_str());
    // let cfg_dir = home_dir.as_path().join(".tsm");
    // fs::create_dir(cfg_dir)?;
    init();
    Ok(())
}

fn get_home_dir() -> PathBuf {
    let dir = dirs::home_dir();
    match dir  {
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

fn init() {
    let home_dir = get_home_dir();
    let cfg_path = &home_dir.as_path().join(".tsm");
    if !cfg_path.exists() {
        fs::create_dir(&cfg_path).unwrap();
        let key_path = &home_dir.as_path().join(".ssh").join("id_rsa.pub");
        let server_path = &cfg_path.join("server.json");
        std::fs::write(server_path, "[]").unwrap();
        let config_path = &cfg_path.join("config.toml");
        let config = &self::CONFIG.replace("{key}", key_path.to_str().unwrap())
            .replace("{server}", server_path.to_str().unwrap());
        // let config = str::replace(&self::CONFIG, "{key}", key_path.to_str().unwrap());
        // let config = str::replace(&*config, "{server}", server_path.to_str().unwrap());
        println!("cfg {}", config);
        std::fs::write(config_path, config.to_string()).unwrap();
        let r = std::fs::read_to_string(server_path).unwrap();
        println!("r {:?}", r);
    }

    println!("{:?}", cfg_path);
}