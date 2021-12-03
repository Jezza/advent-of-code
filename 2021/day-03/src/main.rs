use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 198),
			(INPUT, 4191876),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 230),
			(INPUT, 3414905),
		],
	);
}

fn read_input(input: &str) -> (Vec<u16>, u16, u16) {
	let values: Vec<_> = input.lines()
		.filter_map(|line| u16::from_str_radix(line, 2).ok())
		.collect();

	let bits = input.lines()
		.map(|line| line.len())
		.max()
		.unwrap() as u16;

	let mask: u16 = (0..bits).into_iter()
		.map(|bit| 1 << bit)
		.sum();

	(values, bits, mask)
}

fn generate_report_bit(
	bit: u16,
	input: &[u16],
) -> bool {
	let (zeros, ones) = input.iter()
		.map(|value| *value & (1 << bit) == 0)
		.fold((0, 0), |(zeros, ones), value| {
			if value {
				(zeros + 1, ones)
			} else {
				(zeros, ones + 1)
			}
		});

	zeros <= ones
}

fn part_one(input: &str) -> u32 {
	let (values, bits, mask) = read_input(input);

	let report: u16 = (0..bits).rev()
		.map(|bit| {
			if generate_report_bit(bit, &values) {
				1 << bit
			} else {
				0
			}
		})
		.sum();

	let epsilon = !report & mask;

	report as u32 * epsilon as u32
}

fn part_two(input: &str) -> u32 {
	let (values, bits, _) = read_input(input);

	let generator = |signal: bool| {
		let mut values = values.clone();

		let signal = if signal { 1 } else { 0 };

		for bit in (0..bits).rev() {
			let enabled = generate_report_bit(bit, &values);

			values.retain(|value| ((value >> bit) & 1 == signal) == enabled);

			if values.len() <= 1 {
				break;
			}
		}

		values[0]
	};

	let oxygen = generator(true);
	let co2 = generator(false);

	oxygen as u32 * co2 as u32
}
