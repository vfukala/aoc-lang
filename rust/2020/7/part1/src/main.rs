use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let edges = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).map(|s| s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|ls| {
             let nm = ls[0].clone() + "_" + &ls[1];
             let goesto = if ls[4] == "no" { vec![] } else { ls.iter().zip(ls.iter().skip(1)).map(|p| p.0.clone() + "_" + p.1).skip(5).zip(0..=ls.len()).filter(|e| e.1 % 4 == 0).map(|e| e.0).collect::<Vec<String>>() };
             (nm, goesto)
        }).collect::<Vec<(String, Vec<String>)>>();
    let mut cols = HashSet::new();
    for _ in 0..edges.len() {
        for node in &edges {
            if node.1.iter().any(|s| s == "shiny_gold" || cols.contains(s)) {
                cols.insert(node.0.clone());
            }
        }
    }
    println!("{}", cols.len());
}
