#![feature(array_windows)]
use commons::measure;

fn main() {
	let mut input = parse_input();
	input.sort_unstable();
	let input = &input;

	println!("Part One: {}", measure!(part_one(input)));
	println!("Part Two: {}", measure!(part_two(input)));
	// println!("Part One: {}", part_one(input));
	// println!("Part Two: {}", part_two(input));
}

fn part_one(input: &[usize]) -> usize {
	// let mut input = parse_input();
	// input.sort_unstable();

	let (ones, threes) = input.array_windows()
		.map(|[left, right]| right - left)
		.fold((1, 1), |(left, right), val| {
			if val == 1 {
				(left + 1, right)
			} else {
				(left, right + 1)
			}
		});

	ones * threes
}

fn part_two(input: &[usize]) -> usize {
	// let mut input = parse_input();
	// input.sort_unstable();

	input.array_windows()
		.map(|[left, right]| right - left)
		.fold((0, 0, 1), |(left, middle, right), diff| match diff {
			1 => (middle, right, left + middle + right),
			3 => (0, 0, right),
			_ => unreachable!(),
		})
		.2
}

fn parse_input() -> Box<[usize]> {
	let mut data: Vec<usize> = include_str!("../input/input.txt")
		.lines()
		.map(|line| line.parse::<usize>().unwrap())
		.collect();

	data.push(0);

	data.into_boxed_slice()
}
