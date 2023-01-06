use std::io;
use std::io::BufRead;

fn main() {
    let end_state = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|s| { let (s0, s1) = s.split_at(1); (s0.to_string(), s1.to_string()) })
        .map(|(s0, s1)| (s0.chars().next().unwrap(), s1.parse::<f64>().unwrap()))
        .fold((0.0, 0.0, 0.0), |(x, y, rt), (ins, num)| {
            if ins == 'N' {
                (x, y + num, rt)
            } else if ins == 'S' {
                (x, y - num, rt)
            } else if ins == 'E' {
                (x + num, y, rt)
            } else if ins == 'W' {
                (x - num, y, rt)
            } else if ins == 'L' {
                (x, y, num.mul_add(std::f64::consts::PI / 180.0, rt))
            } else if ins == 'R' {
                (x, y, num.mul_add(-std::f64::consts::PI / 180.0, rt))
            } else if ins == 'F' {
                (num.mul_add(rt.cos(), x), num.mul_add(rt.sin(), y), rt)
            } else {
                unreachable!();
            }
        });
    println!("{}", end_state.0.abs() + end_state.1.abs());
}
