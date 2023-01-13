use std::io;
use std::io::BufRead;

fn main() {
    let mut lns = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap());
    let ranges = lns.by_ref().take_while(|l| !l.is_empty()).map(|l| {
        let nums = l.split(|c| c == ' ' || c == '-')
            .filter_map(|w| w.parse::<i32>().ok())
            .collect::<Vec<_>>();
        (nums[0], nums[1], nums[2], nums[3])
    }).collect::<Vec<_>>();
    let res = lns.skip(5)
        .filter(|l| !l.is_empty())
        .map(|l|
            l.split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .filter(|v| !ranges.iter().any(|rr| rr.0 <= *v && *v <= rr.1 || rr.2 <= *v && *v <= rr.3))
            .sum::<i32>()
        ).sum::<i32>();
    println!("{res}");
}
