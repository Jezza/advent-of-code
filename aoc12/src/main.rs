use helper::measure;
use std::ops::Rem;

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
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

fn part_one() -> usize {
	let commands = include_str!("../input/input.txt")
		.lines()
		.map(|line| {

			let (cmd, value) = line.split_at(1);
			let value = value.parse::<isize>().unwrap();

			match cmd {
				"N" => Command::North(value),
				"S" => Command::South(value),
				"E" => Command::East(value),
				"W" => Command::West(value),
				"L" => Command::Left(value),
				"R" => Command::Right(value),
				"F" => Command::Forward(value),
				_ => panic!("Unsupported command: {:?}", cmd),
			}
		})
		.collect::<Box<[Command]>>();

	#[derive(Debug, Clone)]
	struct Pos {
		x: isize,
		y: isize,
		angle: isize,
	}

	let mut pos = Pos {
		x: 0,
		y: 0,
		angle: 90,
	};

	for cmd in commands.iter() {

		let last = pos.clone();

		match cmd {
			Command::North(value) => pos.y += *value,
			Command::South(value) => pos.y -= *value,
			Command::East(value) => pos.x += *value,
			Command::West(value) => pos.x -= *value,
			Command::Left(value) => pos.angle -= *value,
			Command::Right(value) => pos.angle += *value,
			Command::Forward(value) => {
				let angle = pos.angle.rem_euclid(360);
				match angle {
					0 => pos.y += value,
					90 => pos.x += value,
					180 => pos.y -= value,
					270 => pos.x -= value,
					_ => panic!("Fuck it: {}", angle),
				}
			}
		}
	}

	(pos.x.abs() + pos.y.abs()) as usize
}

fn part_two() -> usize {
	let commands = include_str!("../input/input.txt")
		.lines()
		.map(|line| {

			let (cmd, value) = line.split_at(1);
			let value = value.parse::<isize>().unwrap();

			match cmd {
				"N" => Command::North(value),
				"S" => Command::South(value),
				"E" => Command::East(value),
				"W" => Command::West(value),
				"L" => Command::Left(value),
				"R" => Command::Right(value),
				"F" => Command::Forward(value),
				_ => panic!("Unsupported command: {:?}", cmd),
			}
		})
		.collect::<Box<[Command]>>();

	#[derive(Debug, Clone)]
	struct Pos {
		x: isize,
		y: isize,
	}

	let mut pos = Pos {
		x: 0,
		y: 0,
	};

	let mut waypoint = Pos {
		x: 10,
		y: 1,
	};

	for cmd in commands.iter() {
		let last = pos.clone();

		match cmd {
			Command::North(value) => waypoint.y += *value,
			Command::South(value) => waypoint.y -= *value,
			Command::East(value) => waypoint.x += *value,
			Command::West(value) => waypoint.x -= *value,
			Command::Left(value) => {
				let turns = value / 90;

				for _ in 0..turns {
					let y = waypoint.y;
					waypoint.y = waypoint.x;
					waypoint.x = -y;
				}
			},
			Command::Right(value) => {
				let turns = value / 90;

				for _ in 0..turns {
					let y = waypoint.y;
					waypoint.y = -waypoint.x;
					waypoint.x = y;
				}
			},
			Command::Forward(value) => {
				pos.x += waypoint.x * value;
				pos.y += waypoint.y * value;
			}
		}

		// println!("{:?} => {:#?} => {:#?}", cmd, last, pos);
		// println!("{:?}", waypoint);
	}

	(pos.x.abs() + pos.y.abs()) as usize
}
