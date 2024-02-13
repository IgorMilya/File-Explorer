use std::io;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

pub fn search(all_disks: &Vec<&Path>) {
    all_disks.iter().for_each(|disk| {
        println!("Disk {:?}", disk);
    });


    println!("Choose in what kind of disk you want to search your file. Type a letter");
    let mut user_choice = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read");


    println!("Your directory or file:");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read");

    let root = format!("{}:\\", user_choice.trim().to_uppercase());

    let start_time = Instant::now();

    println!("Loading...\n");
    let mut found = false;

    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .for_each(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string().to_lowercase();
            let file_path = entry.path().to_string_lossy().to_string();

            let dir_filetype = entry.file_type();

            if file_name.contains(&user_input.trim().to_lowercase()) {
                found = true;
                let file_type = if dir_filetype.is_dir() {
                    "DIRECTORY"
                } else {
                    "FILE"
                };
                println!("File Type: {:?},\nFile Name: {:?},\nFile Path: {}\n",
                         file_type, entry.path().file_name().unwrap(), file_path);
            }
        });

    if !found {
        println!("No files or directories matching the search criteria found.");
    }

    let elapsed_time = start_time.elapsed();
    println!("Time elapsed: {:?}", elapsed_time);
}