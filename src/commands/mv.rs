use std::fs;

pub fn mv(args: Vec<&str>) {
    if args.len() != 2 {
        println!("mv: usage: mv source destination");
        return;
    }

    let source = args[0];
    let dest = args[1];

    if let Err(e) = fs::rename(source, dest) {
        eprintln!("mv: cannot move '{}' to '{}': {}", source, dest, e);
    }
}