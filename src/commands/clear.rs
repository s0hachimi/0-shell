use std::process::Command;

pub fn clear(args: Vec<&str>) {
    if args.len() > 0 {
        println!(
            "Usage: clear [options]

Options:
  -T TERM     use this instead of $TERM
  -V          print curses-version
  -x          do not try to clear scrollback"
        );
        return;
    }

    let output = Command::new("clear")
        .output()
        .expect("Failed to execute 'clear' command");

    let stdout_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in output");
    print!("{}", stdout_str)
}
