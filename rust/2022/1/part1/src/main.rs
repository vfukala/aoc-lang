use std::io;
use std::io::BufRead;

fn main() {
    let res: i32 = io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>().split(|l| l.is_empty()).map(|list| list.iter().map(|s| s.parse::<i32>().unwrap()).sum()).max().unwrap();
    println!("{res}");
}
