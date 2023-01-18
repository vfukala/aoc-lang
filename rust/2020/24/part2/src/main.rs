use std::io;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let istate = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .scan(None, |prev, c| {
                    let p = *prev;
                    *prev = Some(c);
                    Some(if c == 'e' {
                        if p == Some('n') {
                            Some((0, 1))
                        } else if p == Some('s') {
                            Some((1, -1))
                        } else {
                            Some((1, 0))
                        }
                    } else if c == 'w' {
                        if p == Some('n') {
                            Some((-1, 1))
                        } else if p == Some('s') {
                            Some((0, -1))
                        } else {
                            Some((-1, 0))
                        }
                    } else {
                        None
                    })
                })
            .flatten()
                .reduce(|(x0, y0), (x1, y1)| (x0 + x1, y0 + y1)).unwrap()
        }).fold(HashSet::new(), |mut coll, ele| {
            if coll.contains(&ele) {
                coll.remove(&ele);
            } else {
                coll.insert(ele);
            }
            coll
        });
    let neis = [ (-1, 0), (0, -1), (1, -1), (1, 0), (0, 1), (-1, 1) ];
    let res = (0..100).fold(istate, |state, _| {
        state.iter().copied()
            .flat_map(|p| std::iter::once(p).chain(neis.iter().map(|nt| (nt.0 + p.0, nt.1 + p.1))).collect::<Vec<_>>())
            .filter(|p| {
                let ncount = neis.iter().filter(|nt| state.contains(&(nt.0 + p.0, nt.1 + p.1))).count();
                if state.contains(p) {
                    (1..=2).contains(&ncount)
                } else {
                    ncount == 2
                }
            })
        .collect::<HashSet<_>>()
    }).len();
    println!("{res}");
}
