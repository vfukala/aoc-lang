use std::io;
use std::io::BufRead;

fn main() {
    let ws = 25;
    let res: i64 = *io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(ws + 1)
        .find(|w| !w.iter().any(|v1| w.iter().any(|v2| v1 != v2 && v1 + v2 == *w.last().unwrap())))
        .unwrap()
        .last()
        .unwrap();
    println!("{res}");
}
