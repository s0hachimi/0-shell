use std::io::*;
mod commands;



fn main() {
    loop {
        

        print!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }


        let args: Vec<&str> = input.split_whitespace().collect();

        match args[0] {
            "echo" => commands::echo::echo(args[1..].to_vec()),
            _  => print!("bash: command not found: {}", args[0])
        }



     
    }
}
