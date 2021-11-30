#![feature(str_split_once)]

use commons::measure;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

fn read_input() -> (usize, impl Iterator<Item = &'static str>) {
	let t = include_str!("../input/input.txt");

	let (start, times) = t.split_once('\n').unwrap();
	let start = start.parse::<usize>().unwrap();

	(start, times.split(','))
}

fn part_one() -> usize {
	let (start, times) = read_input();

	times.filter_map(|v| v.parse::<usize>().ok())
		.map(|id| (id, id - (start % id)))
		.min_by_key(|&(_, val)| val)
		.map(|(id, val)| id * val)
		.unwrap()
}

fn part_two() -> usize {
	let (_, times) = read_input();

	let ids: Vec<_> = times.enumerate()
		.filter_map(|(i, v)| v.parse::<usize>().ok().map(|v| (i, v)))
		.collect();

	let mut time = 0;
	let mut step = 1;

	for (index, id) in ids {
		for time_slot in (time..).step_by(step) {
			if (time_slot + index) % id == 0 {
				time = time_slot;
				step *= id;
				break;
			}
		}
	}

	time
}