extern crate sys_mount;

use std::io::{Error, ErrorKind};
use std::path::*;
use sys_mount::*;

pub use crate::conf::disk::DiskConf;
pub use crate::conf::part::PartConf;
pub use crate::conf::seed::SeedConf;

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
                debug!("Unmounting {}", self.mount.as_ref().unwrap());
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

impl SeedConf {
    /// Mounts the partitions of the seed config in the correct order
    pub fn mount_partitions(&mut self) -> std::io::Result<()> {
        let mut cur_d: usize = 1;
        let mut max_d: usize = 2;

        loop {
            for cur_disk_id in 0..self.disks.len() {
                let cur_disk = &mut self.disks[cur_disk_id];

                for cur_part_id in 0..cur_disk.partitions.len() {
                    let cur_part = &mut cur_disk.partitions[cur_part_id];

                    let mount_depth = Path::new(
                        cur_part
                            .mount
                            .as_ref()
                            .expect("'mount' missing from PartConf, was validate() called?")
                            .as_str(),
                    )
                    .iter()
                    .count();

                    if mount_depth > max_d {
                        max_d = mount_depth + 1;
                    }

                    if mount_depth == cur_d {
                        cur_part.mount(&cur_disk.path)?;
                    }
                }
            }

            cur_d += 1;

            if max_d == cur_d {
                break;
            }
        }

        Ok(())
    }

    /// Unmounts the partitions of the seed config in the correct order
    pub fn unmount_partitions(&mut self) -> std::io::Result<()> {
        let mut cur_d: usize = self.get_max_mount_depth();
        let mut min_d: usize = self.get_max_mount_depth() - 1;

        loop {
            for cur_disk_id in 0..self.disks.len() {
                let cur_disk = &mut self.disks[cur_disk_id];

                for cur_part_id in 0..cur_disk.partitions.len() {
                    let cur_part = &mut cur_disk.partitions[cur_part_id];

                    let mount_depth = Path::new(
                        cur_part
                            .mount
                            .as_ref()
                            .expect("'mount' missing from PartConf, was validate() called?")
                            .as_str(),
                    )
                    .iter()
                    .count();

                    if mount_depth < min_d {
                        min_d = mount_depth - 1;
                    }

                    if mount_depth == cur_d {
						if cur_part.is_mounted(){
							cur_part.unmount()?;
						}
                    }
                }
            }

            cur_d -= 1;

            if min_d == cur_d {
                break;
            }
        }
        Ok(())
    }

    fn get_max_mount_depth(&self) -> usize {
        let mut max_d: usize = 0;
        for cur_disk in &self.disks {
            for cur_part in &cur_disk.partitions {
                let mount_depth = Path::new(cur_part.mount.as_ref().unwrap().as_str())
                    .iter()
                    .count();
                if mount_depth > max_d {
                    max_d = mount_depth;
                }
            }
        }
        max_d
    }
}
