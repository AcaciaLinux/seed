use super::util::*;
use crate::conf::part::*;
use libparted::*;
use std::io;

/// Configures the partitions on the provided disk according to the partition config
/// # Arguments
/// * `p_disk` - The disk to manipulate
/// * `p_conf` - The partition config
/// * `sector_size` - The sector size on the physical disk
pub fn configure_partitions<'a>(
    p_disk: &mut Disk,
    p_conf: &PartConf,
    sector_size: u64,
) -> Result<(), io::Error> {
    match &p_conf.action {
        //Create a partititon
        PartAction::Create => {
            //The partition can't exist already
            match p_disk.get_partition(p_conf.index as u32) {
                Some(p_part) => {
                    return Err(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        format!(
                            "Can't create partition {}: Already exists at {}",
                            p_conf.index,
                            p_part.get_path().unwrap().to_str().unwrap()
                        ),
                    ))
                }
                None => (),
            }

            //Find the starting sector for the new disk
            let start_sector = match get_next_possible_start(p_disk) {
                Some(v) => v,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unable to determine start of new partition",
                    ))
                }
            };

            //Calculate the size
            let size = get_part_size_sectors(p_conf.size.as_ref().unwrap(), sector_size);

            //Inform the user about the cange
            info!(
                "Creating new partition nr.{}: start = sector {}, end = sector {}",
                p_conf.index,
                start_sector,
                start_sector + size
            );

            //Create the new partition
            let mut new_part = Partition::new(
                p_disk,
                PartitionType::PED_PARTITION_NORMAL,
                None,
                start_sector,
                start_sector + size,
            )?;

            //And add it to the disk
            p_disk.add_partition(&mut new_part, p_disk.constraint_any().as_ref().unwrap())?;

            Ok(())
        }

        //We don't need to alter the partition
        PartAction::Keep | PartAction::Format => match p_disk.get_partition(p_conf.index as u32) {
            Some(p_part) => {
                info!(
                    "Found partition {} at path {}",
                    p_conf.index,
                    p_part.get_path().unwrap().to_str().unwrap()
                );
                Ok(())
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Could not find partition with index {}", p_conf.index),
                ))
            }
        },

        //Error on any unimplememted options
        other => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Partition action '{:?}': Not yet implemented", other),
        )),
    }
}
