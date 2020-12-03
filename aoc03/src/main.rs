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

fn main() {
	let map = &Map::new();

	println!("Part One: {}", part_one(map));
	println!("Part Two: {}", part_two(map));
}

fn part_one(map: &Map) -> usize {
	const STEP: (usize, usize) = (3, 1);

	follow_steps(map, STEP)
}

fn part_two(map: &Map) -> usize {
	vec![
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2),
	].into_iter()
		.map(|step| follow_steps(map, step))
		.product()
}

fn follow_steps(map: &Map, step: (usize, usize)) -> usize {
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

	count
}