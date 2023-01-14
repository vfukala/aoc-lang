use std::io;
use std::io::BufRead;

fn main() {
    let tiles = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .fold(vec![], |mut tiles: Vec<(i64, Vec<Vec<bool>>)>, l| {
            if l.starts_with("Tile") {
                let spi = l.find(' ').unwrap();
                tiles.push((l[spi + 1..l.len() - 1].parse::<i64>().unwrap(), vec![]));
            } else if !l.is_empty() {
                tiles.last_mut().unwrap().1.push(l.chars().map(|c| c == '#').collect::<Vec<_>>());
            }
            tiles
        });
    let n = tiles.first().unwrap().1.len();
    let borders = tiles.iter().map(|(id, grid)| {
        let b0 = (0..n).map(|i| grid[0][i]).collect::<Vec<_>>();
        let b1 = (0..n).map(|i| grid[i][0]).collect::<Vec<_>>();
        let b2 = (0..n).map(|i| grid[n - 1][i]).collect::<Vec<_>>();
        let b3 = (0..n).map(|i| grid[i][n - 1]).collect::<Vec<_>>();
        (id, vec![b0, b1, b2, b3])
    }).collect::<Vec<_>>();
    let res = borders.iter().filter(|(_, brds)| {
        brds.iter().filter(|brd|
                           borders.iter().filter(|(_, obrds)|
                                                 obrds.iter().any(|obrd|
                                                                  obrd.iter().eq(brd.iter()) || obrd.iter().rev().eq(brd.iter())
                                                                  )
                                                 ).count() == 1
                           ).count() == 2
    }).map(|(id, _)| **id).product::<i64>();
    println!("{res}");
}
