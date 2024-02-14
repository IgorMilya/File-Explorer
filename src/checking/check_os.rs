use sysinfo::System;
use crate::utils::bytes_to_gigabytes;


pub fn check_os(){
    let mut system = System::new();
    system.refresh_all();

    let total_memory = bytes_to_gigabytes(system.total_memory());
    let available_memory = bytes_to_gigabytes(system.available_memory());

    println!("OS: {:?}", System::distribution_id());
    println!("Version: {:?}", System::os_version().unwrap());

    println!("Total Memory: {:?} Gb", total_memory);
    println!("Available Memory: {:?} Gb", available_memory);
}
