use std::io;
use std::io::BufRead;

enum Ins {
    Nop,
    Acc,
    Jmp
}

use crate::Ins::*;

fn main() {
    let program = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l|
             (if l.starts_with("nop") {
                 Nop
             } else if l.starts_with("acc") {
                 Acc
             } else if l.starts_with("jmp") {
                 Jmp
             } else {
                 unreachable!();
             }, l[4..].parse::<i32>().unwrap())).collect::<Vec<(Ins, i32)>>();
    let n = program.len();
    for cha in 0..=n {
        let mut vis = vec![false; n];
        let mut nxt = 0;
        let mut val = 0;
        while nxt < n && !vis[nxt] {
            vis[nxt] = true;
            let act = match (&program[nxt].0, nxt) {
                (Acc, _) => Acc,
                (Jmp, i) if i == cha => Nop,
                (Jmp, _) => Jmp,
                (Nop, i) if i == cha => Jmp,
                (Nop, _) => Nop
            };
            match act {
                Nop => nxt += 1,
                Acc => {
                    val += program[nxt].1;
                    nxt += 1;
                }
                Jmp => nxt = (nxt as i32 + program[nxt].1) as usize
            };
        }
        if nxt == n {
            println!("{val}");
        }
    }
}
