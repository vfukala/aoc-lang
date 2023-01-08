use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let res = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .fold((HashMap::new(), 0, 0), |(mut mp, mask_and, mask_or), l| {
            if l.chars().nth(2).unwrap() == 's' {
                let mask_chars = l.chars().skip(7).collect::<Vec<char>>();
                (mp,
                 mask_chars.iter().fold(0, |v, c| 2 * v + i64::from(*c != '0')),
                 mask_chars.iter().fold(0, |v, c| 2 * v + i64::from(*c == '1')))
            } else {
                let eqi = l.find('=').unwrap();
                let loc = l[4..(eqi-2)].parse::<i64>().unwrap();
                let val = l[(eqi+2)..].parse::<i64>().unwrap();
                mp.insert(loc, val & mask_and | mask_or);
                (mp, mask_and, mask_or)
            }
        }).0
        .into_values()
        .sum::<i64>();
    println!("{res}");
}
