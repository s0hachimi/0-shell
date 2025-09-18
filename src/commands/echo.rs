pub fn echo(args: Vec<&str>,quote_open: bool) {

      let output = args.iter().map(|arg|{
          if arg.contains("\\n") || arg.contains("\\t") || arg.contains("\\r") ||
          arg.contains("\\a") || arg.contains("\\b") || arg.contains("\\v") ||
          arg.contains("\\f") || arg.contains("\\\\")  {
              arg.replace("\\n", "\n")
              .replace("\\t", "\t")
              .replace("\\r", "\r")
              .replace("\\a", "\x07")
              .replace("\\b", "\x08")
              .replace("\\v", "\x0B")
              .replace("\\f", "\x0C")
              .replace("\\\\", "\\")
            } else {
                arg.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    if quote_open {
        println!("{}",output);
        // println!()
        
    }else {
        println!("{}",output);
    }

}