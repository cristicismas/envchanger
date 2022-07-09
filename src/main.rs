#[macro_use]
mod errors;
mod args;

use args::Args;
use directories::ProjectDirs;
use std::fs::{create_dir_all, read_dir, read_to_string, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args = Args::new();

    match args.command.as_str() {
        "help" => display_available_commands(),
        "list" => list_folder_contents(),
        "folder" => set_folder(&args.folder_name),
        name => {
            println!("{}", name);
        }
    }
}

fn display_available_commands() {
    let commands = vec!["help", "folder", "list", "[env_name]"];
    println!("\nUsage: envch [COMMAND | ENV_NAME] [FOLDER_NAME]\n\n");
    println!("Available commands: \n");

    for command in commands {
        match command {
            "help" => println!("help - Displays usage info and available commands.\n"),
            "folder" => println!("folder - Sets the source folder of all the .env files.\n"),
            "list" => println!("list - Lists all available .env files.\n"),
            "[env_name]" => println!("[env_name] - The name of the file in the selected folder. To view the selected folder type `envch list`\n"),
            // Impossible case.
            _ => ()
        }
    }
    println!();
}

fn set_folder(name_reference: &Option<String>) {
    if let Some(data_directory) = ProjectDirs::from("", "", "envchanger") {
        init_folder_if_not_existent(data_directory.config_dir());
        create_config_file(data_directory.config_dir(), name_reference);

        println!("Folder set to: {:?}", name_reference.as_ref().unwrap());
    } else {
        equit!("Cannot find a data directory for your current operating system.");
    }
}

fn init_folder_if_not_existent(path: &Path) {
    if !path.exists() {
        match create_dir_all(path) {
            Ok(_) => (),
            Err(_) => {
                equit!("Cannot create a directory inside {:?}", path.display());
            }
        }
    }
}

fn create_config_file(path: &Path, folder_name: &Option<String>) {
    let file_name = path.join("folder");

    let mut file: File = match File::create(file_name) {
        Ok(created_file) => created_file,
        Err(_) => {
            equit!("Cannot create config file. Please try again, or file a bug report.",);
        }
    };

    match write!(file, "{}", folder_name.as_ref().unwrap()) {
        Ok(_value) => (),
        Err(error) => panic!("{}", error),
    }
}

fn list_folder_contents() {
    if let Some(data_directory) = ProjectDirs::from("", "", "envchanger") {
        let config_dir = data_directory.config_dir().join("folder");

        let folder_name = read_to_string(config_dir).expect("You need to set the folder where your .env files reside before using `envch list`.\nTo do that please use `envch folder {folder_name}` and make sure this folder exists.");

        let environments = match read_dir(folder_name) {
            Ok(envs) => envs,
            Err(_) => equit!("Cannot read selected folder. Please make sure the folder is set (with `chenv folder [folder_name]`), and that the folder actually exists."),
        };

        println!("Here are the environment which you can import into your current folder:\n");

        for path in environments {
            println!("{}", path.unwrap().file_name().to_str().unwrap())
        }

        println!("\nTo import any of these .env files, simply go to your target directory and type `envch [environment]`.\n");
    } else {
        equit!("Cannot find a data directory for your current operating system.");
    }
}
