use std::io;
use std::io::BufRead;

fn main() {
    let (cou, _) = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).fold((0, 0), |(cou, pos), l| {
        let ncou = cou + (l.chars().nth(pos).unwrap() == '#') as i32;
        let npos = (pos + 3) % l.len();
        return (ncou, npos);
    });
    println!("{}", cou);
}
