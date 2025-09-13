use std::{fs, io::*};
mod commands;
use colored::*;

fn main() {
    
    match fs::read_to_string("./src/art.txt") {
        Ok(content) => println!("{}\n", content.bright_blue().bold()),
        Err(e) => eprintln!("{}", e),
    }

    loop {
        let current_path = commands::pwd::pwd();

        print!("{} $ ", current_path.cyan().bold());
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let input = input.trim();

        // || !input.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '~' || c == '.')  khsni n7iyd special characters
        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();

        match args[0] {
            "echo" => commands::echo::echo(&args[1..].join(" ")),
            "cd" => commands::cd::cd(args[1..].to_vec()),
            "pwd" => {
                if args.len() > 1 {
                    println!("pwd: too many arguments");
                    continue;
                }

                let pwd = commands::pwd::pwd();
                println!("{}", pwd);
            }
            "clear" => commands::clear::clear(args[1..].to_vec()),
            "ls" => commands::ls::ls(args[1..].to_vec()),
            "cat" => commands::cat::cat(args[1..].to_vec()),
            "rm" => commands::rm::rm(args[1..].to_vec()),
            _ => println!("Command {} not found", args[0].red().bold()),
        }
    }
}
