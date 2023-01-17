use std::io;
use std::io::BufRead;

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let n = 1_000_000;
    let istate = {
        let mut state = vec![0; n + 1];
        state[*input.last().unwrap()] = input.len() + 1;
        for i in 0..input.len() - 1 {
            state[input[i]] = input[i + 1];
        }
        for i in input.len() + 1..n {
            state[i] = i + 1;
        }
        state[n] = input[0];
        state
    };
    let final_state = (0..10_000_000).fold((istate, input[0]), |(mut state, current), _| {
        let last_taken = (0..3).fold(current, |poi, _| { state[poi] });
        let new_next = state[last_taken];
        let moved_vals = (0..3).scan(current, |poi, _| {
            *poi = state[*poi];
            Some(*poi)
        }).collect::<Vec<_>>();
        let mut dest_val = if current == 1 { n } else { current - 1 };
        while moved_vals.iter().any(|v| *v == dest_val) {
            dest_val = if dest_val == 1 { n } else { dest_val - 1 };
        }
        let after_dest = state[dest_val];
        state[dest_val] = state[current];
        state[current] = new_next;
        state[last_taken] = after_dest;
        (state, new_next)
    }).0;
    let nxt = final_state[1];
    let nnxt = final_state[nxt];
    println!("{}", nxt * nnxt);
}
