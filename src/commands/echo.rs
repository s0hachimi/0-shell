pub fn echo(args: Vec<&str>,_quote_open: bool) {
    let replaced = if args.iter().any(|f| f.starts_with('"')) {
        args.iter().map(|f|f.replace('"', "")).collect::<Vec<_>>()
    } else if args.iter().any(|f| f.starts_with("'")) {
        args.iter().map(|k|k.replace('\'', "")).collect::<Vec<_>>()
    }else{
      args.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    };
    if _quote_open{
        for arg in replaced{
           for st in arg.split("\\n"){
            if st.contains(" "){
                println!()
            }
            println!("{}",st)
           }
        }
        
    }else{
    let mut no_newline = false;
    let mut i =0;
    let mut interpret_escapes = false;
       while i < args.len(){
        match args[i]{
            "-n"=> no_newline = true ,
            "-e" => interpret_escapes = true,
             "-E" => interpret_escapes = false,
            _=>break
        }
        i+=1;
        break
       }
         let mut output = args[i..].join(" ");
          if interpret_escapes {
            output = output
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\\", "\\")
            .replace("\\r", "\r")
            .replace("\\a", "\x07")
            .replace("\\b", "\x08")
            .replace("\\v", "\x0B")
            .replace("\\f", "\x0C");
    }
         if !no_newline {  
             println!("{}",output);
         }else{
            print!("{}",output);
         }
    }
    // let splited = replaced.split("\\n").collect::<Vec<_>>().to_vec();
    // let checking = replaced.split(" ").collect::<Vec<_>>().to_vec();
    //     for spl in splited{
    //         if spl == " "{
    //             continue;
    //         }
    //         if !quote_open{
    //             println!("{}",spl) ;
    //         }else if quote_open && checking[1].is_empty(){ 
    //             println!("{}",spl);
    //             println!()
    //         }else{
    //             for st in spl.split(" "){
    //                 println!("{}",st)
    //             }
    //         }
    //     } 
   

        }