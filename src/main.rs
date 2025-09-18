use std::{fs, io::*};
mod commands;
use colored::*;

fn read_complete_input() -> (String, bool, bool) {
    let mut input = String::new();
    let mut check_quote = false;
    let mut inp = String::new();
    let mut ctr_d = false;

    loop {
        let mut line = String::new();
        stdout().flush().unwrap();

        match stdin().read_line(&mut line) {
            Ok(0) => {
                ctr_d = true;
                println!();
                break;
            }
            Ok(_) => {}
            Err(_) => {
                eprintln!("Failed to read line");
                continue;
            }
        }
        input.push_str(&line);

        // Count number of quotes
        // let quote_count = input.chars().filter(|&c| c == '"' || c == '\'').count();
        let chars: Vec<char> = input.chars().collect();
        let mut quote_count = 0;

        for (i, &c) in chars.iter().enumerate() {
            if c == '"' {
                // check_quote = true;
                if i == 0 || chars[i - 1] != '\\' {
                    quote_count += 1;
                    continue;
                }
            } else if c == '\'' {
                check_quote = true;
                quote_count += 1;
                continue;
            } else {
                inp.push(c)
            }
        }

        if quote_count % 2 == 0 {
            break;
        } else {
            // Show different prompt while waiting for closing quote
            print!("quote> ");
            inp.clear()
        }
    }
    (inp, check_quote, ctr_d)
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

        let (input, quote_open, ctr_d) = read_complete_input();

        if ctr_d {
            break;
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

        if args.is_empty() {
            continue;
        }

        match args[0] {
            "echo" => commands::echo::echo(args[1..].to_vec(), quote_open),
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
            "mkdir" => commands::mkdir::mkdir(args[1..].to_vec()),
            "cp" => commands::cp::cp(args[1..].to_vec()),
            "mv" => commands::mv::mv(args[1..].to_vec()),
            _ => println!("Command {} not found", args[0].red().bold()),
        }
    }
}
