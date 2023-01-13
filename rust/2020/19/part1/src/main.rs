use std::io;
use std::io::BufRead;

use std::collections::HashMap;
use std::collections::HashSet;

enum Rule {
    Verb(String),
    Compo(Vec<Vec<i32>>)
}

use Rule::*;

fn main() {
    let mut lns = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap());
    let rules = lns.by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let spi = l.find(' ').unwrap();
            let id = l[0..spi - 1].parse::<i32>().unwrap();
            let rst = &l[(spi + 1)..];
            if rst.starts_with('\"') {
                (id, Verb(rst[1..(rst.len() - 1)].to_string()))
            } else {
                (id, Compo(rst.split(" | ").map(|ss| ss.split(' ').map(|sms| sms.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()))
            }
        })
        .fold(HashMap::new(), |mut pmap, (id, rule)| {
            pmap.insert(id, rule);
            pmap
        });
    let mut dfs_stack = vec![];
    let mut visited = HashSet::new();
    let mut top_ord = vec![];
    dfs_stack.push((0, false));
    while !dfs_stack.is_empty() {
        let cur = dfs_stack.pop().unwrap();
        if !visited.contains(&cur.0) {
            if cur.1 {
                top_ord.push(cur.0);
                visited.insert(cur.0);
            } else {
                dfs_stack.push((cur.0, true));
                match rules.get(&cur.0).unwrap() {
                    Verb(_) => (),
                    Compo(chil) => chil.iter().flatten().for_each(|ch| {
                        if !visited.contains(ch) {
                            dfs_stack.push((*ch, false));
                        }
                    })
                }
            }
        }
    }
    let res = lns.filter(|l| !l.is_empty())
        .filter(|s| {
            let n = s.len();
            top_ord.iter().fold(HashMap::new(), |mut pmap: HashMap<i32, Vec<Vec<bool>>>, ri| {
                let new_table = (0..=n).map(|i0| (0..=n).map(|i1| {
                    i0 <= i1 && match rules.get(ri).unwrap() {
                        Verb(rs) => rs[..] == s[i0..i1],
                        Compo(vv) => vv.iter().any(|srules| {
                            let sn = srules.len();
                            let mut dv = vec![i0; sn - 1];
                            loop {
                                if (0..sn).all(|i| pmap.get(&srules[i]).unwrap()[if i == 0 { i0 } else {dv[i - 1] }][if i == sn - 1 { i1 } else { dv[i] }]) {
                                    return true;
                                }

                                if sn == 1 {
                                    break;
                                }

                                dv[sn - 2] += 1;
                                let mut i = sn - 2;
                                while i >= 1 && dv[i] == i1 + 1 {
                                    i -= 1;
                                    dv[i] += 1;
                                }
                                if dv[0] == i1 + 1 {
                                    break;
                                }
                                for j in (i + 1)..(sn - 1) {
                                    dv[j] = dv[i];
                                }
                            }
                            false
                        })
                    }
                }).collect::<Vec<_>>()
            ).collect::<Vec<_>>();
            pmap.insert(*ri, new_table);
            pmap
            }).get(&0).unwrap()[0][n]
        }).count();
    println!("{res}");
}
