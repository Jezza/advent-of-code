#![feature(test)]

extern crate test;

use std::collections::HashSet;

macro_rules! measure {
    ($expr:expr) => {{
        let stats = test::bench::iter(&mut || $expr);
        let median = stats.median as usize;
        let deviation = (stats.max - stats.min) as usize;
        println!(
            "test {:<36}\tbench:\t{:>11} ns/iter (+/- {})",
            stringify!($expr),
            median,
            deviation
        );
        $expr
    }};
}

fn main() {
    println!("Part One: {}", measure!(part_one()));
    println!("Part One: {}", measure!(part_one_bits()));
    println!("Part Two: {}", measure!(part_two()));
    println!("Part Two: {}", measure!(part_two_bits()));
    // println!("Part One: {}", part_one());
    // println!("Part One: {}", part_one_bits());
    // println!("Part One: {}", part_two());
    // println!("Part One: {}", part_two_bits());
}

fn part_one() -> usize {
    process_input(|group| {
        group
            .as_bytes()
            .iter()
            .filter(|&c| *c != b'\n')
            .copied()
            .collect::<HashSet<u8>>()
            .len()
    })
}

fn part_one_bits() -> usize {
    process_input(|group| {
        group
            .split_ascii_whitespace()
            .flat_map(|answer| answer.as_bytes())
            .map(|c| 1u32 << (*c - b'a'))
            .fold(0, |acc, c| acc | c)
            .count_ones() as usize
    })
}

fn part_two() -> usize {
    process_input(|group: &str| {
        group
            .split('\n')
            .map(|answer| answer.as_bytes().iter().copied().collect::<HashSet<u8>>())
            .reduce(|left: HashSet<u8>, right: HashSet<u8>| &left & &right)
            .unwrap()
            .len()
    })
}

fn part_two_bits() -> usize {
    process_input(|group| {
        group
            .split_ascii_whitespace()
            .map(|answer| {
                answer
                    .as_bytes()
                    .iter()
                    .map(|c| 1u32 << (*c - b'a'))
                    .fold(0, |acc, c| acc | c)
            })
            .fold(u32::MAX, |acc, c| acc & c)
            .count_ones() as usize
    })
}

fn process_input(handler: impl Fn(&str) -> usize) -> usize {
    include_str!("../input/input.txt")
        .split("\n\n")
        .map(handler)
        .sum()
}
