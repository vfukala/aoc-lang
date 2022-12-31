use std::io;
use std::io::BufRead;

fn main() {
    let res: usize = io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>().split(|l| l.is_empty()).filter(|list| list.iter().any(|s| !s.is_empty())).map(|list| ('a'..='z').into_iter().filter(|c| list.iter().all(|s| s.contains(*c))).count()).sum();
    println!("{res}");
}
