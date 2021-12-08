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

fn determine_sequences(sequences: &str) -> [&str; 10] {
	/*
	Various segments:

	  00000
	 1     2
	 1     2
	 1     2
	  33333
	 4     5
	 4     5
	 4     5
	  66666
	 */
	let (segment_1, segment_4, segment_5) = {
		let mut segment_1 = ' ';
		let mut segment_4 = ' ';
		let mut segment_5 = ' ';

		for (i, frequency) in frequencies(sequences).into_iter().enumerate() {
			match frequency {
				6 => segment_1 = (b'a' + i as u8) as char,
				4 => segment_4 = (b'a' + i as u8) as char,
				9 => segment_5 = (b'a' + i as u8) as char,
				// Segment 3 and 6
				7 => {}
				// Segment 0 and 2
				8 => {}
				_ => (),
			}
		}

		(segment_1, segment_4, segment_5)
	};

	// num -> sequence
	let mut known_sequences = [""; 10];

	let mut sequences: Vec<_> = sequences.split_ascii_whitespace()
		.collect();

	// Knock out the easy ones.
	sequences.retain(|sequence| {
		match sequence.len() {
			2 => known_sequences[1] = sequence,
			4 => known_sequences[4] = sequence,
			3 => known_sequences[7] = sequence,
			7 => known_sequences[8] = sequence,
			// 5 => 2, 3, 5
			// 6 => 0, 6, 9
			5 | 6 => {
				return true
			},
			l => panic!("Unknown length: {}", l),
		};

		false
	});

	// 1 is built from two segments (2 and 5), we know 5 from the frequencies, so we can deduce 2.
	let segment_2 = known_sequences[1].chars()
		.find(|c| *c != segment_5)
		.unwrap();

	macro_rules! seek {
	    ($num:expr, |$name:ident| $filter:expr) => {{
			let i = sequences.iter()
				.position(|$name| $filter)
				.expect(concat!("Unable to locate number ", stringify!($num),"..."));
			known_sequences[$num] = sequences.remove(i);
		}};
	}

	seek!(2, |sequence| sequence.len() == 5 && !sequence.contains(segment_5));
	seek!(5, |sequence| sequence.len() == 5 && sequence.contains(segment_1));
	seek!(3, |sequence| sequence.len() == 5);
	seek!(6, |sequence| !sequence.contains(segment_2));
	seek!(9, |sequence| !sequence.contains(segment_4));

	known_sequences[0] = sequences.remove(0);

	if !sequences.is_empty() {
		panic!("{:?}", sequences);
	}

	known_sequences
}

fn part_two(input: &str) -> u64 {
	input.lines()
		.filter_map(|line| line.split_once(" | "))
		.map(|(segments, display)| {
			let sequences = determine_sequences(segments);

			// @FIXME Jezza - 08 Dec. 2021: Not ideal...
			//  Short of allocating an intermediate collection to deal with this, it's fine...
			display.rsplit(" ")
				.enumerate()
				.fold(0, |acc, (i, value)| {
					let shift = 10u32.pow(i as u32);

					let value = value.as_bytes();
					let pos = sequences.iter()
						.position(|sequence| {
							// I bring shame on my family...
							// In practice, it's not a problem, because they're about short 7 segment displays.
							sequence.len() == value.len()
								&& sequence.as_bytes().iter().all(|b| value.contains(b))
						})
						.unwrap();

					acc + shift * pos as u32
				}) as u64
		})
		.sum()
}
