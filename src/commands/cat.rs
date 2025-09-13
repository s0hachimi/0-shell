use std::fs;
pub fn cat(files: Vec<&str>) {
    for filename in files {
        match fs::read_to_string(filename) {
            Ok(content) => print!("{}", content), 
            Err(e) => eprintln!("cat: {}: {}", filename, e),
        }
    }
}