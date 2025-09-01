use std::fs;
// use std::cmp::Ordering;

use crate::commands::pwd::pwd;

pub fn ls(mut args: Vec<&str>) {
    // let a = false;
    // let l = false;
    // let F = false;

    args.sort_by_key(|f| !f.starts_with("-"));
    // if args.iter().any(|c| c.contains("-")) {
    // }

    // println!("{:?}", args);

    let path = if !args.is_empty() {
        args[0].to_string()
    } else {
        pwd()
    };

    let dir = match fs::read_dir(path.clone()) {
        Ok(dir) => dir,
        Err(n) => {
            if n.to_string().contains("Permission denied") {
                 println!("ls: cannot open directory '{}': {}", path.clone(), &n.to_string()[..n.to_string().len()-13]);
            } else {
                println!("ls: cannot access '{}': {}", path.clone(), &n.to_string()[..n.to_string().len()-13]);
            }
            return;
        }
   };

    let mut result = Vec::new();

    for entry in dir {
        if let Ok(en) = entry {
            if let Ok(name) = en.file_name().into_string() {
                if !name.starts_with(".") {
                    result.push(name);
                }
            }
        }
    }

    result.sort_by_key(|n| n.chars().next());
    let sorted = result.join(" ");
    println!("{}", sorted);
}
