
pub fn echo(args: Vec<&str>) {
    for arg in args {

        println!("{}",arg);
        if arg.starts_with('"') && arg.ends_with('"') && arg.contains('\n') {
            let trimmed = &arg[1..arg.len() - 1];

            let lines: Vec<&str> = trimmed.split('\n').collect();

            for (i, line) in lines.iter().enumerate() {
                if i != lines.len() - 1 {
                    println!("{}", line);
                } else {
                    print!("{}", line);
                }
            }
        }
    }
}
