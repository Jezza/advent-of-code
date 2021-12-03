use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(
		part_one,
		vec![
			(TEST_1, 150),
			(INPUT, 2070300),
		],
	);
	aoc(
		part_two,
		vec![
			(TEST_1, 900),
			(INPUT, 2078985210),
		],
	);
	aoc(
		part_one_unified,
		vec![
			(TEST_1, 150),
			(INPUT, 2070300),
		],
	);
	aoc(
		part_two_unified,
		vec![
			(TEST_1, 900),
			(INPUT, 2078985210),
		],
	);
}

#[derive(Debug)]
enum Command {
	Forward(u32),
	Down(u32),
	Up(u32),
}

impl Command {
	fn from(command: &str) -> Option<Command> {
		let (command, value) = command.split_once(" ")?;
		let value = value.parse().ok()?;
		let command = match command {
			"forward" => Command::Forward(value),
			"down" => Command::Down(value),
			"up" => Command::Up(value),
			_ => panic!("Command: {}", command),
		};
		Some(command)
	}
}

#[inline(always)]
fn read_input(input: &str) -> impl Iterator<Item = Command> + '_ {
	input.lines()
		.filter_map(Command::from)
}

fn part_one(input: &str) -> u32 {
	let (x, y) = read_input(input)
		.fold((0, 0), |(mut x, mut y), command| {
			match command {
				Command::Forward(value) => x += value,
				Command::Up(value) => y -= value,
				Command::Down(value) => y += value,
			}
			(x, y)
		});

	x * y
}

fn part_two(input: &str) -> u32 {
	let (_, x, y) = read_input(input)
		.fold((0, 0, 0), |(mut aim, mut x, mut y), command| {
			match command {
				Command::Forward(value) => {
					x += value;
					y += aim * value;
				},
				Command::Up(value) => aim -= value,
				Command::Down(value) => aim += value,
			}

			(aim, x, y)
		});

	x * y
}

#[derive(Debug)]
struct Position {
	aim: i32,
	x: i32,
	y: i32,
}

impl Position {
	fn from(command: &str) -> Option<Position> {
		let (command, value) = command.split_once(" ")?;
		let value = value.parse()
			.ok()?;
		let pos = match command {
			"forward" => Position {
				aim: 0,
				x: value,
				y: 0,
			},
			"down" => Position {
				aim: 0,
				x: 0,
				y: value,
			},
			"up" => Position {
				aim: 0,
				x: 0,
				y: -value,
			},
			_ => panic!("Command: {}", command),
		};
		Some(pos)
	}
}

fn part_one_unified(input: &str) -> i32 {
	let pos = input.lines()
		.filter_map(Position::from)
		.reduce(|mut acc, item| {
			acc.aim += item.aim;
			acc.x += item.x;
			acc.y += item.y;
			acc
		})
		.unwrap();

	pos.x * pos.y
}

fn part_two_unified(input: &str) -> i32 {
	let pos = input.lines()
		.filter_map(Position::from)
		.reduce(|mut acc, item| {
			acc.aim += item.y;
			acc.x += item.x;
			acc.y += acc.aim * item.x;
			acc
		})
		.unwrap();

	pos.x * pos.y
}
