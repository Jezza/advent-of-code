#![feature(int_abs_diff)]

use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 37),
			(INPUT, 343468),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 168),
			(INPUT, 96086265),
		],
	);
}

fn part_one(input: &str) -> u32 {
	let mut positions = parse!(input, i32, ",");

	positions.sort_unstable();

	let median = positions[(positions.len() / 2)];
	positions.into_iter()
		.map(|v| v.abs_diff(median) as u32)
		.sum()
}

fn part_two(input: &str) -> u32 {
	let positions = parse!(input, i32, ",");

	let average = positions.iter().sum::<i32>() / positions.len() as i32;
	let check = |average| {
		positions.iter()
			.map(|v| v.abs_diff(average))
			.map(|v| v * (v + 1) / 2)
			.sum::<u32>()
	};
	check(average).min(check(average + 1))
}
