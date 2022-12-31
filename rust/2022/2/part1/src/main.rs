use std::io;
use std::io::BufRead;

fn main() {
    let res: i32 = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).map(|s| (s.chars().nth(0).unwrap() as i32 - 'A' as i32, s.chars().nth(2).unwrap() as i32 - 'X' as i32)).map(|t| t.1 + 1 + (t.1 - t.0 + 4) % 3 * 3).sum();
    println!("{res}");
}
