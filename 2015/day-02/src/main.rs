// use std::str::pattern::Pattern;
use commons::*;
use commons::export::itertools::Itertools;

const INPUT: &str = {
	"2x3x4"
	// include_str!("../input/input.txt")
};

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part Two: {}", part_two());
}

fn part_one() -> u32 {
	INPUT.lines()
		.map(|line| {
			let areas = line.splitn(3, "x")
				.filter_map(|value| value.parse::<u32>().ok())
				.combinations(2)
				.map(|x| x.iter().copied().product::<u32>())
				.collect_vec();

			let sum: u32 = areas.iter()
				.copied()
				.map(|x| x * 2)
				.sum();

			let min: u32 = areas.iter()
				.copied()
				.min()
				.unwrap_or_default();

			sum + min
		})
		.sum()
}

fn part_two() -> u64 {
	INPUT.lines()
		.map(|line| {
			let areas = line.splitn(3, "x")
				.filter_map(|value| value.parse::<u32>().ok())
				.combinations(2)
				.min();

			println!("{:#?}", areas);

			// let sum: u32 = areas.iter()
			// 	.copied()
			// 	.map(|x| x * 2)
			// 	.sum();
			//
			// let min: u32 = areas.iter()
			// 	.copied()
			// 	.min()
			// 	.unwrap_or_default();

			0
		})
		.sum()
}


// fn split<'a, P: Pattern<'a>, const N: usize>(input: &'a str, pattern: P) -> [&'a str; N] {
// 	input.splitn()
// }
