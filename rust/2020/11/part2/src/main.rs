use std::io;
use std::io::BufRead;

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
                    let dirs = [(1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)];
                    let occ = dirs.iter().filter(|(dx, dy)| {
                        let mut sx = x + dx;
                        let mut sy = y + dy;
                        while (0..(w as i32)).contains(&sx) && (0..(h as i32)).contains(&sy) {
                            if chairs[sy as usize][sx as usize] {
                                return occu[sy as usize][sx as usize];
                            }
                            sx += dx;
                            sy += dy;
                        }
                        false
                    }).count();
                    nwst[uy][ux] = occ == 0 || occ < 5 && occu[uy][ux];
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
