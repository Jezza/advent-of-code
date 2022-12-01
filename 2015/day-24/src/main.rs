use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 150),
			(INPUT, 2070300),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 0),
			(INPUT, 0),
		],
	);
}

type Package = u32;

fn part_one(input: &str) -> u64 {
    let items = parse!(input, u32);

    let sum = items.iter().sum::<u32>();
    let buckets = 3;
    let bucket_size = sum / buckets;
    println!("{}, {}, {}", sum, buckets, bucket_size);






	0u64
}

fn part_two(input: &str) -> u64 {
	0u64
}
