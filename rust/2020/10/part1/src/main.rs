use std::io;
use std::io::BufRead;

fn main() {
    let mut nums = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    nums.push(0);
    nums.push(nums.iter().max().unwrap() + 3);
    nums.sort_unstable();
    let ones = nums.windows(2).filter(|a| a[1] - a[0] == 1).count();
    let threes = nums.windows(2).filter(|a| a[1] - a[0] == 3).count();
    println!("{}", ones * threes);
}
