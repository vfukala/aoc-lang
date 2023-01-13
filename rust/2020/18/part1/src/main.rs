use std::io;
use std::io::BufRead;

fn parse_exp<T: Iterator<Item = char>>(iter: &mut std::iter::Peekable<T>) -> i64 {
    let mut val = parse_primary(iter);
    while iter.peek().is_some() && *iter.peek().unwrap() != ')' {
        let op = iter.next().unwrap();
        let operand = parse_primary(iter);
        if op == '+' {
            val += operand;
        } else {
            val *= operand;
        }
    }
    val
}

fn parse_primary<T: Iterator<Item = char>>(iter: &mut std::iter::Peekable<T>) -> i64 {
    let peeked = *iter.peek().unwrap();
    if peeked == '(' {
        iter.next();
        let val = parse_exp(iter);
        iter.next();
        val
    }
    else {
        parse_num(iter)
    }
}

fn parse_num<T: Iterator<Item = char>>(iter: &mut std::iter::Peekable<T>) -> i64 {
    let mut num = 0i64;
    while iter.peek().is_some() && iter.peek().unwrap().is_ascii_digit() {
        num = 10 * num + i64::from(iter.next().unwrap().to_digit(10).unwrap());
    }
    num
}

fn main() {
    let res = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| parse_exp(&mut l.chars()
             .filter(|c| *c != ' ')
             .collect::<Vec<char>>().into_iter().peekable()))
             .sum::<i64>();
    println!("{res}");
}
