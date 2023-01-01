use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let edges = io::stdin().lock().lines().map(|line| line.unwrap()).filter(|l| !l.is_empty()).map(|s| s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|ls| {
             let nm = ls[0].clone() + "_" + &ls[1];
             let goesto = if ls[4] == "no" { vec![] } else { ls.iter().zip(ls.iter().skip(1)).zip(ls.iter().skip(2)).skip(4).zip(0..=ls.len()).filter(|e| e.1 % 4 == 0).map(|e| e.0).map(|((nu, s0), s1)| (nu.parse::<i64>().unwrap(), s0.clone() + "_" + s1)).collect::<Vec<(i64,String)>>() };
             (nm, goesto)
        }).collect::<Vec<(String, Vec<(i64,String)>)>>();
    let mut szs = HashMap::<String, i64>::new();
    for _ in 0..edges.len() {
        for node in &edges {
            if node.1.iter().all(|(_,nm)| szs.contains_key(nm)) && !szs.contains_key(&node.0) {
                szs.insert(node.0.clone(), 1 + node.1.iter().map(|(c, nm)| c * szs.get(nm).unwrap()).sum::<i64>());
            }
        }
    }
    println!("{}", szs.get("shiny_gold").unwrap() - 1);
}
