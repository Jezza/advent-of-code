#![feature(array_windows)]
#![feature(box_into_boxed_slice)]

use helper::measure;

fn main() {
	// let mut input = parse_input();
	// input.sort_unstable();
	// let input = &input;

	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	fn tick(current: &Grid, next: &mut Grid) {
		let offsets: [isize; 8] = [
			-(current.width as isize) - 1,
			-(current.width as isize),
			-(current.width as isize) + 1,
			-1,
			1,
			(current.width as isize) - 1,
			(current.width as isize),
			(current.width as isize) + 1,
		];

		macro_rules! point {
		    ($pos:expr) => {{
		    	let x = $pos % current.width;
				let y = $pos / current.width;
		    	(x, y)
		    }};
		}
		macro_rules! diff {
		    ($left:expr, $right:expr) => {{
		    	let left = $left as isize;
		    	let right = $right as isize;
				if left > right {
					left - right
				} else {
					right - left
				}
		    }};
		}

		for (i, &v) in current.data.iter().enumerate() {
			if v == b'.' {
				continue;
			}

			let (x, y) = point!(i);

			let mut count = 0;
			for &offset in &offsets {
				let pos = i as isize + offset;
				if pos < 0 {
					continue
				}
				let pos = pos as usize;
				if pos >= current.len() {
					continue;
				}

				let (their_x, their_y) = point!(pos);

				if diff!(their_x, x) > 1 || diff!(their_y, y) > 1 {
					continue;
				}

				if current.data[pos] == b'#' {
					count += 1;
				}
			}

			if v == b'L' {
				if count == 0 {
					next.data[i] = b'#';
				}
			} else if v == b'#' {
				if count >= 4 {
					next.data[i] = b'L';
				}
			}
		}
	}

	stabilise_table(tick)
}

fn stabilise_table(tick: impl Fn(&Grid, &mut Grid)) -> usize {
	let mut last = Grid::new();
	let mut current = last.clone();
	tick(&last, &mut current);

	while current != last {
		last.data.as_mut().copy_from_slice(&current.data);
		tick(&last, &mut current)
	}

	let Grid {
		data,
		..
	} = current;

	data.into_vec()
		.into_iter()
		.filter(|&c| c == b'#')
		.count()
}

fn part_two() -> usize {
	fn tick(current: &Grid, next: &mut Grid) {
		let offsets: [isize; 8] = [
			-(current.width as isize) - 1,
			-(current.width as isize),
			-(current.width as isize) + 1,
			-1,
			1,
			(current.width as isize) - 1,
			(current.width as isize),
			(current.width as isize) + 1,
		];

		macro_rules! point {
		    ($pos:expr) => {{
		    	let x = $pos % current.width;
				let y = $pos / current.width;
		    	(x, y)
		    }};
		}
		macro_rules! diff {
		    ($left:expr, $right:expr) => {{
		    	let left = $left as isize;
		    	let right = $right as isize;
				if left > right {
					left - right
				} else {
					right - left
				}
		    }};
		}

		for (i, &v) in current.data.iter().enumerate() {
			if v == b'.' {
				continue;
			}

			let (x, y) = point!(i);

			let mut count = 0;
			for &offset in &offsets {

				let (mut prev_x, mut prev_y) = (x, y);
				let mut pos = i as isize;
				loop {
					pos += offset;
					if pos < 0 {
						break
					}
					let pos = pos as usize;
					if pos >= current.len() {
						break;
					}
					// println!("{}", pos);

					let (their_x, their_y) = point!(pos);

					if diff!(their_x, prev_x) > 1 || diff!(their_y, prev_y) > 1 {
						break;
					}

					prev_x = their_x;
					prev_y = their_y;

					let thing = current.data[pos];
					if thing != b'.' {
						if thing == b'#' {
							count += 1;
						}
						break;
					}
				}
			}

			if v == b'L' {
				if count == 0 {
					next.data[i] = b'#';
				}
			} else if v == b'#' {
				if count >= 5 {
					next.data[i] = b'L';
				}
			}
		}
	}

	stabilise_table(tick)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
	data: Box<[u8]>,
	width: usize,
}

impl Grid {
	fn new() -> Self {
		let input = include_str!("../input/input.txt");

		let width = input.find('\n').unwrap();
		let data = input.lines()
			.collect::<String>()
			.into_bytes()
			.into_boxed_slice();

		Grid {
			data,
			width,
		}
	}

	fn len(&self) -> usize {
		self.data.len()
	}
}
