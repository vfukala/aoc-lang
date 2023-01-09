use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let n = 30_000_000;
    let inums = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mp = inums.iter().take(inums.len() - 1).enumerate().fold(HashMap::<i32, usize>::new(), |mut mp, (i, val)| { mp.insert(*val, i); mp });
    let res = ((inums.len() - 1)..(n - 1))
        .fold((mp, *inums.last().unwrap()), |(mut cur, nxt), i| {
            let nval = cur.get(&nxt).map_or(0, |ii| i - ii) as i32;
            cur.insert(nxt, i);
            (cur, nval)
        }).1;
    println!("{res}");
}
