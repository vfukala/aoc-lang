use std::io;
use std::io::BufRead;

use std::collections::VecDeque;

fn main() {
    let mut lns = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap());
    let mut p1 = lns.by_ref().skip(1).take_while(|l| !l.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect::<VecDeque<_>>();
    let mut p2 = lns.skip(1).take_while(|l| !l.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect::<VecDeque<_>>();
    while !p1.is_empty() && !p2.is_empty() {
        let pc1 = p1.pop_front().unwrap();
        let pc2 = p2.pop_front().unwrap();
        if pc1 < pc2 {
            p2.push_back(pc2);
            p2.push_back(pc1);
        } else {
            p1.push_back(pc1);
            p1.push_back(pc2);
        }
    }
    let winning = if p1.is_empty() { p2 } else { p1 };
    let res = winning.iter().rev().enumerate().map(|(i, v)| (i + 1) * v).sum::<usize>();
    println!("{res}");
}
