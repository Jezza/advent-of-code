#![feature(array_windows)]
#![feature(let_else)]

use std::collections::HashMap;
use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 1588), // 14155 ns/iter (+/- 4161)
			(INPUT, 2602),  // 71234 ns/iter (+/- 24887)
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 2188189693529), //  50504 ns/iter (+/- 1287)
			(INPUT, 2942885922173),  // 260260 ns/iter (+/- 8548)
		],
	);
}

type Pairs = HashMap<[u8; 2], u64>;

fn solve(input: &str, steps: u32) -> u64 {
	let (template, actions) = input.split_once("\n\n").unwrap();

	let actions = actions.lines()
		.filter_map(|line| line.split_once(" -> "))
		.map(|(left, right)| ([left.as_bytes()[0], left.as_bytes()[1]], right.as_bytes()))
		.collect::<HashMap<_, _>>();

	let mut pairs: Pairs = HashMap::new();
	template.as_bytes()
		.array_windows()
		.copied()
		.for_each(|pair: [u8; 2]| *pairs.entry(pair).or_insert(0) += 1);

	let mut scratch_space: Pairs = HashMap::new();

	for _ in 0..steps {
		for (key, value) in pairs.drain() {
			let Some(addition) = actions.get(&key) else {
				continue;
			};

			let left_pair = [key[0], addition[0]];
			let right_pair = [addition[0], key[1]];

			*scratch_space.entry(left_pair).or_insert(0) += value;
			*scratch_space.entry(right_pair).or_insert(0) += value;
		}

		std::mem::swap(&mut pairs, &mut scratch_space);
	}

	let mut frequencies = [0u64; 26];
	for ([left, _], value) in pairs {
		frequencies[(left - b'A') as usize] += value;
	}

	if let Some(index) = template.as_bytes().last() {
		frequencies[(*index - b'A') as usize] += 1;
	}

	let mut min = u64::MAX;
	let mut max = u64::MIN;

	for frequency in frequencies {
		if frequency == 0 {
			continue;
		}
		min = min.min(frequency);
		max = max.max(frequency);
	}

	max - min
}

fn part_one(input: &str) -> u64 {
	solve(input, 10)
}

fn part_two(input: &str) -> u64 {
	solve(input, 40)
}
