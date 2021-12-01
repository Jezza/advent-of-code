#![feature(array_windows)]

use commons::*;

const INPUT: &str = {
	include_str!("../input/input.txt")
};

fn main() {
	measure!(part_one());
	println!("Part One: {}", part_one());

	measure!(part_two());
	println!("Part Two: {}", part_two());
}

fn part_one() -> u16 {
	INPUT.lines()
		.filter_map(|line| line.parse::<u16>().ok())
		.fold((0u16, None), |(count, previous), item| {
			let count = previous.filter(|previous| *previous < item)
				.map(|_| count + 1)
				.unwrap_or(count);

			(count, Some(item))
		}).0
}

fn part_two() -> u64 {
	let values: Vec<_> = INPUT.lines()
		.filter_map(|line| line.parse::<u16>().ok())
		.collect();

	values.array_windows::<3>()
		.fold((0, None), |(count, previous), values| {
			let sum: u16 = values.iter().sum();

			let count = previous.filter(|previous| *previous < sum)
				.map(|_| count + 1)
				.unwrap_or(count);

			(count, Some(sum))
		}).0
}
