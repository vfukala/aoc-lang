use std::io;
use std::io::BufRead;

fn main() {
    let res = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|s| s.split(',')
             .flat_map(|ss| ss.split('-'))
             .map(|ss| ss.parse::<i32>().unwrap())
             .collect::<Vec<i32>>())
        .filter(|li| li[2] <= li[1] && li[0] <= li[3])
        .count();
    println!("{res}");
}
