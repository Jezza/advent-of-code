use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 0),
			(INPUT, 0),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 0),
			(INPUT, 0),
		],
	);
}

fn part_one(input: &str) -> u64 {
	0u64
}

fn part_two(input: &str) -> u64 {
	0u64
}
