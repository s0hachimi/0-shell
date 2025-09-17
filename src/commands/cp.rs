use std::fs;
use std::path::Path;

pub fn cp(args: Vec<&str>) {
    if args.len() != 2 {
        println!("cp: usage: cp source destination");
        return;
    }

    let source = args[0];
    let dest = args[1];

    if let Err(e) = fs::copy(source, dest) {
        eprintln!("cp: cannot copy '{}' to '{}': {}", source, dest, e);
    }
}