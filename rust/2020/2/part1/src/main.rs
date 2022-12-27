use std::io;
use std::io::BufRead;

fn main() {
    let valid_count = io::stdin().lock().lines().filter(| line | {
        let l = line.as_ref().unwrap();
        if !l.is_empty() {
            let di = l.find('-').unwrap();
            let si = l.find(' ').unwrap();
            let low = l.chars().take(di).collect::<String>().parse::<usize>().unwrap();
            let high = l.chars().skip(di + 1).take(si - di - 1).collect::<String>().parse::<usize>().unwrap();
            let schar = l.chars().nth(si + 1).unwrap();
            let ci = l.find(':').unwrap();
            let pass : String = l.chars().skip(ci + 2).collect();
            let mt = pass.matches(schar).count();
            return low <= mt && mt <= high;
        }
        return false;
    }).count();
    println!("{}", valid_count);
}
