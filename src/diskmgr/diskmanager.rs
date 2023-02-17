use super::filesystem::*;
pub use super::part::configure_partitions;
use crate::conf::seed::*;
use libparted::*;
use std::io;

/// Configures the disks from the seedconf
/// # Arguments
/// * `conf` - The config to implement
pub fn configure_disks(conf: &SeedConf) -> Result<(), io::Error> {
    //Iterate over every disk
    for cur_disk_conf in &conf.disks {
        let mut p_dev = Device::new(&cur_disk_conf.path)?;
        let p_dev_sector_size = p_dev.sector_size();
        let mut p_disk = create_disk(&mut p_dev, cur_disk_conf)?;

        //Iterate over the partitions to create them
        for cur_part_conf in &cur_disk_conf.partitions {
            configure_partitions(&mut p_disk, cur_part_conf, p_dev_sector_size)?;
        }

        //Commit that to disk
        p_disk.commit_to_dev()?;
        p_disk.commit_to_os()?;

        //And now create filesystems on the new partitions
        for cur_part_conf in &cur_disk_conf.partitions {
            create_filesystem(cur_disk_conf, cur_part_conf)?;
        }
    }

    Ok(())
}

/// Creates a disk object for the use in configure_disks()
/// # Arguments
/// * `p_dev` - The device to create the partition object from
/// * `d_conf` - The disk configuration to implement
fn create_disk<'a>(p_dev: &'a mut Device, d_conf: &DiskConf) -> Result<Disk<'a>, io::Error> {
    match d_conf.action {
        DiskAction::New => {
            info!(
                "Creating new partition table {} for disk {}",
                d_conf.table.as_ref().unwrap(),
                d_conf.path
            );
            Ok(Disk::new_fresh(
                p_dev,
                match DiskType::get(&d_conf.table.as_ref().unwrap()) {
                    Some(t) => t,
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::Unsupported,
                            format!(
                                "Unknown partition table {} for {}",
                                d_conf.table.as_ref().unwrap(),
                                d_conf.path
                            ),
                        ));
                    }
                },
            )?)
        }

        //Reuse the existing partition table
        _ => {
            info!("Loading partition table from disk {}...", d_conf.path);
            Ok(Disk::new(p_dev)?)
        }
    }
}
