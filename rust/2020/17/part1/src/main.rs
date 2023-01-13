use std::io;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let init_set = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, s): (usize, String)| s.chars().enumerate().filter(|(_, c)| *c == '#').map(|(x, _)| (x as i32, y as i32, 0)).collect::<Vec<_>>())
        .collect::<HashSet<_>>();
    let res_set = (0..6).fold(init_set, |prev_set, _| prev_set.iter().fold(HashSet::new(), |mut hset, (cx, cy, cz)| {
        for x in (cx - 1)..=(cx + 1) {
            for y in (cy - 1)..=(cy + 1) {
                for z in (cz - 1)..=(cz + 1) {
                    let mut ncount = 0;
                    for lx in (x - 1)..=(x + 1) {
                        for ly in (y - 1)..=(y + 1) {
                            for lz in (z - 1)..=(z + 1) {
                                if prev_set.contains(&(lx, ly, lz)) {
                                    ncount += 1;
                                }
                            }
                        }
                    }
                    if prev_set.contains(&(x, y, z)) {
                        if [3, 4].contains(&ncount) {
                            hset.insert((x, y, z));
                        }
                    }
                    else {
                        if ncount == 3 {
                            hset.insert((x, y, z));
                        }
                    }
                }
            }
        }
        hset
    }));
    println!("{}", res_set.len());
}
