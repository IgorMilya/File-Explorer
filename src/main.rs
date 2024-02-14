use std::io;
use sysinfo::{Disks};
use file_explorer::checking::{check_os, check_all_disks};
use file_explorer::search::search;

fn main() {
    let mut all_disks = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    loop {
        println!("\nFile Explorer");
        println!("1. Check your OS, version adn memory");
        println!("2. Check all possible disks and their memory");
        println!("3. Search files or directories in particular disk");
        println!("4. Exit");

        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Error reading");

        let user_choice = match user_choice.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("Wrong value, please tyr another");
                continue;
            }
        };

        match user_choice {
            1 => check_os(),
            2 => check_all_disks(&disks, &mut all_disks),
            3 => {
                if all_disks.is_empty() {
                    println!("Initially, you should check your possible disks for searching");
                    continue;
                }
                all_disks.iter().for_each(|disk| {
                    println!("Disk {:?}", disk);
                });


                println!("Choose in what kind of disk you want to search your file. Type a letter");
                let mut user_choice = String::new();

                io::stdin()
                    .read_line(&mut user_choice)
                    .expect("Failed to read");

                let root = format!("{}:\\", user_choice.trim().to_uppercase());

                if !all_disks.iter().any(|disk| disk.to_string_lossy().to_string() == root) {
                    println!("Wrong disk");
                    continue;
                }

                search(root);
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => { break; }
        }
    }
}