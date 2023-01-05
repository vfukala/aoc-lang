use std::io;
use std::io::BufRead;
use core::cmp::min;
use core::cmp::max;

fn main() {
    let chairs = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().map(|c| c == 'L').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let w = chairs[0].len();
    let h = chairs.len();
    let mut occu = vec![vec![false; w]; h];
    let mut change = true;
    while change {
        change = false;
        let mut nwst = vec![vec![false; chairs[0].len()]; chairs.len()];
        for x in 0..(w as i32) {
            for y in 0..(h as i32) {
                let ux = x as usize;
                let uy = y as usize;
                if chairs[uy][ux] {
                    let mut occ = 0;
                    for xx in max(0, x - 1)..=min(w as i32 - 1, x + 1) {
                        for yy in max(0, y - 1)..=min(h as i32 - 1, y + 1) {
                            if (xx != x || yy != y) && occu[yy as usize][xx as usize] {
                                occ += 1;
                            }
                        }
                    }
                    nwst[uy][ux] = occ == 0 || occ < 4 && occu[uy][ux];
                    if nwst[uy][ux] != occu[uy][ux] {
                        change = true;
                    }
                }
            }
        }
        occu = nwst;
    }
    let res = occu.iter().map(|li| li.iter().filter(|v| **v).count()).sum::<usize>();
    println!("{res}");
}
