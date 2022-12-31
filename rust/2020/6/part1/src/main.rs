use std::io;
use std::io::BufRead;

fn main() {
    let res: usize = io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>().split(|l| l.is_empty()).map(|list| list.join("").trim().to_string()).filter(|l| !l.is_empty()).map(|s| ('a'..='z').into_iter().filter(|c| s.contains(*c)).count()).sum();
    println!("{res}");
}
