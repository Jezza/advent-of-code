use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 1656),
			(INPUT, 1679),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 195),
			(INPUT, 519),
		],
	);
}

type Grid = grid::Grid<u8, 10, 10>;

fn read_input(input: &str) -> Grid {
	grid::parse_grid(
		input,
		str::lines,
		|line| line.split(""),
		|width, height| Grid::from_values(width, height, [0; 100]),
		|grid, x, y, segment| *grid.get_mut(x, y) = segment.parse().unwrap(),
	)
}

const OFFSETS: [(i32, i32); 8] = [
	(-1, -1), (0, -1), (1, -1),
	(-1, 0), (1, 0),
	(-1, 1), (0, 1), (1, 1),
];

fn step(grid: &mut Grid) -> usize {
	let mut flashes: Vec<_> = grid.iter_pos_tuples().collect();
	let mut count = 0;

	// let mut reset = vec![];

	while let Some((x, y)) = flashes.pop() {
		let value = grid.get_mut(x, y);
		let new_value = *value + 1;
		*value = new_value;
		if new_value == 10 {
			count += 1;
			// reset.push(x + y * grid.width);

			for (offset_x, offset_y) in OFFSETS {
				let x = x as i32 + offset_x;
				let y = y as i32 + offset_y;

				if x >= 0 && y >= 0 && x < grid.width as i32 && y < grid.height as i32 {
					flashes.push((x as usize, y as usize));
				}
			}
		}
	}

	// for value in &mut grid.values {
	// 	if *value > 9 {
	// 		*value = 0;
	// 	}
	// }

	// for i in reset {
	// 	grid.values[i] = 0;
	// }

	for pos in grid.iter_pos() {
		let v = grid.values[pos];
		if v > 9 {
			grid.values[pos] = 0;
		}
	}
	count
}

fn part_one(input: &str) -> usize {
	let mut grid = read_input(input);

	(0..100)
		.map(|_| step(&mut grid))
		.sum()
}

fn part_two(input: &str) -> u64 {
	let mut grid = read_input(input);

	(0..).find(|_| grid.values.len() == step(&mut grid))
		.unwrap_or_default() + 1
}
