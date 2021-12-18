use std::ops::RangeInclusive;

use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 45),
			(INPUT, 6555),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 112),
			(INPUT, 4973),
		],
	);
}

type Range = RangeInclusive<i32>;

fn parse_input(input: &str) -> (Range, Range) {
	let (x_range, y_range) = &input["target area: ".len()..].split_once(", ").unwrap();

	fn parse(input: &str) -> Range {
		let (start, end) = &input[2..].split_once("..").unwrap();

		let start = start.parse().unwrap();
		let end = end.parse().unwrap();

		Range::new(start, end)
	}

	(parse(x_range), parse(y_range))
}

fn part_one(input: &str) -> u64 {
	let (_, y_range) = parse_input(input);

	// Whatever value we pick for our y velocity, it'll always return to 0 and reverse.
	// Meaning, we have a "perfect" parabola.

	// We go up and back down at the same speed, meaning we'll hit the zero mark at the same speed as when we left.
	// So if we fire it at 10, we'll get back to the ground level at -10. (Because we're going in the opposite direction)
	// So, we want our last step to take us straight to the lowest level we _can_ go.

	// As my example was showing, if we threw it at 10, then we'd be back at ground level with -10, but the next step would be -11.
	// In other words, we'd over shoot.

	// So, we need to work backwards.
	// Taking the value from the bottom row, we can work out our "final" velocity.

	// The example has -10, so our velocity needs to hit -10 as we're _leaving_ the ground level. (As in, we arrive at the ground level with -9).
	// Which means our velocity is `abs(bottom_row) - 1`.

	// And to calculate the max height, good 'ol gauss is the way to go: (n * (n + 1)) / 2

	// Our `n` is just our velocity, plug it in, and boom.

	let velocity = y_range.start().abs() - 1;
	((velocity * (velocity + 1)) / 2) as u64
}

fn part_two(input: &str) -> u64 {
	// We can split x component and y component, because they don't effect each other.

	fn distance(velocity: i32) -> i32 {
		(velocity * (velocity + 1)) / 2
	}

	let (x_range, y_range) = parse_input(input);

	let max_y_velocity = y_range.start().abs() - 1;

	// let mut min_x = i32::MAX;
	// let mut max_x = i32::MIN;
	// for x in 0..=*x_range.end() {
	// 	let distance = distance(x);
	// 	if x_range.contains(&x) || x_range.contains(&distance) {
	// 		min_x = min_x.min(x);
	// 		max_x = max_x.max(x);
	// 	}
	// }

	let mut count = 0;
	for x in 0..=*x_range.end() {
		for y in *y_range.start()..=max_y_velocity {
			count += shoot(x, y, x_range.clone(), y_range.clone()) as i32;
		}
	}

	count as u64
}

fn shoot(mut vel_x: i32, mut vel_y: i32, x_range: Range, y_range: Range) -> bool {
	let (mut x, mut y) = (0, 0);

	loop {
		x += vel_x;
		y += vel_y;

		vel_x -= vel_x.signum();
		vel_y -= 1;

		if y < *y_range.start() || x > *x_range.end() || (vel_x == 0 && x < *x_range.start()) {
			return false;
		}

		if x_range.contains(&x) && y_range.contains(&y) {
			return true;
		}
	}
}
