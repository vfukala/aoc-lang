use std::io;
use std::io::BufRead;

fn main() {
    let res: i32 = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().map(|c| { if c.is_ascii_lowercase() { c as i32 - 'a' as i32 + 1 } else { c as i32 - 'A' as i32 + 27 } }).collect::<Vec<i32>>())
        .map(|li| { let (l, r) = li.split_at(li.len() / 2); *l.iter().find(|v| r.iter().any(|x| x == *v)).unwrap() })
        .sum();
    println!("{res}");
}
