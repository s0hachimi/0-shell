use chrono::{Duration, Local, NaiveDateTime, TimeZone};
use colored::Colorize;
use libc::{major, minor};
use std::{
    fs,
    os::{linux::fs::MetadataExt, unix::fs::FileTypeExt},
    path::Path,
};

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
            if !l {
                l = flag.contains("l");
            }
            if !a {
                a = flag.contains("a");
            }
            if !f {
                f = flag.contains("F");
            }
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
    let mut no_flag_or_a_f: Vec<(String, String)> = Vec::new();
    let mut flag_l = Vec::new();
    let mut total = 0;

    let dir_path = std::path::Path::new(&path);

    let metadata = match fs::metadata(&path) {
        Ok(meta) => meta,
        Err(e) => {
            println!("ls: cannot access '{}': {}", path, e);
            return;
        }
    };

    // ls files
    if metadata.is_file()
        || metadata.file_type().is_char_device()
        || metadata.file_type().is_block_device()
    {
        if l {
            let perm_str = perms(metadata.clone(), dir_path);

            let st = flag_f(perm_str.clone());

            add_to_flag_l(&path, metadata, &mut flag_l, st.to_string(), perm_str);
            format_flag_l(flag_l);
        } else {
            println!("{}", path);
        }

        return;
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
            } else if n.to_string().contains("Not a directory") {
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

    // Add . and ..
    if a {
        // Add current directory (.)
        if let Ok(meta_data) = fs::metadata(dir_path) {
            total += meta_data.st_blocks() / 2;

            let mut name = ".".blue().bold().to_string();
            if f {
                name.push('/');
            }

            let perms_str = perms(meta_data.clone(), Path::new(&name));

            if l {
                add_to_flag_l(&name, meta_data, &mut flag_l, "/".to_string(), perms_str);
            } else {
                no_flag_or_a_f.push((name.blue().bold().to_string(), "/".to_string()));
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

                let mut name = "..".blue().bold().to_string();
                if f {
                    name.push('/');
                }

                let perms_str = perms(meta_data.clone(), Path::new(&name));

                if l {
                    add_to_flag_l(&name, meta_data, &mut flag_l, "/".to_string(), perms_str);
                } else {
                    no_flag_or_a_f.push((name.blue().bold().to_string(), "/".to_string()));
                }
            }
        } else {
            // if directory is root '/'
            if let Ok(meta_data) = fs::metadata(dir_path) {
                total += meta_data.st_blocks() / 2;

                let mut name = "..".blue().bold().to_string();
                if f {
                    name.push('/');
                }

                let perms_str = perms(meta_data.clone(), Path::new(&name));

                if l {
                    add_to_flag_l(&name, meta_data, &mut flag_l, "/".to_string(), perms_str);
                } else {
                    no_flag_or_a_f.push((name.blue().bold().to_string(), "/".to_string()));
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
            // let debug_output: String = format!("{:?}", meta_data.permissions());

            if !a && name.starts_with(".") {
                continue;
            }

            total += meta_data.st_blocks() / 2;

            let perm_str = perms(meta_data.clone(), &en.path());

            let st = flag_f(perm_str.clone());

            if f {
                if l && st != "@" || !l {
                    name.push_str(&st);
                }
            }

            if perm_str.contains("l") && l {
                if let Ok(target) = fs::read_link(&en.path()) {
                    name.push_str(&format!(" -> {}", target.display()));
                }
            }

            // add l atawich
            if l {
                add_to_flag_l(&name, meta_data, &mut flag_l, st.to_string(), perm_str);
            } else {
                no_flag_or_a_f.push((name, st.to_string()));
            }
        }
    }

    if l {
        sort_l(&mut flag_l, a);
        println!("total {}", total);
        format_flag_l(flag_l);
    } else {
        sort(&mut no_flag_or_a_f, a);
        format_flag(no_flag_or_a_f);
        // println!(
        //     "{}",
        //     no_flag_or_a_f
        //         .iter()
        //         .map(|f| f.0.clone())
        //         .collect::<Vec<String>>()
        //         .join(" ")
        // )
    }
}

fn add_to_flag_l(
    name: &str,
    meta_data: fs::Metadata,
    flag_l: &mut Vec<Vec<String>>,
    color: String,
    perms_str: String,
) {
    // let debug_output = format!("{:?}", meta_data.permissions());
    // let perms_str = perms(meta_data.clone(), Path::new(&name));
    let nlink = meta_data.st_nlink();
    let uid = meta_data.st_uid();
    let gid = meta_data.st_gid();
    let size = if meta_data.file_type().is_char_device() || meta_data.file_type().is_block_device()
    {
        let rdev = meta_data.st_rdev();
        let major_num = major(rdev);
        let minor_num = minor(rdev);

        format!("{}, {}", major_num, minor_num)
    } else {
        meta_data.st_size().to_string()
    };

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
            color,
            perms_str,
            nlink.to_string(),
            username,
            groupname,
            size,
            datetime,
            name.to_string(),
        ]
        .to_vec(),
    );
}

