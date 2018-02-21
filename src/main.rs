use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("Please provide a filename as argument");}
    let filename = &args[1];
    // Open the path in read-only mode, returns 'io::Result<File>'
    let mut file = match File::open(&filename){
        // The description method of io::Error returns a string that
        // describes the error
        Err(why) => panic!("Could't open {}: {}", filename, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into string, returns io::Result<usize>
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could't read {}: {}", filename, why.description()),
        Ok(_) => print!("The file {} contains:\n{}", filename, s),
    }
    println!("---------------------------------------------------");
    // Split the string into a Vec<&str>
    let split_string = s.split("\n");
    let vec_string: Vec<&str> = split_string.collect();
    // Create the origin, destination and folders vars
    let origin = vec_string[0];
    let destination = vec_string[1];
    let folders = &vec_string[2..vec_string.len()];
    println!("origin: {}",origin);
    println!("destination: {}", destination);
    println!("Folders: {:?}", folders);

    println!("---------------------------------------------------");
    // use for in folders Vec
    for folder in folders.iter() {
        if folder == &"" {
            continue
        }
        let cmd_rsync = create_command(origin, destination, folder);
        println!("{}", cmd_rsync);
        let originful = format!("{}{}", origin, folder);
        let destfull = format!("{}{}", destination, folder);
        let output = Command::new("rsync")
            .arg("-rtvu")
            .arg("--delete")
            .arg(originful)
            .arg(destfull)
            .output()
            .expect("rsync command failed to start");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("---------------------------------------------------");
    }
}

fn create_command(origin: &str, dest: &str, folder: &str) -> String {
    format!("rsync -rtuv --delete {}{} {}{}",origin,folder,dest,folder)
}
