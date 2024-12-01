use std::cmp;

pub const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Solving Advent of Code 2024 Day 1 Puzzle 1.");

    // This is quick enough implementation, even though this could use optimization.
    let mut first = Vec::new();
    let mut last = Vec::new();
    for line in INPUT.split('\n') {
        let (a, b) = line.split_once(char::is_whitespace).unwrap();
        let b = b.trim_start();

        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();

        first.push(a);
        last.push(b);
    }

    first.sort();
    last.sort();

    let mut sum = 0u64;
    for (a, b) in first.iter().zip(last.iter()) {
        sum += (cmp::max(a, b) - cmp::min(a, b)) as u64;
    }

    println!("{sum}");
}
