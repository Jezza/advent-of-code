#![feature(array_windows)]

use itertools::Itertools;

use commons::{measure, time};

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part One: {}", measure!(part_one()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
}

fn run_task(n: usize) -> usize {
	let input = INPUT.bytes()
		.map(|c| c - b'0')
		.collect::<Box<[u8]>>();

	std::iter::successors(Some(input), |previous: &Box<[u8]>| {
		let next = previous.iter()
			.group_by(|&c| *c)
			.into_iter()
			.flat_map(|(k, group)| {
				format!("{}{}", group.count() as u8, k)
					.into_bytes()
					.into_iter()
					.map(|c| c - b'0')
			})
			.collect();
		Some(next)
	}).skip(n)
		.next()
		.unwrap()
		.len()
}

fn part_one() -> usize {
	run_task(40)
}

fn part_two() -> usize {
	run_task(50)
}
