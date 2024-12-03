#![feature(let_chains)]

use std::num::ParseIntError;

fn main() {
    let input = include_str!("../input.txt");
    let pattern = "mul(";

    let mut multiplications = Vec::new();
    let mut states = Vec::new();

    for (n, _) in input.match_indices(pattern) {
        let sub = &input[n + pattern.len()..];
        let a = sub.find(',').or(Some(0));
        let b = sub.find(')').or(Some(0));

        let (a_n, b_n) = if let Some(a) = a
            && let Some(b) = b
        {
            (a, b)
        } else {
            continue;
        };

        let res = || -> Result<(u32, u32), ParseIntError> {
            let a = sub[0..a_n].parse::<u32>()?;
            let b = sub[a_n + 1..b_n].parse::<u32>()?;

            Ok((a, b))
        }();

        if let Ok((a, b)) = res {
            multiplications.push((n, a * b));
        }
    }

    let pattern = "do";
    for (n, _) in input.match_indices(pattern) {
        let sub = &input[n..];

        let pattern = "do()";
        if &sub[..pattern.len()] == pattern {
            states.push((n, true));
        }

        let pattern = "don't()";
        if &sub[..pattern.len()] == pattern {
            states.push((n, false));
        }
    }

    let mut states = states
        .windows(2)
        .map(|x| (x[0].1, x[0].0..x[1].0))
        .collect::<Vec<_>>();
    
    if let Some((last_state, last_pos)) = states.last() {
        states.push((*last_state, last_pos.end..input.len()));
    }
    if let Some((_, first_pos)) = states.first() {
        states.push((true, 0..first_pos.start));
    }

    println!("{states:?}");

    let mut sum = 0;
    for mul in multiplications {
        let mul_ = mul.clone();
        let enabled = || -> bool {
            for state in &states {
                if state.1.contains(&mul_.0) {
                    return state.0;
                }
            }

            println!("NOT FOUND!??? {mul_:?}");
            return false;
        }();

        if enabled {
            sum += mul.1;
        }
    }
    
    println!("{sum}");
}
