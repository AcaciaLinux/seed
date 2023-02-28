extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod conf;
mod diskmgr;
mod libc;

use conf::installfile::*;
use diskmgr::*;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace")
    }
    pretty_env_logger::init();

    let mut conf: InstallFile =
        match serde_json::from_str(std::fs::read_to_string("install.json").unwrap().as_str()) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to parse InstallFile: {}", e.to_string());
                return;
            }
        };

    match conf.validate() {
        Ok(_) => info!("Installation file is valid"),
        Err(e) => {
            error!("{}", e.to_string());
            return;
        }
    }

    match diskmanager::configure_disks(&mut conf.seed) {
        Ok(_) => info!("Configured disks"),
        Err(e) => {
            error!("{}", e.to_string());
            return;
        }
    };
}
