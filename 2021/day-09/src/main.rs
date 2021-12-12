use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 15),
			(INPUT, 465),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 1134),
			(INPUT, 1269555),
		],
	);
}

#[derive(Clone)]
struct Grid {
	width: usize,
	height: usize,
	values: [u8; 100 * 100],
}

fn parse_input(input: &str) -> Grid {
	grid::parse_grid(
		input,
		str::lines,
		|line| line.split(""),
		|width, height| Grid {
			width,
			height,
			values: [0u8; 100 * 100],
		},
		|grid, x, y, segment| {
			let index = y * grid.width + x;
			grid.values[index] = segment.parse::<u8>().unwrap();
		},
	)
}

fn find_points(grid: &Grid) -> Vec<(usize, u8)> {
	let width = grid.width;
	let height = grid.height;
	let grid = &grid.values;

	let mut points = vec![];

	for i in 0..width * height {
		let v = grid[i];
		if v == 9 {
			continue;
		}

		let x = i % width;
		let y = i / width;

		let smaller = false
			|| x != 0 && grid[i - 1] < v
			|| x != width - 1 && grid[i + 1] < v
			|| y != 0 && grid[i - width] < v
			|| y != height - 1 && grid[i + width] < v;

		if !smaller {
			points.push((i, v));
		}
	}

	points
}

fn part_one(input: &str) -> u64 {
	let grid = parse_input(input);
	let points = find_points(&grid);

	let len = points.len() as u64;
	len + points.into_iter()
		.map(|(_, v)| v as u64)
		.sum::<u64>()
}

fn part_two(input: &str) -> u64 {
	let grid = parse_input(input);
	let points = find_points(&grid);

	let width = grid.width;
	let height = grid.height;

	let mut sizes = vec![];

	let mut seen = vec![];
	let mut stack = vec![];

	for (i, _) in points {
		stack.clear();
		stack.push(i);

		seen.clear();

		while let Some(i) = stack.pop() {
			let x = i % width;
			let y = i / width;

			if x != 0 {
				let i = i - 1;
				if !seen.contains(&i) && grid.values[i] != 9 {
					seen.push(i);
					stack.push(i);
				}
			}
			if x != width - 1 {
				let i = i + 1;
				if !seen.contains(&i) && grid.values[i] != 9 {
					seen.push(i);
					stack.push(i);
				}
			}
			if y != 0 {
				let i = i - width;
				if !seen.contains(&i) && grid.values[i] != 9 {
					seen.push(i);
					stack.push(i);
				}
			}
			if y != height - 1 {
				let i = i + width;
				if !seen.contains(&i) && grid.values[i] != 9 {
					seen.push(i);
					stack.push(i);
				}
			}
		}

		sizes.push(seen.len());
	}

	let index = sizes.len() - 4;
	let slice = sizes.select_nth_unstable(index).2;
	(slice[0] * slice[1] * slice[2]) as u64
}
