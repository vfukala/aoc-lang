use std::io;
use std::io::BufRead;

fn main() {
    let istate = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    let n = istate.len() as i32;
    let final_state: Vec<i32> = (0..100).fold(istate, |state: Vec<i32>, _| {
        let sel = state.iter().skip(1).take(3).collect::<Vec<_>>();
        let mut dest_val = if state[0] == 1 { n } else { state[0] - 1 };
        while sel.iter().any(|v| **v == dest_val) {
            dest_val = if dest_val == 1 { n } else { dest_val - 1 };
        }
        let dest_pos = state.iter().position(|v| *v == dest_val).unwrap();
        state.iter().skip(4).take(dest_pos - 3).chain(sel.iter().copied()).chain(state.iter().skip(dest_pos + 1)).chain(state.iter().take(1)).copied().collect::<Vec<i32>>()
    });
    let one_pos = final_state.iter().position(|v| *v == 1).unwrap();
    let aligned_final = final_state.iter().skip(one_pos + 1).chain(final_state.iter().take(one_pos));
    let res = aligned_final.map(|v| char::from_digit(*v as u32, 10).unwrap()).collect::<String>();
    println!("{res}");
}
