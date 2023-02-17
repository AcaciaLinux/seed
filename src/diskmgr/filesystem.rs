use crate::conf::disk::*;
use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn create_filesystem(d_conf: &DiskConf, p_conf: &PartConf) -> Result<(), std::io::Error> {
    let fs = match &p_conf.fs {
        Some(f) => f,
        None => panic!("Help! No filesystem set!"),
    };

    let a_fs = format!("-t{}", fs);
    let a_path = format!("{}{}", d_conf.path, p_conf.index);

    info!("Creating filesystem using 'mkfs {a_fs} {a_path}'");

    let output = Command::new("mkfs")
        .arg(a_fs)
        .arg(a_path)
        .output()?;

    if !output.status.success() {
        let err_msg = String::from_utf8(output.stderr).unwrap().replace("\n", "");
        return Err(Error::new(ErrorKind::Other, err_msg));
    }

    Ok(())
}
