#![no_std]

fn sums(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split("\n\n").map(|group| {
        group
            .lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .sum()
    })
}

pub fn part_one(_: &mut [u8], input: &str) -> u64 {
    sums(input).max().unwrap()
}

pub fn part_two(_mem: &mut [u8], _input: &str) -> u64 {
    // sums(input)
    0
}
