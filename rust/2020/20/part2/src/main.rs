use std::io;
use std::io::BufRead;

fn rot(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = grid.len();
    (0..n).map(|x| (0..n).map(|y| grid[n - 1 - y][x]).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn flip(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = grid.len();
    (0..n).map(|x| (0..n).map(|y| grid[n - 1 - x][y]).collect::<Vec<_>>()).collect::<Vec<_>>()
}

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
    let alltiles = tiles.iter().flat_map(|(id, grid)| {
        let r1 = rot(grid);
        let r2 = rot(&r1);
        let r3 = rot(&r2);
        [ flip(grid), flip(&r1), flip(&r2), flip(&r3), grid.clone(), r1, r2, r3 ].map(|g| (id, g))
    }).collect::<Vec<_>>();
    let n = tiles.first().unwrap().1.len();
    let m = (tiles.len() as f64).sqrt() as usize;
    let mut spaced_board = vec![vec![]; m];
    let tile0 = alltiles.iter().find(|(id, grid)| alltiles.iter().all(|(o_id, o_grid)| id == o_id || (0..n).any(|i| grid[0][i] != o_grid[0][i]) && (0..n).any(|i| grid[i][0] != o_grid[i][0]))).unwrap().clone();
    spaced_board[0].push(tile0);
    for y in 1..m {
        let conn_tile = &spaced_board[0][y - 1];
        let tile = alltiles.iter().find(|(id, grid)| (0..n).all(|i| grid[i][0] == conn_tile.1[i][n - 1] && conn_tile.0 != *id)).unwrap().clone();
        spaced_board[0].push(tile);
    }
    for x in 1..m {
        for y in 0..m {
            let conn_tile = &spaced_board[x - 1][y];
            let tile = alltiles.iter().find(|(id, grid)| (0..n).all(|i| grid[0][i] == conn_tile.1[n - 1][i] && conn_tile.0 != *id)).unwrap().clone();
            spaced_board[x].push(tile);
        }
    }
    let k = (n - 2) * m;
    let mut board = vec![vec![false; k]; k];
    for x in 0..m {
        for y in 0..m {
            for sx in 0..n - 2 {
                for sy in 0..n - 2 {
                    board[x * (n - 2) + sx][y * (n - 2) + sy] = spaced_board[x][y].1[sx + 1][sy + 1];
                }
            }
        }
    }
    let monster = [(0, 1), (1, 2), (4, 2), (5, 1), (6, 1), (7, 2), (10, 2), (11, 1), (12, 1), (13, 2), (16, 2), (17, 1), (18, 1), (18, 0), (19, 1)];
    let m_width = 20;
    let m_height = 3;
    let b1 = rot(&board);
    let b2 = rot(&b1);
    let b3 = rot(&b2);
    let boards = [ flip(&board), flip(&b1), flip(&b2), flip(&b3), board, b1, b2, b3 ];
    let rot_board: Vec<Vec<bool>> = boards.into_iter().find(|b| (0..k - m_width + 1).any(|x| (0..k - m_height + 1).any(|y| monster.iter().all(|(dx, dy)| b[x + dx][y + dy])))).unwrap();
    let monster_locs = (0..k - m_width + 1).flat_map(
        |x| (0..k - m_height + 1).filter(|y| monster.iter().all(|(dx, dy)| rot_board[x + dx][*y + dy])).map(|y| (x, y)).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let cleared_board = monster_locs.iter().fold(rot_board, |mut b, (mx, my)| {
        monster.iter().for_each(|(dx, dy)| b[mx + dx][my + dy] = false);
        b
    });
    let res = cleared_board.iter().map(|col| col.iter().filter(|v| **v).count()).sum::<usize>();
    println!("{res}");
}
