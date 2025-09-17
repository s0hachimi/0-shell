use std::{fs, io::*};
mod commands;
use colored::*;
fn read_complete_input() -> (String,bool) {
    let mut input = String::new();
    let mut check_quote = false;

    loop {
        let mut line = String::new();
        stdout().flush().unwrap();

     match stdin().read_line(&mut line) {
            Ok(0) => {
                // Ctrl+D pressed (EOF)
                println!(); // Add newline for clean exit
                break;
            }
            Ok(_) => {
                // Successfully read input
            }
            Err(_) => {
                eprintln!("Failed to read line");
                continue;
            }
        }
        input.push_str(&line);

        // Count number of quotes
        let quote_count = input.chars().filter(|&c| c == '"' || c == '\'').count();

        if quote_count % 2 == 0 {
            break;
        } else {
            // Show different prompt while waiting for closing quote
            print!("quote> ");
            stdout().flush().unwrap();
            check_quote = true;

        }
    }

    (input,check_quote)
}

fn parse_arguments(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut quote_char = '"';
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            ch if ch == quote_char && in_quotes => {
                in_quotes = false;
            }
            ' ' | '\t' if !in_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                current_arg.push(ch);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

fn main() {
    
    match fs::read_to_string("./src/art.txt") {
        Ok(content) => println!("{}\n", content.bright_blue().bold()),
        Err(e) => eprintln!("{}", e),
    }

    loop {
        let current_path = commands::pwd::pwd();

        print!("{} $ ", current_path.cyan().bold());
        stdout().flush().unwrap();

        let  (input,quote_open) = read_complete_input();
      

        let input = input.trim();

        // || !input.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '~' || c == '.')  khsni n7iyd special characters
        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        let args = parse_arguments(&input);

        if args.is_empty() {
            continue;
        }

        match args[0].as_str() {
            "echo" => commands::echo::echo(args[1..].iter().map(|s| s.as_str()).collect(),quote_open),
            "cd" => commands::cd::cd(args[1..].iter().map(|s| s.as_str()).collect()),
            "pwd" => {
                if args.len() > 1 {
                    println!("pwd: too many arguments");
                    continue;
                }

                let pwd = commands::pwd::pwd();
                println!("{}", pwd);
            }
            "clear" => commands::clear::clear(args[1..].iter().map(|s| s.as_str()).collect()),
            "ls" => commands::ls::ls(args[1..].iter().map(|s| s.as_str()).collect()),
            "cat" => commands::cat::cat(args[1..].iter().map(|s| s.as_str()).collect()),
            "rm" => commands::rm::rm(args[1..].iter().map(|s| s.as_str()).collect()),
            "mkdir" => commands::mkdir::mkdir(args[1..].iter().map(|s| s.as_str()).collect()),
            "cp" => commands::cp::cp(args[1..].iter().map(|s| s.as_str()).collect()),
            "mv" => commands::mv::mv(args[1..].iter().map(|s| s.as_str()).collect()),
            _ => println!("Command {} not found", args[0].red().bold()),
        }
    }
}
