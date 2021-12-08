use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 26),
			(INPUT, 381),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 61229),
			(INPUT, 1023686),
		],
	);
}

fn part_one(input: &str) -> usize {
	input.lines()
		.filter_map(|line| Some(line.split_once(" | ")?))
		.flat_map(|(_, right)| right.split_ascii_whitespace())
		.filter(|line| matches!(line.len(), 2 | 4 | 3 | 7))
		.count()
}

fn frequencies(segments: &str) -> [u8; 7] {
	let mut frequencies = [0u8; 7];
	for b in segments.as_bytes() {
		if *b != b' ' {
			frequencies[(b - b'a') as usize] += 1;
		}
	}
	frequencies
}

fn calculate_number(frequencies: [u8; 7], sequence: &str) -> u16 {
	let sum: u16 = sequence.as_bytes()
		.iter()
		.map(|c| *c - b'a')
		.map(|i| frequencies[i as usize] as u16)
		.sum();

	match sum {
		39 => 3,
		41 => 6,
		25 => 7,
		34 => 2,
		17 => 1,
		45 => 9,
		30 => 4,
		42 => 0,
		37 => 5,
		49 => 8,
		_ => panic!("Unknown number: {}", sum),
	}
}

fn calculate_display_output(sequences: &str, display: &str) -> u64 {
	let frequencies = frequencies(sequences);

	display.rsplit(" ")
		.enumerate()
		.map(|(i, sequence)| {
			let num = calculate_number(frequencies, sequence);
			let shift = 10u32.pow(i as u32);
			shift * num as u32
		})
		.sum::<u32>() as u64
}

fn part_two(input: &str) -> u64 {
	input.lines()
		.filter_map(|line| line.split_once(" | "))
		.map(|(sequences, display)| calculate_display_output(sequences, display))
		.sum()
}
