use std::io;
use std::io::BufRead;

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    let mut res = 1;
    while e != 0 {
        if e & 1 == 1 {
            res *= a;
            res %= m;
        }
        a *= a;
        a %= m;
        e /= 2;
    }
    res
}

fn main() {
    let mut res = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap());
    let p0 = res.next().unwrap();
    let p1 = res.next().unwrap();
    let m = 20201227;
    let pr0 = (0..m-1).find(|e| mod_pow(7, *e, m) == p0).unwrap();
    let res = mod_pow(p1, pr0, m);
    println!("{res}");
}
