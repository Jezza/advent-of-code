use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 58),
			(INPUT, 360),
		],
	);
}

type Position = (usize, usize);
type Grid = grid::Grid<u8, 139, 137>;

fn step<const X: usize, const Y: usize, const C: u8>(
	grid: &mut Grid,
	scratch_space: &mut Vec<Position>,
) -> bool {

	let mut changed = false;

	for pos @ (x, y) in grid.iter_pos_tuples() {
		if *grid.get(x, y) != C {
			continue;
		}
		let new_x = if x + X >= grid.width  { 0 } else { x + X };
		let new_y = if y + Y >= grid.height { 0 } else { y + Y };

		if *grid.get(new_x, new_y) != b'.' {
			continue;
		}

		changed = true;

		*grid.get_mut(x, y) = b'#';
		*grid.get_mut(new_x, new_y) = b'X';

		scratch_space.push(pos);
	}

	for (x, y) in scratch_space.drain(..) {
		*grid.get_mut(x, y) = b'.';

		let new_x = if x + X >= grid.width  { 0 } else { x + X };
		let new_y = if y + Y >= grid.height { 0 } else { y + Y };
		*grid.get_mut(new_x, new_y) = C;
	}

	changed
}


fn part_one(input: &str) -> u64 {
	let mut grid = grid::parse_grid(
		input,
		str::lines,
		|line| line.split(""),
		|width, height| Grid::from_values(width, height, [b'.'; 139 * 137]),
		|grid, x, y, segment| {
			*grid.get_mut(x, y) = segment.as_bytes()[0];
		},
	);

	let mut scratch_space = vec![];

	let mut count = 0;
	while step::<1, 0, b'>'>(&mut grid, &mut scratch_space) | step::<0, 1, b'v'>(&mut grid, &mut scratch_space) {
		count += 1;
	}

	count + 1
}
