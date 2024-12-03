#![feature(let_chains)]

use std::num::ParseIntError;

fn main() {
    let input = include_str!("../input.txt");
    let pattern = "mul(";

    // Yes, this could be a variable of all sums, but this is just a leftover from part 2 as I accidentally made the part 2 code be part 1 code.
    // So this is my part 2 implementation stripped down.
    let mut multiplications = Vec::new();

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

    println!("{:?}", multiplications.iter().map(|x| x.1).sum::<u32>());
}
