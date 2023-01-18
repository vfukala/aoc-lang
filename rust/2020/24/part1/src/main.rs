use std::io;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let res = io::stdin()
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
                .reduce(|(x0, y0), (x1, y1)| (x0 + x1, y0 + y1))
        }).fold(HashSet::new(), |mut coll, ele| {
            if coll.contains(&ele) {
                coll.remove(&ele);
            } else {
                coll.insert(ele);
            }
            coll
        }).len();
    println!("{res}");
}
