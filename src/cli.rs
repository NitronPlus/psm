use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
#[clap(subcommand_negates_reqs(true))]
pub struct Cli {
    #[clap(default_value = "-", hide_default_value(true), hide(true))]
    pub alias: String,
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
    #[clap(about = "Copy RSA public key to remote server", name = "ln")]
    Link { alias: String },
    #[clap(about = "Copy files to remote server", name = "cp")]
    Scp {
        source: PathBuf,
        alias: String,
    },
    #[clap(about = "Configure PSM")]
    Set {
        #[clap(short = 'k')]
        pub_key_path: Option<PathBuf>,
        #[clap(short)]
        server_path: Option<PathBuf>,
        #[clap(short)]
        client_path: Option<PathBuf>,
        #[clap(short = 'a')]
        scp_path: Option<PathBuf>,
    },
}
