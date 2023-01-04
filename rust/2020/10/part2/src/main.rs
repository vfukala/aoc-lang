use std::io;
use std::io::BufRead;
use std::collections::VecDeque;

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
    let res = nums.iter()
        .fold(VecDeque::<(i64, i64)>::from([ (-3, 0), (-3, 0), (-3, 1) ]), |mut qq, nu| {
            let val = qq.iter()
                .filter(|tup| tup.0 >= nu - 3)
                .map(|tup| tup.1)
                .sum();
            qq.push_back((*nu, val));
            qq.pop_front();
            qq
        })
        .back()
        .unwrap()
        .1;
    println!("{res}");
}
