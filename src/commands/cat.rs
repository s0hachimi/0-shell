use std::fs;
use std::{ io::*};
pub fn cat(files: Vec<&str>) {
    if files.is_empty() || files[0] == "-"{
      let mut input = String::new();
      loop {
        let mut line = String::new();

        stdout().flush().unwrap();
          match stdin().read_line(&mut line) {
            Ok(0) => break, // Ctrl+D
            Ok(_) => {}
            Err(_) => { eprintln!("Failed to read line"); continue; }
        }

        let filtered_line: String = line.chars().filter(|c| {
            // Only allow printable ASCII (space to ~), newline, and tab
            (*c >= ' ' && *c <= '~') || *c == '\n' || *c == '\t'
        }).collect();

        input.push_str(&filtered_line);
        print!("{}",input);
        input.clear();
      }
    } else {

      for filename in files {
        match fs::read_to_string(filename) {
            Ok(content) => print!("{}", content), 
            Err(e) => eprintln!("cat: {}: {}", filename, e),
        }
      }
    }
    
}