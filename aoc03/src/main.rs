
fn main() {
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	follow_steps(&[
		(3, 1)
	])
}

fn part_two() -> usize {
	follow_steps(&[
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2),
	])
}

fn follow_steps(steps: &[(usize, usize)]) -> usize {
	const INPUT: &str = include_str!("../input/input.txt");
	let width = INPUT.find('\n').unwrap();

	steps.iter()
		.fold(1, |acc, step| acc * INPUT.lines()
			.step_by(step.1)
			.zip((0..width)
				.cycle()
				.step_by(step.0))
			.skip(1)
			.filter(|&(line, x_pos)| line.bytes()
				.nth(x_pos)
				.map(|c| c == b'#')
				.unwrap_or(false))
			.count())
}

fn follow_steps_structural(steps: &[(usize, usize)]) -> usize {
	let map = &Map::new();

	steps.iter()
		.fold(1, |acc, step| {
			let mut pos = (0, 0);
			let mut count = 0;

			loop {
				pos.0 += step.0;
				pos.1 += step.1;

				if let Some(value) = map.at(pos) {
					if value == '#' {
						count += 1;
					}
				} else {
					break;
				}
			}

			acc * count
		})
}

struct Map {
	width: usize,
	data: Box<[u8]>,
}

impl Map {
	fn new() -> Self {
		let input = include_str!("../input/input.txt");

		let width = input.find('\n').unwrap();
		let data = input.lines()
			.collect::<String>()
			.into_bytes()
			.into_boxed_slice();

		Map {
			width,
			data,
		}
	}

	fn at(&self, pos: (usize, usize)) -> Option<char> {
		self.data.get((pos.0 % self.width) + pos.1 * self.width)
			.cloned()
			.map(|c| c as char)
	}
}
