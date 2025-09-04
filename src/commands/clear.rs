
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

    print!("\x1B[H\x1B[2J\x1B[3J")
}
