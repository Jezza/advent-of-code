use std::collections::HashMap;
use commons::*;
use commons::utils::parse_range;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 590784),
			(INPUT, 653798),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 900),
			(INPUT, 2078985210),
		],
	);
}

fn clamp((mut left, mut right): (i32, i32)) -> Option<(i32, i32)> {
	assert!(left < right);
	// Way outside our range.
	if left > 50 || right < -50 {
		return None;
	}

	if left < -50 {
		left = -50;
	}
	if right > 50 {
		right = 50;
	}
	Some((left, right))
}

fn part_one(input: &str) -> u64 {
	let mut cube = HashMap::new();

	input.lines()
		.map(|line| {
			let (action, line) = line.split_once(" ").unwrap();
			let (x_range, line) = line.split_once(",").unwrap();
			let (y_range, z_range) = line.split_once(",").unwrap();

			(action == "on", parse_range::<i32>(&x_range[2..]), parse_range::<i32>(&y_range[2..]), parse_range::<i32>(&z_range[2..]))
		})
		.filter_map(|(activate, x_range, y_range, z_range)| {
			Some((activate, clamp(x_range)?, clamp(y_range)?, clamp(z_range)?))
		})
		.for_each(|(activate, x_range, y_range, z_range)| {
			for x in x_range.0..=x_range.1 {
				for y in y_range.0..=y_range.1 {
					for z in z_range.0..=z_range.1 {
						cube.insert((x, y, z), activate);
					}
				}
			}
		});

	cube.values()
		.filter(|v| **v)
		.count() as u64
}

fn part_two(input: &str) -> u64 {
	0u64
}
