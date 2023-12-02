#![feature(str_split_once)]
#![feature(bool_to_option)]

use std::ops::Not;

use commons::measure;

fn main() {
    println!("Part One: {}", measure!(part_one()));
    println!("Part Two: {}", measure!(part_two()));
    // println!("Part One: {}", part_one());
    // println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
    find_target(&parse_input())
}

fn part_two() -> usize {
    let numbers = parse_input();
    let target = find_target(&numbers);

    let mut start = 0;
    let mut end = 2;

    loop {
        let sum: usize = numbers[start..end].iter().sum();

        if sum > target {
            start += 1;
        } else if sum < target {
            end += 1;
        } else {
            break;
        }
    }

    let min = numbers[start..end].iter().min().unwrap();
    let max = numbers[start..end].iter().max().unwrap();
    min + max
}

fn find_target(numbers: &[usize]) -> usize {
    numbers
        .windows(26)
        .find_map(|items| {
            let (last, rest) = items.split_last()?;
            rest.iter()
                .map(|v| last.saturating_sub(*v))
                .any(|v| rest.contains(&v))
                .not()
                .then_some(*last)
        })
        .unwrap() as usize
}

fn parse_input() -> Box<[usize]> {
    include_str!("../input/input.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}
