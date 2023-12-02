#![feature(array_windows)]

use commons::*;

const INPUT: &str = { include_str!("../input/input.txt") };

fn main() {
    println!("Part One: {} == {}", measure!(part_one()), 1301);
    println!("Part Two: {} == {}", measure!(part_two()), 1346);
}

fn read_input() -> impl Iterator<Item = u16> {
    INPUT.lines().filter_map(|line| line.parse::<u16>().ok())
}

fn solve_input(input: impl Iterator<Item = u16>) -> u16 {
    input
        .fold((0, None), |(count, previous), item| {
            let count = previous
                .filter(|previous| *previous < item)
                .map(|_| count + 1)
                .unwrap_or(count);

            (count, Some(item))
        })
        .0
}

fn part_one() -> u16 {
    solve_input(read_input())
}

fn part_two() -> u16 {
    let values: Vec<_> = read_input().collect();

    let transformed = values
        .array_windows::<3>()
        .map(|values| values.iter().sum::<u16>());

    solve_input(transformed)
}
