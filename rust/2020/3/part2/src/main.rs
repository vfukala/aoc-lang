use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).collect::<Vec<String>>();
    let res : i64 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(|(dx, dy)| {
        let mut cou = 0;
        let mut x = 0;
        let mut y = 0;
        while y < lines.len() {
            if lines[y].chars().nth(x).unwrap() == '#' {
                cou += 1;
            }
            x += dx;
            x %= lines[0].len();
            y += dy;
        }
        return cou;
    }).product();
    println!("{}", res);
}
