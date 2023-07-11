use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    let state_lines = lines.clone()
        .into_iter()
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let tower_count = state_lines.last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let tower_height = state_lines.len() - 1;
    let crates = state_lines.into_iter()
        .take(tower_height)
        .map(|l| l.chars()
             .skip(1)
             .step_by(4)
             .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let init_state = crates.into_iter()
        .rfold(
            vec![Vec::new(); tower_count as usize],
            |state, line| state.into_iter()
                .zip(line.into_iter())
                .map(|(mut ve, c)| { if c != ' ' { ve.push(c); } ve })
                .collect::<Vec<_>>()
                );
    let instructions = lines.into_iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums = l.split(' ')
                .skip(1)
                .step_by(2)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (nums[0], nums[1], nums[2])
        })
        .collect::<Vec<_>>();
    let final_state = instructions.into_iter()
        .fold(init_state, |mut state, (am, from, to)| {
            for _ in 0..am {
                let val = state[from - 1].pop().unwrap();
                state[to - 1].push(val);
            }
            state
        });
    let result = final_state.into_iter()
        .map(|tower| *tower.last().unwrap())
        .collect::<String>();
    println!("{result}");
}
