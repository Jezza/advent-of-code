use std::collections::HashMap;

use helper::measure;

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn stupid_game(nth: usize) -> usize {
	let starting = include_str!("../input/input.txt")
		.split(',')
		.filter_map(|v| v.parse::<usize>().ok())
		.collect::<Vec<usize>>();

	let starting_point = starting.len();
	let last = starting.last().copied().unwrap();

	let mut last_spoken = starting.into_iter()
		.enumerate()
		.map(|(i, v)| (v, i + 1))
		.collect::<HashMap<_, _>>();

	(starting_point..nth)
		.fold(last, |last, i| i - last_spoken.insert(last, i).unwrap_or(i))
}

fn part_one() -> usize {
	stupid_game(2020)
}

fn part_two() -> usize {
	stupid_game(30_000_000)
}
