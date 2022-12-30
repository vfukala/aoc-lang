use std::io;
use std::io::BufRead;

fn main() {
    let mut ve = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).map(|l| l.chars().fold(0, |p, c| 2 * p + i32::from(c == 'B' || c == 'R'))).collect::<Vec<i32>>();
    ve.sort_unstable();
    let res = ve.iter().fold((0, 0), |(pv, pm), v| if pv == v - 2 { (*v, v - 1) } else { (*v, pm) }).1;
    println!("{res}");
}
