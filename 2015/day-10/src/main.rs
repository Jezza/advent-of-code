#![feature(array_windows)]

use itertools::Itertools;

use helper::measure;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	let numbers = INPUT.bytes()
		.map(|c| c - b'0')
		.collect::<Box<[u8]>>();


	// let mut it = std::iter::successors(Some(numbers), |previous: &Box<[u8]>| {
	// 	Some(numbers(&previous))
	// });

	// fn numbers(input: &[u8]) -> Box<[u8]> {
	// 	todo!()
	// }

	// let mut numbers = vec![];

	// let mut numbers =

	// for i in 0..40 {
	//
	// }


	// let mut output = String::new();
	// for (k, group) in &group {
	// 	output.push_str(&format!("{}{}", group.count(), k));
	// }

	// println!("{}", output);
	//
	// output.len()
	0
}

fn part_two() -> usize {
	0
}
