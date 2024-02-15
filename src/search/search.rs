use std::io;
use std::time::Instant;
use walkdir::{WalkDir};
use crate::utils::{DIRECTORY, FILE};

pub fn search(root: String) {
    println!("Your directory or file:");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read");


    let start_time = Instant::now();

    println!("Loading...\n");
    let mut found = false;
    let mut found_files = Vec::new();

    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .for_each(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string().to_lowercase();
            let file_path = entry.path().to_string_lossy().to_string();
            let dir_filetype = entry.file_type();
            let file_type = if entry.file_type().is_dir() {
                DIRECTORY
            } else {
                FILE
            };

            if file_name.contains(&user_input.trim().to_lowercase()) {
                found = true;

                if dir_filetype.is_dir() {
                    println!("File Type: {:?},\nFile Name: {:?},\nFile Path: {}\n",
                             file_type, entry.path().file_name().unwrap(), file_path);
                } else {
                    found_files.push((file_type, entry));
                };
            }
        });

    if !found_files.is_empty() {
        println!("Files found:");
        for (file_type, entry) in found_files {
            println!("File Type: {:?},\nFile Name: {:?},\nFile Path: {}\n",
                     file_type, entry.path().file_name().unwrap(), entry.path().to_string_lossy());
        }
    }

    // Simple version without filtering on files and directories
    // WalkDir::new(root)
    //     .into_iter()
    //     .filter_map(Result::ok)
    //     .for_each(|entry| {
    //         let file_name = entry.file_name().to_string_lossy().to_string().to_lowercase();
    //         let file_path = entry.path().to_string_lossy().to_string();
    //
    //         let dir_filetype = entry.file_type();
    //
    //         if file_name.contains(&user_input.trim().to_lowercase()) {
    //             found = true;
    //             let file_type = if dir_filetype.is_dir() {
    //                 "DIRECTORY"
    //             } else {
    //                 "FILE"
    //             };
    //             println!("File Type: {:?},\nFile Name: {:?},\nFile Path: {}\n",
    //                      file_type, entry.path().file_name().unwrap(), file_path);
    //         }
    //     });

    if !found {
        println!("No files or directories matching the search criteria found.");
    }

    let elapsed_time = start_time.elapsed();
    println!("Time elapsed: {:?}", elapsed_time);
}