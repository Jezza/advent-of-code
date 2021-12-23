#![feature(int_abs_diff)]

use commons::*;
use commons::utils::parse_range as pr;

fn main() {
	const TEST_0: &str = include_str!("../input/test-0.txt");
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const TEST_2: &str = include_str!("../input/test-2.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_0, 39),     //   1,620 ns/iter (+/- 274)
			(TEST_1, 590784), // 779,918 ns/iter (+/- 156724)
			(INPUT, 653798),  // 310,575 ns/iter (+/- 50272)
		],
	);
	aoc(part_two,
		vec![
			(TEST_2, 2758514936282235), //   232,052 ns/iter (+/- 13014)
			(INPUT, 1257350313518866),  // 6,378,460 ns/iter (+/- 313962)
		],
	);
}

type Range = (i32, i32);
type Cube = (Range, Range, Range);
type Cuboid = (bool, Range, Range, Range);

fn parse_input(input: &str) -> impl Iterator<Item = Cuboid> + '_ {
	input.lines()
		.map(|line| {
			let (action, x_range, y_range, z_range) = split!(line, " ", ",", ",");
			(action == "on", pr::<i32>(&x_range[2..]), pr::<i32>(&y_range[2..]), pr::<i32>(&z_range[2..]))
		})
}

fn intersect_range(left: &Range, right: &Range) -> Option<Range> {
	let (left_lower, left_upper) = left;
	let (right_lower, right_upper) = right;

	let lower = left_lower.max(right_lower);
	let upper = left_upper.min(right_upper);

	if lower <= upper {
		Some((*lower, *upper))
	} else {
		None
	}
}

fn intersect(left: &Cuboid, right: &Cuboid) -> Option<Cube> {
	let (_, left_x, left_y, left_z) = left;
	let (_, right_x, right_y, right_z) = right;

	let intersect_x = intersect_range(left_x, right_x)?;
	let intersect_y = intersect_range(left_y, right_y)?;
	let intersect_z = intersect_range(left_z, right_z)?;

	Some((intersect_x, intersect_y, intersect_z))
}

fn clamp_range((mut lower, mut upper): Range) -> Option<(i32, i32)> {
	assert!(lower <= upper);
	// Way outside our range.
	if lower > 50 || upper < -50 {
		return None;
	}

	if lower < -50 {
		lower = -50;
	}
	if upper > 50 {
		upper = 50;
	}
	Some((lower, upper))
}

fn clamp((activate, x, y, z): Cuboid) -> Option<Cuboid> {
	let x = clamp_range(x)?;
	let y = clamp_range(y)?;
	let z = clamp_range(z)?;

	Some((activate, x, y, z))
}

fn solve(cuboids: Vec<Cuboid>) -> isize {
	let mut all = vec![];
	let mut scratch_space = vec![];

	for cuboid in cuboids {
		for other in &all {
			if let Some((x, y, z)) = intersect(&cuboid, other) {
				scratch_space.push((!other.0, x, y, z));
			}
		}

		if cuboid.0 {
			scratch_space.push(cuboid);
		}

		all.extend(scratch_space.drain(..));
	}

	all.into_iter()
		.map(|(activate, x, y, z)| {
			(if activate { 1 } else { -1 }) *
				((x.1 + 1 - x.0) as isize) *
				((y.1 + 1 - y.0) as isize) *
				((z.1 + 1 - z.0) as isize)
		})
		.sum::<isize>() as isize
}

fn part_one(input: &str) -> isize {
	let cuboids: Vec<_> = parse_input(input)
		.filter_map(clamp)
		.collect();

	solve(cuboids)
}

fn part_two(input: &str) -> u64 {
	let cuboids = parse_input(input)
		.collect();

	solve(cuboids) as u64
}
