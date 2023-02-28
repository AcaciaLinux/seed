extern crate sys_mount;

use std::io::{Error, ErrorKind};
use sys_mount::*;

pub use crate::conf::disk::DiskConf;
pub use crate::conf::part::PartConf;

impl PartConf {
    pub fn mount(&mut self, disk_path: &str) -> std::io::Result<()> {
        let mount = match &self.mount {
            Some(m) => Ok(m),
            None => Err(Error::new(
                ErrorKind::Other,
                "PartConf: Missing attribute 'mount'",
            )),
        }?;

        let mount_source = format!("{}{}", disk_path, self.index);
        let mount_point = format!("/root/mount{}", mount);

        if self.mount_point.is_some() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Mount point {} is already mounted", mount_point),
            ));
        }

        debug!("Mounting '{}' to '{}'...", mount_source, mount_point);
        std::fs::create_dir_all(&mount_point)?;

        self.mount_point = Some(Mount::builder().mount(mount_source, mount_point)?);
        crate::libc::sync();

        Ok(())
    }

    pub fn unmount(&mut self) -> std::io::Result<()> {
        match &self.mount_point {
            Some(s) => {
                s.unmount(UnmountFlags::empty())?;
                self.mount_point = None;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.mount_point.is_some()
    }
}
