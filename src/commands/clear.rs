
pub fn clear(_args: Vec<&str>) {
    print!("\x1B[H\x1B[2J\x1B[3J")
}
