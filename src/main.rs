use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("Please provide a filename as argument");}
    let filename = &args[1];
    
    // Read the file a return a string
    let msg_err = format!("Failed to read: {}", &filename);
    let string_content = read_file_to_string(&filename)
        .expect(&msg_err);

    println!("____________________Initiating Process____________________");
    // Split the file by '\n' and return in a Vec<&str>
    let (origin, destination, folders) = string_to_vector(&string_content);
    println!("origin: {}",origin);
    println!("destination: {}", destination);
    println!("Folders: {:?}", folders);

    println!("---------------------------------------------------");
    // Iterate the folders vector to build the commands
    for folder in folders.iter() {
        if folder == &"" {
            continue
        }
        // Show the full command
        println!("{}", create_command(origin, destination, folder));
        
        // build the full paths for origin and destination
        let originfull = format!("{}{}", origin, folder);
        let destfull = format!("{}{}", destination, folder);
        
        // Execute the command
        cmd_exec(&originfull, &destfull)
    }

    println!("_________________The Process are Finished_________________");
}

fn read_file_to_string(filename: &str) -> io::Result<String> {
    // Open the path in read-only mode, returns 'io::Result<File>'
    let mut file = File::open(filename)?;

    // Read the file contents into string, returns io::Result<usize>
    let mut string_content = String::new();
    file.read_to_string(&mut string_content)?;

    Ok(string_content)
}

fn string_to_vector(text: &str) -> (&str, &str, Vec<&str>) {
    // Split the string into a Vec<&str>
    let mut split_string = text.split("\n");
    
    // using the iterator that returns split 
    let origin = split_string.next().unwrap();
    let destination = split_string.next().unwrap();
    let folders: Vec<_> = split_string.collect();
    (origin, destination, folders)
}

fn cmd_exec(originfull: &str, destfull: &str){
    let mut cmd = Command::new("rsync")
            .arg("-rtvu")
            .arg("--delete")
            .arg(originfull)
            .arg(destfull)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("rsync command failed to start");

        let status = cmd.wait();
        println!("\nProcess complete with status: {:?}", status);
        println!("---------------------------------------------------");
}

fn create_command(origin: &str, dest: &str, folder: &str) -> String {
    format!("rsync -rtuv --delete {}{} {}{}",origin,folder,dest,folder)
}
