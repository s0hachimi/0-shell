use chrono::{Duration, Local, NaiveDateTime, TimeZone};
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
        let mut path = Vec::new();
        for p in args.iter() {
            if !p.starts_with("-") {
                path.push(*p);
            }
        }
        path.join(" ")
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

    let dirs: Vec<&str> = path.split_whitespace().collect();
    let mut nl = 0;

    // print lmli7
    for pa in dirs.iter() {
        if dirs.len() > 1 {
            println!("{pa}:");
            ls_output(l, a, f, pa.to_string());

            if dirs.len() - 1 != nl {
                println!()
            }
        } else {
            ls_output(l, a, f, pa.to_string());
        }

        nl += 1;
    }
}

fn ls_output(l: bool, a: bool, f: bool, path: String) {
    let dir_path = std::path::Path::new(&path);

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
    let mut total = 0;

    // Add . and ..
    if a {
        // Add current directory (.)
        if let Ok(meta_data) = fs::metadata(dir_path) {
            total += meta_data.st_blocks() / 2;

            let mut name = ".".to_string();
            if f {
                name.push('/');
            }

            if l {
                add_to_flag_l(&name, meta_data, &mut flag_l);
            } else {
                no_flag_or_a_f.push(name.blue().bold().to_string());
            }
        }

        // Add parent directory (..)
        if let Some(mut parent) = dir_path.parent() {
            let mut st = pwd();

            if parent.to_string_lossy().to_string().is_empty() {
                st.push_str(&parent.to_string_lossy().to_string());
                parent = std::path::Path::new(&st)
            }

            if let Ok(meta_data) = fs::metadata(parent) {
                total += meta_data.st_blocks() / 2;

                let mut name = "..".to_string();
                if f {
                    name.push('/');
                }

                if l {
                    add_to_flag_l(&name, meta_data, &mut flag_l);
                } else {
                    no_flag_or_a_f.push(name.blue().bold().to_string());
                }
            }
        } else {
            // if directory is root '/'
            if let Ok(meta_data) = fs::metadata(dir_path) {
                total += meta_data.st_blocks() / 2;

                let mut name = "..".to_string();
                if f {
                    name.push('/');
                }

                if l {
                    add_to_flag_l(&name, meta_data, &mut flag_l);
                } else {
                    no_flag_or_a_f.push(name.blue().bold().to_string());
                }
            }
        }
    }

    for entry in dir {
        if let Ok(en) = entry {
            let mut name = match en.file_name().into_string() {
                Ok(n) => n,
                Err(_) => "".to_string(),
            };

            let meta_data = en.metadata().unwrap();
            let debug_output = format!("{:?}", meta_data.permissions());

            total += meta_data.st_blocks() / 2;

            if !a && name.starts_with(".") {
                continue;
            }

            let st = flag_f(perms(debug_output.clone()));

            // color o kda
            name = match st {
                "/" => name.blue().bold().to_string(),
                "@" => name.cyan().bold().to_string(),
                "|" => name.red().bold().to_string(),
                "=" => name.purple().bold().to_string(),
                "*" => name.green().bold().to_string(),
                _ => name,
            };

            

            if f {
                name.push_str(&st);
            }

            // add l atawich
            if l {
                add_to_flag_l(&name, meta_data, &mut flag_l);
            } else {
                no_flag_or_a_f.push(name);
            }
        }
    }

    if l {
        println!("total {}", total);
        format_flag_l(flag_l);
    } else {
        let sorted = no_flag_or_a_f.join(" ");
        println!("{}", sorted);
    }
}

fn add_to_flag_l(name: &str, meta_data: fs::Metadata, flag_l: &mut Vec<Vec<String>>) {
    let debug_output = format!("{:?}", meta_data.permissions());
    let perms_str = perms(debug_output);
    let nlink = meta_data.st_nlink();
    let uid = meta_data.st_uid();
    let gid = meta_data.st_gid();
    let size = meta_data.st_size();

    let mtime = meta_data.st_mtime();
    #[allow(deprecated)]
    let dt = NaiveDateTime::from_timestamp(mtime, 0);
    let dt = dt + Duration::hours(1);
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
            name.to_string(),
        ]
        .to_vec(),
    );
}

fn perms(perm: String) -> String {
    let arr: Vec<&str> = perm.split_whitespace().collect();
    let st = arr[arr.len() - 2];

    // if st.chars().any(|f| f.is_numeric()) {
    //     println!("{}", m);
    // }

    st[1..st.len() - 1].to_string()
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

        let name = rows[rows.len() - 1].clone();

        result.push(name);
        println!("{}", result.join(" "));
        result.clear();
    }
}

fn flag_f(perms: String) -> &'static str {
    let mut st = match perms.chars().next() {
        Some('d') => "/",
        Some('l') => "@",
        Some('s') => "=",
        Some('p') => "|",
        _ => "",
    };

    if st.is_empty() && perms.contains("x") {
        st = "*"
    }

    st
}

// fn sort(forms: &mut Vec<String>)  {
//     // println!("{:?}", forms)
// }
