use commons::measure;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

#[derive(Debug)]
enum Command {
	North(isize),
	South(isize),
	East(isize),
	West(isize),
	Left(isize),
	Right(isize),
	Forward(isize),
}

fn parse_input() -> impl Iterator<Item = Command> {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| line.split_at(1))
		.map(|(cmd, value)| (cmd, value.parse::<isize>().unwrap()))
		.map(|(cmd, value)| match cmd {
			"N" => Command::North(value),
			"S" => Command::South(value),
			"E" => Command::East(value),
			"W" => Command::West(value),
			"L" => Command::Left(value),
			"R" => Command::Right(value),
			"F" => Command::Forward(value),
			_ => panic!("Unsupported command: {:?}", cmd),
		})
}

fn part_one() -> usize {
	let (mut x, mut y, mut angle) = (0, 0, 90isize);

	for cmd in parse_input() {
		match (cmd, angle) {
			(Command::North(value), _) | (Command::Forward(value), 0) => y += value,
			(Command::South(value), _) | (Command::Forward(value), 180) => y -= value,
			(Command::East(value), _) | (Command::Forward(value), 90) => x += value,
			(Command::West(value), _) | (Command::Forward(value), 270) => x -= value,
			(Command::Left(value), _) => angle = (angle - value).rem_euclid(360),
			(Command::Right(value), _) => angle = (angle + value).rem_euclid(360),
			_ => unreachable!(),
		}
	}

	(x.abs() + y.abs()) as usize
}

fn part_two() -> usize {
	let (mut ship_x, mut ship_y) = (0, 0);
	let (mut waypoint_x, mut waypoint_y) = (10, 1);

	for cmd in parse_input() {
		match cmd {
			Command::North(value) => waypoint_y += value,
			Command::South(value) => waypoint_y -= value,
			Command::East(value) => waypoint_x += value,
			Command::West(value) => waypoint_x -= value,
			Command::Left(value) => for _ in 0..(value / 90) {
				std::mem::swap(&mut waypoint_x, &mut waypoint_y);
				waypoint_x = -waypoint_x
			},
			Command::Right(value) => for _ in 0..(value / 90) {
				std::mem::swap(&mut waypoint_x, &mut waypoint_y);
				waypoint_y = -waypoint_y;
			},
			Command::Forward(value) => {
				ship_x += waypoint_x * value;
				ship_y += waypoint_y * value;
			}
		}
	}

	(ship_x.abs() + ship_y.abs()) as usize
}
