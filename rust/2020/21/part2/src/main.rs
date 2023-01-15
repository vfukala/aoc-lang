use std::io;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let meals = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let pari = l.find('(').unwrap();
            let ingredients = l[0..pari - 1].split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
            let allergens = l[pari + 10..l.len() - 1].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            (ingredients, allergens)
        }).collect::<Vec<_>>();
    let allergens = meals.iter().flat_map(|(_, allergs)| allergs).fold(HashSet::new(), |mut p_set, allg| {
        p_set.insert(allg);
        p_set
    });
    let mut options = allergens.iter().map(|a| {
        (a, meals.iter().filter(|(_, allergs)| allergs.iter().any(|a1| a1 == *a)).map(|(ingrs, _)| ingrs.iter().cloned().collect::<HashSet<_>>()).fold(HashSet::new(), |poss: HashSet<String>, cur: HashSet<String>| {
            if poss.is_empty() {
                cur
            } else {
                poss.intersection(&cur).cloned().collect::<HashSet<_>>()
            }
        }))
    }).collect::<Vec<_>>();
    let mut assignment = vec![];
    while !options.is_empty() {
        let cur = options.iter().find(|(_, opts)| opts.len() == 1).unwrap().clone();
        let alle = cur.0;
        let ing = cur.1.iter().next().unwrap().clone();
        assignment.push((alle, ing.clone()));
        options = options.into_iter().filter(|(a, _)| *a != alle).map(|(a, mut opts)| { opts.remove(&ing); (a, opts) }).collect::<Vec<_>>();
    }
    assignment.sort_by(|(a0, _), (a1, _)| a0.cmp(a1));
    let res = assignment.into_iter().map(|(_, ing)| ing).reduce(|p0, p1| p0 + "," + &p1).unwrap();
    println!("{res}");
}
