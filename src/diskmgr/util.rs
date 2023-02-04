use crate::conf::part::*;
use libparted::*;

///	Converts the provided byte count to sector count by aligning to next sector
/// If the count doesn't fit perfectly into the sector, the sector count will be higher
/// # Arguments
/// * `bytes` - The bytes to convert
/// * `sector_size` - The sector size to align to
pub fn bytes_to_sectors(bytes: u64, sector_size: u64) -> u64 {
    let f_bytes = bytes as f64;
    let f_sector_size = sector_size as f64;

    let temp = (f_bytes / f_sector_size) as u64;

    match bytes % sector_size {
        0 => temp,
        _ => temp + 1,
    }
}

/// Determines the next possible start for a new partition in sector number
/// # Arguments
/// * `p_disk` - The disk to align to
pub fn get_next_possible_start<'a>(p_disk: &Disk<'a>) -> Option<i64> {
    let last_part_num = match p_disk.get_last_partition_num() {
        Some(n) => n,
        None => return Some(0),
    };

    let part = p_disk.get_partition(last_part_num)?;

    Some(part.geom_end() + 1)
}

/// Convert the supplied PartSize to sectors
/// # Arguments
/// * `p_size` - The size enum to convert
/// * `sector_size` - The sector size to use
pub fn get_part_size_sectors(p_size: &PartSize, sector_size: u64) -> i64 {
    bytes_to_sectors(
        match p_size {
            PartSize::Size(count, unit) => data_size_bytes(*count as i64, unit) as u64,
            _ => 1,
        },
        sector_size,
    ) as i64
}
