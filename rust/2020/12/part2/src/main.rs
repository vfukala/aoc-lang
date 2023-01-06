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
        .fold((0.0_f64, 0.0_f64, 10.0_f64, 1.0_f64), |(sx, sy, wx, wy), (ins, num)| {
            if ins == 'N' {
                (sx, sy, wx, wy + num)
            } else if ins == 'S' {
                (sx, sy, wx, wy - num)
            } else if ins == 'E' {
                (sx, sy, wx + num, wy)
            } else if ins == 'W' {
                (sx, sy, wx - num, wy)
            } else if ins == 'L' {
                let cs = num.to_radians().cos();
                let sn = num.to_radians().sin();
                (sx, sy, wx.mul_add(cs, -sn * wy), wy.mul_add(cs, sn * wx))
            } else if ins == 'R' {
                let cs = num.to_radians().cos();
                let sn = num.to_radians().sin();
                (sx, sy, wx.mul_add(cs, sn * wy), wy.mul_add(cs, -sn * wx))
            } else if ins == 'F' {
                (wx.mul_add(num, sx), wy.mul_add(num, sy), wx, wy)
            } else {
                unreachable!();
            }
        });
    println!("{}", end_state.0.abs() + end_state.1.abs());
}
