use std::io;
use std::io::BufRead;

fn main() {
    let res = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).map(|l| l.chars().fold(0, |p, c| 2 * p + i32::from(c == 'B' || c == 'R'))).max().unwrap();
    println!("{res}");
}
