use std::fs;
pub fn rm(args: Vec<&str>){
    let mut recursive = false;
    let mut targets = Vec::new();
    for arg in args{
        if arg == "-r" || arg == "-R"{
            recursive = true;
        }else{
        targets.push(arg);
        }
    }
      if targets.is_empty() {
        println!("rm: missing operand");
        return;
    }
    for target in targets {
        let metadata = fs::metadata(target);
        match metadata{
            Ok(meta) =>{
                if meta.is_dir(){
                    if recursive {
                       match  fs::remove_dir_all(target){
                        Ok(_)=> println!("Removed directory: {}", target),
                        Err(e)=> println!("Failed to remove directory {}: {}", target, e),
                       }
                    }else{
                        println!("rm: cannot remove {}: Is a directory",target)
                    }
                }else {
                    match fs::remove_file(target) {
                        Ok(_) => println!("Removed file: {}", target),
                        Err(e) => println!("Failed to remove file {}: {}", target, e),
                    }
                }
            }
            Err(e) => println!("rm: cannot access '{}': {}", target, e),
        }
    }
}
    

