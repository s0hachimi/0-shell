use std::env;
use std::path::Path;

use crate::commands::pwd::pwd;

pub fn cd(args: Vec<&str>) {
    if args.len() > 1 {
        println!("bash: cd: too many arguments");
        return;
    }

    let old = pwd();

    let mut target = if args.is_empty() || args[0] == "~" {
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                println!("bash: cd: HOME not set");
                return;
            }
        }
    } else {
        args[0].to_string()
    };

    target = if target == "-" {

        match env::var("OLDPWD") {
            Ok(old_pwd) => old_pwd,
            Err(_) => {
                println!("bash: cd: OLDPWD not set");
                return;
            }
        }
    } else {

        target
    };

   if args[0] == "-" { println!("{}", target) }

    let path = Path::new(&target);

    if let Err(e) = env::set_current_dir(&path) {
        let err_msg = &e.to_string()[..e.to_string().len()-13];
        println!("bash: cd: {}: {}", target, err_msg);
        return;
    }

     env::set_var("OLDPWD", old);
}