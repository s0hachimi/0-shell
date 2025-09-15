pub fn echo(args: &str,quote_open: bool) {
    let replaced = if args.starts_with('"') {
        args.replace('"', "")
    } else {
        args.replace('\'', "")
    };
    let splited = replaced.split("\\n").collect::<Vec<_>>().to_vec();
    let checking = replaced.split(" ").collect::<Vec<_>>().to_vec();
        for spl in splited{
            if spl == " "{
                continue;
            }
            if !quote_open{
                println!("{}",spl) ;
            }else if quote_open && checking[1].is_empty(){ 
                println!("{}",spl);
                println!()
            }else{
                for st in spl.split(" "){
                    println!("{}",st)
                }
            }
        } 
}