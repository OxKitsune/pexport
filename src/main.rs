use std::io::prelude::*;
use std::{env, fs::OpenOptions, process::exit, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid command-line arguments!");
        exit(1);
    }

    let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("$HOME environment variable doesn't exist!");
            exit(1);
        }
    };
    
    let path = match std::fs::canonicalize(&args[1]) {
        Ok(buff) => format!("{:?}", buff),
        Err(err) => {
            eprintln!("Failed to get absoulte path: {}", err);
            exit(1);
        }
    };

    println!("Exporting {:?}...", path);
    let file_open = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/.zshrc", home_dir));

    match file_open {
        Ok(mut file) => {
            let command = &*format!("export PATH=\"{}:$PATH\"", path);
            let result = writeln!(file, "\n{}", command);

            match result {
                Ok(()) => {
                    println!("Added {} to path!", path);

                    let ses_command = Command::new("sh").arg("-c").arg(command).output();

                    match ses_command {
                        Ok(_) => {
                            println!("Added {} to current session!", path);
                            exit(0);
                        }
                        Err(err) => {
                            eprintln!("Failed to add {} to curent session: {}", path, err);
                            exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to write to .zshrc: {}", err);
                    exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to open .zshrc: {}", err);
            exit(1);
        }
    }
}