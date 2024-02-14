use std::path::Path;
use sysinfo::Disks;
use crate::utils::bytes_to_gigabytes;

pub fn check_all_disks<'a>(disks: &'a Disks, all_disks: &mut Vec<&'a Path>) {
    for (index, disk) in disks.iter().enumerate() {
        let mount_point = if all_disks.len() == disks.len() {
            all_disks[index]
        } else {
            all_disks.push(disk.mount_point());
            disk.mount_point()
        };

        let total_space = bytes_to_gigabytes(disk.total_space());
        let available_space = bytes_to_gigabytes(disk.available_space());

        println!("Disk: {:?},
                    \rTotal Space: {:?} Gb,
                    \rAvailable Space: {:?} Gb,
                    \rType of disk: {:?}\n",
                 mount_point,
                 total_space,
                 available_space,
                 disk.kind());
    }
}