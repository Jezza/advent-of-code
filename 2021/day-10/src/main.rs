#![feature(let_else)]

use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 26397),
			(INPUT, 389589),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 288957),
			(INPUT, 1190420163),
		],
	);
}

fn stack(line: &str) -> Result<Vec<u8>, u8> {
	let mut stack = vec![];
	for b in line.as_bytes() {
		let (expect, got) = match b {
			b'[' | b'(' | b'{' | b'<' => {
				stack.push(*b);
				continue;
			}
			b']' => (b'[', stack.pop().unwrap()),
			b')' => (b'(', stack.pop().unwrap()),
			b'}' => (b'{', stack.pop().unwrap()),
			b'>' => (b'<', stack.pop().unwrap()),
			_ => panic!("Unexpected {}", *b as char),
		};

		if expect != got {
			return Err(expect);
		}
	}
	Ok(stack)
}

fn part_one(input: &str) -> u64 {
	input.lines()
		.filter_map(|line| stack(line).err())
		.map(|value| match value {
			b'(' => 3,
			b'[' => 57,
			b'{' => 1197,
			b'<' => 25137,
			_ => panic!("Unexpected {}", value as char),
		})
		.sum()
}

fn part_two(input: &str) -> u64 {
	let mut scores = input.lines()
		.filter_map(|line| stack(line).ok())
		.map(|mut stack| {
			stack.reverse();
			stack.into_iter()
				.fold(0u64, |score, value| {
					score * 5 + match value {
						b'(' => 1,
						b'[' => 2,
						b'{' => 3,
						b'<' => 4,
						_ => panic!("Unexpected {}", value as char),
					}
				})
		})
		.collect::<Vec<u64>>();

	let index = scores.len() / 2;
	*scores.select_nth_unstable(index).1
}
