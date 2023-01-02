use std::io;
use std::io::BufRead;

enum Ins {
    Nop,
    Acc(i32),
    Jmp(i32),
}

use crate::Ins::*;

fn main() {
    let program = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l|
             if l.starts_with("nop") {
                 Nop
             } else if l.starts_with("acc") {
                 Acc(l[4..].parse::<i32>().unwrap())
             } else if l.starts_with("jmp") {
                 Jmp(l[4..].parse::<i32>().unwrap())
             } else {
                 unreachable!();
             }).collect::<Vec<Ins>>();
    let n = program.len();
    let mut vis = vec![false; n];
    let mut nxt = 0;
    let mut val = 0;
    while !vis[nxt] {
        vis[nxt] = true;
        match program[nxt] {
            Nop => nxt += 1,
            Acc(a) => {
                nxt += 1;
                val += a;
            }
            Jmp(j) => nxt = (nxt as i32 + j) as usize
        }
    }
    println!("{val}");
}
