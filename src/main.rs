mod utils;

use sysinfo::{System, Disks};
use walkdir::WalkDir;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let mut system = System::new();
    system.refresh_all();

    println!("{:?}", System::distribution_id());
    println!("{:?}", System::os_version().unwrap());


    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{:?}, {:?}, {:?}", disk.mount_point(), disk.available_space(), disk.kind());
    }


    for entry in WalkDir::new("D://Test").contents_first(true) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }

    WalkDir::new("D://")
        .into_iter()
        .filter_map(Result::ok)
        .for_each(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path().to_string_lossy().to_string();


            let walkdir_filetype = entry.file_type();
            if file_name == "Test" {
                println!("file_type: {:?}, file_name: {}, file_path: {}", walkdir_filetype, file_name, file_path);
            }
            // let file_type = if walkdir_filetype.is_dir() {
            //     "DIRECTORY"
            // } else {
            //     "FILE"
            // }
            //     .to_string();
            //
            // println!("file_type: {}, file_name: {}, file_path: {}", file_type, file_name, file_path);
        });

    let elapsed_time = start_time.elapsed();
    println!("Time elapsed: {:?}", elapsed_time);
}