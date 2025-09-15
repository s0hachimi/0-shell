use std::fs;
use std::{ io::*};
pub fn cat(files: Vec<&str>) {
    if files.is_empty() || files[0] == "-"{
     loop {
        let mut input = String::new();
        stdout().flush().unwrap();
         if stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }
        print!("{}",input);
      }
    }
    for filename in files {
        match fs::read_to_string(filename) {
            Ok(content) => print!("{}", content), 
            Err(e) => eprintln!("cat: {}: {}", filename, e),
        }
    }
}