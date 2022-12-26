use std::io;
use std::io::BufRead;

fn main() {
    let mut vals = Vec::new();
    for line in io::stdin().lock().lines() {
        if let Ok(num) = line.unwrap().parse::<i32>() {
            vals.push(num);
        }
    }
    for x in &vals {
        for y in &vals {
            if x + y == 2020 {
                println!("{}", x * y);
                return;
            }
        }
    }
}
