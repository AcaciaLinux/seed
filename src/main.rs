extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod conf;
mod diskmgr;
mod leaf;
mod libc;

use conf::installfile::*;
use diskmgr::*;

use clap::Parser;

/// The AcaciaLinux installer daemon
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory seed should mount its root
    #[arg(short, long, default_value = "./seed_workdir/")]
    workdir: String,

    /// The installfile to process
    file: String,
}

fn main() {
    let args = Args::parse();

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
    conf.seed.workdir = "/root/".to_owned();

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

    match leaf::leaf_install_system(&conf.seed) {
        Ok(_) => info!("Installed system"),
        Err(e) => {
            error!("{}", e.to_string());
            return;
        }
    }
}
