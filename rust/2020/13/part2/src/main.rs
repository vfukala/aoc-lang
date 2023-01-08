use std::io;
use std::io::BufRead;

const fn mult_inv(mut a: i64, p: i64) -> i64 {
    let mut res = 1;
    let mut e = p - 2;
    while e != 0 {
        if e % 2 == 1 {
            res *= a;
            res %= p;
        }
        a *= a;
        a %= p;
        e /= 2;
    }
    res
}

fn main() {
    let reqs = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>())
        .enumerate()
        .filter(|(_, r)| r.is_ok())
        .map(|(i, r)| (i as i64, r.unwrap()))
        .collect::<Vec<(i64, i64)>>();
    let prod = reqs.iter().map(|(_, m)| m).product::<i64>();
    let res = reqs.iter().map(|(i, m)| {
        let oprod = prod / m;
        (-i * oprod % prod + prod) % prod * mult_inv(oprod % m, *m)
    }).sum::<i64>() % prod;
    println!("{res}");
}
