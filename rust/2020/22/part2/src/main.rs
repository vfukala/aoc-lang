use std::io;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::VecDeque;

fn playround(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    let pc1 = p1.pop_front().unwrap();
    let pc2 = p2.pop_front().unwrap();
    let p1w = if pc1 <= p1.len() && pc2 <= p2.len() { p1wins(&mut p1.iter().copied().take(pc1).collect::<VecDeque<_>>(), &mut p2.iter().copied().take(pc2).collect::<VecDeque<_>>()) } else { pc1 > pc2 };
    if p1w {
        p1.push_back(pc1);
        p1.push_back(pc2);
    } else {
        p2.push_back(pc2);
        p2.push_back(pc1);
    }
}

fn play(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    while !p1.is_empty() && !p2.is_empty() {
        playround(p1, p2);
    }
}

fn p1wins(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
    let mut visited = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        if visited.contains(&(p1.clone(), p2.clone())) {
            return true;
        }
        visited.insert((p1.clone(), p2.clone()));
        playround(p1, p2);
    }
    p2.is_empty()
}

fn main() {
    let mut lns = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap());
    let mut p1 = lns.by_ref().skip(1).take_while(|l| !l.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect::<VecDeque<_>>();
    let mut p2 = lns.skip(1).take_while(|l| !l.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect::<VecDeque<_>>();
    play(&mut p1, &mut p2);
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
