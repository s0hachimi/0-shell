use std::fs;

pub fn mkdir(args: Vec<&str>) {
    if args.is_empty() {
        println!("mkdir: missing operand");
        return;
    }

    for dir in args {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
            }
        }
    }
}