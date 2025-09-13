
pub fn echo(args: &str) {
    let replaced = args.replace('"',"");
    if replaced.contains("\\n"){
        for spl in replaced.split("\\n"){
            println!("{}",spl)
        }
    }else {
        println!("{}",replaced);
    }
}
