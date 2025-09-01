use std::env;

pub fn pwd() -> String {
     let path = env::current_dir().unwrap();
     return path.display().to_string();
}