use chrono::{Local, NaiveDateTime, TimeZone};
use colored::Colorize;
use std::{fs, os::linux::fs::MetadataExt};
use users::{get_group_by_gid, get_user_by_uid};

use crate::commands::pwd::pwd;

pub fn ls(mut args: Vec<&str>) {
    let mut a = false;
    let mut l = false;
    let mut f = false;

    args.sort_by_key(|f| f.starts_with("-"));

    let path = if !args.is_empty() && !args[0].starts_with("-") {
        args[0].to_string()
    } else {
        pwd()
    };

    for flag in args {
        if flag.starts_with("-") {
            l = flag.contains("l");
            a = flag.contains("a");
            f = flag.contains("F");
        }
    }

    let dir = match fs::read_dir(path.clone()) {
        Ok(dir) => dir,
        Err(n) => {
            if n.to_string().contains("Permission denied") {
                println!(
                    "ls: cannot open directory '{}': {}",
                    path.clone(),
                    &n.to_string()[..n.to_string().len() - 13]
                );
            } else {
                println!(
                    "ls: cannot access '{}': {}",
                    path.clone(),
                    &n.to_string()[..n.to_string().len() - 13]
                );
            }
            return;
        }
    };

    let mut no_flag_or_a_f = Vec::new();
    let mut flag_l = Vec::new();

    for entry in dir {
        if let Ok(en) = entry {
            let mut name = match en.file_name().into_string() {
                Ok(n) => n,
                Err(_) => "".to_string(),
            };

            let meta_data = en.metadata().unwrap();
            let debug_output = format!("{:?}", meta_data.permissions());

            if f {
                let (dir, file_x) = flag_f(perms(debug_output.clone()));

                if dir {
                    name.push('/');
                } else if file_x {
                    name.push('*');
                }
            }

            if !a {
                if name.starts_with(".") {
                    continue;
                }
            }

            if meta_data.is_dir() {
                name = name.blue().bold().to_string();
            } else if meta_data.is_file() {
                let (_, file_x) = flag_f(perms(debug_output.clone()));
                name = if file_x {
                    name.green().bold().to_string()
                } else {
                    name
                };
            } else if meta_data.is_symlink() {
                name = name.cyan().bold().to_string();
            }

            if l {
                let perms_str = perms(debug_output.clone());
                let nlink = meta_data.st_nlink();
                let uid = meta_data.st_uid();
                let gid = meta_data.st_gid();
                let size = meta_data.st_size();

                let mtime = meta_data.st_mtime();
                #[allow(deprecated)]
                let dt = NaiveDateTime::from_timestamp(mtime, 0);
                let local_dt = Local.from_utc_datetime(&dt);
                let datetime = local_dt.format("%b %e %H:%M").to_string();

                let username = get_user_by_uid(uid)
                    .map(|u| u.name().to_string_lossy().to_string())
                    .unwrap_or_else(|| format!("{}", uid));

                let groupname = get_group_by_gid(gid)
                    .map(|u| u.name().to_string_lossy().to_string())
                    .unwrap_or_else(|| format!("{}", gid));

                flag_l.push(
                    [
                        perms_str,
                        nlink.to_string(),
                        username,
                        groupname,
                        size.to_string(),
                        datetime,
                        name.clone(),
                    ]
                    .to_vec(),
                );
            } else {
                no_flag_or_a_f.push(name);
            }
        }
    }

    if l {
        format_flag_l(flag_l);
    } else {
        let sorted = no_flag_or_a_f.join(" ");
        println!("{}", sorted);
    }
}

fn perms(perm: String) -> String {
    let arr: Vec<&str> = perm.split_whitespace().collect();
    let st = arr[arr.len()-2];

    st[1..st.len()-1].to_string()
}

fn format_flag_l(flag: Vec<Vec<String>>) {
    let mut sizes = vec![0; flag[0].len() - 1];

    for i in 0..flag[0].len() - 1 {
        let mut len = 0;

        for row in flag.iter() {
            if len < row[i].len() {
                len = row[i].len();
            }
        }
        sizes[i] = len;
    }

    let mut result = Vec::new();

    for rows in flag.into_iter() {
        for (i, v) in rows[..rows.len() - 1].iter().enumerate() {
            let mut st = String::new();
            let n = sizes[i] - v.len();

            st.push_str(&" ".repeat(n));
            st.push_str(v);
            result.push(st);
        }

        result.push(rows[rows.len() - 1].clone());
        println!("{}", result.join(" "));
        result.clear();
    }
}

fn flag_f(perms: String) -> (bool, bool) {
    let dir = perms.contains("d");
    let file_x = perms.contains("x");  

    (dir, file_x)
}
