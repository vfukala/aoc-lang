use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let res = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .fold((HashMap::new(), vec![]), |(mut mp, mask), l| {
            if l.chars().nth(2).unwrap() == 's' {
                (mp, l.chars().skip(7).collect::<Vec<char>>())
            } else {
                let eqi = l.find('=').unwrap();
                let loc = l[4..(eqi-2)].parse::<i64>().unwrap();
                let val = l[(eqi+2)..].parse::<i64>().unwrap();
                let xs = mask.iter()
                    .rev()
                    .enumerate()
                    .filter(|(_, c)| **c == 'X')
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>();
                let mask_or = mask.iter().fold(0, |v, c| 2 * v + i64::from(*c == '1'));
                let locor = loc | mask_or;
                for xx in 0..(1 << xs.len()) {
                    let mask_xor = xs.iter().fold((xx, 0), |(re, msk), ps|
                                           (re / 2, msk + if (re & 1) == 1 { 1 << ps } else { 0 })
                                           ).1;
                    mp.insert(locor ^ mask_xor, val);
                }
                (mp, mask)
            }
        }).0
        .into_values()
        .sum::<i64>();
    println!("{res}");
}
