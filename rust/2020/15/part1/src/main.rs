use std::io;
use std::io::BufRead;

fn main() {
    let inums = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let res = *(inums.len()..2020)
        .fold(inums, |mut cur, _| {
            let nval = cur.iter()
                .rev()
                .skip(1)
                .position(|v| v == cur.last().unwrap())
                .map_or(0, |i| i as i32 + 1);
            cur.push(nval);
            cur
        })
        .last()
        .unwrap();
    println!("{res}");
}