fn perms(meta_data: fs::Metadata, path: &Path) -> String {
    let mode = meta_data.st_mode();

    let owner = (mode & 0o700) >> 6;
    let group = (mode & 0o070) >> 3;
    let others = mode & 0o007;

    let mut perm_str = String::new();

    let file_type = meta_data.file_type();

    perm_str.push(if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else if file_type.is_file() {
        '-'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else {
        '?'
    });

    perm_str.push(if owner & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if owner & 0o2 != 0 { 'w' } else { '-' });
    if mode & 0o4000 != 0 {
        perm_str.push(if owner & 0o1 != 0 { 's' } else { 'S' });
    } else {
        perm_str.push(if owner & 0o1 != 0 { 'x' } else { '-' });
    }

    perm_str.push(if group & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if group & 0o2 != 0 { 'w' } else { '-' });
    if mode & 0o2000 != 0 {
        perm_str.push(if group & 0o1 != 0 { 's' } else { 'S' });
    } else {
        perm_str.push(if group & 0o1 != 0 { 'x' } else { '-' });
    }

    perm_str.push(if others & 0o4 != 0 { 'r' } else { '-' });
    perm_str.push(if others & 0o2 != 0 { 'w' } else { '-' });
    if mode & 0o1000 != 0 {
        perm_str.push(if others & 0o1 != 0 { 't' } else { 'T' });
    } else {
        perm_str.push(if others & 0o1 != 0 { 'x' } else { '-' });
    }

    let attr_len = unsafe {
        libc::listxattr(
            path.to_str().unwrap_or("").as_ptr() as *const _,
            std::ptr::null_mut(),
            0,
        )
    };
    if attr_len > 0 {
        perm_str.push('+');
    }

    perm_str
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
            if i == 0 {
                continue;
            }

            let mut st = String::new();
            let n = sizes[i] - v.len();

            if i == 3 || i == 4 || i == 1 {
                st.push_str(v);
                st.push_str(&" ".repeat(n));
            } else {
                st.push_str(&" ".repeat(n));
                st.push_str(v);
            }

            result.push(st);
        }

        let name = rows[rows.len() - 1].clone();

        result.push(name);
        println!("{}", result.join(" "));
        result.clear();
    }
}

fn format_flag(files: Vec<(String, String)>) {
    let names: Vec<String> = files.into_iter().map(|(name, _)| name).collect();

    if names.is_empty() {
        return;
    }

    // Get terminal width, default to 80 if unable to determine
    let terminal_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

    // Find the maximum length of file names (without ANSI codes for calculation)
    let max_name_len = names
        .iter()
        .map(|name| strip_ansi_codes(name).len())
        .max()
        .unwrap_or(0);

    // Add some padding between columns
    let column_width = max_name_len + 2;

    // Calculate number of columns that fit in terminal width
    let num_columns = if column_width > 0 {
        (terminal_width / column_width).max(1)
    } else {
        1
    };

    // Calculate number of rows needed
    let num_rows = (names.len() + num_columns - 1) / num_columns;

    // Print the table
    for row in 0..num_rows {
        for col in 0..num_columns {
            let index = row + col * num_rows;
            if index < names.len() {
                let name = &names[index];
                let stripped_len = strip_ansi_codes(name).len();
                print!("{}", name);

                // Add padding for alignment, except for the last column
                if col < num_columns - 1 && index + num_rows < names.len() {
                    let padding = column_width - stripped_len;
                    print!("{}", " ".repeat(padding));
                }
            }
        }
        println!();
    }
}

// Helper function to strip ANSI color codes for length calculation
fn strip_ansi_codes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch == '\x1B' {
            // Skip the escape sequence
            if chars.next() == Some('[') {
                // Skip until we find the ending character (letter)
                while let Some(esc_ch) = chars.next() {
                    if esc_ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            };
        } else {
            result.push(ch);
        }
    }
    result
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

fn sort_l(files: &mut Vec<Vec<String>>, a: bool) {
    let mut one = Vec::new();
    let mut two = Vec::new();

    if a {
        one = files[0].clone();
        two = files[1].clone();
        *files = files[2..].to_vec();
    }

    files.sort_by(|a, b| {
        let one = a[a.len() - 1]
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        let two = b[b.len() - 1]
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();

        one.cmp(&two)
    });

    if a {
        files.insert(0, one);
        files.insert(1, two);
    }

    for f in files.iter_mut() {
        let prefix = f[0].clone();
        let last_index = f.len() - 1;
        let content = f[last_index].clone();

        f[last_index] = match prefix.as_str() {
            "/" => content.blue().bold().to_string(),
            "@" => content.cyan().bold().to_string(),
            "|" => content.red().bold().to_string(),
            "=" => content.purple().bold().to_string(),
            "*" => content.green().bold().to_string(),
            _ => content,
        };
    }
}

fn sort(files: &mut Vec<(String, String)>, a: bool) {
    let mut one = (String::new(), String::new());
    let mut two = (String::new(), String::new());

    if a {
        one = files[0].clone();
        two = files[1].clone();

        *files = files[2..].to_vec();
    }

    files.sort_by(|a, b| {
        let one =
            a.0.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>();
        let two =
            b.0.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>();
        one.to_lowercase().cmp(&two.to_lowercase())
    });

    if a {
        files.insert(0, one);
        files.insert(1, two);
    }

    for f in files.iter_mut() {
        let prefix = f.1.clone();
        let content = f.0.clone();

        f.0 = match prefix.as_str() {
            "/" => content.blue().bold().to_string(),
            "@" => content.cyan().bold().to_string(),
            "|" => content.red().bold().to_string(),
            "=" => content.purple().bold().to_string(),
            "*" => content.green().bold().to_string(),
            _ => content,
        };
    }
}
