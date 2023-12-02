#![feature(array_chunks)]

use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 157), (INPUT, 8240)]);
    aoc(part_two, vec![(TEST_1, 70), (INPUT, 2587)]);
}

fn priority(value: u8) -> u64 {
    match value {
        b'a'..=b'z' => (value - b'a') as u64 + 1,
        b'A'..=b'Z' => (value - b'A') as u64 + 27,
        _ => panic!("at the disco"),
    }
}

fn as_bits(input: &str) -> u64 {
    input.bytes().fold(0, |acc, b| acc | 1 << priority(b))
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line: &str| {
            let (left, right) = line.split_at(line.len() / 2);
            let bits = as_bits(left) & as_bits(right);
            bits.trailing_zeros() as u64
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    let bits: Vec<_> = input.lines().map(|line: &str| as_bits(line)).collect();

    bits.array_chunks()
        .map(|[first, second, third]: &[u64; 3]| {
            let bits = first & second & third;
            bits.trailing_zeros() as u64
        })
        .sum()
}
