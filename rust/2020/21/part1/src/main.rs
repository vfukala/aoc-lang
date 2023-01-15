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
    let tainted = allergens.iter().map(|a| {
        meals.iter().filter(|(_, allergs)| allergs.iter().any(|a1| a1 == *a)).map(|(ingrs, _)| ingrs.iter().cloned().collect::<HashSet<_>>()).fold(HashSet::new(), |poss: HashSet<String>, cur: HashSet<String>| {
            if poss.is_empty() {
                cur
            } else {
                poss.intersection(&cur).cloned().collect::<HashSet<_>>()
            }
        })
    }).fold(HashSet::new(), |mut prev_t, t| {
        prev_t.extend(t.iter().cloned());
        prev_t
    });
    let res = meals.iter().map(|(ingrs, _)| ingrs.iter().filter(|ing| !tainted.contains(*ing)).count()).sum::<usize>();
    println!("{res}");
}
