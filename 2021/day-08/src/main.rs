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

fn determine_sequence(segments: &str) -> [&str; 10] {
	let (segment_1, segment_4, segment_5) = {
		let mut segment_1 = ' ';
		let mut segment_4 = ' ';
		let mut segment_5 = ' ';

		for (i, frequency) in frequencies(segments).into_iter().enumerate() {
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

	let mut parts = [""; 10];

	let mut segments: Vec<_> = segments.split_ascii_whitespace()
		.collect();

	// Knock out the easy ones.
	segments.retain(|segment| {
		match segment.len() {
			2 => parts[1] = segment,
			4 => parts[4] = segment,
			3 => parts[7] = segment,
			7 => parts[8] = segment,
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
	let segment_2 = parts[1].chars()
		.find(|c| *c != segment_5)
		.unwrap();

	macro_rules! seek {
	    ($num:expr, |$name:ident| $filter:expr) => {{
			let i = segments.iter()
				.position(|$name| $filter)
				.expect(concat!("Unable to locate number ", stringify!($num),"..."));
			parts[$num] = segments.remove(i);
		}};
	}

	seek!(2, |segment| segment.len() == 5 && !segment.contains(segment_5));
	seek!(5, |segment| segment.len() == 5 && segment.contains(segment_1));
	seek!(3, |segment| segment.len() == 5);
	seek!(6, |segment| !segment.contains(segment_2));
	seek!(9, |segment| !segment.contains(segment_4));

	parts[0] = segments.remove(0);

	if !segments.is_empty() {
		panic!("{:?}", segments);
	}

	parts
}

fn part_two(input: &str) -> u64 {
	input.lines()
		.filter_map(|line| line.split_once(" | "))
		.map(|(segments, display)| {
			let segments = determine_sequence(segments);

			// @FIXME Jezza - 08 Dec. 2021: Not ideal...
			//  Short of allocating an intermediate collection to deal with this, it's fine...
			display.rsplit(" ")
				.enumerate()
				.fold(0, |acc, (i, value)| {
					let shift = 10u32.pow(i as u32);

					let value = value.as_bytes();
					let pos = segments.iter()
						.position(|segment| {
							// Technically, this is O(n^3)...
							// I bring shame on my family...
							// In practice, it's not a problem, because they're about short 7 segment displays.
							segment.len() == value.len()
								&& segment.as_bytes().iter().all(|b| value.contains(b))
						})
						.unwrap();

					acc + shift * pos as u32
				}) as u64
		})
		.sum()
}
