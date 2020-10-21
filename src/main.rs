use std::process::{Command, Stdio};

use ansi_term::Colour;
use serde_derive::Deserialize;
use structopt::StructOpt;
use std::path::PathBuf;

/// rustrsync
/// Need a toml file with the next format:
///
/// [origin_folder]
///
/// origin = "/home/user/"
/// 
/// [destination_folder]
///
/// destination = "/media/user/external/backup/"
/// 
/// [folders]
///
/// folders = [
///     "Downloads/",
///     "Pictures/",
///     "Music/",
///     "PersonalFiles/"
/// ]
/// 
/// Note:
/// The final backslash is important for the correct execution of rsync.
#[derive(StructOpt, Debug)]
struct Opt {
    /// file with a valid toml configuration for rustrsync
    /// try rustrsync --help
    #[structopt(short, long)]
    file: PathBuf,
}

#[derive(Deserialize, Debug)]
struct OriginFolder {
    origin: String,
}

#[derive(Deserialize, Debug)]
struct DestinationFolder {
    destination: String,
}

#[derive(Deserialize, Debug)]
struct Folders {
    folders: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct FileConfig {
    origin_folder: OriginFolder,
    folders: Folders,
    destination_folder: DestinationFolder,
}

// TODO: - Validate that the file contains a valid configuration
//         and manage the Error panics.
//       - Add a flag --init for create a basic structure toml file.

fn main() {
    // Read and convert the toml file.
    let file_values: FileConfig = {
        // read the argument. I can improve with clap or structopt
        let opt = Opt::from_args();
        let file_path = opt.file; 
        // Read de file.
        let file_text = std::fs::read_to_string(&file_path).expect("Fail to read the file");

        // Parse the toml as the struct FileConfig.
        toml::from_str(&file_text).expect("Error to convert text to toml structure")
    };

    // better names for de values: note the origin and destination are type std::string::String.
    let origin = file_values.origin_folder.origin.as_str();
    let destination = file_values.destination_folder.destination.as_str();
    let folders = file_values.folders.folders;
 
    println!("{}", 
        Colour::Cyan.bold().paint(
            "____________________Initiating Process____________________"));
    println!("{} {}",
        Colour::Yellow.paint("origin:"),
        Colour::Purple.bold().paint(origin));
    println!("{} {}",
        Colour::Yellow.paint("destination:"),
        Colour::Purple.bold().paint(destination));
    println!("{} {:?}",
        Colour::Yellow.paint("Folders:"),
        folders);

    println!("{}",
        Colour::Blue.bold().paint("---------------------------------------------------"));
    // Iterate the folders vector to build and execute the commands
    for folder in folders.iter() {
        if folder == &"" {
            continue
        }
        // Show the full command
        println!("{}",
            Colour::Green.bold().paint(create_command(origin, destination, folder)));
        
        // build the full paths for origin and destination
        let originfull = format!("{}{}", origin, folder);
        let destfull = format!("{}{}", destination, folder);
        // Execute the command
        cmd_exec(&originfull, &destfull)
    }

    println!("{}",
        Colour::Cyan.bold().paint(
            "_________________The Process are Finished_________________"));
}

/// Execute the rsync command
fn cmd_exec(originfull: &str, destfull: &str){
    let mut cmd = Command::new("rsync")
            .arg("-rtvu")
            .arg("--delete")
            .arg("--exclude")
            .arg("node_modules")
            .arg(originfull)
            .arg(destfull)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("rsync command failed to start");

        let status = cmd.wait();
        println!("\n{} {:?}",
            Colour::Blue.bold().paint("Process complete with status:"),
            status);
        println!("{}",
            Colour::Blue.bold().paint(
                "---------------------------------------------------"));
}

/// create a String with the rsync command
fn create_command(origin: &str, dest: &str, folder: &str) -> String {
    format!("rsync -rtuv --delete --exclude node_modules {}{} {}{}",origin,folder,dest,folder)
}
