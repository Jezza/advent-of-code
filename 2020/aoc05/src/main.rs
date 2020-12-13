#![feature(test)]

extern crate test;

macro_rules! measure {
    ($expr:expr) => {{
    	let stats = test::bench::iter(&mut || $expr);
		let median = stats.median as usize;
		let deviation = (stats.max - stats.min) as usize;
		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($expr), median, deviation);
		$expr
    }};
}

fn main() {
	{
		println!("Part One: {}", measure!(part_one()));
		println!("Part Two: {}", measure!(part_two()));
	}

	{
		let input = measure! {{
			let mut input = read_ids().collect::<Vec<usize>>();
			input.sort_unstable();
			input
		}};

		println!("Part One: {}", measure!(part_one_prepared(&input)));
		println!("Part Two: {}", measure!(part_two_prepared(&input)));
	}
}

fn part_one() -> usize {
	read_ids()
		.max()
		.unwrap()
}

fn part_two() -> usize {
	let mut all = read_ids()
		.collect::<Vec<usize>>();

	all.sort_unstable();
	all.windows(2)
		.find(|slice| slice[0] + 1 != slice[1])
		.unwrap()[0] + 1usize
}

fn part_one_prepared(data: &[usize]) -> usize {
	*data.last().unwrap()
}

fn part_two_prepared(data: &[usize]) -> usize {
	data.windows(2)
		.find(|slice| slice[0] + 1 != slice[1])
		.unwrap()[0] + 1usize
}

fn read_ids() -> impl Iterator<Item = usize> {
	include_str!("../input/input.txt")
		.lines()
		.map(|l| l.as_bytes()
			.iter()
			.map(|c| if *c == b'B' || *c == b'R' { 1 } else { 0 })
			.fold(0, |acc, value| (acc << 1) | value))
}
