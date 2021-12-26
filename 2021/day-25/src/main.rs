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
	let width = grid.width;
	let area = grid.width * grid.height;

	for i in grid.iter_pos() {
		if *grid.get_raw(i) != C {
			continue;
		}

		let new_i = if C == b'>' {
			let y = i / width;
			let x = i % width;
			y * width + ((x + 1) % width)
		} else {
			(i + width) % area
		};


		if *grid.get_raw(new_i) != b'.' {
			continue;
		}

		*grid.get_raw_mut(new_i) = b'X';
		scratch_space.push((i, new_i));
	}

	let changed = !scratch_space.is_empty();

	for (i, new_i) in scratch_space.drain(..) {
		*grid.get_raw_mut(i) = b'.';
		*grid.get_raw_mut(new_i) = C;
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

	let mut scratch_space = Vec::with_capacity(grid.width * grid.height);

	let mut count = 0;
	while step::<1, 0, b'>'>(&mut grid, &mut scratch_space) | step::<0, 1, b'v'>(&mut grid, &mut scratch_space) {
		count += 1;
	}

	count + 1
}
