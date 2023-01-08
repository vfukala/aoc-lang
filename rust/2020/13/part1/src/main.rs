use std::io;
use std::io::BufRead;

fn main() {
    let mut lns = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty());
    let k = lns.next().unwrap().parse::<i64>().unwrap();
    let mnb = lns.next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .map(|id| (id, id - 1 - (k - 1) % id))
        .min_by(|v0, v1| v0.1.cmp(&v1.1))
        .unwrap();
    println!("{}", mnb.0 * mnb.1);
}
