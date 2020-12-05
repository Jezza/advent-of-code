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
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	println!("Part Two: {}", measure!(part_two_sorted()));
	println!("Part Two: {}", measure!(part_two_unsorted()));
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

fn part_two_sorted() -> usize {
	let mut all = read_ids()
		.collect::<Vec<usize>>();

	all.sort_unstable();

	let min = all.first().cloned().unwrap();
	let max = all.last().cloned().unwrap();

	for id in min..max {
		if !all.contains(&id) {
			return id;
		}
	}

	panic!("Wrong plane, dumbass.")
}

fn part_two_unsorted() -> usize {
	let all = read_ids()
		.collect::<Vec<usize>>();

	let min = *all.iter()
		.min()
		.unwrap();

	let max = *all.iter()
		.max()
		.unwrap();

	for id in min..max {
		if !all.contains(&id) {
			return id;
		}
	}

	panic!("Wrong plane, dumbass.")
}

#[inline(always)]
fn read_ids() -> impl Iterator<Item = usize> {
	read_input_fold()
}

fn read_input_fold() -> impl Iterator<Item = usize> {
	include_str!("../input/input.txt")
		.lines()
		.map(|l| l.as_bytes().iter().fold(0, |acc, half| {
			(acc << 1) | if *half == b'B' || *half == b'R' { 1 } else { 0 }
		}))
}

fn read_input_range() -> impl Iterator<Item = usize> {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| {
			let mut row = 0..128;
			let mut col = 0..8;

			for c in line.as_bytes() {
				match c {
					b'F' => row.end -= row.len() / 2,
					b'B' => row.start += row.len() / 2,
					b'L' => col.end -= col.len() / 2,
					b'R' => col.start += col.len() / 2,
					_ => panic!("Unknown character: {}", c),
				}
			}

			row.start * 8 + col.start
		})
}