use std::fs;
use std::path::{Path, PathBuf};


pub fn cp(args: Vec<&str>) {
    if args.len() != 2 {
        println!("cp: usage: cp source destination");
        return;
    }

    let source = args[0];
    let dest = args[1];
    let dest_path = Path::new(dest);
    let source_filename = Path::new(source)
        .file_name()
        .unwrap_or_default();

    let is_dir = dest_path.is_dir() || dest.ends_with('/') || dest.ends_with("\\");
    let final_dest: PathBuf = if is_dir {
        dest_path.join(source_filename)
    } else {
        dest_path.to_path_buf()
    };

    if let Err(e) = fs::copy(source, &final_dest) {
        eprintln!("cp: cannot copy '{}' to '{}': {}", source, final_dest.display(), e);
    }
}