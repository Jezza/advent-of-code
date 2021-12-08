use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 5934),
			(INPUT, 390923),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 26984457539),
			(INPUT, 1749945484935),
		],
	);
}

fn solve(input: &str, days: u16) -> u64 {
	let mut counts = [0u64; 9];

	input.split(",")
		.filter_map(|v| v.parse::<u8>().ok())
		.for_each(|v| counts[v as usize] += 1);

	for _ in 0..days {
		let v = counts[0];
		counts.copy_within(1..9, 0);
		counts[8] = v;
		counts[6] += v;
	}

	counts.into_iter()
		.sum()
}

fn part_one(input: &str) -> u64 {
	solve(input, 80)
}

fn part_two(input: &str) -> u64 {
	solve(input, 256)
}