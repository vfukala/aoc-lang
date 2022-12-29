use std::io;
use std::io::BufRead;

fn main() {
    let res = io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>()
        .split(|l| l.is_empty())
        .map(|list| list.join(" "))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().split(' ').map(|ss| ss.split_once(':').unwrap().0.to_string()).collect::<Vec<String>>())
        .filter(|list| ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|ele| list.contains(&ele.to_string())))
        .count();
    println!("{}", res);
}
