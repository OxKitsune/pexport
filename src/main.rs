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

    println!("Exporting {}", args[1]);
    let file_open = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/.zshrc", home_dir));

    match file_open {
        Ok(mut file) => {
            let command = &*format!("export PATH=\"{}:$PATH\"", args[1]);
            let result = writeln!(file, "\n{}", command);

            match result {
                Ok(()) => {
                    println!("Added {} to path!", args[1]);

                    let ses_command = Command::new("sh").arg("-c").arg(command).output();

                    match ses_command {
                        Ok(_) => {
                            println!("Added {} to current session!", args[1]);
                            exit(0);
                        }
                        Err(err) => {
                            eprintln!("Failed to add {} to curent session: {}", args[1], err);
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