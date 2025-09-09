use chrono::{Local, NaiveDateTime, TimeZone};
use colored::Colorize;
use std::{fs, os::linux::fs::MetadataExt};

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
            let mode = meta_data.st_mode();

            if f {
                let (dir, file_x) = flag_f(mode);

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
                let (_, file_x) = flag_f(mode);
                name = if file_x {
                    name.green().bold().to_string()
                } else {
                    name
                };
            } else if meta_data.is_symlink() {
                name = name.cyan().bold().to_string();
            }

            if l {
                let perms_str = perms(mode);
                let nlink = meta_data.st_nlink();
                let uid = meta_data.st_uid();
                let gid = meta_data.st_gid();
                let size = meta_data.st_size();

                let mtime = meta_data.st_mtime();
                #[allow(deprecated)]
                let dt = NaiveDateTime::from_timestamp(mtime, 0);
                let local_dt = Local.from_utc_datetime(&dt);
                let datetime = local_dt.format("%b %e %H:%M").to_string();

                flag_l.push(
                    [
                        perms_str,
                        nlink.to_string(),
                        uid.to_string(),
                        gid.to_string(),
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
        sort(&mut flag_l[0]);
        format_flag_l(flag_l);
    } else {
        sort(&mut no_flag_or_a_f);
        let sorted = no_flag_or_a_f.join(" ");
        println!("{}", sorted);
    }
}

fn perms(m: u32) -> String {
    let mut perms = Vec::new();

    // directory or file
    perms.push(if (m & 0o040000) != 0 {
        "d" // directory
    } else if (m & 0o100000) != 0 {
        "-" // regular file
    } else if (m & 0o120000) != 0 {
        "l" // symlink
    } else if (m & 0o020000) != 0 {
        "c" // character device
    } else if (m & 0o060000) != 0 {
        "b" // block device
    } else if (m & 0o010000) != 0 {
        "p" // FIFO / pipe
    } else if (m & 0o140000) != 0 {
        "s" // socket
    } else {
        "?" // unknown
    });

    // user
    perms.push(if (m & 0o400) != 0 { "r" } else { "-" });
    perms.push(if (m & 0o200) != 0 { "w" } else { "-" });
    perms.push(if (m & 0o100) != 0 { "x" } else { "-" });

    // group
    perms.push(if (m & 0o040) != 0 { "r" } else { "-" });
    perms.push(if (m & 0o020) != 0 { "w" } else { "-" });
    perms.push(if (m & 0o010) != 0 { "x" } else { "-" });

    // others
    perms.push(if (m & 0o004) != 0 { "r" } else { "-" });
    perms.push(if (m & 0o002) != 0 { "w" } else { "-" });
    perms.push(if (m & 0o001) != 0 { "x" } else { "-" });

    perms.join("")
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

fn flag_f(m: u32) -> (bool, bool) {
    let dir = if (m & 0o040000) != 0 { true } else { false };
    let file_x = if m & 0o100 != 0 || m & 0o010 != 0 || m & 0o001 != 0 {
        true
    } else {
        false
    };

    (dir, file_x)
}


fn sort(files: &mut Vec<String>) {
    files.sort_by(|a, b| {
        let name_a = a.split_whitespace().last().unwrap_or("");
        let name_b = b.split_whitespace().last().unwrap_or("");
        name_a.to_lowercase().cmp(&name_b.to_lowercase())
    });
}