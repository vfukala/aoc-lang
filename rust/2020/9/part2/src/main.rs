use std::io;
use std::io::BufRead;

fn main() {
    let ws = 25;
    let nums = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let num: i64 = *nums
        .windows(ws + 1)
        .find(|w| !w.iter().any(|v1| w.iter().any(|v2| v1 != v2 && v1 + v2 == *w.last().unwrap())))
        .unwrap()
        .last()
        .unwrap();
    let n = nums.len();
    let psums = std::iter::once(0).chain(nums.iter().scan(0, |ps, &v| {
        *ps += v;
        Some(*ps)
    })).collect::<Vec<i64>>();
    let mut slc: &[i64] = &nums;
    for i in 0..=n {
        for j in 0..=n {
            if i + 1 < j && psums[j] - psums[i] == num {
                slc = &nums[i..j];
            }
        }
    }
    println!("{}", slc.iter().min().unwrap() + slc.iter().max().unwrap());
}
