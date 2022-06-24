use app::App;

mod app;
mod cli;
mod config;
mod server;

fn main() {
    App::init(config::Config::init()).run();
}
