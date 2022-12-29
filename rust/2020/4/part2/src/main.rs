use std::io;
use std::io::BufRead;

fn is_in_range(s: &str, low: i32, high: i32) -> bool {
    s.parse::<i32>().map_or(false, |val| low <= val && val <= high)
}

fn fdig_pred(low: i32, high: i32) -> Box<dyn Fn(&str) -> bool> {
    Box::new(move |s| if s.len() == 4 { is_in_range(s, low, high) } else { false })
}

fn main() {
    let res = io::stdin().lock().lines().map(unwrap).collect::<Vec<String>>()
        .split(|l| l.is_empty())
        .map(|list| list.join(" "))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().split(' ').map(|ss| { let pa = ss.split_once(':').unwrap(); (pa.0.to_string(), pa.1.to_string()) }).collect::<Vec<(String, String)>>())
        .filter(|list| [
                ("byr", fdig_pred(1920, 2002)),
                ("iyr", fdig_pred(2010, 2020)),
                ("eyr", fdig_pred(2020, 2030)),
                ("hgt", Box::new(|s: &str| {
                    if s.ends_with("cm") {
                        is_in_range(&s[0..s.len() - 2], 150, 193)
                    } else if s.ends_with("in") {
                        is_in_range(&s[0..s.len() - 2], 59, 76)
                    } else {
                        false
                    }
                })),
                ("hcl", Box::new(|s| s.len() == 7 && s.starts_with('#') && s.chars().skip(1).all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
                )),
                ("ecl", Box::new(|s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s))),
                ("pid", Box::new(|s| s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())))]
                    .iter().all(|(nm, pre)| 
                        list.iter().find(|(id, _)| id == nm).map_or(false, |val| pre(&val.1))                ))
        .count();
    println!("{res}");
}
