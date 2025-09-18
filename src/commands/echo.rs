pub fn echo(args: Vec<&str>,quote_open: bool) {
    // let mut check_quote = false;
    // let replaced = if args.iter().any(|f| f.starts_with('"')) {
    //   (  args.iter().map(|f|f.replace('"', "")).collect::<Vec<_>>(),
    //      true)

    // } else if args.iter().any(|f| f.starts_with("'")) {
    // (    args.iter().map(|k|k.replace('\'', "")).collect::<Vec<_>>(),
    //     true)
    // }else{
    //     (args.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
    //   false)

    // };
    let mut no_newline = false;
    let mut i =0;
    // let mut interpret_escapes = false;
       while i < args.len(){
        match args[i]{
            "-n"=> no_newline = true ,
            // "-e" => interpret_escapes = true,
            //  "-E" => interpret_escapes = false,
            _=>break
        }
        i+=1;
       }
    // if quote_open {
    
    //    let output = args[i..].iter().map(|arg|{
    //     if arg.contains("\\n") || arg.contains("\\t") || arg.contains("\\r") ||
    //      arg.contains("\\a") || arg.contains("\\b") || arg.contains("\\v") ||
    //      arg.contains("\\f") || arg.contains("\\\\")  {
    //             arg.replace("\\n", "\n")
    //                .replace("\\t", "\t")
    //                .replace("\\r", "\r")
    //                .replace("\\a", "\x07")
    //                .replace("\\b", "\x08")
    //                .replace("\\v", "\x0B")
    //                .replace("\\f", "\x0C")
    //                .replace("\\\\", "\\")
    //         } else {
    //             arg.to_string()
    //         }
    //    })
    //     .collect::<Vec<String>>()
    //     .join(" ");
    // //      let mut output = args[i..].join(" ");
    // //       if interpret_escapes {
    // //         output = output
    // //         .replace("\\n", "\n")
    // //         .replace("\\t", "\t")
    // //         .replace("\\\\", "\\")
    // //         .replace("\\r", "\r")
    // //         .replace("\\a", "\x07")
    // //         .replace("\\b", "\x08")
    // //         .replace("\\v", "\x0B")
    // //         .replace("\\f", "\x0C");
    // // }
    //      if !no_newline {  
    //          println!("--{}",output);
    //      }else{
    //         print!("{}",output);
    //      }
    // }else{
        // let output = replaced.0.iter().map(|arg|{
        //   if arg.contains("\\\\"){
        //     arg.replace("\\\\","\\")
        //    }else if arg.contains("\\"){
        //     arg.replace("\\","")
        //    }else{
        //     arg.to_string()
        //    }
        // })
        // .collect::<Vec<String>>()
        // .join(" ");
        // if !no_newline {  
        //      println!("{}",output);
        //  }else{
        //     print!("{}",output);
        //  }
        for arg in &args[i..] {
            // let mut count_bs = 0;
            // for st in arg.chars(){
            //     if st == '\\'{
            //         count_bs +=1
            //     }
            // }
       
            // let jj = arg.replace("\\".repeat(count_bs).as_str(),"\\".repeat(count_bs/2).as_str());
            // println!("{}",jj)
            let result = echo_like_backslash_parse(arg,quote_open);
            if !no_newline {  
                println!("{}",result);
            }else{
             print!("{}",result);
            }
        }

}
fn echo_like_backslash_parse(input: &str,quote_open: bool) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let mut backslash_count :f64= 1.0;

            // نعدو backslashes متتاليين
            while let Some(&'\\') = chars.peek() {
                chars.next();
                backslash_count += 1.0;
            }
        // println!("--{}",backslash_count % 2.0 == 0.0);
        // println!("++{}",backslash_count % 2.0 == 1.0);
            // نضيف backslashes حسب عدد الأزواج
           if quote_open{
                 let pair  = (backslash_count / 2.0).round() as usize;
                for _ in 0..pair {
                // println!("f");
                result.push('\\');
            }
           } else if backslash_count % 2.0 == 0.0 {
           
               
                let pair  = (backslash_count / 3.0).round() as usize;
                for _ in 0..pair {
                // println!("f");    //      if !no_newline {  
    //          println!("--{}",outp    //      if !no_newline {  
    //          println!("--{}",output);
    //      }else{
    //         print!("{}",output);
    //      }ut);
    //      }else{
    //         print!("{}",output);
    //      }
                result.push('\\');
            }
            }else{
                let pair  = (backslash_count / 3.0).floor() as usize;

                 println!("{pair}");
                  for _ in 0..pair {
                    
                result.push('\\');
            }
            }
            

            // إذا كان عدد backslashes فردي، يعني واحد زائد خاصو يهرب الحرف اللي بعدو
            if backslash_count % 2.0 == 1.0 && backslash_count != 1.0{
                if let Some(next_char) = chars.next() {
                    // إذا next_char عندها معنى escape خاص، نعالجها (مثلا \n، \t...)  
                    // لكن هنا بما أن j أو غيرها ماشي حرف escape معروف، نطبعها مباشرة
                    match next_char {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        _ => result.push(next_char),
                    }
                } 
                // else {
                //     // backslash في آخر السلسلة بدون حرف بعده، نطبع backslash
                //     result.push('\\');
                // }
            }
        } else {
            // حرف عادي، نضيفوه
            result.push(c);
        }
    }
  
    result
}
// fn replace_first_backslash(s: &str) -> String {
//     if let Some(pos) = s.find('\\') {
//         let mut res = String::new();
//         res.push_str(&s[..pos]);       // قبل أول backslash
//         res.push_str("");              // تعويض: هنا بغيتي تمسح، إذا بغيتي تبدلها بشي حاجة ديرها هنا
//         res.push_str(&s[pos+1..]);    // بعد أول backslash
//         res
//     } else {
//         s.to_string() // إلى ما كانش backslash ترجّع النص كما هو
//     }
// }