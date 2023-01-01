use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let bags = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().map(|c| { if c.is_ascii_lowercase() { c as i32 - 'a' as i32 + 1 } else { c as i32 - 'A' as i32 + 27 } }).collect::<HashSet<i32>>()).collect::<Vec<HashSet<i32>>>();
    let res: i32 = bags.windows(3)
        .step_by(3)
        .map(|trip| *trip[0].iter().find(|v| trip[1].contains(v) && trip[2].contains(v)).unwrap())
        .sum();
    println!("{res}");
}
