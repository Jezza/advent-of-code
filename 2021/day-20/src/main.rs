use std::collections::HashSet;
use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 35),
			(INPUT, 5349),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 3351),
			(INPUT, 15806),
		],
	);
}

fn part_one(input: &str) -> u64 {
	solve(input, 2)
}

fn part_two(input: &str) -> u64 {
	solve(input, 50)
}

type Unit = isize;
type Point = (Unit, Unit);

fn solve(input: &str, steps: usize) -> u64 {
	let (lookup, grid) = input.split_once("\n\n").unwrap();

	let mut points = grid::parse_grid(
		grid,
		str::lines,
		|line| line.split(""),
		|width, height| HashSet::with_capacity(width * height),
		|acc, x, y, segment| {
			if segment == "#" {
				acc.insert((x as Unit, y as Unit));
			}
		},
	);

	let mut offsets: utils::Offsets<i8, 2> = utils::gen_offsets::<i8, 2>();
	offsets.reverse();

	let mut default_state = false;
	let toggle = lookup.as_bytes()[0] == b'#';

	for _ in 0..steps {
		let bounds @ (min_x, max_x, min_y, max_y) = bounds(&points);

		points = (min_y..=max_y)
			.flat_map(move |y| (min_x..=max_x).map(move |x| (x, y)))
			.filter(|(x, y)| {
				let index = offsets.iter()
					.map(|offset| (x + offset[0] as Unit, y + offset[1] as Unit))
					.enumerate()
					.fold(0, |acc, (i, point @ (x, y))| {
						let alive = if x > min_x && x < max_x && y > min_y && y < max_y {
							points.contains(&point)
						} else {
							default_state
						};
						acc | ((alive as i16) << i)
					});

				lookup.as_bytes()[index as usize] == b'#'
			}).collect();

		if toggle {
			default_state = !default_state;
		}

		println!("{}x{}", bounds.1 - bounds.0, bounds.3 - bounds.2);
	}

	points.len() as u64
}

fn bounds(points: &HashSet<Point>) -> (isize, isize, isize, isize) {
	let mut min_x = isize::MAX;
	let mut max_x = isize::MIN;
	let mut min_y = isize::MAX;
	let mut max_y = isize::MIN;

	for (x, y) in points {
		let x = *x;
		let y = *y;

		min_x = min_x.min(x);
		max_x = max_x.max(x);
		min_y = min_y.min(y);
		max_y = max_y.max(y);
	}

	(min_x - 1, max_x + 1, min_y - 1, max_y + 1)
}