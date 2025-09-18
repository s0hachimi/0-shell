pub fn echo(args: Vec<&str>) {
    let mut no_newline = false;
    let mut i = 0;
    if !args.is_empty() && args[0] == "-n" {
        no_newline = true;
        i = 1;
    }
    let output = args[i..].join(" ");
    let interpreted = output
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\a", "\x07")
        .replace("\\b", "\x08")
        .replace("\\v", "\x0B")
        .replace("\\f", "\x0C")
        .replace("\\\\", "\\");
    if no_newline {
        print!("{}", interpreted);
    } else {
        println!("{}", interpreted);
    }
}
